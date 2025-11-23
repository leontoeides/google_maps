use backon::Retryable;
use crate::{Error, traits::ClassifiableError};

// -----------------------------------------------------------------------------

impl crate::Client {
    /// Performs an HTTP GET request and returns raw binary response data.
    ///
    /// Similar to `get_request` but returns raw bytes instead of deserializing JSON. Use this for
    /// endpoints that return binary data like images, PDFs, or other non-JSON content.
    ///
    /// ## Arguments
    ///
    /// * `request` · This request will be converted into a URL and forwarded to Google Maps.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    ///
    /// * The request `struct` fails validation.
    /// * The HTTP client cannot make a connection to the Google Maps API server.
    /// * The Google Maps API server returns a non-success status code.
    #[cfg_attr(feature = "tracing-instrumentation", tracing::instrument(
        level = "debug",
        skip(self, request),
        fields(
            endpoint = %REQ::title(),
        ),
        err
    ))]
    pub(crate) async fn get_binary_request<REQ>(
        &self,
        request: REQ
    ) -> Result<Vec<u8>, Error>
    where
        REQ: std::fmt::Debug
            + crate::traits::EndPoint
            + crate::traits::QueryUrl
            + crate::traits::RequestHeaders
            + Send,
    {
        // Build the URL and query string
        let url: String = request
            .query_url()
            .inspect_err(|error| tracing::error!(error = %error, "failed to build request URL"))?
            .trim_matches('?')
            .to_string();

        tracing::info!(url = %url, "GET request");

        // Get and set custom headers
        let mut headers = request.request_headers();

        if REQ::send_x_goog_api_key() {
            let mut api_key = reqwest::header::HeaderValue::from_str(&self.key)
                .map_err(|_error| Error::InvalidHeaderValue {
                    header_name: "X-Goog-Api-Key".to_string()
                })?;

            api_key.set_sensitive(true);
            headers.insert("X-Goog-Api-Key", api_key);
        }

        // Observe any rate limiting before executing request
        self
            .rate_limit
            .limit_apis(REQ::apis())
            .await;

        // Build an async function for the HTTP request with retry logic
        let http_requestor = || async {
            match self.reqwest_client.get(url.clone()).headers(headers.clone()).build() {
                Ok(request) => match self.reqwest_client.execute(request).await {
                    Ok(response) => if response.status().is_success() {
                        match response.bytes().await {
                            Ok(bytes) => {
                                tracing::debug!(size_bytes = bytes.len(), "received binary response");
                                Ok(bytes.to_vec())
                            },
                            Err(error) => {
                                tracing::error!(error = %error, "failed to read response bytes");
                                Err(Error::from(error))
                            },
                        }
                    } else {
                        let status = response.status();

                        tracing::error!(
                            http.status_code = %status.as_u16(),
                            http.status_text = %status.canonical_reason().unwrap_or("unknown"),
                            "request returned non-success status"
                        );

                        Err(Error::from(response.status()))
                    },
                    Err(error) => {
                        tracing::error!(error = %error, "HTTP request error");
                        Err(Error::from(error))
                    },
                },
                Err(error) => {
                    tracing::error!(error = %error, "failed to build HTTP request");
                    Err(Error::from(crate::ReqError::from(error)))
                },
            }
        };

        // Perform the HTTP request with retry logic
        let response = http_requestor
            .retry(backon::ExponentialBuilder::default())
            .when(|e: &Error| e.classify().is_transient())
            .notify(|err, dur: std::time::Duration| {
                tracing::warn!("error {} retrying after {:?}", err, dur);
            }).await?;

        Ok(response)
    }
}