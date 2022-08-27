//! Contains the `DrivingManeuver` enum and its associated traits. It is often used
//! to determine which icon to display for the current step.

use crate::directions::error::Error;
use phf::phf_map;
use serde::{Deserialize, Serialize, Deserializer};

// -----------------------------------------------------------------------------

/// The action to take for the current step (turn left, merge, straight, etc.).
/// This field is used to determine which icon to display. Values in this
/// [list](https://developers.google.com/maps/documentation/directions/intro#Steps)
/// are subject to change.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DrivingManeuver {
    #[serde(alias = "ferry")]
    Ferry,
    #[serde(alias = "ferry-train")]
    FerryTrain,
    #[serde(alias = "fork-left")]
    ForkLeft,
    #[serde(alias = "fork-right")]
    ForkRight,
    #[serde(alias = "keep-left")]
    KeepLeft,
    #[serde(alias = "keep-right")]
    KeepRight,
    #[serde(alias = "merge")]
    Merge,
    #[serde(alias = "ramp")]
    Ramp,
    #[serde(alias = "ramp-left")]
    RampLeft,
    #[serde(alias = "ramp-right")]
    RampRight,
    #[serde(alias = "roundabout-left")]
    RoundaboutLeft,
    #[serde(alias = "roundabout-right")]
    RoundaboutRight,
    #[serde(alias = "straight")]
    Straight,
    #[serde(alias = "turn-left")]
    TurnLeft,
    #[serde(alias = "turn-right")]
    TurnRight,
    #[serde(alias = "turn-sharp-left")]
    TurnSharpLeft,
    #[serde(alias = "turn-sharp-right")]
    TurnSharpRight,
    #[serde(alias = "turn-slight-left")]
    TurnSlightLeft,
    #[serde(alias = "turn-slight-right")]
    TurnSlightRight,
    #[serde(alias = "uturn-left")]
    UturnLeft,
    #[serde(alias = "uturn-right")]
    UturnRight,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for DrivingManeuver {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match DrivingManeuver::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string()))
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&DrivingManeuver> for String {
    /// Converts a `DrivingManeuver` enum to a `String` that contains a
    /// [maneuver
    /// type](https://developers.google.com/maps/documentation/directions/intro#Steps)
    /// code.
    fn from(maneuver_type: &DrivingManeuver) -> String {
        match maneuver_type {
            DrivingManeuver::Ferry => String::from("ferry"),
            DrivingManeuver::FerryTrain => String::from("ferry-train"),
            DrivingManeuver::ForkLeft => String::from("fork-left"),
            DrivingManeuver::ForkRight => String::from("fork-right"),
            DrivingManeuver::KeepLeft => String::from("keep-left"),
            DrivingManeuver::KeepRight => String::from("keep-right"),
            DrivingManeuver::Merge => String::from("merge"),
            DrivingManeuver::Ramp => String::from("ramp"),
            DrivingManeuver::RampLeft => String::from("ramp-left"),
            DrivingManeuver::RampRight => String::from("ramp-right"),
            DrivingManeuver::RoundaboutLeft => String::from("roundabout-left"),
            DrivingManeuver::RoundaboutRight => String::from("roundabout-right"),
            DrivingManeuver::Straight => String::from("straight"),
            DrivingManeuver::TurnLeft => String::from("turn-left"),
            DrivingManeuver::TurnRight => String::from("turn-right"),
            DrivingManeuver::TurnSharpLeft => String::from("turn-sharp-left"),
            DrivingManeuver::TurnSharpRight => String::from("turn-sharp-right"),
            DrivingManeuver::TurnSlightLeft => String::from("turn-slight-left"),
            DrivingManeuver::TurnSlightRight => String::from("turn-slight-right"),
            DrivingManeuver::UturnLeft => String::from("uturn-left"),
            DrivingManeuver::UturnRight => String::from("uturn-right"),
        } // match
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

impl std::convert::TryFrom<&str> for DrivingManeuver {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;
    /// Gets a `DrivingManeuver` enum from a `String` that contains a valid
    /// [maneuver
    /// type](https://developers.google.com/maps/documentation/directions/intro#Steps)
    /// code.
    fn try_from(driving_maneuver_type_code: &str) -> Result<DrivingManeuver, Error> {
        DRIVING_MANEUVERS_BY_CODE
            .get(driving_maneuver_type_code)
            .cloned()
            .ok_or_else(|| Error::InvalidDrivingManeuverCode(driving_maneuver_type_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for DrivingManeuver {
    /// Returns a reasonable default variant for the `DrivingManeuver` enum
    /// type.
    fn default() -> Self {
        DrivingManeuver::Straight
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for DrivingManeuver {
    /// Formats a `DrivingManeuver` enum into a string that is presentable to
    /// the end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DrivingManeuver::Ferry => write!(f, "Ferry"),
            DrivingManeuver::FerryTrain => write!(f, "Ferry Train"),
            DrivingManeuver::ForkLeft => write!(f, "Fork Left"),
            DrivingManeuver::ForkRight => write!(f, "Fork Right"),
            DrivingManeuver::KeepLeft => write!(f, "Keep Left"),
            DrivingManeuver::KeepRight => write!(f, "Keep Right"),
            DrivingManeuver::Merge => write!(f, "Merge"),
            DrivingManeuver::Ramp => write!(f, "Ramp"),
            DrivingManeuver::RampLeft => write!(f, "Ramp Left"),
            DrivingManeuver::RampRight => write!(f, "Ramp Right"),
            DrivingManeuver::RoundaboutLeft => write!(f, "Roundabout Left"),
            DrivingManeuver::RoundaboutRight => write!(f, "Roundabout Right"),
            DrivingManeuver::Straight => write!(f, "Straight"),
            DrivingManeuver::TurnLeft => write!(f, "Turn Left"),
            DrivingManeuver::TurnRight => write!(f, "Turn Right"),
            DrivingManeuver::TurnSharpLeft => write!(f, "Turn Sharp Left"),
            DrivingManeuver::TurnSharpRight => write!(f, "Turn Sharp Right"),
            DrivingManeuver::TurnSlightLeft => write!(f, "Turn Slight Left"),
            DrivingManeuver::TurnSlightRight => write!(f, "Turn Slight Right"),
            DrivingManeuver::UturnLeft => write!(f, "U-turn Left"),
            DrivingManeuver::UturnRight => write!(f, "U-turn Right"),
        } // match
    } // fn
} // impl