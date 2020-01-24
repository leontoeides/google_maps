use crate::directions::request::waypoint::Waypoint;
use crate::distance_matrix::request::Request;

impl Request {

    pub fn new(key: String, origins: Vec<Waypoint>, destinations: Vec<Waypoint>) -> Request {
        Request {
            // Required parameters:
            destinations,
            key,
            origins,
            // Optional parameters:
            arrival_time: None,
            departure_time: None,
            language: None,
            region: None,
            restrictions: None,
            traffic_model: None,
            transit_modes: None,
            transit_route_preference: None,
            travel_mode: None,
            unit_system: None,
            // Internal use only:
            query: None,
            validated: false,
        } // struct
    } // fn

} // impl