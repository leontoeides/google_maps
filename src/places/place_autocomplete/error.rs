//! Places API _Place Autocomplete_ error types and error messages.

// -----------------------------------------------------------------------------

use crate::places::place_autocomplete::response::status::Status;
use miette::Diagnostic;
use thiserror::Error;

// -----------------------------------------------------------------------------
//
/// Errors that may be produced by the Google Maps Places API client.

#[derive(Debug, Diagnostic, Error)]
#[diagnostic(code(google_maps::place_autocomplete::error), url(docsrs))]
pub enum Error {
    /// Google Maps Places API server generated an error. See the `Status`
    /// enum for more information.
    GoogleMapsService(Status, Option<String>),
    /// The HTTP request was unsuccessful.
    HttpUnsuccessful(String),
    /// API client library attempted to parse a string that contained an invalid
    /// status code. See
    /// `google_maps\src\places\place_autocomplete\response\status.rs` for more
    /// information.
    InvalidStatusCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// autocomplete type. See
    /// `google_maps\src\places\place_autocomplete\request\autocomplete_type.rs`
    /// for more information.
    InvalidAutocompleteType(String),
    /// The query string must be built before the request may be sent to the
    /// Google Maps Places API server.
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
                // If the Google Maps Places API server generated an error
                // message, return that:
                Some(error_message) => write!(f, "Google Maps Places API service: {error_message}"),
                // If the Google Maps Places API server did not generate an
                // error message, return a generic message derived from the
                // response status:
                None => match status {
                    Status::InvalidRequest => write!(f, "Google Maps Places API service: \
                        Invalid request. \
                        The request was malformed."),
                    Status::Ok => write!(f, "Google Maps Time Zone service: \
                        Ok. \
                        The request was successful."),
                    Status::OverQueryLimit => write!(f, "Google Maps Places API service: \
                        Over query limit. \
                        Requestor has exceeded quota."),
                    Status::RequestDenied => write!(f, "Google Maps Places API service: \
                        Request denied \
                        Service did not complete the request."),
                    Status::UnknownError => write!(f, "Google Maps Places API service: \
                        Unknown error."),
                    Status::ZeroResults => write!(f, "Google Maps Places API service: \
                        Zero results. \
                        This may occur if the geocoder was passed a non-existent address."),
                } // match
            }, // match
            Self::HttpUnsuccessful(status) => write!(f,
                "Google Maps Places API client: \
                Could not successfully query the Google Cloud Platform service. \
                The service last responded with a `{status}` status."),
            Self::InvalidStatusCode(status_code) => write!(f, "Google Maps Places API client: \
                `{status_code}` is not a valid status code. \
                Valid codes are `INVALID_REQUEST`, `OK`, `OVER_DAILY_LIMIT`, \
                `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, `UNKNOWN_ERROR`, and \
                `ZERO_RESULTS`."),
            Self::InvalidAutocompleteType(autocomplete_type) => write!(f, "Google Maps Places API client: \
                `{autocomplete_type}` is not a valid autocomplete type. \
                Valid types are `geocode`, `address`, `establishment`, \
                `(regions)`, `(cities)`."),
            Self::QueryNotBuilt => write!(f, "Google Maps Places API client library: \
                The query string must be built before the request may be sent to the Google Cloud Maps Platform. \
                Ensure the build() method is called before run()."),
            #[cfg(feature = "reqwest")]
            Self::Reqwest(error) => write!(f, "Google Maps Places API client in the Reqwest library: {error}"),
            #[cfg(feature = "reqwest")]
            Self::ReqwestMessage(error) => write!(f, "Google Maps Geocoding API client in the Reqwest library: {error}"),
            Self::SimdJson(error) => write!(f, "Google Maps Places API client in the Serde JSON library: {error}"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
    /// This trait converts from an Reqwest error type (`reqwest::Error`) into a
    /// Google Maps Places API error type
    /// (`google_maps::time_zone::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: reqwest::Error) -> Self {
        Self::Reqwest(crate::ReqError::from(error))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl From<simd_json::Error> for Error {
    /// This trait converts from an Serde JSON (`simd_json::Error`)
    /// error type into a Google Maps Places API error type
    /// (`google_maps::time_zone::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: simd_json::Error) -> Self {
        Self::SimdJson(error)
    } // fn
} // impl
