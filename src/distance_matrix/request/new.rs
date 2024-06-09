use crate::{
    client::GoogleMapsClient, directions::request::waypoint::Waypoint,
    distance_matrix::request::Request,
}; // use

// =============================================================================

impl<'a> Request<'a> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Distance Matrix API query with the
    /// required, non-optional parameters.
    ///
    /// ## Arguments
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.

    #[must_use]
    pub fn new(
        client: &GoogleMapsClient,
        origins: Vec<Waypoint>,
        destinations: Vec<Waypoint>
    ) -> Request {
        Request {
            // Required parameters:
            client,
            destinations,
            origins,
            // Optional parameters:
            arrival_time: None,
            departure_time: None,
            language: None,
            region: None,
            restrictions: Vec::new(),
            traffic_model: None,
            transit_modes: Vec::new(),
            transit_route_preference: None,
            travel_mode: None,
            unit_system: None,
            // Internal use only:
            query: None,
            validated: false,
        } // struct
    } // fn
} // impl
