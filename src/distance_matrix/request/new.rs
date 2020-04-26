use crate::{
    client_settings::ClientSettings, directions::request::waypoint::Waypoint,
    distance_matrix::request::Request,
}; // use

impl<'a> Request<'a> {
    pub fn new(
        client_settings: &mut ClientSettings,
        origins: Vec<Waypoint>,
        destinations: Vec<Waypoint>,
    ) -> Request {
        Request {
            // Required parameters:
            destinations,
            client_settings,
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
