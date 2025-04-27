//! Google Maps Platform API error types and error messages.

// -----------------------------------------------------------------------------

use miette::Diagnostic;
use thiserror::Error;

// -----------------------------------------------------------------------------
//
/// Errors that may be produced by the root part of the Google Maps Platform API
/// client.
#[derive(Debug, Diagnostic, Error)]
#[diagnostic(url(docsrs))]
pub enum Error {
    /// Error originating from the types and structs in the `google_maps` crate.
    #[error(transparent)]
    #[diagnostic(code(google_maps::types))]
    Type(#[from] crate::types::Error),

    /// Error originating from the Google Maps
    /// [Address Validation API](https://developers.google.com/maps/documentation/address-validation)
    /// client module or server.
    #[cfg(feature = "address_validation")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::address_validation))]
    AddressValidation(#[from] crate::address_validation::Error),

    /// Error originating from the Google Maps
    /// [Directions API](https://developers.google.com/maps/documentation/directions)
    /// client module or server.
    #[cfg(any(feature = "directions", feature = "distance_matrix"))]
    #[error(transparent)]
    #[diagnostic(code(google_maps::directions))]
    Directions(#[from] crate::directions::Error),

    /// Error originating from the Google Maps
    /// [Distance Matrix API](https://developers.google.com/maps/documentation/distance-matrix)
    /// client module or server.
    #[cfg(feature = "distance_matrix")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::distance_matrix))]
    DistanceMatrix(#[from] crate::distance_matrix::Error),

    /// Error originating from the Google Maps
    /// [Elevation API](https://developers.google.com/maps/documentation/elevation/overview)
    /// client module or server.
    #[cfg(feature = "elevation")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::elevation))]
    Elevation(#[from] crate::elevation::Error),

    /// Error originating from the Google Maps
    /// [Geocoding API](https://developers.google.com/maps/documentation/geocoding)
    /// client module or server.
    #[cfg(feature = "geocoding")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::geocoding))]
    Geocoding(#[from] crate::geocoding::Error),

    /// Error originating from the Google Maps
    /// [Places API](https://developers.google.com/maps/documentation/places/web-service)
    /// client module or server.
    #[cfg(any(feature = "places", feature = "autocomplete"))]
    #[error(transparent)]
    #[diagnostic(code(google_maps::places))]
    Places(#[from] crate::places::Error),

    /// Error originating from the Google Maps
    /// [Roads API](https://developers.google.com/maps/documentation/roads)
    /// client module or server.
    #[cfg(feature = "roads")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::roads))]
    Roads(#[from] crate::roads::Error),

    /// Error originating from the Google Maps
    /// [Time Zone API](https://developers.google.com/maps/documentation/timezone/overview)
    /// client module or server.
    #[cfg(feature = "time_zone")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::time_zone))]
    TimeZone(#[from] crate::time_zone::Error),

    /// Error originating from the [reqwest](https://crates.io/crates/reqwest)
    /// crate.
    #[cfg(feature = "reqwest")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::reqwest))]
    Reqwest(#[from] reqwest::Error),

    /// An HTTP status code returned by the remote server indicates an error.
    /// This represents a successful HTTP connection but an unsuccessful HTTP
    /// request or transaction with the server.
    ///
    /// * `403 Forbidden` · Ensure that a correct Google Maps API key is being
    ///   supplied and that the [target API has been
    ///   enabled](https://console.cloud.google.com/google/maps-apis/api-list)
    ///   for this API key.
    #[cfg(feature = "reqwest")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::http))]
    Http(#[from] HttpErrorStatus),

    /// Error originating from the [simd-json](https://crates.io/crates/simd-json)
    /// crate.
    ///
    /// Generally this means that the Google Maps API returned data in an
    /// unexpected format. A `struct` or `enum` may need to be changed in [this
    /// crate](https://crates.io/crates/google_maps) to match what Google Maps
    /// is returning.
    ///
    /// Please [file an issue](https://github.com/leontoeides/google_maps/issues)
    /// and include as much information as possible, including the request and
    /// the JSON response if available.
    #[error(transparent)]
    #[diagnostic(code(google_maps::json))]
    Json(#[from] serde_json::Error),

    /// Error originating from the [polyline](https://crates.io/crates/polyline)
    /// crate.
    #[cfg(feature = "polyline")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::polyline))]
    Polyline(#[from] polyline::errors::PolylineError),
} // enum Error

// -----------------------------------------------------------------------------

#[cfg(feature = "reqwest")]
#[derive(Clone, Debug, Diagnostic, Error)]
#[diagnostic(url(docsrs))]
pub struct HttpErrorStatus(pub reqwest::StatusCode);

#[cfg(feature = "reqwest")]
impl std::convert::From<reqwest::StatusCode> for HttpErrorStatus {
    fn from(status_code: reqwest::StatusCode) -> Self { Self(status_code) }
} // impl

#[cfg(feature = "reqwest")]
impl std::convert::From<reqwest::StatusCode> for Error {
    fn from(status_code: reqwest::StatusCode) -> Self {
        Self::from(HttpErrorStatus(status_code))
    } // fn
} // impl

#[cfg(feature = "reqwest")]
impl std::fmt::Display for HttpErrorStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP status: {}", self.0)
    } // fn
} // impl

// -----------------------------------------------------------------------------

use crate::ClassifiedError;
use crate::traits::ClassifiableError;

impl ClassifiableError<'_, Self> for Error {
    /// Classifies a [google_maps](https://crates.io/crates/google_maps) error
    /// as a `Transient` error or `Permanent` error.
    ///
    /// This classification will, in turn, be used to decide whether the HTTP
    /// request should be retried or not.
    fn classify(&self) -> ClassifiedError<'_, Self> {
        match self {
            Self::Type(_type_error) => ClassifiedError::Permanent(self),

            #[cfg(feature = "address_validation")]
            Self::AddressValidation(address_validation_error) =>
                if address_validation_error.classify().is_transient() {
                    ClassifiedError::Transient(self)
                } else {
                    ClassifiedError::Permanent(self)
                }, // AddressValidation

            #[cfg(any(feature = "directions", feature = "distance_matrix"))]
            Self::Directions(directions_error) =>
                if directions_error.classify().is_transient() {
                    ClassifiedError::Transient(self)
                } else {
                    ClassifiedError::Permanent(self)
                }, // Directions

            #[cfg(feature = "distance_matrix")]
            Self::DistanceMatrix(distance_matrix_error) =>
                if distance_matrix_error.classify().is_transient() {
                    ClassifiedError::Transient(self)
                } else {
                    ClassifiedError::Permanent(self)
                }, // DistanceMatrix

            #[cfg(feature = "elevation")]
            Self::Elevation(elevation_error) =>
                if elevation_error.classify().is_transient() {
                    ClassifiedError::Transient(self)
                } else {
                    ClassifiedError::Permanent(self)
                }, // Elevation

            #[cfg(feature = "geocoding")]
            Self::Geocoding(geocoding_error) =>
                if geocoding_error.classify().is_transient() {
                    ClassifiedError::Transient(self)
                } else {
                    ClassifiedError::Permanent(self)
                }, // Geocoding

            #[cfg(any(feature = "autocomplete", feature = "places"))]
            Self::Places(places_error) =>
                if places_error.classify().is_transient() {
                    ClassifiedError::Transient(self)
                } else {
                    ClassifiedError::Permanent(self)
                }, // Places

            #[cfg(feature = "roads")]
            Self::Roads(roads_error) =>
                if roads_error.classify().is_transient() {
                    ClassifiedError::Transient(self)
                } else {
                    ClassifiedError::Permanent(self)
                }, // Roads

            #[cfg(feature = "time_zone")]
            Self::TimeZone(time_zone_error) =>
                if time_zone_error.classify().is_transient() {
                    ClassifiedError::Transient(self)
                } else {
                    ClassifiedError::Permanent(self)
                }, // TimeZone

            #[cfg(feature = "reqwest")]
            Self::Reqwest(reqwest_error) =>
                if reqwest_error.classify().is_transient() {
                    ClassifiedError::Transient(self)
                } else {
                    ClassifiedError::Permanent(self)
                }, // Reqwest

            #[cfg(feature = "reqwest")]
            Self::Http(http_error) =>
                if http_error.0.classify().is_transient() {
                    ClassifiedError::Transient(self)
                } else {
                    ClassifiedError::Permanent(self)
                }, // Http

            Self::Json(_json_error) => ClassifiedError::Permanent(self),

            #[cfg(feature = "polyline")]
            Self::Polyline(_polyline_error) => ClassifiedError::Permanent(self),
        } // match
    } // fn
} // impl