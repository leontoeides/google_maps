use crate::directions::request::{
    location::Location,
    Request,
}; // use

impl Request {

    pub fn new(key: String, origin: Location, destination: Location) -> Request {
        Request {
            // Required parameters:
            destination,
            key,
            origin,
            // Optional parameters:
            alternatives: None,
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
            waypoints: None,
            // Internal use only:
            query: None,
            validated: false,
        } // struct
    } // fn

} // impl