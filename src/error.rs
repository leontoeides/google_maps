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

    /// Error originating from the `directions` module in the `google_maps`
    /// crate.
    #[cfg(any(feature = "directions", feature = "distance_matrix"))]
    #[error(transparent)]
    #[diagnostic(code(google_maps::directions))]
    Directions(#[from] crate::directions::error::Error),

    /// Error originating from the `distance_matrix` module in the `google_maps`
    /// crate.
    #[cfg(feature = "distance_matrix")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::distance_matrix))]
    DistanceMatrix(#[from] crate::distance_matrix::error::Error),

    /// Error originating from the `elevation` module in the `google_maps`
    /// crate.
    #[cfg(feature = "elevation")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::elevation))]
    Elevation(#[from] crate::elevation::error::Error),

    /// Error originating from the `geocoding` module in the `google_maps`
    /// crate.
    #[cfg(feature = "geocoding")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::geocoding))]
    Geocoding(#[from] crate::geocoding::error::Error),

    /// Error originating from the `places` module in the `google_maps` crate.
    #[cfg(feature = "places")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::places))]
    Places(#[from] crate::places::error::Error),

    /// Error originating from the `place_autocomplete` module in the
    /// `google_maps` crate.
    #[cfg(feature = "autocomplete")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::place_autocomplete))]
    PlaceAutocomplete(#[from] crate::places::place_autocomplete::error::Error),

    /// Error originating from the `roads` module in the `google_maps` crate.
    #[cfg(feature = "roads")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::roads))]
    Roads(#[from] crate::roads::error::Error),

    /// Error originating from the `time_zone` module in the `google_maps`
    /// crate.
    #[cfg(feature = "time_zone")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::time_zone))]
    TimeZone(#[from] crate::time_zone::error::Error),

    /// Error originating from the [reqwest](https://crates.io/crates/reqwest)
    /// crate.
    #[cfg(feature = "enable-reqwest")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::reqwest))]
    Reqwest(#[from] reqwest::Error),

    /// Error originating from the [polyline](https://crates.io/crates/polyline)
    /// crate.
    #[cfg(feature = "polyline")]
    #[error(transparent)]
    #[diagnostic(code(google_maps::polyline))]
    Polyline(#[from] polyline::errors::PolylineError),
} // enum Error
