//! Contains the `VehicleType` enum and its associated traits. It specifies the
//! mode of transportation for transit directions.

use crate::directions::error::Error;
use phf::phf_map;
use serde::{Deserialize, Serialize, Deserializer};

// -----------------------------------------------------------------------------

/// Indicates the [vehicle
/// type](https://developers.google.com/maps/documentation/directions/intro#VehicleType)

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VehicleType {
    /// Bus.
    #[serde(alias = "BUS")]
    Bus,
    /// A vehicle that operates on a cable, usually on the ground. Aerial cable
    /// cars may be of the type VehicleType::Gondola.
    #[serde(alias = "CABLE_CAR")]
    CableCar,
    /// Commuter rail.
    #[serde(alias = "COMMUTER_TRAIN")]
    CommuterTrain,
    /// Ferry.
    #[serde(alias = "FERRY")]
    Ferry,
    /// A vehicle that is pulled up a steep incline by a cable. A Funicular
    /// typically consists of two cars, with each car acting as a counterweight
    /// for the other.
    #[serde(alias = "FUNICULAR")]
    Funicular,
    /// An aerial cable car.
    #[serde(alias = "GONDOLA_LIFT")]
    GondolaLift,
    /// Heavy rail.
    #[serde(alias = "HEAVY_RAIL")]
    HeavyRail,
    /// High speed train.
    #[serde(alias = "HIGH_SPEED_TRAIN")]
    HighSpeedTrain,
    /// Intercity bus.
    #[serde(alias = "INTERCITY_BUS")]
    IntercityBus,
    /// Long distance train.
    #[serde(alias = "LONG_DISTANCE_TRAIN")]
    LongDistanceTrain,
    /// Light rail transit.
    #[serde(alias = "METRO_RAIL")]
    MetroRail,
    /// Monorail.
    #[serde(alias = "MONORAIL")]
    Monorail,
    /// All other vehicles will return this type.
    #[serde(alias = "OTHER")]
    Other,
    /// Rail.
    #[serde(alias = "RAIL")]
    Rail,
    /// Share taxi is a kind of bus with the ability to drop off and pick up
    /// passengers anywhere on its route.
    #[serde(alias = "SHARE_TAXI")]
    ShareTaxi,
    /// Underground light rail.
    #[serde(alias = "SUBWAY")]
    Subway,
    /// Above ground light rail.
    #[serde(alias = "TRAM")]
    Tram,
    /// Trolleybus.
    #[serde(alias = "TROLLEYBUS")]
    Trolleybus,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for VehicleType {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match VehicleType::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string()))
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&VehicleType> for String {
    /// Converts a `VehicleType` enum to a `String` that contains a [vehicle
    /// type](https://developers.google.com/maps/documentation/directions/intro#VehicleType)
    /// code.
    fn from(vehicle_type: &VehicleType) -> String {
        match vehicle_type {
            VehicleType::Bus => String::from("BUS"),
            VehicleType::CableCar => String::from("CABLE_CAR"),
            VehicleType::CommuterTrain => String::from("COMMUTER_TRAIN"),
            VehicleType::Ferry => String::from("FERRY"),
            VehicleType::Funicular => String::from("FUNICULAR"),
            VehicleType::GondolaLift => String::from("GONDOLA_LIFT"),
            VehicleType::HeavyRail => String::from("HEAVY_RAIL"),
            VehicleType::HighSpeedTrain => String::from("HIGH_SPEED_TRAIN"),
            VehicleType::IntercityBus => String::from("INTERCITY_BUS"),
            VehicleType::LongDistanceTrain => String::from("LONG_DISTANCE_TRAIN"),
            VehicleType::MetroRail => String::from("METRO_RAIL"),
            VehicleType::Monorail => String::from("MONORAIL"),
            VehicleType::Other => String::from("OTHER"),
            VehicleType::Rail => String::from("RAIL"),
            VehicleType::ShareTaxi => String::from("SHARE_TAXI"),
            VehicleType::Subway => String::from("SUBWAY"),
            VehicleType::Tram => String::from("TRAM"),
            VehicleType::Trolleybus => String::from("TROLLEYBUS"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static VEHICLE_TYPES_BY_CODE: phf::Map<&'static str, VehicleType> = phf_map! {
    "BUS" => VehicleType::Bus,
    "CABLE_CAR" => VehicleType::CableCar,
    "COMMUTER_TRAIN" => VehicleType::CommuterTrain,
    "FERRY" => VehicleType::Ferry,
    "FUNICULAR" => VehicleType::Funicular,
    "GONDOLA_LIFT" => VehicleType::GondolaLift,
    "HEAVY_RAIL" => VehicleType::HeavyRail,
    "HIGH_SPEED_TRAIN" => VehicleType::HighSpeedTrain,
    "INTERCITY_BUS" => VehicleType::IntercityBus,
    "LONG_DISTANCE_TRAIN" => VehicleType::LongDistanceTrain,
    "METRO_RAIL" => VehicleType::MetroRail,
    "MONORAIL" => VehicleType::Monorail,
    "OTHER" => VehicleType::Other,
    "RAIL" => VehicleType::Rail,
    "SHARE_TAXI" => VehicleType::ShareTaxi,
    "SUBWAY" => VehicleType::Subway,
    "TRAM" => VehicleType::Tram,
    "TROLLEYBUS" => VehicleType::Trolleybus,
};

impl std::convert::TryFrom<&str> for VehicleType {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;
    /// Gets a `VehicleType` enum from a `String` that contains a valid [vehicle
    /// type](https://developers.google.com/maps/documentation/directions/intro#VehicleType)
    /// code.
    fn try_from(vehicle_type_code: &str) -> Result<VehicleType, Error> {
        VEHICLE_TYPES_BY_CODE
            .get(vehicle_type_code)
            .cloned()
            .ok_or_else(|| Error::InvalidVehicleTypeCode(vehicle_type_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for VehicleType {
    /// Returns a reasonable default variant for the `VehicleType` enum type.
    fn default() -> Self {
        VehicleType::Bus
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for VehicleType {
    /// Formats a `VehicleType` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VehicleType::Bus => write!(f, "Bus"),
            VehicleType::CableCar => write!(f, "Cable Car"),
            VehicleType::CommuterTrain => write!(f, "Commuter Train"),
            VehicleType::Ferry => write!(f, "Ferry"),
            VehicleType::Funicular => write!(f, "Funicular"),
            VehicleType::GondolaLift => write!(f, "Gondola Lift"),
            VehicleType::HeavyRail => write!(f, "Heavy Rail"),
            VehicleType::HighSpeedTrain => write!(f, "High Speed Train"),
            VehicleType::IntercityBus => write!(f, "Intercity Bus"),
            VehicleType::LongDistanceTrain => write!(f, "Long Distance Train"),
            VehicleType::MetroRail => write!(f, "Metro Rail"),
            VehicleType::Monorail => write!(f, "Monorail"),
            VehicleType::Other => write!(f, "Other"),
            VehicleType::Rail => write!(f, "Rail"),
            VehicleType::ShareTaxi => write!(f, "Share Taxi"),
            VehicleType::Subway => write!(f, "Subway"),
            VehicleType::Tram => write!(f, "Tram"),
            VehicleType::Trolleybus => write!(f, "Trolleybus"),
        } // match
    } // fn
} // impl