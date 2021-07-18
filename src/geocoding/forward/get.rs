use backoff::Error::{Permanent, Transient};
use backoff::ExponentialBackoff;
use backoff::future::retry;
use crate::geocoding::{
    error::Error,
    forward::ForwardRequest,
    response::Response,
    response::status::Status,
}; // use crate::geocoding
use crate::request_rate::api::Api;
use tracing_futures::Instrument;

impl<'a> ForwardRequest<'a> {

    /// Performs the HTTP get request and returns the response to the caller.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub async fn get(&mut self) -> Result<Response, Error> {

        // Build the URI stem for the HTTP get request:

        const SERVICE_URI: &str = "https://maps.googleapis.com/maps/api/geocode";
        const OUTPUT_FORMAT: &str = "json"; // json or xml
        let mut uri = format!("{}/{}?", SERVICE_URI, OUTPUT_FORMAT);

        match &self.query {
            // If query string built, append it to the URL stem.
            Some(query) => uri.push_str(query.as_ref()),
            // If query string not built, return an error.
            None => return Err(Error::QueryNotBuilt),
        } // match

        // Enter a tracing (logging) span. Span is closed when function ends:
        let geocoding_span = tracing::info_span!(
            "Querying Google Maps Geocoding API",
            query_string = %self.query.as_ref().unwrap()
        ); // info_span!
        let _geocoding_span_guard = geocoding_span.enter();

        // Observe any rate limiting before executing request:
        let rate_span = tracing::trace_span!("Observing rate limit before executing query");
        self.client_settings.rate_limit.limit_apis(vec![&Api::All, &Api::Geocoding])
            .instrument(rate_span)
            .await;

        // Retries the get request until successful, an error ineligible for
        // retries is returned, or we have reached the maximum retries. Note:
        // errors wrapped in `Transient()` will retried by the `backoff` crate
        // while errors wrapped in `Permanent()` will exit the retry loop.
        let backoff_span = tracing::trace_span!("Trying query");
        retry(ExponentialBackoff::default(), || async {

            // Query the Google Cloud Maps Platform using using an HTTP get
            // request, and return result to caller:
            let response: Result<reqwest::Response, reqwest::Error> =
                match &self.client_settings.reqwest_client {
                    Some(reqwest_client) =>
                        match reqwest_client.get(&*uri).build() {
                            Ok(request) => reqwest_client.execute(request).await,
                            Err(error) => Err(error),
                        }, // Some
                    None => reqwest::get(&*uri).await,
                }; // match

            // Check response from the HTTP client:
            match response {
                Ok(response) => {
                    // HTTP client was successful getting a response from the
                    // server. Check the HTTP status code:
                    if response.status().is_success() {
                        // If the HTTP GET request was successful, get the
                        // response text:
                        let text = &response.text().await;
                        match text {
                            Ok(text) => {
                                match serde_json::from_str::<Response>(text) {
                                    Ok(deserialized) => {
                                        // If the response JSON was successfully
                                        // parsed, check the Google API status
                                        // before returning it to the caller:
                                        if deserialized.status == Status::Ok {
                                            // If Google's response was "Ok"
                                            // return the struct deserialized
                                            // from JSON:
                                            Ok(deserialized)
                                        // Google API returned an error. This
                                        // indicates an issue with the request.
                                        // In most cases, retrying will not
                                        // help:
                                        } else {
                                            let error = Error::GoogleMapsService(
                                                deserialized.status.to_owned(),
                                                deserialized.error_message,
                                            );
                                            // Check Google API response status
                                            // for error type:
                                            if deserialized.status == Status::UnknownError {
                                                // Only Google's "Unknown Error"
                                                // is eligible for retries:
                                                tracing::warn!("Google Maps API returned: {}", error);
                                                Err(Transient(error))
                                            } else {
                                                // Not an "Unknown Error." The
                                                // error is permanent, do not
                                                // retry:
                                                tracing::error!("Google Maps API returned: {}", error);
                                                Err(Permanent(error))
                                            } // if
                                        } // if
                                    }, // Ok(deserialized)
                                    Err(error) => {
                                        tracing::error!("JSON parsing error: {}", error);
                                        Err(Permanent(Error::SerdeJson(error)))
                                    }, // Err
                                } // match
                            }, // Ok(text)
                            Err(error) => {
                                tracing::error!("HTTP client returned: {}", error);
                                Err(Permanent(Error::ReqwestMessage(error.to_string())))
                            }, // Err
                        } // match
                    // We got a response from the server but it was not OK.
                    // Only HTTP "500 Server Errors", and HTTP "429 Too Many
                    // Requests" are eligible for retries.
                    } else if response.status().is_server_error() || response.status() == 429 {
                        tracing::warn!("HTTP client returned: {}", response.status());
                        Err(Transient(Error::HttpUnsuccessful(response.status().to_string())))
                    // Not a 500 Server Error or "429 Too Many Requests" error.
                    // The error is permanent, do not retry:
                    } else {
                        tracing::error!("HTTP client returned: {}", response.status());
                        Err(Permanent(Error::HttpUnsuccessful(response.status().to_string())))
                    } // if
                } // case
                // HTTP client did not get a response from the server. Retry:
                Err(error) => {
                    tracing::warn!("HTTP client returned: {}", error);
                    Err(Transient(Error::Reqwest(error)))
                } // case
            } // match

        }).instrument(backoff_span).await

    } // fn

} // impl