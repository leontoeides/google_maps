use crate::ClassifiedError;
use crate::places_new::LatLng;
use rust_decimal::Decimal;

// -------------------------------------------------------------------------------------------------
//
/// Errors that can occur when using the Google Maps Places API (New).
#[derive(Clone, Debug, thiserror::Error, miette::Diagnostic)]
#[non_exhaustive]
pub enum Error {
    /// Error from the Google Maps Places API (New) Autocomplete service.
    #[cfg(feature = "places-new-autocomplete")]
    #[error("autocomplete error")]
    Autocomplete {
        #[from]
        #[source]
        source: crate::places_new::autocomplete::Error,
    },

    /// Error from the Google Maps Places API (New) Text Search service.
    #[cfg(feature = "places-new-text-search")]
    #[error("text search error")]
    TextSearch {
        #[from]
        #[source]
        source: crate::places_new::text_search::Error,
    },

    /// Error from the Google Maps Places API (New) Nearby Search service.
    #[cfg(feature = "places-new-nearby-search")]
    #[error("nearby search error")]
    NearbySearch {
        #[from]
        #[source]
        source: crate::places_new::nearby_search::Error,
    },

    /// Error from the Google Maps Places API (New) Place Photos service.
    #[cfg(feature = "places-new-place-photos")]
    #[error("place photos error")]
    PlacePhotos {
        #[from]
        #[source]
        source: crate::places_new::place_photos::Error,
    },

