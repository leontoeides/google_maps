//! Contains the `LocationType` enum and its associated traits. It specifies the
//! nature and accuracy of the Geocoding response.

use crate::geocoding::error::Error;
use serde::{Serialize, Deserialize};

/// Stores additional data about the specified location.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
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

impl std::convert::TryFrom<String> for LocationType {

    // Error definitions are contained in the
    // `google_maps\src\geocoding\error.rs` module.
    type Error = crate::geocoding::error::Error;

    /// Gets a `LocationType` enum from a `String` that contains a supported
    /// [location
    /// type](https://developers.google.com/maps/documentation/geocoding/intro#Results)
    /// code.
    fn try_from(location_type: String) -> Result<LocationType, Error> {
        match location_type.as_ref() {
            "APPROXIMATE" => Ok(LocationType::Approximate),
            "GEOMETRIC_CENTER" => Ok(LocationType::GeometricCenter),
            "RANGE_INTERPOLATED" => Ok(LocationType::RangeInterpolated),
            "ROOFTOP" => Ok(LocationType::RoofTop),
            _ => Err(Error::InvalidLocationTypeCode(location_type)),
        } // match
    } // fn

} // impl

impl std::default::Default for LocationType {
    /// Returns a reasonable default variant for the `LocationType` enum type.
    fn default() -> Self {
        LocationType::Approximate
    } // fn
} // impl

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