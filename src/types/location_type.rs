//! Contains the `LocationType` enum and its associated traits. It specifies the
//! nature and accuracy of the Geocoding response.

use crate::error::Error as GoogleMapsError;
use crate::types::Error as TypeError;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------

/// Stores additional data about the specified location.

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum LocationType {
    /// Indicates that the returned result is approximate.
    #[default]
    Approximate = 0,
    /// Indicates that the returned result is the geometric center of a result
    /// such as a polyline (for example, a street) or polygon (region).
    GeometricCenter = 1,
    /// Indicates that the returned result reflects an approximation (usually on
    /// a road) interpolated between two precise points (such as intersections).
    /// Interpolated results are generally returned when rooftop geocodes are
    /// unavailable for a street address.
    RangeInterpolated = 2,
    /// Indicates that the returned result is a precise geocode for which we
    /// have location information accurate down to street address precision.
    RoofTop = 3,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for LocationType {
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

impl Serialize for LocationType {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Self> for LocationType {
    /// Converts a borrowed `&LocationType` enum into an owned `LocationType`
    /// enum by copying it.
    fn from(location_type: &Self) -> Self {
        *location_type
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&LocationType> for &str {
    /// Converts a `LocationType` enum to a `String` that contains a [location
    /// type](https://developers.google.com/maps/documentation/geocoding/intro#Results) code.
    fn from(location_type: &LocationType) -> Self {
        match location_type {
            LocationType::Approximate => "APPROXIMATE",
            LocationType::GeometricCenter => "GEOMETRIC_CENTER",
            LocationType::RangeInterpolated => "RANGE_INTERPOLATED",
            LocationType::RoofTop => "ROOFTOP",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for LocationType {
    /// Converts a `LocationType` enum to a `String` that contains a [location
    /// type](https://developers.google.com/maps/documentation/geocoding/intro#Results) code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&LocationType> for String {
    /// Converts a `LocationType` enum to a `String` that contains a [location
    /// type](https://developers.google.com/maps/documentation/geocoding/intro#Results) code.
    fn from(location_type: &LocationType) -> Self {
        std::convert::Into::<&str>::into(location_type).to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

static LOCATION_TYPES_BY_CODE: phf::Map<&'static str, LocationType> = phf_map! {
    "APPROXIMATE" => LocationType::Approximate,
    "GEOMETRIC_CENTER" => LocationType::GeometricCenter,
    "RANGE_INTERPOLATED" => LocationType::RangeInterpolated,
    "ROOFTOP" => LocationType::RoofTop,
};

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for LocationType {
    // Error definitions are contained in the
    // `google_maps\src\geocoding\error.rs` module.
    type Error = GoogleMapsError;
    /// Gets a `LocationType` enum from a `String` that contains a supported
    /// [location
    /// type](https://developers.google.com/maps/documentation/geocoding/intro#Results)
    /// code.
    fn try_from(location_code: &str) -> Result<Self, Self::Error> {
        Ok(LOCATION_TYPES_BY_CODE
            .get(location_code)
            .copied()
            .ok_or_else(|| TypeError::InvalidLocationTypeCode(location_code.to_string()))?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for LocationType {
    // Error definitions are contained in the
    // `google_maps\src\geocoding\error.rs` module.
    type Err = GoogleMapsError;
    /// Gets a `LocationType` enum from a `String` that contains a supported
    /// [location
    /// type](https://developers.google.com/maps/documentation/geocoding/intro#Results)
    /// code.
    fn from_str(location_code: &str) -> Result<Self, Self::Err> {
        Ok(LOCATION_TYPES_BY_CODE
            .get(location_code)
            .copied()
            .ok_or_else(|| TypeError::InvalidLocationTypeCode(location_code.to_string()))?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl LocationType {
    /// Formats a `LocationType` enum into a string that is presentable to the
    /// end user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::Approximate => "Approximate",
            Self::GeometricCenter => "Geometric Center",
            Self::RangeInterpolated => "Range Interpolated",
            Self::RoofTop => "Roof Top",
        } // match
    } // fn
} // impl
