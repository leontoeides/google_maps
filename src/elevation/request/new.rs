// =============================================================================

impl<'r> crate::elevation::request::Request<'r> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Elevation API query with the
    /// required, non-optional parameters.
    ///
    /// ## Arguments
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    #[must_use]
    pub const fn new(
        client: &'r crate::client::Client
    ) -> Self {
        // Instantiate struct and return it to caller:
        Self {
            // Required parameters:
            client,
            // Positional requests:
            locations: None,
            // Sampled path requests:
            path: None,
            samples: None,
        } // struct
    } // fn
} // impl
