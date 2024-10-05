//! Geocoding API error types and error messages.

// -----------------------------------------------------------------------------

use crate::geocoding::response::status::Status;
use miette::Diagnostic;
use thiserror::Error;

// -----------------------------------------------------------------------------
//
/// Errors that may be produced by the Google Maps Geocoding API client.

#[derive(Debug, Diagnostic, Error)]
#[diagnostic(code(google_maps::geocoding::error), url(docsrs))]
pub enum Error {
    /// Forward geocoding requests (address to latlng) must specify an `address`
    /// or at least one `component`.
    AddressOrComponentsRequired,
    /// Google Maps Geocoding API server generated an error. See the `Status`
    /// enum for more information.
    GoogleMapsService(Status, Option<String>),
    /// The HTTP request was unsuccessful.
    HttpUnsuccessful(String),
    /// API client library attempted to parse a string that contained an invalid
    /// status code.
    InvalidStatusCode(String),
    /// The query string must be built before the request may be sent to the
    /// Google Maps Geocoding API server.
    QueryNotBuilt,
    /// The request must be validated before a query string may be built.
    RequestNotValidated,
    /// The dependency library Reqwest generated an error.
    #[cfg(feature = "reqwest")]
    Reqwest(crate::ReqError),
    /// The dependency library Reqwest generated an error. The error could
    /// not be passed normally so a `String` representation is passed instead.
    #[cfg(feature = "reqwest")]
    ReqwestMessage(String),
    /// The dependency library Serde JSON generated an error.
    SerdeJson(serde_json::error::Error),
} // enum

impl std::fmt::Display for Error {
    /// This trait converts the error code into a format that may be presented
    /// to the user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AddressOrComponentsRequired => write!(f,
                "Google Maps Geocoding API client: \
                Forward geocoding requests must specify an `address` or at least one `component`. \
                Ensure that the with_address() and/or with_component methods are being called before run()."),
            Self::GoogleMapsService(status, error_message) => match error_message {
                // If the Google Maps Geocoding API server generated an error
                // message, return that:
                Some(error_message) => write!(f, "Google Maps Geocoding API server: {error_message}"),
                // If the Google Maps Geocoding API server did not generate an
                // error message, return a generic message derived from the
                // response status:
                None => match status {
                    Status::InvalidRequest => write!(f, "Google Maps Geocoding API server: \
                        Invalid request. \
                        This may indicate that the query (address, components, or latlng) is missing, an invalid result type, or an invalid location type."),
                    Status::Ok => write!(f, "Google Maps Geocoding server: \
                        Ok. \
                        The request was successful."),
                    Status::OverDailyLimit => write!(f, "Google Maps Geocoding API server: \
                        Over daily limit. \
                        Usage cap has been exceeded, API key is invalid, billing has not been enabled, or method of payment is no longer valid."),
                    Status::OverQueryLimit => write!(f, "Google Maps Geocoding API server: \
                        Over query limit. \
                        Requestor has exceeded quota."),
                    Status::RequestDenied => write!(f, "Google Maps Geocoding API server: \
                        Request denied \
                        Service did not complete the request."),
                    Status::UnknownError => write!(f, "Google Maps Geocoding API server: \
                        Unknown error."),
                    Status::ZeroResults => write!(f, "Google Maps Geocoding API server: \
                        Zero results. \
                        This may occur if the geocoder was passed a non-existent address."),
                } // match
            }, // match
            Self::HttpUnsuccessful(status) => write!(f,
                "Google Maps Geocoding API client: \
                Could not successfully query the Google Cloud Platform service. \
                The service last responded with a `{status}` status."),
            Self::InvalidStatusCode(status_code) => write!(f,
                "Google Maps Geocoding API client: \
                `{status_code}` is not a valid status code. \
                Valid codes are `INVALID_REQUEST`, `OK`, `OVER_DAILY_LIMIT`, \
                `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, `UNKNOWN_ERROR`, and \
                `ZERO_RESULTS`."),
            Self::QueryNotBuilt => write!(f,
                "Google Maps Geocoding API client: \
                The query string must be built before the request may be sent to the Google Cloud Maps Platform. \
                Ensure the build() method is called before run()."),
            Self::RequestNotValidated => write!(f,
                "Google Maps Geocoding API client: \
                The request must be validated before a query string may be built. \
                Ensure the validate() method is called before build()."),
            #[cfg(feature = "reqwest")]
            Self::Reqwest(error) => write!(f, "Google Maps Geocoding API client in the Reqwest library: {error}"),
            #[cfg(feature = "reqwest")]
            Self::ReqwestMessage(error) => write!(f, "Google Maps Geocoding API client in the Reqwest library: {error}"),
            Self::SerdeJson(error) => write!(f, "Google Maps Geocoding API client in the Serde JSON library: {error}"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
    /// This trait converts from an Reqwest error type (`reqwest::Error`) into a
    /// Google Maps Geocoding API error type
    /// (`google_maps::geocoding::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: reqwest::Error) -> Self {
        Self::Reqwest(crate::ReqError::from(error))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl From<serde_json::error::Error> for Error {
    /// This trait converts from an Serde JSON (`serde_json::error::Error`)
    /// error type into a Google Maps Geocoding API error type
    /// (`google_maps::geocoding::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: serde_json::error::Error) -> Self {
        Self::SerdeJson(error)
    } // fn
} // impl
