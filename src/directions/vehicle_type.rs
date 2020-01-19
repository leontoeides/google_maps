use serde::{Serialize, Deserialize};

/// Indicates the vehicle type.
///
/// [Vehicle Types](https://developers.google.com/maps/documentation/directions/intro#Legs)

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VehicleType {
    /// Rail.
    Rail,
    /// Light rail transit.
    MetroRail,
    /// Underground light rail.
    Subway,
    /// Above ground light rail.
    Tram,
    /// Monorail.
    Monorail,
    /// Heavy rail.
    HeavyRail,
    /// Commuter rail.
    CommuterTrain,
    /// High speed train.
    HighSpeedTrain,
    /// Long distance train.
    LongDistanceTrain,
    /// Bus.
    Bus,
    /// Intercity bus.
    IntercityBus,
    /// Trolleybus.
    Trolleybus,
    /// Share taxi is a kind of bus with the ability to drop off and pick up
    /// passengers anywhere on its route.
    ShareTaxi,
    /// Ferry.
    Ferry,
    /// A vehicle that operates on a cable, usually on the ground. Aerial cable
    /// cars may be of the type VehicleType::Gondola.
    CableCar,
    /// An aerial cable car.
    GondolaLift,
    /// A vehicle that is pulled up a steep incline by a cable. A Funicular
    /// typically consists of two cars, with each car acting as a counterweight
    /// for the other.
    Funicular,
    /// All other vehicles will return this type.
    Other,
} // enum

impl VehicleType {
    /// Converts a `VehicleType` enum to a `String` that contains a pretty
    /// source-code-style [vehicle type](https://developers.google.com/maps/documentation/javascript/reference/directions#VehicleType)
    /// code for debugging.
    pub fn source_code_print(&self) -> String {
        match self {
            VehicleType::Rail => String::from("VehicleType::Rail"),
            VehicleType::MetroRail => String::from("VehicleType::MetroRail"),
            VehicleType::Subway => String::from("VehicleType::Subway"),
            VehicleType::Tram => String::from("VehicleType::Tram"),
            VehicleType::Monorail => String::from("VehicleType::Monorail"),
            VehicleType::HeavyRail => String::from("VehicleType::HeavyRail"),
            VehicleType::CommuterTrain => String::from("VehicleType::CommuterTrain"),
            VehicleType::HighSpeedTrain => String::from("VehicleType::HighSpeedTrain"),
            VehicleType::LongDistanceTrain => String::from("VehicleType::LongDistanceTrain"),
            VehicleType::Bus => String::from("VehicleType::Bus"),
            VehicleType::IntercityBus => String::from("VehicleType::IntercityBus"),
            VehicleType::Trolleybus => String::from("VehicleType::Trolleybus"),
            VehicleType::ShareTaxi => String::from("VehicleType::ShareTaxi"),
            VehicleType::Ferry => String::from("VehicleType::Ferry"),
            VehicleType::CableCar => String::from("VehicleType::CableCar"),
            VehicleType::GondolaLift => String::from("VehicleType::GondolaLift"),
            VehicleType::Funicular => String::from("VehicleType::Funicular"),
            VehicleType::Other => String::from("VehicleType::Other"),
        } // match
    } // fn
} // impl

impl From<&VehicleType> for String {
    /// Converts a `VehicleType` enum to a `String` that contains a
    /// [vehicle type](https://developers.google.com/maps/documentation/javascript/reference/directions#VehicleType)
    /// code.
    fn from(vehicle_type: &VehicleType) -> String {
        match vehicle_type {
            VehicleType::Rail => String::from("RAIL"),
            VehicleType::MetroRail => String::from("METRO_RAIL"),
            VehicleType::Subway => String::from("SUBWAY"),
            VehicleType::Tram => String::from("TRAM"),
            VehicleType::Monorail => String::from("MONORAIL"),
            VehicleType::HeavyRail => String::from("HEAVY_RAIL"),
            VehicleType::CommuterTrain => String::from("COMMUTER_TRAIN"),
            VehicleType::HighSpeedTrain => String::from("HIGH_SPEED_TRAIN"),
            VehicleType::LongDistanceTrain => String::from("LONG_DISTANCE_TRAIN"),
            VehicleType::Bus => String::from("BUS"),
            VehicleType::IntercityBus => String::from("INTERCITY_BUS"),
            VehicleType::Trolleybus => String::from("TROLLEYBUS"),
            VehicleType::ShareTaxi => String::from("SHARE_TAXI"),
            VehicleType::Ferry => String::from("FERRY"),
            VehicleType::CableCar => String::from("CABLE_CAR"),
            VehicleType::GondolaLift => String::from("GONDOLA_LIFT"),
            VehicleType::Funicular => String::from("FUNICULAR"),
            VehicleType::Other => String::from("OTHER"),
        } // match
    } // fn
} // impl

impl From<String> for VehicleType {
    /// Gets a `VehicleType` enum from a `String` that contains a valid
    /// [vehicle type](https://developers.google.com/maps/documentation/javascript/reference/directions#VehicleType)
    /// code.
    fn from(vehicle_type: String) -> VehicleType {
        match vehicle_type.as_ref() {
            "RAIL" => VehicleType::Rail,
            "METRO_RAIL" => VehicleType::MetroRail,
            "SUBWAY" => VehicleType::Subway,
            "TRAM" => VehicleType::Tram,
            "MONORAIL" => VehicleType::Monorail,
            "HEAVY_RAIL" => VehicleType::HeavyRail,
            "COMMUTER_TRAIN" => VehicleType::CommuterTrain,
            "HIGH_SPEED_TRAIN" => VehicleType::HighSpeedTrain,
            "LONG_DISTANCE_TRAIN" => VehicleType::LongDistanceTrain,
            "BUS" => VehicleType::Bus,
            "INTERCITY_BUS" => VehicleType::IntercityBus,
            "TROLLEYBUS" => VehicleType::Trolleybus,
            "SHARE_TAXI" => VehicleType::ShareTaxi,
            "FERRY" => VehicleType::Ferry,
            "CABLE_CAR" => VehicleType::CableCar,
            "GONDOLA_LIFT" => VehicleType::GondolaLift,
            "FUNICULAR" => VehicleType::Funicular,
            "OTHER" => VehicleType::Other,
            _ => panic!("'{}' is not a valid vehicle type code. Valid codes are `RAIL`, `METRO_RAIL`, `SUBWAY`, `TRAM`, `MONORAIL`, `HEAVY_RAIL`, `COMMUTER_TRAIN`, `HIGH_SPEED_TRAIN`, `LONG_DISTANCE_TRAIN`, `BUS`, `INTERCITY_BUS`, `TROLLEYBUS`, `SHARE_TAXI`, `FERRY`, `CABLE_CAR`, `GONDOLA_LIFT`, `FUNICULAR`, and `OTHER`.", vehicle_type)
        } // match
    } // fn
} // impl