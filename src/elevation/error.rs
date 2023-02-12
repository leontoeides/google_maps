//! Elevation API error types and error messages.

use crate::elevation::response::status::Status;

/// Errors that may be produced by the Google Maps Elevation API client.
#[derive(Debug)]
pub enum Error {
    /// A sampled_path_request() method cannot be used when postional_request()
    /// has been set.
    EitherPositionalOrSampledPath,
    /// Google Maps Elevation API server generated an error. See the `Status`
    /// enum for more information.
    GoogleMapsService(Status, Option<String>),
    /// The HTTP request was unsuccessful.
    HttpUnsuccessful(String),
    /// API client library attempted to parse a string that contained an invalid
    /// status code. See `google_maps\src\elevation\response\status.rs` for
    /// more information.
    InvalidStatusCode(String),
    /// The query string must be built before the request may be sent to the
    /// Google Maps Elevation API server.
    QueryNotBuilt,
    /// The request must be validated before a query string may be built.
    RequestNotValidated,
    /// The dependency library Reqwest generated an error.
    #[cfg(feature = "enable-reqwest")]
    Reqwest(reqwest::Error),
    /// The dependency library Reqwest generated an error. The error could
    /// not be passed normally so a `String` representation is passed instead.
    #[cfg(feature = "enable-reqwest")]
    ReqwestMessage(String),
    /// The dependency library Serde JSON generated an error.
    SerdeJson(serde_json::error::Error),
} // enum

impl std::fmt::Display for Error {
    /// This trait converts the error code into a format that may be presented
    /// to the user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::EitherPositionalOrSampledPath => write!(f,
                "Google Maps Elevation API client: \
                A for_sampled_path_request() method cannot be used when for_postional_request() has been set. \
                Try again with only a positional request or only a sampled path request."),
            Error::GoogleMapsService(status, error_message) => match error_message {
                // If the Google Maps Elevation API server generated an error
                // message, return that:
                Some(error_message) => write!(f, "Google Maps Elevation API service: {error_message}"),
                // If the Google Maps Elevation API server did not generate an
                // error message, return a generic message derived from the
                // response status:
                None => match status {
                    Status::InvalidRequest => write!(f, "Google Maps Elevation API service: \
                        Invalid request. \
                        The request was malformed."),
                    Status::Ok => write!(f, "Google Maps Elevation service: \
                        Ok. \
                        The request was successful."),
                    Status::OverDailyLimit => write!(f, "Google Maps Elevation API service: \
                        Over daily limit. \
                        Usage cap has been exceeded, API key is invalid, billing has not been enabled, or method of payment is no longer valid."),
                    Status::OverQueryLimit => write!(f, "Google Maps Elevation API service: \
                        Over query limit. \
                        Requestor has exceeded quota."),
                    Status::RequestDenied => write!(f, "Google Maps Elevation API service: \
                        Request denied \
                        Service did not complete the request."),
                    Status::UnknownError => write!(f, "Google Maps Elevation API service: \
                        Unknown error."),
                } // match
            }, // match
            Error::HttpUnsuccessful(status) => write!(f,
                "Google Maps Elevation API client: \
                Could not successfully query the Google Cloud Platform service. \
                The service last responded with a `{status}` status."),
            Error::InvalidStatusCode(status_code) => write!(f,
                "Google Maps Elevation API client: \
                `{status_code}` is not a valid status code. \
                Valid codes are `INVALID_REQUEST`, `OK`, `OVER_DAILY_LIMIT`, \
                `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, and `UNKNOWN_ERROR`."
                ),
            Error::RequestNotValidated => write!(f,
                "Google Maps Elevation API client: \
                The request must be validated before a query string may be built. \
                Ensure the validate() method is called before build()."),
            #[cfg(feature = "enable-reqwest")]
            Error::Reqwest(error) => write!(f, "Google Maps Elevation API client in the Reqwest library: {error}"),
            #[cfg(feature = "enable-reqwest")]
            Error::ReqwestMessage(error) => write!(f, "Google Maps Geocoding API client in the Reqwest library: {error}"),
            Error::SerdeJson(error) => write!(f, "Google Maps Elevation API client in the Serde JSON library: {error}"),
            Error::QueryNotBuilt => write!(f,
                "Google Maps Elevation API client: \
                The query string must be built before the request may be sent to the Google Cloud Maps Platform. \
                Ensure the build() method is called before run()."),
        } // match
    } // fn
} // impl

impl std::error::Error for Error {
    /// If the cause for the error is in an underlying library (not this
    /// library but a library this one depends on), this trait unwraps the
    /// original source error. This trait converts a Google Maps Elevation API
    /// error type into the native error type of the underlying library.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::EitherPositionalOrSampledPath => None,
            Error::GoogleMapsService(_error, _message) => None,
            Error::HttpUnsuccessful(_status) => None,
            Error::InvalidStatusCode(_status_code) => None,
            Error::QueryNotBuilt => None,
            Error::RequestNotValidated => None,
            #[cfg(feature = "enable-reqwest")]
            Error::Reqwest(error) => Some(error),
            #[cfg(feature = "enable-reqwest")]
            Error::ReqwestMessage(_error) => None,
            Error::SerdeJson(error) => Some(error),
        } // match
    } // fn
} // impl

#[cfg(feature = "enable-reqwest")]
impl From<reqwest::Error> for Error {
    /// This trait converts from an Reqwest error type (`reqwest::Error`) into a
    /// Google Maps Elevation API error type
    /// (`google_maps::elevation::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: reqwest::Error) -> Error {
        Error::Reqwest(error)
    } // fn
} // impl

impl From<serde_json::error::Error> for Error {
    /// This trait converts from an Serde JSON (`serde_json::error::Error`)
    /// error type into a Google Maps Elevation API error type
    /// (`google_maps::elevation::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: serde_json::error::Error) -> Error {
        Error::SerdeJson(error)
    } // fn
} // impl