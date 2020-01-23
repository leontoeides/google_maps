//! Contains the `ManeuverType` enum and its associated traits. `ManeuverType`
//! is often used to determine which icon to display for the current step.

use crate::directions::error::Error;
use serde::{Serialize, Deserialize};

/// The action to take for the current step (turn left, merge, straight, etc.).
/// This field is used to determine which icon to display. Values in this
/// [list](https://developers.google.com/maps/documentation/directions/intro#Steps)
/// are subject to change.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum ManeuverType {
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

impl std::convert::From<&ManeuverType> for String {

    /// Converts a `ManeuverType` enum to a `String` that contains a [maneuver
    /// type](https://developers.google.com/maps/documentation/directions/intro#Steps)
    /// code.

    fn from(maneuver_type: &ManeuverType) -> String {
        match maneuver_type {
            ManeuverType::Ferry => String::from("ferry"),
            ManeuverType::FerryTrain => String::from("ferry-train"),
            ManeuverType::ForkLeft => String::from("fork-left"),
            ManeuverType::ForkRight => String::from("fork-right"),
            ManeuverType::KeepLeft => String::from("keep-left"),
            ManeuverType::KeepRight => String::from("keep-right"),
            ManeuverType::Merge => String::from("merge"),
            ManeuverType::RampLeft => String::from("ramp-left"),
            ManeuverType::RampRight => String::from("ramp-right"),
            ManeuverType::RoundaboutLeft => String::from("roundabout-left"),
            ManeuverType::RoundaboutRight => String::from("roundabout-right"),
            ManeuverType::Straight => String::from("straight"),
            ManeuverType::TurnLeft => String::from("turn-left"),
            ManeuverType::TurnRight => String::from("turn-right"),
            ManeuverType::TurnSharpLeft => String::from("turn-sharp-left"),
            ManeuverType::TurnSharpRight => String::from("turn-sharp-right"),
            ManeuverType::TurnSlightLeft => String::from("turn-slight-left"),
            ManeuverType::TurnSlightRight => String::from("turn-slight-right"),
            ManeuverType::UturnLeft => String::from("uturn-left"),
            ManeuverType::UturnRight => String::from("uturn-right"),
        } // match
    } // fn

} // impl

impl std::convert::TryFrom<String> for ManeuverType {

    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.

    type Error = crate::directions::error::Error;

    /// Gets a `ManeuverType` enum from a `String` that contains a valid [maneuver
    /// type](https://developers.google.com/maps/documentation/directions/intro#Steps)
    /// code.

    fn try_from(maneuver_type: String) -> Result<ManeuverType, Error> {
        match maneuver_type.as_ref() {
            "ferry" => Ok(ManeuverType::Ferry),
            "ferry-train" => Ok(ManeuverType::FerryTrain),
            "fork-left" => Ok(ManeuverType::ForkLeft),
            "fork-right" => Ok(ManeuverType::ForkRight),
            "keep-left" => Ok(ManeuverType::KeepLeft),
            "keep-right" => Ok(ManeuverType::KeepRight),
            "merge" => Ok(ManeuverType::Merge),
            "ramp-left" => Ok(ManeuverType::RampLeft),
            "ramp-right" => Ok(ManeuverType::RampRight),
            "roundabout-left" => Ok(ManeuverType::RoundaboutLeft),
            "roundabout-right" => Ok(ManeuverType::RoundaboutRight),
            "straight" => Ok(ManeuverType::Straight),
            "turn-left" => Ok(ManeuverType::TurnLeft),
            "turn-right" => Ok(ManeuverType::TurnRight),
            "turn-sharp-left" => Ok(ManeuverType::TurnSharpLeft),
            "turn-sharp-right" => Ok(ManeuverType::TurnSharpRight),
            "turn-slight-left" => Ok(ManeuverType::TurnSlightLeft),
            "turn-slight-right" => Ok(ManeuverType::TurnSlightRight),
            "uturn-left" => Ok(ManeuverType::UturnLeft),
            "uturn-right" => Ok(ManeuverType::UturnRight),
            _ => Err(Error::InvalidManeuverTypeCode(maneuver_type)),
        } // match
    } // fn

} // impl

impl std::default::Default for ManeuverType {

    /// Returns a reasonable default variant for the `ManeuverType` enum type.

    fn default() -> Self {
        ManeuverType::Straight
    } // fn

} // impl

impl std::fmt::Display for ManeuverType {

    /// Formats a `ManeuverType` enum into a string that is presentable to the
    /// end user.

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ManeuverType::Ferry => write!(f, "Ferry"),
            ManeuverType::FerryTrain => write!(f, "Ferry Train"),
            ManeuverType::ForkLeft => write!(f, "Fork Left"),
            ManeuverType::ForkRight => write!(f, "Fork Right"),
            ManeuverType::KeepLeft => write!(f, "Keep Left"),
            ManeuverType::KeepRight => write!(f, "Keep Right"),
            ManeuverType::Merge => write!(f, "Merge"),
            ManeuverType::RampLeft => write!(f, "Ramp Left"),
            ManeuverType::RampRight => write!(f, "Ramp Right"),
            ManeuverType::RoundaboutLeft => write!(f, "Roundabout Left"),
            ManeuverType::RoundaboutRight => write!(f, "Roundabout Right"),
            ManeuverType::Straight => write!(f, "Straight"),
            ManeuverType::TurnLeft => write!(f, "Turn Left"),
            ManeuverType::TurnRight => write!(f, "Turn Right"),
            ManeuverType::TurnSharpLeft => write!(f, "Turn Sharp Left"),
            ManeuverType::TurnSharpRight => write!(f, "Turn Sharp Right"),
            ManeuverType::TurnSlightLeft => write!(f, "Turn Slight Left"),
            ManeuverType::TurnSlightRight => write!(f, "Turn Slight Right"),
            ManeuverType::UturnLeft => write!(f, "U-turn Left"),
            ManeuverType::UturnRight => write!(f, "U-turn Right"),
        } // match
    } // fn

} // impl