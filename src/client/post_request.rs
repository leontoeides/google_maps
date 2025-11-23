use backon::Retryable;
use crate::{Error, traits::ClassifiableError};
use reqwest::header::HeaderMap;

// -------------------------------------------------------------------------------------------------

impl crate::Client {
    /// Performs the HTTP POST request and returns the response to the caller.
    ///
    /// ## Arguments
    ///
    /// * `request` - This request will be converted into a URL query string, a (presumably JSON)
    ///   request body, and forwarded to Google Maps.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    ///
    /// * The request `struct` fails validation. For example, parameters in the request conflict
    ///   with one another, or the request parameters are set in a way that's incompatible.
    ///
    ///   For example, Google Maps Address Validation API will refuse a `PostalAddress` greater than
    ///   280 characters. This will cause a validation failure.
    ///
    /// * The HTTP client cannot make a connection to the Google Maps API server, or successfully
    ///   send the request or receive the response over the network.
    ///
    /// * The Google Maps API server returns an unexpected response, or data in a format that's not
    ///   expected.
    #[cfg_attr(feature = "tracing-instrumentation", tracing::instrument(
        level = "debug",
        skip(self, request),
        fields(
            endpoint = %REQ::title(),
        ),
        err
    ))]
    pub(crate) async fn post_request<REQ, RSP, ERR>(
        &self,
        request: REQ
    ) -> Result<RSP, Error>
    where
        REQ: std::fmt::Debug
            + crate::traits::EndPoint
            + crate::traits::QueryUrl
            + crate::traits::RequestBody
            + crate::traits::RequestHeaders
            + Send,
        RSP: serde::de::DeserializeOwned + Into<Result<RSP, ERR>>,
        ERR: std::fmt::Display + From<ERR> + Into<Error>
    {
        // Build the URL and query string
        let url = request
            .query_url()
            .inspect_err(|error| tracing::error!(error = %error, "failed to build request URL"))?
            .trim_matches('?')
            .to_string();

        tracing::info!(url = %url, "POST request");

        // Build the request body
        let body = request
            .request_body()
            .inspect_err(|error| tracing::error!(error = %error, "failed to build request body"))?;

        // Get and set custom headers
        let mut headers = request.request_headers(); // Request-specific headers

        if REQ::send_x_goog_api_key() { // For requests that require API key in headers
            let mut api_key = reqwest::header::HeaderValue::from_str(&self.key)
                .map_err(|_error| Error::InvalidHeaderValue {
                    header_name: "X-Goog-Api-Key".to_string()
                })?;

            api_key.set_sensitive(true);

            headers.insert("X-Goog-Api-Key", api_key);
        }

        // Observe any rate limiting before executing request:
        self
            .rate_limit
            .limit_apis(REQ::apis())
            .await;

        // Build the HTTP request function with retry logic
        let http_requestor = || async {
            self.execute_post_request(&url, &body, &headers).await
        };

        // Perform the HTTP request with exponential backoff retry
        let response = http_requestor
            .retry(backon::ExponentialBuilder::default())
            .when(|e: &Error| e.classify().is_transient())
            .notify(|err, dur: std::time::Duration| {
                tracing::warn!(
                    error = %err,
                    retry_after = ?dur,
                    "retrying request after error"
                );
            })
            .await?;

        tracing::debug!("request completed successfully");
        Ok(response)
    }

    /// Executes a single POST request attempt.
    ///
    /// Performs the HTTP POST, checks the response status, deserializes the response body, and
    /// converts any API errors.
    ///
    /// This is called by the retry logic in `post_request`.
    #[tracing::instrument(
        level = "trace",
        skip(self, body, headers),
        fields(
            http.method = "POST",
            http.url = %url,
            http.headers_count = headers.len(),
        ),
        err
    )]
    async fn execute_post_request<RSP, ERR>(
        &self,
        url: &str,
        body: &str,
        headers: &HeaderMap,
    ) -> Result<RSP, Error>
    where
        RSP: serde::de::DeserializeOwned + Into<Result<RSP, ERR>>,
        ERR: std::fmt::Display + Into<Error>
    {
        // Build the request with custom headers
        let request = self
            .reqwest_client
            .post(url)
            .body(body.to_string())
            .headers(headers.clone())
            .build()
            .inspect_err(|error| {
                tracing::error!(
                    error = %error,
                    "failed to build HTTP request"
                );
            })
            .map_err(|e| Error::from(crate::ReqError::from(e)))?;

        // Execute the request
        let response = self
            .reqwest_client
            .execute(request)
            .await
            .inspect_err(|error| tracing::error!(error = %error, "HTTP request failed"))
            .map_err(Error::from)?;

        // Check status
        let status = response.status();

        if !status.is_success() {
            // Try to get the response body for error details
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "failed to read response body".to_string());

            let google_api_error: crate::places_new::GoogleApiError =
                serde_json::from_str(&error_body)?;

            tracing::error!(
                http.status_code = %status.as_u16(),
                http.status_text = %status.canonical_reason().unwrap_or("unknown"),
                response_body = %google_api_error,
                "request returned non-success status"
            );

            return Err(Error::from(crate::places_new::Error::from(google_api_error)));
        }

        tracing::trace!(http.status_code = %status.as_u16(), "received successful response");

        // Get response body
        let bytes = response
            .text()
            .await
            .map(String::into_bytes)
            .inspect_err(|error| tracing::error!(error = %error, "failed to read response body"))
            .map_err(Error::from)?;

        tracing::trace!(body_size = bytes.len(), "received response body");

        // Deserialize JSON
        let deserialized: RSP = serde_json::from_slice(&bytes)
            .inspect_err(|error| {
                tracing::error!(
                    error = %error,
                    "failed to deserialize JSON response"
                );
                if let Ok(text) = String::from_utf8(bytes.clone()) {
                    tracing::trace!(response_body = %text, "raw response body");
                }
            })
            .inspect_err(|_| tracing::error!(
                "error serializing from JSON:\n{}",
                String::from_utf8(bytes).unwrap_or_default()))
            .map_err(Error::from)?;

        // Convert to final result (handles API-level errors)
        let result: Result<RSP, ERR> = deserialized.into();
        result
            .inspect_err(|error| {
                tracing::error!(
                    api_error = %error,
                    "API returned error response"
                );
            })
            .map_err(Into::into)
    }
}