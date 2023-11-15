use crate::{
    client::GoogleMapsClient,
    directions::request::{location::Location, Request},
}; // use crate

// =============================================================================

impl<'a> Request<'a> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the data structure for the builder pattern.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn new(
        client: &'a GoogleMapsClient,
        origin: Location,
        destination: Location,
    ) -> Request<'a> {
        Request {
            // Required parameters:
            client,
            destination,
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
            waypoint_optimization: false,
            waypoints: None,
            // Internal use only:
            query: None,
            validated: false,
        } // struct
    } // fn
} // impl
