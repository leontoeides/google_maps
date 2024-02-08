//! Contains the `TravelMode` enum and its associated traits. It specifies the
//! mode of transportation.

use crate::directions::error::Error;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------

/// Specifies the [mode of
/// transportation](https://developers.google.com/maps/documentation/directions/intro#TravelModes).
///
/// When you calculate directions, you may specify the transportation `mode` to
/// use. By default, directions are calculated as `driving` directions.
///
/// Note: Both walking and bicycling directions may sometimes not include
/// clear pedestrian or bicycling paths, so these directions will return
/// `warnings` in the returned result which you must display to the user.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum TravelMode {
    /// (Default) Indicates standard driving directions using the road network.
    #[default]
    Driving = 0,
    /// Requests walking directions via pedestrian paths & sidewalks (where
    /// available).
    Walking = 1,
    /// Requests bicycling directions via bicycle paths & preferred streets
    /// (where available).
    Bicycling = 2,
    /// Requests directions via public transit routes (where available). If you
    /// set the mode to `transit`, you can optionally specify either a
    /// `departure_time` or an `arrival_time`. If neither time is specified, the
    /// `departure_time` defaults to now (that is, the departure time defaults
    /// to the current time). You can also optionally include a `transit_mode`
    /// and/or a `transit_routing_preference`.
    Transit = 3,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for TravelMode {
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

impl Serialize for TravelMode {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&TravelMode> for &str {
    /// Converts a `TravelMode` enum to a `String` that contains a [travel
    /// mode](https://developers.google.com/maps/documentation/directions/intro#TravelModes)
    /// code.
    fn from(travel_mode: &TravelMode) -> Self {
        match travel_mode {
            TravelMode::Bicycling => "BICYCLING",
            TravelMode::Driving => "DRIVING",
            TravelMode::Transit => "TRANSIT",
            TravelMode::Walking => "WALKING",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for TravelMode {
    /// Converts a `TravelMode` enum to a `String` that contains a [travel
    /// mode](https://developers.google.com/maps/documentation/directions/intro#TravelModes)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&TravelMode> for String {
    /// Converts a `TravelMode` enum to a `String` that contains a [travel
    /// mode](https://developers.google.com/maps/documentation/directions/intro#TravelModes)
    /// code.
    fn from(travel_mode: &TravelMode) -> Self {
        std::convert::Into::<&str>::into(travel_mode).to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

static TRAVEL_MODES_BY_CODE: phf::Map<&'static str, TravelMode> = phf_map! {
    "BICYCLING" => TravelMode::Bicycling,
    "DRIVING" => TravelMode::Driving,
    "TRANSIT" => TravelMode::Transit,
    "WALKING" => TravelMode::Walking,
};

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for TravelMode {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;
    /// Gets a `TravelMode` enum from a `String` that contains a valid [travel
    /// mode](https://developers.google.com/maps/documentation/directions/intro#TravelModes)
    /// code.
    fn try_from(travel_mode_code: &str) -> Result<Self, Self::Error> {
        TRAVEL_MODES_BY_CODE
            .get(travel_mode_code)
            .cloned()
            .ok_or_else(|| Error::InvalidTravelModeCode(travel_mode_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for TravelMode {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Err = crate::directions::error::Error;
    /// Gets a `TravelMode` enum from a `String` that contains a valid [travel
    /// mode](https://developers.google.com/maps/documentation/directions/intro#TravelModes)
    /// code.
    fn from_str(travel_mode_code: &str) -> Result<Self, Self::Err> {
        TRAVEL_MODES_BY_CODE
            .get(travel_mode_code)
            .cloned()
            .ok_or_else(|| Error::InvalidTravelModeCode(travel_mode_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TravelMode {
    /// Formats a `TravelMode` enum into a string that is presentable to the
    /// end user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::Bicycling => "Bicycling",
            Self::Driving => "Driving",
            Self::Transit => "Transit",
            Self::Walking => "Walking",
        } // match
    } // fn
} // impl
