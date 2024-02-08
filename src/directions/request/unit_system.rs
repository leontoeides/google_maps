//! Contains the `UnitSystem` enum and its associated traits. It is used specify
//! whether imperial or metric units are used in Directions responses.

use crate::directions::error::Error as DirectionsError;
use crate::error::Error as GoogleMapsError;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------

/// Specifies the [unit
/// system](https://developers.google.com/maps/documentation/directions/intro#UnitSystems)
/// to use when displaying results.
///
/// Directions results contain `text` within `distance` fields that may be
/// displayed to the user to indicate the distance of a particular "step" of the
/// route. By default, this text uses the unit system of the origin's country or
/// region.
///
/// For example, a route from "Chicago, IL" to "Toronto, ONT" will display
/// results in miles, while the reverse route will display results in
/// kilometers. You may override this unit system by setting one explicitly
/// within the request's `units` parameter, passing one of the following values:
///
/// **Note**: this unit system setting only affects the `text` displayed within
/// `distance` fields. The `distance` fields also contain `values` which are
/// always expressed in meters.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum UnitSystem {
    /// Specifies that distances in the response should be expressed in imperial
    /// units, miles and feet.
    Imperial = 0,
    /// Specifies that distances in the response should be expressed in metric
    /// units, using kilometres and metres.
    #[default]
    Metric = 1,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for UnitSystem {
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

impl Serialize for UnitSystem {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&UnitSystem> for &str {
    /// Converts a `UnitSystem` enum to a `String` that contains a [unit
    /// system](https://developers.google.com/maps/documentation/directions/intro#UnitSystems)
    /// code.
    fn from(units: &UnitSystem) -> Self {
        match units {
            UnitSystem::Imperial => "imperial",
            UnitSystem::Metric => "metric",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for UnitSystem {
    /// Converts a `UnitSystem` enum to a `String` that contains a [unit
    /// system](https://developers.google.com/maps/documentation/directions/intro#UnitSystems)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&UnitSystem> for String {
    /// Converts a `UnitSystem` enum to a `String` that contains a [unit
    /// system](https://developers.google.com/maps/documentation/directions/intro#UnitSystems)
    /// code.
    fn from(unit_system: &UnitSystem) -> Self {
        std::convert::Into::<&str>::into(unit_system).to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

static UNIT_SYSTEMS_BY_CODE: phf::Map<&'static str, UnitSystem> = phf_map! {
    "imperial" => UnitSystem::Imperial,
    "metric" => UnitSystem::Metric,
};

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for UnitSystem {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = GoogleMapsError;
    /// Gets a `UnitSystem` enum from a `String` that contains a valid [unit
    /// system](https://developers.google.com/maps/documentation/directions/intro#UnitSystems)
    /// code.
    fn try_from(unit_system_code: &str) -> Result<Self, Self::Error> {
        Ok(UNIT_SYSTEMS_BY_CODE
            .get(unit_system_code)
            .cloned()
            .ok_or_else(|| DirectionsError::InvalidUnitSystemCode(unit_system_code.to_string()))?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for UnitSystem {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Err = GoogleMapsError;
    /// Gets a `UnitSystem` enum from a `String` that contains a valid [unit
    /// system](https://developers.google.com/maps/documentation/directions/intro#UnitSystems)
    /// code.
    fn from_str(unit_system_code: &str) -> Result<Self, Self::Err> {
        Ok(UNIT_SYSTEMS_BY_CODE
            .get(unit_system_code)
            .cloned()
            .ok_or_else(|| DirectionsError::InvalidUnitSystemCode(unit_system_code.to_string()))?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl UnitSystem {
    /// Formats a `UnitSystem` enum into a string that is presentable to the
    /// end user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::Imperial => "Imperial",
            Self::Metric => "Metric",
        } // match
    } // fn
} // impl
