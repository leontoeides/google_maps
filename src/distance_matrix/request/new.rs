use crate::directions::request::waypoint::Waypoint;

// =============================================================================

impl<'a> crate::distance_matrix::request::Request<'a> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Distance Matrix API query with the
    /// required, non-optional parameters.
    ///
    /// ## Arguments
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    #[must_use]
    pub const fn new(
        client: &'a crate::client::Client,
        origins: Vec<Waypoint>,
        destinations: Vec<Waypoint>
    ) -> Self {
        Self {
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
        } // struct
    } // fn
} // impl
