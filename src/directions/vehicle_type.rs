use crate::directions::error::Error;
use serde::{Serialize, Deserialize};

/// Indicates the [vehicle
/// type](https://developers.google.com/maps/documentation/directions/intro#VehicleType)

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum VehicleType {
    /// Bus.
    Bus,
    /// A vehicle that operates on a cable, usually on the ground. Aerial cable
    /// cars may be of the type VehicleType::Gondola.
    CableCar,
    /// Commuter rail.
    CommuterTrain,
    /// Ferry.
    Ferry,
    /// A vehicle that is pulled up a steep incline by a cable. A Funicular
    /// typically consists of two cars, with each car acting as a counterweight
    /// for the other.
    Funicular,
    /// An aerial cable car.
    GondolaLift,
    /// Heavy rail.
    HeavyRail,
    /// High speed train.
    HighSpeedTrain,
    /// Intercity bus.
    IntercityBus,
    /// Long distance train.
    LongDistanceTrain,
    /// Light rail transit.
    MetroRail,
    /// Monorail.
    Monorail,
    /// All other vehicles will return this type.
    Other,
    /// Rail.
    Rail,
    /// Share taxi is a kind of bus with the ability to drop off and pick up
    /// passengers anywhere on its route.
    ShareTaxi,
    /// Underground light rail.
    Subway,
    /// Above ground light rail.
    Tram,
    /// Trolleybus.
    Trolleybus,
} // enum

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

impl std::convert::TryFrom<String> for VehicleType {

    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;

    /// Gets a `VehicleType` enum from a `String` that contains a valid [vehicle
    /// type](https://developers.google.com/maps/documentation/directions/intro#VehicleType)
    /// code.
    fn try_from(vehicle_type: String) -> Result<VehicleType, Error> {
        match vehicle_type.as_ref() {
            "BUS" => Ok(VehicleType::Bus),
            "CABLE_CAR" => Ok(VehicleType::CableCar),
            "COMMUTER_TRAIN" => Ok(VehicleType::CommuterTrain),
            "FERRY" => Ok(VehicleType::Ferry),
            "FUNICULAR" => Ok(VehicleType::Funicular),
            "GONDOLA_LIFT" => Ok(VehicleType::GondolaLift),
            "HEAVY_RAIL" => Ok(VehicleType::HeavyRail),
            "HIGH_SPEED_TRAIN" => Ok(VehicleType::HighSpeedTrain),
            "INTERCITY_BUS" => Ok(VehicleType::IntercityBus),
            "LONG_DISTANCE_TRAIN" => Ok(VehicleType::LongDistanceTrain),
            "METRO_RAIL" => Ok(VehicleType::MetroRail),
            "MONORAIL" => Ok(VehicleType::Monorail),
            "OTHER" => Ok(VehicleType::Other),
            "RAIL" => Ok(VehicleType::Rail),
            "SHARE_TAXI" => Ok(VehicleType::ShareTaxi),
            "SUBWAY" => Ok(VehicleType::Subway),
            "TRAM" => Ok(VehicleType::Tram),
            "TROLLEYBUS" => Ok(VehicleType::Trolleybus),
            _ => Err(Error::InvalidVehicleTypeCode(vehicle_type)),
        } // match
    } // fn

} // impl

impl std::default::Default for VehicleType {
    /// Returns a reasonable default variant for the `VehicleType` enum type.
    fn default() -> Self {
        VehicleType::Bus
    } // fn
} // impl

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