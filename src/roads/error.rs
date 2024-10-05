//! Roads API error types and error messages.

// -----------------------------------------------------------------------------

use crate::roads::status::Status;
use miette::Diagnostic;
use thiserror::Error;

// -----------------------------------------------------------------------------
//
/// Errors that may be produced by the Google Maps Roads API client.

#[derive(Debug, Diagnostic, Error)]
#[diagnostic(code(google_maps::roads::error), url(docsrs))]
pub enum Error {
    /// Google Maps Roads API server generated an error. See the `Status`
    /// enum for more information.
    GoogleMapsService(Status, Option<String>),

    /// The HTTP request was unsuccessful.
    HttpUnsuccessful(String),

    /// API client library attempted to parse a string that contained an invalid
    /// status code. See `google_maps\src\time_zone\response\status.rs` for more
    /// information.
    InvalidStatusCode(String),

    /// The query string must be built before the request may be sent to the
    /// Google Maps Roads API server.
    QueryNotBuilt,

    /// The dependency library Reqwest generated an error.
    #[cfg(feature = "reqwest")]
    Reqwest(crate::ReqError),

    /// The dependency library Reqwest generated an error. The error could
    /// not be passed normally so a `String` representation is passed instead.
    #[cfg(feature = "reqwest")]
    ReqwestMessage(String),

    /// The dependency library Serde JSON generated an error.
    SimdJson(simd_json::Error),
} // enum

// -----------------------------------------------------------------------------

impl std::fmt::Display for Error {
    /// This trait converts the error code into a format that may be presented
    /// to the user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::GoogleMapsService(status, error_message) => match error_message {
                // If the Google Maps Roads API server generated an error
                // message, return that:
                Some(error_message) => write!(f, "Google Maps Roads API service: {error_message}"),
                // If the Google Maps Roads API server did not generate an
                // error message, return a generic message derived from the
                // response status:
                None => match status {
                    Status::InvalidArgument => write!(f, "Google Maps Roads API service: \
                        Invalid argument. \
                        1. Your API key is not valid or was not included in the request. \
                        or, 2. Your request contained invalid arguments."),
                    Status::PermissionDenied => write!(f, "Google Maps Roads API service: \
                        Permission Denied. \
                        1. API key missing or invalid. \
                        2. Billing not enabled. \
                        3. Self-imposed usage cap exceeded. \
                        or, 4. Method of payment no longer valid."),
                    Status::NotFound => write!(f, "Google Maps Roads API service: \
                        Not found. \
                        Ensure that you are sending requests \
                        to `https://roads.googleapis.com/` and not \
                        `http://roads.googleapis.com/`."),
                    Status::ResourceExhausted => write!(f, "Google Maps Roads API service: \
                        Not found. \
                        You have exceeded the request limit that you configured \
                        in the Google Cloud Platform Console."),
                } // match
            }, // match
            Self::HttpUnsuccessful(status) => write!(f,
                "Google Maps Roads API client: \
                Could not successfully query the Google Cloud Platform service. \
                The service last responded with a `{status}` status."),
            Self::InvalidStatusCode(status_code) => write!(f, "Google Maps Roads API client: \
                `{status_code}` is not a valid status code. \
                Valid codes are `INVALID_ARGUMENT`, `PERMISSION_DENIED`, \
                `NOT_FOUND`, and `RESOURCE_EXHAUSTED`."),
            Self::QueryNotBuilt => write!(f, "Google Maps Roads API client library: \
                The query string must be built before the request may be sent to the Google Cloud Maps Platform. \
                Ensure the build() method is called before run()."),
            #[cfg(feature = "reqwest")]
            Self::Reqwest(error) => write!(f, "Google Maps Roads API client in the Reqwest library: {error}"),
            #[cfg(feature = "reqwest")]
            Self::ReqwestMessage(error) => write!(f, "Google Maps Geocoding API client in the Reqwest library: {error}"),
            Self::SimdJson(error) => write!(f, "Google Maps Roads API client in the Serde JSON library: {error}"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
    /// This trait converts from an Reqwest error type (`reqwest::Error`) into a
    /// Google Maps Roads API error type
    /// (`google_maps::time_zone::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: reqwest::Error) -> Self {
        Self::Reqwest(crate::ReqError::from(error))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl From<simd_json::Error> for Error {
    /// This trait converts from an Serde JSON (`simd_json::Error`)
    /// error type into a Google Maps Roads API error type
    /// (`google_maps::time_zone::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: simd_json::Error) -> Self {
        Self::SimdJson(error)
    } // fn
} // impl
