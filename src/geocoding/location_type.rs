//! Contains the `LocationType` enum and its associated traits. It specifies the
//! nature and accuracy of the Geocoding response.

use crate::geocoding::error::Error;
use phf::phf_map;
use serde::{Deserialize, Serialize, Deserializer};

// -----------------------------------------------------------------------------

/// Stores additional data about the specified location.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LocationType {
    /// Indicates that the returned result is approximate.
    #[serde(alias = "APPROXIMATE")]
    Approximate,
    /// Indicates that the returned result is the geometric center of a result
    /// such as a polyline (for example, a street) or polygon (region).
    #[serde(alias = "GEOMETRIC_CENTER")]
    GeometricCenter,
    /// Indicates that the returned result reflects an approximation (usually on
    /// a road) interpolated between two precise points (such as intersections).
    /// Interpolated results are generally returned when rooftop geocodes are
    /// unavailable for a street address.
    #[serde(alias = "RANGE_INTERPOLATED")]
    RangeInterpolated,
    /// Indicates that the returned result is a precise geocode for which we
    /// have location information accurate down to street address precision.
    #[serde(alias = "ROOFTOP")]
    RoofTop,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for LocationType {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match LocationType::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string()))
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&LocationType> for String {
    /// Converts a `LocationType` enum to a `String` that contains a [location
    /// type](https://developers.google.com/maps/documentation/geocoding/intro#Results) code.
    fn from(location_type: &LocationType) -> String {
        match location_type {
            LocationType::Approximate => String::from("APPROXIMATE"),
            LocationType::GeometricCenter => String::from("GEOMETRIC_CENTER"),
            LocationType::RangeInterpolated => String::from("RANGE_INTERPOLATED"),
            LocationType::RoofTop => String::from("ROOFTOP"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static LOCATION_TYPES_BY_CODE: phf::Map<&'static str, LocationType> = phf_map! {
    "APPROXIMATE" => LocationType::Approximate,
    "GEOMETRIC_CENTER" => LocationType::GeometricCenter,
    "RANGE_INTERPOLATED" => LocationType::RangeInterpolated,
    "ROOFTOP" => LocationType::RoofTop,
};

impl std::convert::TryFrom<&str> for LocationType {
    // Error definitions are contained in the
    // `google_maps\src\geocoding\error.rs` module.
    type Error = crate::geocoding::error::Error;
    /// Gets a `LocationType` enum from a `String` that contains a supported
    /// [location
    /// type](https://developers.google.com/maps/documentation/geocoding/intro#Results)
    /// code.
    fn try_from(location_code: &str) -> Result<LocationType, Error> {
        LOCATION_TYPES_BY_CODE
            .get(location_code)
            .cloned()
            .ok_or_else(|| Error::InvalidLocationTypeCode(location_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for LocationType {
    /// Returns a reasonable default variant for the `LocationType` enum type.
    fn default() -> Self {
        LocationType::Approximate
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for LocationType {
    /// Formats a `LocationType` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LocationType::Approximate => write!(f, "Approximate"),
            LocationType::GeometricCenter => write!(f, "Geometric Center"),
            LocationType::RangeInterpolated => write!(f, "Range Interpolated"),
            LocationType::RoofTop => write!(f, "Roof Top"),
        } // match
    } // fn
} // impl