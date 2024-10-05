//! Contains the `DrivingManeuver` enum and its associated traits. It is often used
//! to determine which icon to display for the current step.

use crate::directions::error::Error as DirectionsError;
use crate::error::Error as GoogleMapsError;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------

/// The action to take for the current step (turn left, merge, straight, etc.).
///
/// This field is used to determine which icon to display. Values in this
/// [list](https://developers.google.com/maps/documentation/directions/intro#Steps)
/// are subject to change.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum DrivingManeuver {
    Ferry = 0,
    FerryTrain = 1,
    ForkLeft = 2,
    ForkRight = 3,
    KeepLeft = 4,
    KeepRight = 5,
    Merge = 6,
    Ramp = 7,
    RampLeft = 8,
    RampRight = 9,
    RoundaboutLeft = 10,
    RoundaboutRight = 11,
    #[default]
    Straight = 12,
    TurnLeft = 13,
    TurnRight = 14,
    TurnSharpLeft = 15,
    TurnSharpRight = 16,
    TurnSlightLeft = 17,
    TurnSlightRight = 18,
    UturnLeft = 19,
    UturnRight = 20,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for DrivingManeuver {
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

impl Serialize for DrivingManeuver {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&DrivingManeuver> for &str {
    /// Converts a `DrivingManeuver` enum to a `String` that contains a
    /// [maneuver
    /// type](https://developers.google.com/maps/documentation/directions/intro#Steps)
    /// code.
    fn from(maneuver_type: &DrivingManeuver) -> Self {
        match maneuver_type {
            DrivingManeuver::Ferry => "ferry",
            DrivingManeuver::FerryTrain => "ferry-train",
            DrivingManeuver::ForkLeft => "fork-left",
            DrivingManeuver::ForkRight => "fork-right",
            DrivingManeuver::KeepLeft => "keep-left",
            DrivingManeuver::KeepRight => "keep-right",
            DrivingManeuver::Merge => "merge",
            DrivingManeuver::Ramp => "ramp",
            DrivingManeuver::RampLeft => "ramp-left",
            DrivingManeuver::RampRight => "ramp-right",
            DrivingManeuver::RoundaboutLeft => "roundabout-left",
            DrivingManeuver::RoundaboutRight => "roundabout-right",
            DrivingManeuver::Straight => "straight",
            DrivingManeuver::TurnLeft => "turn-left",
            DrivingManeuver::TurnRight => "turn-right",
            DrivingManeuver::TurnSharpLeft => "turn-sharp-left",
            DrivingManeuver::TurnSharpRight => "turn-sharp-right",
            DrivingManeuver::TurnSlightLeft => "turn-slight-left",
            DrivingManeuver::TurnSlightRight => "turn-slight-right",
            DrivingManeuver::UturnLeft => "uturn-left",
            DrivingManeuver::UturnRight => "uturn-right",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for DrivingManeuver {
    /// Converts a `DrivingManeuver` enum to a `String` that contains a
    /// [maneuver
    /// type](https://developers.google.com/maps/documentation/directions/intro#Steps)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&DrivingManeuver> for String {
    /// Converts a `DrivingManeuver` enum to a `String` that contains a
    /// [maneuver
    /// type](https://developers.google.com/maps/documentation/directions/intro#Steps)
    /// code.
    fn from(driving_maneuver: &DrivingManeuver) -> Self {
        std::convert::Into::<&str>::into(driving_maneuver).to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

static DRIVING_MANEUVERS_BY_CODE: phf::Map<&'static str, DrivingManeuver> = phf_map! {
    "ferry" => DrivingManeuver::Ferry,
    "ferry-train" => DrivingManeuver::FerryTrain,
    "fork-left" => DrivingManeuver::ForkLeft,
    "fork-right" => DrivingManeuver::ForkRight,
    "keep-left" => DrivingManeuver::KeepLeft,
    "keep-right" => DrivingManeuver::KeepRight,
    "merge" => DrivingManeuver::Merge,
    "ramp" => DrivingManeuver::Ramp,
    "ramp-left" => DrivingManeuver::RampLeft,
    "ramp-right" => DrivingManeuver::RampRight,
    "roundabout-left" => DrivingManeuver::RoundaboutLeft,
    "roundabout-right" => DrivingManeuver::RoundaboutRight,
    "straight" => DrivingManeuver::Straight,
    "turn-left" => DrivingManeuver::TurnLeft,
    "turn-right" => DrivingManeuver::TurnRight,
    "turn-sharp-left" => DrivingManeuver::TurnSharpLeft,
    "turn-sharp-right" => DrivingManeuver::TurnSharpRight,
    "turn-slight-left" => DrivingManeuver::TurnSlightLeft,
    "turn-slight-right" => DrivingManeuver::TurnSlightRight,
    "uturn-left" => DrivingManeuver::UturnLeft,
    "uturn-right" => DrivingManeuver::UturnRight,
};

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for DrivingManeuver {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = GoogleMapsError;
    /// Gets a `DrivingManeuver` enum from a `String` that contains a valid
    /// [maneuver
    /// type](https://developers.google.com/maps/documentation/directions/intro#Steps)
    /// code.
    fn try_from(driving_maneuver_type_code: &str) -> Result<Self, Self::Error> {
        Ok(DRIVING_MANEUVERS_BY_CODE
            .get(driving_maneuver_type_code)
            .cloned()
            .ok_or_else(|| {
                DirectionsError::InvalidDrivingManeuverCode(driving_maneuver_type_code.to_string())
            })?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for DrivingManeuver {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Err = GoogleMapsError;
    /// Gets a `DrivingManeuver` enum from a `String` that contains a valid
    /// [maneuver
    /// type](https://developers.google.com/maps/documentation/directions/intro#Steps)
    /// code.
    fn from_str(driving_maneuver_type_code: &str) -> Result<Self, Self::Err> {
        Ok(DRIVING_MANEUVERS_BY_CODE
            .get(driving_maneuver_type_code)
            .cloned()
            .ok_or_else(|| {
                DirectionsError::InvalidDrivingManeuverCode(driving_maneuver_type_code.to_string())
            })?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl DrivingManeuver {
    /// Formats a `DrivingManeuver` enum into a string that is presentable to
    /// the end user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::Ferry => "Ferry",
            Self::FerryTrain => "Ferry Train",
            Self::ForkLeft => "Fork Left",
            Self::ForkRight => "Fork Right",
            Self::KeepLeft => "Keep Left",
            Self::KeepRight => "Keep Right",
            Self::Merge => "Merge",
            Self::Ramp => "Ramp",
            Self::RampLeft => "Ramp Left",
            Self::RampRight => "Ramp Right",
            Self::RoundaboutLeft => "Roundabout Left",
            Self::RoundaboutRight => "Roundabout Right",
            Self::Straight => "Straight",
            Self::TurnLeft => "Turn Left",
            Self::TurnRight => "Turn Right",
            Self::TurnSharpLeft => "Turn Sharp Left",
            Self::TurnSharpRight => "Turn Sharp Right",
            Self::TurnSlightLeft => "Turn Slight Left",
            Self::TurnSlightRight => "Turn Slight Right",
            Self::UturnLeft => "U-turn Left",
            Self::UturnRight => "U-turn Right",
        } // match
    } // fn
} // impl
