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
    #[tracing::instrument(level = "info")]
    pub(crate) async fn get_request<REQ, RSP, ERR>(
        &self,
        request: REQ
    ) -> Result<RSP, Error>
    where
        REQ: std::fmt::Debug + crate::traits::EndPoint + crate::traits::QueryUrl + Send,
        RSP: serde::de::DeserializeOwned + Into<Result<RSP, ERR>>,
        ERR: std::fmt::Display + From::<ERR> + Into<Error>
    {
        // Tracing output:
        tracing::info!("making HTTP GET request to {}", REQ::title());

        // Attempt to build the URL and query string for the HTTP `GET` request:
        let url: String = request
            .query_url()
            .inspect_err(|error| tracing::error!(
                "could not build HTTP request URL: {error}"
            ))?;

        // Tracing output:
        tracing::debug!("{url}");

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
            match self.reqwest_client.get(url.clone()).build() {
                // Attempt to build a `GET` request for the `reqwest` client
                // using the URL and query string:
                Ok(request) => match self.reqwest_client.execute(request).await {
                    Ok(response) => if response.status().is_success() {
                        match response.text().await.map(String::into_bytes) {
                            Ok(bytes) => match serde_json::from_slice::<RSP>(&bytes) {
                                Ok(deserialized) => {
                                    let result: Result<RSP, ERR> = deserialized.into();
                                    if let Err(error) = &result {
                                        tracing::error!("API error: {error}");
                                    } // if
                                    result.map_err(Into::into)
                                },
                                Err(error) => {
                                    tracing::error!("JSON deserialization error: {error}");
                                    if let Ok(text) = String::from_utf8(bytes) {
                                        tracing::trace!("{text}");
                                    }
                                    Err(Error::from(error))
                                },
                            }, // Ok
                            Err(error) => {
                                tracing::error!("HTTP request error: {error}");
                                Err(Error::from(error))
                            },
                        } // match
                    } else {
                        tracing::error!(
                            "Google Maps API HTTP request unsuccessful: {status}",
                            status = response.status(),
                        );
                        Err(Error::from(response.status()))
                    }, // Ok
                    Err(error) => {
                        tracing::error!("HTTP request error: {error}");
                        Err(Error::from(error))
                    },
                }, // match
                Err(error) => {
                    tracing::error!("HTTP request error: {error}");
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