//! Resources (enums, structs) for the client process the Directions API
//! response from the Google Cloud server.

mod distance;
mod duration;
mod fare;
mod geocoded_waypoint;
mod geocoder_status;
mod leg;
mod maneuver_type;
mod overview_polyline;
mod polyline;
mod route;
mod step;
mod time;
mod transit_agency;
mod transit_details;
mod transit_line;
mod transit_stop;
mod transit_vehicle;
pub mod status;

use crate::directions::{
    response::{
        geocoded_waypoint::GeocodedWaypoint,
        route::Route,
        status::Status,
    },
    travel_mode::TravelMode,
};
use serde::{Serialize, Deserialize};

/// Directions responses contain the following root elements.

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Response {

    /// Contains an array of available travel modes. This field is returned when
    /// a request specifies a travel `mode` and gets no results. The array
    /// contains the available travel modes in the countries of the given set of
    /// waypoints. This field is not returned if one or more of the waypoints
    /// are `via:` waypoints.
    pub available_travel_modes: Option<Vec<TravelMode>>,

    /// Contains an array with details about the geocoding of origin,
    /// destination and waypoints. See [Geocoded
    /// Waypoints](https://developers.google.com/maps/documentation/directions/intro#GeocodedWaypoints).
    pub geocoded_waypoints: Vec<GeocodedWaypoint>,

    /// Contains an array of routes from the origin to the destination. See
    /// [Routes](https://developers.google.com/maps/documentation/directions/intro#Routes).
    /// Routes consist of nested
    /// [Legs](https://developers.google.com/maps/documentation/directions/intro#Legs)
    /// and [Steps](https://developers.google.com/maps/documentation/directions/intro#Steps).
    pub routes: Vec<Route>,

    /// Contains metadata on the request. See [Status
    /// Codes](https://developers.google.com/maps/documentation/directions/intro#StatusCodes).
    pub status: Status,

} // struct