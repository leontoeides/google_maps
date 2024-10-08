//! Resources (enums, structs) for processing the _Directions API_ response from
//! the Google Maps Platform. Look in here for more information about the data
//! returned from Google's server and how to parse it with your program.

pub mod directions_distance;
pub mod directions_duration;
pub mod driving_maneuver;
pub mod geocoded_waypoint;
pub mod geocoder_status;
pub mod leg;
pub mod overview_polyline;
pub mod polyline;
pub mod route;
pub mod status;
pub mod step;
pub mod transit_agency;
pub mod transit_currency;
pub mod transit_details;
pub mod transit_fare;
pub mod transit_line;
pub mod transit_stop;
pub mod transit_time;
pub mod transit_vehicle;

use crate::directions::{
    response::{geocoded_waypoint::GeocodedWaypoint, route::Route, status::Status}, // response
    travel_mode::TravelMode,
}; // crate::directions
use serde::{Deserialize, Serialize};

/// Directions responses contain the following root elements.

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// Contains an array of available travel modes. This field is returned when
    /// a request specifies a travel `mode` and gets no results. The array
    /// contains the available travel modes in the countries of the given set of
    /// waypoints. This field is not returned if one or more of the waypoints
    /// are `via:` waypoints.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub available_travel_modes: Vec<TravelMode>,

    /// When the status code is other than `OK`, there may be an additional
    /// `error_message` field within the Directions response object. This field
    /// contains more detailed information about the reasons behind the given
    /// status code.
    ///
    /// **Note**: This field is not guaranteed to be always present, and its
    /// content is subject to change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    /// Contains an array with details about the geocoding of origin,
    /// destination and waypoints. See [Geocoded
    /// Waypoints](https://developers.google.com/maps/documentation/directions/intro#GeocodedWaypoints).
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub geocoded_waypoints: Vec<GeocodedWaypoint>,

    /// Contains an array of routes from the origin to the destination. See
    /// [Routes](https://developers.google.com/maps/documentation/directions/intro#Routes).
    /// Routes consist of nested
    /// [Legs](https://developers.google.com/maps/documentation/directions/intro#Legs)
    /// and [Steps](https://developers.google.com/maps/documentation/directions/intro#Steps).
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<Route>,

    /// Contains metadata on the request. See [Status
    /// Codes](https://developers.google.com/maps/documentation/directions/intro#StatusCodes).
    pub status: Status,
} // struct

impl std::str::FromStr for Response {
    type Err = simd_json::Error;
    /// Parse a Google Maps Directions API JSON `String` response into a
    /// usable `Response` struct.
    fn from_str(s: &str) -> Result<Self, simd_json::Error> {
        let mut bytes = s.to_string().into_bytes();
        simd_json::serde::from_slice(&mut bytes)
    }
}