    /// Failed to convert a floating-point value to Decimal.
    ///
    /// Occurs when a float value (`f32` or `f64`) cannot be represented as a `Decimal`, typically
    /// due to special values like `NaN` or infinity.
    #[diagnostic(
        code(google_maps::places_new::float_conversion_error),
        severity(Error),
        help("ensure the float value is finite (not NaN or infinity)")
    )]
    #[error("failed to convert float '{value}' to Decimal")]
    FloatToDecimalConversionError {
        value: String,
        #[source_code]
        debug: String,
        #[label("this float value cannot be represented as Decimal")]
        span: (usize, usize),
    },

    /// Circle radius is outside the valid range.
    ///
    /// Occurs when attempting to create a circle with a radius that doesn't meet Google Maps API
    /// requirements (0.0 to 50,000.0 metres).
    #[diagnostic(
        code(google_maps::places_new::invalid_circle_radius),
        severity(Error),
        help("radius must be between 0.0 and 50,000.0 metres (50 kilometres)")
    )]
    #[error("invalid circle radius: {radius} metres")]
    InvalidCircleRadius {
        radius: Decimal,
        #[source_code]
        debug: String,
        #[label("radius out of range [0.0, 50000.0]")]
        span: (usize, usize),
    },

    /// Viewport bounds form an empty or invalid region.
    ///
    /// Occurs when attempting to create a viewport where the latitude range is inverted
    /// (low > high) or the longitude range is empty (low == 180° and high == -180°).
    ///
    /// Both corner points must define a valid rectangular region according to Google Maps API
    /// requirements.
    #[diagnostic(
        code(google_maps::places::invalid_viewport),
        severity(Error),
        help(
            "ensure low.latitude <= high.latitude and \
            that the longitude range is not empty (low: 180°, high: -180°)"
        )
    )]
    #[error("invalid viewport bounds")]
    InvalidViewport {
        low: LatLng,
        high: LatLng,
        #[source_code]
        debug: String,
        #[label("viewport bounds form an empty region")]
        span: (usize, usize),
    },

    /// Latitude value is outside the valid range of -90.0° to 90.0°.
    ///
    /// Occurs when attempting to create coordinates with an invalid latitude.
    #[diagnostic(
        code(google_maps::places_new::invalid_latitude),
        severity(Error),
        help("latitude must be between -90.0 and 90.0 degrees")
    )]
    #[error("invalid latitude {latitude}° in coordinate ({latitude}°, {longitude}°)")]
    InvalidLatitude {
        latitude: Decimal,
        longitude: Decimal,
        #[source_code]
        coordinate_string: String,
        #[label("latitude out of range [-90.0, 90.0]")]
        span: (usize, usize),
    },

    /// Longitude value is outside the valid range of -180.0° to 180.0°.
    ///
    /// Occurs when attempting to create coordinates with an invalid longitude.
    #[diagnostic(
        code(google_maps::places_new::invalid_longitude),
        severity(Error),
        help("longitude must be between -180.0 and 180.0 degrees")
    )]
    #[error("invalid longitude {longitude}° in coordinate ({latitude}°, {longitude}°)")]
    InvalidLongitude {
        latitude: Decimal,
        longitude: Decimal,
        #[source_code]
        coordinate_string: String,
        #[label("longitude out of range [-180.0, 180.0]")]
        span: (usize, usize),
    },

    /// Failed to parse a latitude/longitude coordinate string.
    ///
    /// Occurs when a string cannot be parsed as a valid comma-delimited coordinate pair in the
    /// format "latitude,longitude".
    #[diagnostic(
        code(google_maps::places_new::invalid_latlng_string),
        severity(Error),
        help("expected format: 'latitude,longitude' (e.g., '37.7749,-122.4194')")
    )]
    #[error("could not parse '{input}' as latitude/longitude coordinates")]
    InvalidLatLongString {
        input: String,
        #[source_code]
        debug: String,
        #[label("expected 'latitude,longitude' format")]
        span: (usize, usize),
    },

    /// Failed to convert a tuple to latitude/longitude coordinates.
    ///
    /// Occurs when a coordinate tuple cannot be converted to valid `Decimal` values for latitude
    /// and longitude.
    #[diagnostic(
        code(google_maps::places_new::invalid_latlng_tuple),
        severity(Error),
        help("ensure the tuple contains two values that can be converted to Decimal")
    )]
    #[error("invalid latitude/longitude tuple")]
    InvalidLatLongTuple,

    /// Standard Google API error response.
    ///
    /// Represents errors returned by Google APIs following the `google.rpc.Status` format. Contains
    /// status code, developer message, and optional detailed error information including localized
    /// messages, retry guidance, and links to documentation.
    #[diagnostic(
        code(google_maps::places_new::api_error),
        severity(Error),
        help("check the error details for specific guidance on resolving the issue")
    )]
    #[error("Google API error")]
    Api {
        #[from]
        #[source]
        source: crate::places_new::types::error::Error,
    },

    /// Decimal arithmetic or parsing operation failed.
    ///
    /// Wraps errors from the `rust_decimal` crate when performing decimal arithmetic operations or
    /// parsing decimal strings.
    #[diagnostic(
        code(google_maps::places::decimal_error),
        severity(Error),
        help("check that decimal values are within valid range and properly formatted")
    )]
    #[error("decimal operation error: {source}")]
    Decimal {
        #[from]
        #[source]
        source: rust_decimal::Error,
    },

    /// Time/date operation failed.
    ///
    /// Wraps errors from the `jiff` datetime library when parsing or manipulating temporal values.
    #[diagnostic(
        code(google_maps::places_new::jiff_error),
        severity(Error),
        help("check that the date-time string is in valid ISO 8601 format")
    )]
    #[error("jiff date-time error: {source}")]
    Jiff {
        #[from]
        #[source]
        source: jiff::Error,
    },

    /// URL parse operation failed.
    #[diagnostic(
        code(google_maps::places_new::url_error),
        severity(Error),
        help("check that URL string is in valid format")
    )]
    #[error("URL parse error: {source}")]
    Url {
        #[from]
        #[source]
        source: url::ParseError,
    },

    /// Integer conversion failed.
    ///
    /// Occurs when converting between integer types of different sizes or signedness fails due to
    /// value overflow or underflow.
    #[diagnostic(
        code(google_maps::places_new::int_conversion_error),
        severity(Error),
        help("the value is outside the valid range for the target integer type")
    )]
    #[error("integer conversion error: {source}")]
    StdNumTryFromInt {
        #[from]
        #[source]
        source: std::num::TryFromIntError,
    },

    /// The error type for errors that can never happen.
    ///
    /// This error is guaranteed to be unreachable.
    #[error("an error that can never happen")]
    StdConvertInfallible {
        #[from]
        #[source]
        source: std::convert::Infallible,
    },
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Error {
    // Helper functions for creating errors with proper spans
    #[must_use]
    pub fn from_invalid_latitude(latitude: Decimal, longitude: Decimal) -> Self {
        let coordinate_string = format!("{latitude},{longitude}");
        let span = (0, latitude.to_string().len());
        Self::InvalidLatitude {
            latitude,
            longitude,
            coordinate_string,
            span,
        }
    }

    #[must_use]
    pub fn from_invalid_longitude(latitude: Decimal, longitude: Decimal) -> Self {
        let coordinate_string = format!("{latitude},{longitude}");
        let lat_len = latitude.to_string().len();
        let span = (lat_len + 1, coordinate_string.len());
        Self::InvalidLongitude {
            latitude,
            longitude,
            coordinate_string,
            span,
        }
    }

    #[must_use]
    pub fn from_invalid_latlng_string(input: String) -> Self {
        let span = (0, input.len());
        Self::InvalidLatLongString {
            input: input.clone(),
            debug: input,
            span,
        }
    }

    #[must_use]
    pub fn from_float_conversion(value: impl std::fmt::Display, field_name: &str) -> Self {
        let value_str = value.to_string();
        let debug = format!("{field_name}: {value_str}");
        let span = (field_name.len() + 2, debug.len());
        Self::FloatToDecimalConversionError {
            value: value_str,
            debug,
            span,
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl crate::traits::ClassifiableError<'_, Self> for Error {
    /// Classifies errors as transient or permanent for retry logic.
    ///
    /// Transient errors may succeed on retry (network issues, rate limits, service unavailability).
    /// Permanent errors require changes to the request or system state before retrying
    /// (authentication, invalid arguments, not found).
    fn classify(&self) -> ClassifiedError<'_, Self> {
        match self {
            #[cfg(feature = "places-new-autocomplete")]
            Self::Autocomplete { source, .. } =>
                if source.classify().is_transient() {
                    ClassifiedError::Transient(self)
                } else {
                    ClassifiedError::Permanent(self)
                },
            #[cfg(feature = "places-new-place-photos")]
            Self::PlacePhotos { source, .. } =>
                if source.classify().is_transient() {
                    ClassifiedError::Transient(self)
                } else {
                    ClassifiedError::Permanent(self)
                },
            #[cfg(feature = "places-new-text-search")]
            Self::TextSearch { source, .. } =>
                if source.classify().is_transient() {
                    ClassifiedError::Transient(self)
                } else {
                    ClassifiedError::Permanent(self)
                },
            #[cfg(feature = "places-new-nearby-search")]
            Self::NearbySearch { source, .. } =>
                if source.classify().is_transient() {
                    ClassifiedError::Transient(self)
                } else {
                    ClassifiedError::Permanent(self)
                },
            Self::Api { source, .. } =>
                if source.classify().is_transient() {
                    ClassifiedError::Transient(self)
                } else {
                    ClassifiedError::Permanent(self)
                },
            Self::FloatToDecimalConversionError { .. } |
            Self::InvalidCircleRadius { .. } |
            Self::InvalidViewport { .. } |
            Self::InvalidLatitude { .. } |
            Self::InvalidLongitude { .. } |
            Self::InvalidLatLongString { .. } |
            Self::InvalidLatLongTuple { .. } |
            Self::Decimal { .. } |
            Self::Jiff { .. } |
            Self::Url { .. } |
            Self::StdNumTryFromInt { .. } |
            Self::StdConvertInfallible { .. } => ClassifiedError::Permanent(self),
        }
    }
}