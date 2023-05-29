//! Google Maps Platform API error types and error messages.

// -----------------------------------------------------------------------------

use miette::Diagnostic;
use thiserror::Error;

// -----------------------------------------------------------------------------
//
/// Errors that may be produced by the root part of the Google Maps Platform API
/// client.

#[derive(Debug, Diagnostic, Error)]
#[diagnostic(code(google_maps::types::error), url(docsrs))]
pub enum Error {

    /// Errors from the types and structs in the `google_maps` crate.
    #[error(transparent)]
    Types(#[from] crate::types_error::Error),

    /// Errors from the `directions` module in the `google_maps` crate.
    #[error(transparent)]
    Directions(#[from] crate::directions::error::Error),

    /// Errors from the `distance_matrix` module in the `google_maps` crate.
    #[error(transparent)]
    DistanceMatrix(#[from] crate::distance_matrix::error::Error),

    /// Errors from the `geocoding` module in the `google_maps` crate.
    #[error(transparent)]
    Geocoding(#[from] crate::geocoding::error::Error),

    /// Errors from the `places` module in the `google_maps` crate.
    #[error(transparent)]
    Places(#[from] crate::places::error::Error),

    /// Errors from the `place_autocomplete` module in the `google_maps` crate.
    #[error(transparent)]
    PlaceAutocomplete(#[from] crate::places::place_autocomplete::error::Error),

    /// Errors from the `roads` module in the `google_maps` crate.
    #[error(transparent)]
    Roads(#[from] crate::roads::error::Error),

    /// Errors from the `time_zone` module in the `google_maps` crate.
    #[error(transparent)]
    TimeZone(#[from] crate::time_zone::error::Error),

} // enum Error