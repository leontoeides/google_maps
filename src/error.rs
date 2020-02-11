//! Google Maps Platform API error types and error messages.

/// Errors that may be produced by the root part of the Google Maps Platform API
/// client.
#[derive(Debug)]
pub enum Error {
    /// API client library attempted to parse a string that contained an invalid
    /// language code. See `google_maps\src\language.rs` for more information.
    InvalidLanguageCode(String),
    /// API client library attempted to convert a latitude/longitude pair that
    /// contained an invalid latitude. See `google_maps\src\latlng.rs` for more
    /// information.
    InvalidLatitude(f64, f64),
    /// API client library attempted to convert a latitude/longitude pair that
    /// contained an invalid longitude. See `google_maps\src\latlng.rs` for more
    /// information.
    InvalidLongitude(f64, f64),
    /// API client library attempted to parse a string that contained an invalid
    /// place type code. See `google_maps\src\place_type.rs` for more
    /// information.
    InvalidPlaceTypeCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// region code. See `google_maps\src\region.rs` for more information.
    InvalidRegionCode(String),
} // enum

impl std::fmt::Display for Error {
    /// This trait converts the error code into a format that may be presented
    /// to the user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidLanguageCode(language_code) => write!(f,
                "Google Maps Platform API client: \
                `{}` is not a recognized language code.
                For a list of supported languages see \
                https://developers.google.com/maps/faq#languagesupport",
                language_code),
            Error::InvalidLatitude(latitude, longitude) => write!(f,
                "Google Maps Platform API client: \
                `{lat}` from the `{lat},{lng}` pair is an invalid latitudinal value.
                A latitude must be between -90.0° and 90.0°.",
                lat=latitude, lng=longitude),
            Error::InvalidLongitude(latitude, longitude) => write!(f,
                "Google Maps Platform API client: \
                `{lng}` from the `{lat},{lng}` pair is an invalid longitudinal value.
                A longitude must be between -180.0° and 180.0°.",
                lat=latitude, lng=longitude),
            Error::InvalidPlaceTypeCode(place_type_code) => write!(f,
                "Google Maps Platform API client: \
                `{}` is not a recognized place type code.
                For a list of supported place types see \
                https://developers.google.com/places/web-service/supported_types",
                place_type_code),
            Error::InvalidRegionCode(region_code) => write!(f,
                "Google Maps Platform API client: \
                `{}` is not a recognized region code.
                For a list of supported regions see \
                https://developers.google.com/maps/coverage",
                region_code),
        } // match
    } // fn
} // impl

impl std::error::Error for Error {
    /// If the cause for the error is in an underlying library (not this
    /// library but a library this one depends on), this trait unwraps the
    /// original source error. This trait converts a Google Maps Platform API
    /// error type into the native error type of the underlying library.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::InvalidLanguageCode(_language_code) => None,
            Error::InvalidLatitude(_latitude, _longitude) => None,
            Error::InvalidLongitude(_latitude, _longitude) => None,
            Error::InvalidPlaceTypeCode(_place_type_code) => None,
            Error::InvalidRegionCode(_region_code) => None,
        } // match
    } // fn
} // impl