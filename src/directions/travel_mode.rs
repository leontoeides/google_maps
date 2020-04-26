//! Contains the `TravelMode` enum and its associated traits. It specifies the
//! mode of transportation.

use crate::directions::error::Error;
use serde::{Deserialize, Serialize};

/// Specifies the [mode of
/// transportation](https://developers.google.com/maps/documentation/directions/intro#TravelModes).
///
/// When you calculate directions, you may specify the transportation `mode` to
/// use. By default, directions are calculated as `driving` directions.
///
/// Note: Both walking and bicycling directions may sometimes not include
/// clear pedestrian or bicycling paths, so these directions will return
/// `warnings` in the returned result which you must display to the user.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum TravelMode {
    /// (Default) Indicates standard driving directions using the road network.
    #[serde(alias = "DRIVING")]
    Driving,

    /// Requests walking directions via pedestrian paths & sidewalks (where
    /// available).
    #[serde(alias = "WALKING")]
    Walking,

    /// Requests bicycling directions via bicycle paths & preferred streets
    /// (where available).
    #[serde(alias = "BICYCLING")]
    Bicycling,

    /// Requests directions via public transit routes (where available). If you
    /// set the mode to `transit`, you can optionally specify either a
    /// `departure_time` or an `arrival_time`. If neither time is specified, the
    /// `departure_time` defaults to now (that is, the departure time defaults
    /// to the current time). You can also optionally include a `transit_mode`
    /// and/or a `transit_routing_preference`.
    #[serde(alias = "TRANSIT")]
    Transit,
} // enum

impl std::convert::From<&TravelMode> for String {
    /// Converts a `TravelMode` enum to a `String` that contains a [travel
    /// mode](https://developers.google.com/maps/documentation/directions/intro#TravelModes)
    /// code.
    fn from(travel_mode: &TravelMode) -> String {
        match travel_mode {
            TravelMode::Bicycling => String::from("BICYCLING"),
            TravelMode::Driving => String::from("DRIVING"),
            TravelMode::Transit => String::from("TRANSIT"),
            TravelMode::Walking => String::from("WALKING"),
        } // match
    } // fn
} // impl

impl std::convert::TryFrom<&str> for TravelMode {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;

    /// Gets a `TravelMode` enum from a `String` that contains a valid [travel
    /// mode](https://developers.google.com/maps/documentation/directions/intro#TravelModes)
    /// code.
    fn try_from(travel_mode: &str) -> Result<TravelMode, Error> {
        match travel_mode {
            "BICYCLING" => Ok(TravelMode::Bicycling),
            "DRIVING" => Ok(TravelMode::Driving),
            "TRANSIT" => Ok(TravelMode::Transit),
            "WALKING" => Ok(TravelMode::Walking),
            _ => Err(Error::InvalidTravelModeCode(travel_mode.to_string())),
        } // match
    } // fn
} // impl

impl std::default::Default for TravelMode {
    /// Returns a reasonable default variant for the `TravelMode` enum type.
    fn default() -> Self {
        TravelMode::Driving
    } // fn
} // impl

impl std::fmt::Display for TravelMode {
    /// Formats a `TravelMode` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TravelMode::Bicycling => write!(f, "Bicycling"),
            TravelMode::Driving => write!(f, "Driving"),
            TravelMode::Transit => write!(f, "Transit"),
            TravelMode::Walking => write!(f, "Walking"),
        } // match
    } // fn
} // impl
