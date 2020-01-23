//! Geocoding API error types and error messages.

use crate::geocoding::response::status::Status;

/// Errors that may be produced by the Google Maps Geocoding API client.
#[derive(Debug)]
pub enum Error {
    /// Forward geocoding requests (address to latlng) must specify an `address`
    /// or at least one `component`.
    AddressOrComponentsRequired,
    /// Google Maps Geocoding API server generated an error. See the `Status`
    /// enum for more information.
    GoogleMapsGeocodingServer(Status, Option<String>),
    /// API client library attempted to parse a string that contained an invalid
    /// country code. See `google_maps\src\geocoding\forward\country.rs` for
    /// more information.
    InvalidCountryCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// location type code. See `google_maps\src\geocoding\location_type.rs` for
    /// more information.
    InvalidLocationTypeCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// status code. See `google_maps\src\geocoding\response\status.rs` for more
    /// information.
    InvalidStatusCode(String),
    /// The query string must be built before the request may be sent to the
    /// Google Maps Geocoding API server.
    QueryNotBuilt,
    /// The request must be validated before a query string may be built.
    RequestNotValidated,
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
            Error::AddressOrComponentsRequired => write!(f,
                "Google Maps Geocoding API client: \
                Forward geocoding requests must specify an `address` or at least one `component`. \
                Ensure that the with_address() and/or with_component methods are being called before run()."),
            Error::GoogleMapsGeocodingServer(status, error_message) => match error_message {
                // If the Google Maps Geocoding API server generated an error
                // message, return that:
                Some(error_message) => write!(f, "Google Maps Geocoding API server: {}", error_message),
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
                        Zero results.
                        This may occur if the geocoder was passed a non-existent address."),
                } // match
            }, // match
            Error::InvalidCountryCode(country_code) => write!(f,
                "Google Maps Geocoding API client: \
                `{}` is not a valid ISO 3166-1 Alpha-2 country code. \
                Note that the country code must be in uppercase. \
                For a list of country codes see \
                https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes",
                country_code),
            Error::InvalidLocationTypeCode(location_type_code) => write!(f,
                "Google Maps Geocoding API client: \
                `{}` is not a known location type code. \
                Valid codes are `APPROXIMATE`, `GEOMETRIC_CENTER`, \
                `RANGE_INTERPOLATED`, and `ROOFTOP`.", location_type_code),
            Error::InvalidStatusCode(status_code) => write!(f,
                "Google Maps Geocoding API client: \
                `{}` is not a valid status code. \
                Valid codes are `INVALID_REQUEST`, `OK`, `OVER_DAILY_LIMIT`, \
                `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, `UNKNOWN_ERROR`, and \
                `ZERO_RESULTS`.", status_code),
            Error::QueryNotBuilt => write!(f,
                "Google Maps Geocoding API client: \
                The query string must be built before the request may be sent to the Google Cloud Maps Platform. \
                Ensure the build() method is called before run()."),
            Error::RequestNotValidated => write!(f,
                "Google Maps Geocoding API client: \
                The request must be validated before a query string may be built. \
                Ensure the validate() method is called before build()."),
            Error::Reqwest(error) => write!(f, "Google Maps Geocoding API client in the Reqwest library: {}", error),
            Error::SerdeJson(error) => write!(f, "Google Maps Geocoding API client in the Serde JSON library: {}", error),
        } // match
    } // fn
} // impl

impl std::error::Error for Error {
    /// If the cause for the error is in an underlying library (not this
    /// library but a library this one depends on), this trait unwraps the
    /// original source error. This trait converts a Google Maps Geocoding API
    /// error type into the native error type of the underlying library.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::AddressOrComponentsRequired => None,
            Error::GoogleMapsGeocodingServer(_error, _message) => None,
            Error::InvalidCountryCode(_country_code) => None,
            Error::InvalidLocationTypeCode(_location_type_code) => None,
            Error::InvalidStatusCode(_status_code) => None,
            Error::QueryNotBuilt => None,
            Error::RequestNotValidated => None,
            Error::Reqwest(error) => Some(error),
            Error::SerdeJson(error) => Some(error),
        } // match
    } // fn
} // impl

impl From<reqwest::Error> for Error {
    /// This trait converts from an Reqwest error type (`reqwest::Error`) into a
    /// Google Maps Geocoding API error type
    /// (`google_maps::geocoding::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: reqwest::Error) -> Error {
        Error::Reqwest(error)
    } // fn
} // impl

impl From<serde_json::error::Error> for Error {
    /// This trait converts from an Serde JSON (`serde_json::error::Error`)
    /// error type into a Google Maps Geocoding API error type
    /// (`google_maps::geocoding::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: serde_json::error::Error) -> Error {
        Error::SerdeJson(error)
    } // fn
} // impl