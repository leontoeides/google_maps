//! Contains the `GeocoderStatus` enum and its associated traits. It indicates
//! the status resulting from geocoding operations.

use crate::directions::error::Error;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize};

// -----------------------------------------------------------------------------
//
/// Indicates the [status
/// code](https://developers.google.com/maps/documentation/directions/intro#GeocodedWaypoints)
/// resulting from the geocoding operation.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE", deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum GeocoderStatus {
    /// Indicates that no errors occurred; the address was successfully parsed
    /// and at least one geocode was returned.
    #[serde(alias = "Ok")]
    Ok,

    /// Indicates that the geocode was successful but returned no results. This
    /// may occur if the geocoder was passed a non-existent `address`.
    #[serde(alias = "ZeroResults")]
    ZeroResults,

    /// Indicates that the request could not be processed due to a server error.
    /// The request may succeed if you try again.
    #[serde(alias = "UnknownError")]
    UnknownError,
} // struct

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for GeocoderStatus {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match Self::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string())),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&GeocoderStatus> for String {
    /// Converts a `GeocoderStatus` enum to a `String` that contains a [geocoder
    /// status](https://developers.google.com/maps/documentation/directions/intro#GeocodedWaypoints)
    /// code.
    fn from(geocoder_status: &GeocoderStatus) -> Self {
        match geocoder_status {
            GeocoderStatus::Ok => Self::from("OK"),
            GeocoderStatus::ZeroResults => Self::from("ZERO_RESULTS"),
            GeocoderStatus::UnknownError => Self::from("UNKNOWN_ERROR"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static GEOCODER_STATUSES_BY_CODE: phf::Map<&'static str, GeocoderStatus> = phf_map! {
    "OK" => GeocoderStatus::Ok,
    "ZERO_RESULTS" => GeocoderStatus::ZeroResults,
    "UNKNOWN_ERROR" => GeocoderStatus::UnknownError,
};

impl std::convert::TryFrom<&str> for GeocoderStatus {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;
    /// Gets a `GeocoderStatus` enum from a `String` that contains a valid
    /// [geocoder
    /// status](https://developers.google.com/maps/documentation/directions/intro#GeocodedWaypoints)
    /// code.
    fn try_from(geocoder_status_code: &str) -> Result<Self, Self::Error> {
        GEOCODER_STATUSES_BY_CODE
            .get(geocoder_status_code)
            .cloned()
            .ok_or_else(|| Error::InvalidGeocoderStatusCode(geocoder_status_code.to_string()))
    } // fn
} // impl

impl std::str::FromStr for GeocoderStatus {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Err = crate::directions::error::Error;
    /// Gets a `GeocoderStatus` enum from a `String` that contains a valid
    /// [geocoder
    /// status](https://developers.google.com/maps/documentation/directions/intro#GeocodedWaypoints)
    /// code.
    fn from_str(geocoder_status_code: &str) -> Result<Self, Self::Err> {
        GEOCODER_STATUSES_BY_CODE
            .get(geocoder_status_code)
            .cloned()
            .ok_or_else(|| Error::InvalidGeocoderStatusCode(geocoder_status_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for GeocoderStatus {
    /// Returns a reasonable default variant for the `GeocoderStatus` enum type.
    fn default() -> Self {
        Self::Ok
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for GeocoderStatus {
    /// Formats a `GeocoderStatus` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "OK"),
            Self::ZeroResults => write!(f, "Zero Results"),
            Self::UnknownError => write!(f, "Unknown Error"),
        } // match
    } // fn
} // impl
