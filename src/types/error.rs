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
    /// language code.
    InvalidLanguageCode(String),

    /// API client library attempted to convert a latitude/longitude pair that
    /// contained an invalid latitude.
    InvalidLatitude(Decimal, Decimal),

    /// API client library attempted to convert a latitude/longitude pair that
    /// contained an invalid longitude.
    InvalidLongitude(Decimal, Decimal),

    /// API client library attempted to convert a latitude/longitude pair string
    /// that is invalid.
    InvalidLatLongString(String),

    /// API client library attempted to convert a latitude/longitude pair tuple
    /// that is invalid.
    InvalidLatLongTuple,

    /// API client library attempted to convert a latitude/longitude pair that
    /// contained an invalid floating-point value.
    FloatToDecimalConversionError(String),

    /// API client library attempted to convert a bounds string that is invalid.
    InvalidBoundsString(String),

    /// API client library attempted to parse a string that contained an invalid
    /// country code.
    InvalidCountryCode(String),

    /// API client library attempted to parse a string that contained an invalid
    /// place type code.
    InvalidPlaceTypeCode(String),

    /// API client library attempted to parse a string that contained an invalid
    /// region code.
    InvalidRegionCode(String),

    /// API client library attempted to parse a string that contained an invalid
    /// location type code.
    InvalidLocationTypeCode(String),
} // enum

// -----------------------------------------------------------------------------

impl std::fmt::Display for Error {
    /// This trait converts the error code into a format that may be presented
    /// to the user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidLanguageCode(language_code) => write!(
                f,
                "Google Maps Platform API client: \
                `{language_code}` is not a recognized language code. \
                For a list of supported languages see \
                https://developers.google.com/maps/faq#languagesupport"
            ),
            Self::InvalidLatitude(latitude, longitude) => write!(
                f,
                "`{latitude}` from the `{latitude},{longitude}` pair is an invalid latitudinal value. \
                A latitude must be between -90.0° and 90.0°."
            ),
            Self::InvalidLongitude(latitude, longitude) => write!(
                f,
                "`{longitude}` from the `{latitude},{longitude}` pair is an invalid longitudinal value. \
                A longitude must be between -180.0° and 180.0°."
            ),
            Self::InvalidLatLongString(value) => write!(
                f,
                "`{value}` is an invalid latitude & longitude coordinate string. \
                The string must consist of two comma-separated coordinates \
                where the latitude is specified first \
                and the longitude is specified second."
            ),
            Self::InvalidLatLongTuple => write!(
                f,
                "Invalid latitude & longitude tuple coordinates. \
                The tuple must consist of coordinates \
                where the latitude is specified first \
                and the longitude is specified second."
            ),
            Self::FloatToDecimalConversionError(value) => write!(
                f,
                "`{value}` could not be converted from a `f64` type to a `Decimal` type.",
            ),
            Self::InvalidBoundsString(value) => write!(
                f,
                "Google Maps Platform API client: \
                `{value}` is an invalid `Bounds` string."
            ),
            Self::InvalidCountryCode(country_code) => write!(f,
                "Google Maps Geocoding API client: \
                `{country_code}` is not a valid ISO 3166-1 Alpha-2 country code. \
                Note that the country code must be in uppercase. \
                For a list of country codes see \
                https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes"),
            Self::InvalidPlaceTypeCode(place_type_code) => write!(
                f,
                "Google Maps Platform API client: \
                `{place_type_code}` is not a recognized place type code. \
                For a list of supported place types see \
                https://developers.google.com/places/web-service/supported_types"
            ),
            Self::InvalidRegionCode(region_code) => write!(
                f,
                "Google Maps Platform API client: \
                `{region_code}` is not a recognized region code. \
                For a list of supported regions see \
                https://developers.google.com/maps/coverage"
            ),
            Self::InvalidLocationTypeCode(location_type_code) => write!(f,
                "Google Maps Geocoding API client: \
                `{location_type_code}` is not a known location type code. \
                Valid codes are `APPROXIMATE`, `GEOMETRIC_CENTER`, \
                `RANGE_INTERPOLATED`, and `ROOFTOP`."),
        } // match
    } // fn
} // impl
