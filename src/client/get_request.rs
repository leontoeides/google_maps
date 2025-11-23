use backon::Retryable;
use crate::{Error, traits::ClassifiableError};

// -----------------------------------------------------------------------------

impl crate::Client {
    /// Performs the HTTP get request and returns the response to the caller.
    ///
    /// ## Arguments
    ///
    /// * `request` · This request will be converted into a URL query string and
    ///   forwarded to Google Maps.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    ///
    /// * This can fail if the request `struct` fails validation. For example,
    ///   parameters in the request conflict with one another, or the request
    ///   parameters are set in a way that's incompatible.
    ///
    ///   For example, Google Maps Directions API cannot calculate alternative
    ///   routes if waypoints have been set. This will cause a validation
    ///   failure.
    ///
    /// * The HTTP client cannot make a connection to the Google Maps API
    ///   server, or successfully send the request or receive the resposne over
    ///   the network.
    ///
    /// * The Google Maps API server returns an unexpected response, or data in
    ///   a format that's not expected.
    #[cfg_attr(feature = "tracing-instrumentation", tracing::instrument(
        level = "debug",
        skip(self, request),
        fields(
            endpoint = %REQ::title(),
        ),
        err
    ))]
    pub(crate) async fn get_request<REQ, RSP, ERR>(
        &self,
        request: REQ
    ) -> Result<RSP, Error>
    where
        REQ: std::fmt::Debug
            + crate::traits::EndPoint
            + crate::traits::QueryUrl
            + crate::traits::RequestHeaders
            + Send,
        RSP: serde::de::DeserializeOwned + Into<Result<RSP, ERR>>,
        ERR: std::fmt::Display + From::<ERR> + Into<Error>
    {
        // Build the URL and query string
        // Attempt to build the URL and query string for the HTTP `GET` request:
        let url: String = request
            .query_url()
            .inspect_err(|error| tracing::error!(error = %error, "failed to build request URL"))?
            .trim_matches('?')
            .to_string();

        tracing::info!(url = %url, "GET request");

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

        // Build an async function that will be used to perform the HTTP
        // request, deserialize the response, and anaylze the results. This
        // function will be called by the `backon` crate, which will handle
        // exponential back-offs:
        let http_requestor = || async {
            match self.reqwest_client.get(url.clone()).headers(headers.clone()).build() {
                // Attempt to build a `GET` request for the `reqwest` client
                // using the URL and query string:
                Ok(request) => match self.reqwest_client.execute(request).await {
                    Ok(response) => if response.status().is_success() {
                        match response.text().await.map(String::into_bytes) {
                            Ok(bytes) => match serde_json::from_slice::<RSP>(&bytes) {
                                Ok(deserialized) => {
                                    let result: Result<RSP, ERR> = deserialized.into();
                                    if let Err(error) = &result {
                                        tracing::error!(error = %error, "API error");
                                    } // if
                                    result.map_err(Into::into)
                                },
                                Err(error) => {
                                    tracing::error!(error = %error, "JSON deserialization error");
                                    if let Ok(text) = String::from_utf8(bytes) {
                                        tracing::trace!("{text}");
                                    }
                                    Err(Error::from(error))
                                },
                            }, // Ok
                            Err(error) => {
                                tracing::error!(error = %error, "HTTP request error");
                                Err(Error::from(error))
                            },
                        } // match
                    } else {
                        let status = response.status();

                        tracing::error!(
                            http.status_code = %status.as_u16(),
                            http.status_text = %status.canonical_reason().unwrap_or("unknown"),
                            "request returned non-success status"
                        );

                        Err(Error::from(response.status()))
                    }, // Ok
                    Err(error) => {
                        tracing::error!(error = %error, "HTTP request error");
                        Err(Error::from(error))
                    },
                }, // match
                Err(error) => {
                    tracing::error!(error = %error, "HTTP request error");
                    Err(Error::from(crate::ReqError::from(error)))
                },
            } // match
        }; // async function closure

        // Perform the HTTP request. Retry on when error is possibly transient,
        // according to the default `backon` policy:
        let response = http_requestor
            .retry(backon::ExponentialBuilder::default())
            .when(|e: &Error| e.classify().is_transient())
            .notify(|err, dur: std::time::Duration| {
                tracing::warn!("error {} retrying after {:?}", err, dur);
            }).await?;

        // Return the response to the caller:
        Ok(response)
    } // fn
} // impl