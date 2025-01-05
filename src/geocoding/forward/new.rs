// =============================================================================

impl<'a> crate::geocoding::forward::ForwardRequest<'a> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Geolocation API query with the
    /// required, non-optional parameters.
    ///
    /// ## Arguments
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    #[must_use]
    pub const fn new(client: &'a crate::client::Client) -> Self {
        // Instantiate struct and return it to caller:
        Self {
            // Required parameters:
            client,
            // Optional parameters:
            address: None,
            place_id: None,
            bounds: None,
            components: Vec::new(),
            language: None,
            region: None,
        } // struct
    } // fn
} // impl
