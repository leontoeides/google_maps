use crate::directions::request::{
    location::Location,
    Request,
}; // use

impl Request {

    pub fn new(origin: Location, destination: Location, key: String) -> Request {
        Request {
            // Required parameters:
            origin,
            destination,
            key,
            // Optional parameters:
            travel_mode: None,
            waypoints: None,
            alternatives: None,
            restrictions: None,
            language: None,
            unit_system: None,
            region: None,
            arrival_time: None,
            departure_time: None,
            traffic_model: None,
            transit_modes: None,
            transit_route_preference: None,
            // Internal use only:
            query: None,
        } // struct
    } // fn

} // impl