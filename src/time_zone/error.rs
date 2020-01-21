//! Time Zone API error types and error messages.

use crate::time_zone::response::Status;

/// Errors that may be produced by the Google Maps Time Zone API client.
#[derive(Debug)]
pub enum Error {
    /// Google Maps Time Zone API server generated an error. See the `Status`
    /// enum for more information.
    GoogleMapsTimeZoneServer(Status, Option<String>),
    /// The query string must be built before the request may be sent to the
    /// Google Maps Time Zone API server.
    QueryNotBuilt,
    /// The dependency library Reqwest generated an error.
    Reqwest(reqwest::Error),
    /// The dependency library Serde JSON generated an error.
    SerdeJson(serde_json::error::Error),
} // enum

impl std::fmt::Display for Error {
    /// This trait converts the error code into a format that may be presented
    /// to the user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::GoogleMapsTimeZoneServer(status, error_message) => match error_message {
                // If the Google Maps Time Zone API server generated an error
                // message, return that:
                Some(error_message) => write!(f, "Google Maps Time Zone API server: {}", error_message),
                // If the Google Maps Time Zone API server did not generate an
                // error message, return a generic message derived from the
                // response status:
                None => match status {
                    Status::InvalidRequest => write!(f, "Google Maps Time Zone API server: \
                        Invalid request. \
                        The request was malformed."),
                    Status::Ok => write!(f, "Google Maps Time Zone server: \
                        Ok. \
                        The request was successful."),
                    Status::OverDailyLimit => write!(f, "Google Maps Time Zone API server: \
                        Over daily limit. \
                        Usage cap has been exceeded, API key is invalid, billing has not been enabled, or method of payment is no longer valid."),
                    Status::OverQueryLimit => write!(f, "Google Maps Time Zone API server: \
                        Over query limit. \
                        Requestor has exceeded quota."),
                    Status::RequestDenied => write!(f, "Google Maps Time Zone API server: \
                        Request denied \
                        Service did not complete the request."),
                    Status::UnknownError => write!(f, "Google Maps Time Zone API server: \
                        Unknown error."),
                    Status::ZeroResults => write!(f, "Google Maps Time Zone API server: \
                        Zero results.
                        This may occur if the geocoder was passed a non-existent address."),
                } // match
            }, // match
            Error::Reqwest(error) => write!(f, "Google Maps Time Zone API client in the Reqwest library: {}", error),
            Error::SerdeJson(error) => write!(f, "Google Maps Time Zone API client in the Serde JSON library: {}", error),
            Error::QueryNotBuilt => write!(f, "Google Maps Time Zone API client library: \
                The query string must be built before the request may be sent to the Google Cloud Maps Platform. \
                Ensure the build() method is called before run()."),
        } // match
    } // fn
} // impl

impl std::error::Error for Error {
    /// If the cause for the error is in an underlying library (not this
    /// library but a library this one depends on), this trait unwraps the
    /// original source error. This trait converts a Google Maps Time Zone API
    /// error type into the native error type of the underlying library.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::GoogleMapsTimeZoneServer(_error, _message) => None,
            Error::Reqwest(error) => Some(error),
            Error::SerdeJson(error) => Some(error),
            Error::QueryNotBuilt => None,
        } // match
    } // fn
} // impl

impl From<reqwest::Error> for Error {
    /// This trait converts from an Reqwest error type (`reqwest::Error`) into a
    /// Google Maps Time Zone API error type
    /// (`google_maps::time_zone::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: reqwest::Error) -> Error {
        Error::Reqwest(error)
    } // fn
} // impl

impl From<serde_json::error::Error> for Error {
    /// This trait converts from an Serde JSON (`serde_json::error::Error`)
    /// error type into a Google Maps Time Zone API error type
    /// (`google_maps::time_zone::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: serde_json::error::Error) -> Error {
        Error::SerdeJson(error)
    } // fn
} // impl