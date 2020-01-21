use crate::directions::error::Error;
use serde::{Serialize, Deserialize};

/// Indicates the [status
/// code](https://developers.google.com/maps/documentation/directions/intro#GeocodedWaypoints)
/// resulting from the geocoding operation.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum GeocoderStatus {
    /// Indicates that no errors occurred; the address was successfully parsed
    /// and at least one geocode was returned.
    #[serde(alias = "OK")]
    Ok,
    /// Indicates that the geocode was successful but returned no results. This
    /// may occur if the geocoder was passed a non-existent `address`.
    #[serde(alias = "ZERO_RESULTS")]
    ZeroResults,
} // struct

impl std::convert::From<&GeocoderStatus> for String {
    /// Converts a `GeocoderStatus` enum to a `String` that contains a [geocoder
    /// status](https://developers.google.com/maps/documentation/directions/intro#GeocodedWaypoints)
    /// code.
    fn from(geocoder_status: &GeocoderStatus) -> String {
        match geocoder_status {
            GeocoderStatus::Ok => String::from("OK"),
            GeocoderStatus::ZeroResults => String::from("ZERO_RESULTS"),
        } // match
    } // fn
} // impl

impl std::convert::TryFrom<String> for GeocoderStatus {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;
    /// Gets a `GeocoderStatus` enum from a `String` that contains a valid
    /// [geocoder
    /// status](https://developers.google.com/maps/documentation/directions/intro#GeocodedWaypoints)
    /// code.
    fn try_from(geocoder_status: String) -> Result<GeocoderStatus, Error> {
        match geocoder_status.as_ref() {
            "OK" => Ok(GeocoderStatus::Ok),
            "ZERO_RESULTS" => Ok(GeocoderStatus::ZeroResults),
            _ => Err(Error::InvalidGeocoderStatusCode(geocoder_status)),
        } // match
    } // fn
} // impl

impl std::default::Default for GeocoderStatus {
    /// Returns a reasonable default variant for the `GeocoderStatus` enum type.
    fn default() -> Self {
        GeocoderStatus::Ok
    } // fn
} // impl

impl std::fmt::Display for GeocoderStatus {
    /// Formats a `GeocoderStatus` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GeocoderStatus::Ok => write!(f, "OK"),
            GeocoderStatus::ZeroResults => write!(f, "Zero Results"),
        } // match
    } // fn
} // impl