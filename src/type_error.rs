//! Google Maps crate errors for types originating from their implementations
//! and associated functions.

// -----------------------------------------------------------------------------

use miette::Diagnostic;
use rust_decimal::Decimal;
use thiserror::Error;

// -----------------------------------------------------------------------------
//
/// Errors that may be produced by crate types from implementations and
/// associated functions. For example, type conversions, instantiations, etc.

#[derive(Debug, Diagnostic, Error)]
#[diagnostic(code(google_maps::types::error), url(docsrs))]
pub enum Error {

    /// API client library attempted to parse a string that contained an invalid
    /// language code. See `google_maps\src\language.rs` for more information.
    InvalidLanguageCode(String),

    /// API client library attempted to convert a latitude/longitude pair that
    /// contained an invalid latitude. See `google_maps\src\latlng.rs` for more
    /// information.
    InvalidLatitude(Decimal, Decimal),

    /// API client library attempted to convert a latitude/longitude pair that
    /// contained an invalid longitude. See `google_maps\src\latlng.rs` for more
    /// information.
    InvalidLongitude(Decimal, Decimal),

    /// API client library attempted to convert a latitude/longitude pair that
    /// contained an invalid floating-point value.
    FloatToDecimalConversionError(String),

    /// API client library attempted to convert a latitude/longitude pair string
    /// that is invalid. See `google_maps\src\latlng.rs` for more
    /// information.
    InvalidLatLongString(String),

    /// API client library attempted to convert a bounds string that is invalid.
    /// See `google_maps\src\bounds.rs` for more information.
    InvalidBoundsString(String),

    /// API client library attempted to parse a string that contained an invalid
    /// place type code. See `google_maps\src\place_type.rs` for more
    /// information.
    InvalidPlaceTypeCode(String),

    /// API client library attempted to parse a string that contained an invalid
    /// region code. See `google_maps\src\region.rs` for more information.
    InvalidRegionCode(String),

} // enum

// -----------------------------------------------------------------------------

impl std::fmt::Display for Error {
    /// This trait converts the error code into a format that may be presented
    /// to the user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidLanguageCode(language_code) => write!(
                f,
                "Google Maps Platform API client: \
                `{language_code}` is not a recognized language code.
                For a list of supported languages see \
                https://developers.google.com/maps/faq#languagesupport"
            ),
            Error::InvalidLatitude(latitude, longitude) => write!(
                f,
                "Google Maps Platform API client: \
                `{latitude}` from the `{latitude},{longitude}` pair is an invalid latitudinal value.
                A latitude must be between -90.0° and 90.0°."
            ),
            Error::InvalidLongitude(latitude, longitude) => write!(
                f,
                "Google Maps Platform API client: \
                `{longitude}` from the `{latitude},{longitude}` pair is an invalid longitudinal value.
                A longitude must be between -180.0° and 180.0°."
            ),
            Error::FloatToDecimalConversionError(value) => write!(
                f,
                "Google Maps Platform API client: \
                `{value}` could not be converted from a `f64` type to a `Decimal` type.",
            ),
            Error::InvalidPlaceTypeCode(place_type_code) => write!(
                f,
                "Google Maps Platform API client: \
                `{place_type_code}` is not a recognized place type code.
                For a list of supported place types see \
                https://developers.google.com/places/web-service/supported_types"
            ),
            Error::InvalidRegionCode(region_code) => write!(
                f,
                "Google Maps Platform API client: \
                `{region_code}` is not a recognized region code.
                For a list of supported regions see \
                https://developers.google.com/maps/coverage"
            ),
            Error::InvalidLatLongString(value) => write!(
                f,
                "Google Maps Platform API client: \
                `{value}` is an invalid `LatLng` string."
            ),
            Error::InvalidBoundsString(value) => write!(
                f,
                "Google Maps Platform API client: \
                `{value}` is an invalid `Bounds` string."
            ),
        } // match
    } // fn
} // impl