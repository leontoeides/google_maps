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

use crate::directions::response::{
    geocoded_waypoint::GeocodedWaypoint,
    route::Route,
    status::Status,
};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    status: Status,
    geocoded_waypoints: Vec<GeocodedWaypoint>,
    routes: Vec<Route>,
} // struct