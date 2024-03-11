//! Contains the `VehicleType` enum and its associated traits. It specifies the
//! mode of transportation for transit directions.

use crate::directions::error::Error;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------

/// Indicates the [vehicle
/// type](https://developers.google.com/maps/documentation/directions/intro#VehicleType)

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum VehicleType {
    /// Bus.
    #[default]
    Bus = 0,
    /// A vehicle that operates on a cable, usually on the ground. Aerial cable
    /// cars may be of the type `VehicleType::Gondola`.
    CableCar = 1,
    /// Commuter rail.
    CommuterTrain = 2,
    /// Ferry.
    Ferry = 3,
    /// A vehicle that is pulled up a steep incline by a cable. A Funicular
    /// typically consists of two cars, with each car acting as a counterweight
    /// for the other.
    Funicular = 4,
    /// An aerial cable car.
    GondolaLift = 5,
    /// Heavy rail.
    HeavyRail = 6,
    /// High speed train.
    HighSpeedTrain = 7,
    /// Intercity bus.
    IntercityBus = 8,
    /// Long distance train.
    LongDistanceTrain = 9,
    /// Light rail transit.
    MetroRail = 10,
    /// Monorail.
    Monorail = 11,
    /// All other vehicles will return this type.
    Other = 12,
    /// Rail.
    Rail = 13,
    /// Share taxi is a kind of bus with the ability to drop off and pick up
    /// passengers anywhere on its route.
    ShareTaxi = 14,
    /// Underground light rail.
    Subway = 15,
    /// Above ground light rail.
    Tram = 16,
    /// Trolleybus.
    Trolleybus = 17,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for VehicleType {
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

impl Serialize for VehicleType {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&VehicleType> for &str {
    /// Converts a `VehicleType` enum to a `String` that contains a [vehicle
    /// type](https://developers.google.com/maps/documentation/directions/intro#VehicleType)
    /// code.
    fn from(vehicle_type: &VehicleType) -> Self {
        match vehicle_type {
            VehicleType::Bus => "BUS",
            VehicleType::CableCar => "CABLE_CAR",
            VehicleType::CommuterTrain => "COMMUTER_TRAIN",
            VehicleType::Ferry => "FERRY",
            VehicleType::Funicular => "FUNICULAR",
            VehicleType::GondolaLift => "GONDOLA_LIFT",
            VehicleType::HeavyRail => "HEAVY_RAIL",
            VehicleType::HighSpeedTrain => "HIGH_SPEED_TRAIN",
            VehicleType::IntercityBus => "INTERCITY_BUS",
            VehicleType::LongDistanceTrain => "LONG_DISTANCE_TRAIN",
            VehicleType::MetroRail => "METRO_RAIL",
            VehicleType::Monorail => "MONORAIL",
            VehicleType::Other => "OTHER",
            VehicleType::Rail => "RAIL",
            VehicleType::ShareTaxi => "SHARE_TAXI",
            VehicleType::Subway => "SUBWAY",
            VehicleType::Tram => "TRAM",
            VehicleType::Trolleybus => "TROLLEYBUS",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for VehicleType {
    /// Converts a `VehicleType` enum to a `String` that contains a [vehicle
    /// type](https://developers.google.com/maps/documentation/directions/intro#VehicleType)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&VehicleType> for String {
    /// Converts a `VehicleType` enum to a `String` that contains a [vehicle
    /// type](https://developers.google.com/maps/documentation/directions/intro#VehicleType)
    /// code.
    fn from(vehicle_type: &VehicleType) -> Self {
        std::convert::Into::<&str>::into(vehicle_type).to_string()
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

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for VehicleType {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;
    /// Gets a `VehicleType` enum from a `String` that contains a valid [vehicle
    /// type](https://developers.google.com/maps/documentation/directions/intro#VehicleType)
    /// code.
    fn try_from(vehicle_type_code: &str) -> Result<Self, Self::Error> {
        VEHICLE_TYPES_BY_CODE
            .get(vehicle_type_code)
            .cloned()
            .ok_or_else(|| Error::InvalidVehicleTypeCode(vehicle_type_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for VehicleType {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Err = crate::directions::error::Error;
    /// Gets a `VehicleType` enum from a `String` that contains a valid [vehicle
    /// type](https://developers.google.com/maps/documentation/directions/intro#VehicleType)
    /// code.
    fn from_str(vehicle_type_code: &str) -> Result<Self, Self::Err> {
        VEHICLE_TYPES_BY_CODE
            .get(vehicle_type_code)
            .cloned()
            .ok_or_else(|| Error::InvalidVehicleTypeCode(vehicle_type_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl VehicleType {
    /// Formats a `VehicleType` enum into a string that is presentable to the
    /// end user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::Bus => "Bus",
            Self::CableCar => "Cable Car",
            Self::CommuterTrain => "Commuter Train",
            Self::Ferry => "Ferry",
            Self::Funicular => "Funicular",
            Self::GondolaLift => "Gondola Lift",
            Self::HeavyRail => "Heavy Rail",
            Self::HighSpeedTrain => "High Speed Train",
            Self::IntercityBus => "Intercity Bus",
            Self::LongDistanceTrain => "Long Distance Train",
            Self::MetroRail => "Metro Rail",
            Self::Monorail => "Monorail",
            Self::Other => "Other",
            Self::Rail => "Rail",
            Self::ShareTaxi => "Share Taxi",
            Self::Subway => "Subway",
            Self::Tram => "Tram",
            Self::Trolleybus => "Trolleybus",
        } // match
    } // fn
} // impl
