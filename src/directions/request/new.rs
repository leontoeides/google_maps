use crate::directions::request::location::Location;

// =============================================================================

impl<'a> crate::directions::request::Request<'a> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the data structure for the builder pattern.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    #[must_use]
    pub const fn new(
        client: &'a crate::client::Client,
        origin: Location,
        destination: Location
    ) -> Self {
        Self {
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
            restrictions: Vec::new(),
            traffic_model: None,
            transit_modes: Vec::new(),
            transit_route_preference: None,
            travel_mode: None,
            unit_system: None,
            waypoint_optimization: false,
            waypoints: Vec::new(),
        } // struct
    } // fn
} // impl
