// =============================================================================

impl<'r> crate::places::place_autocomplete::request::Request<'r> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Place API Place Autocomplete query
    /// with the required, non-optional parameters.
    ///
    /// ## Arguments
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    ///
    /// * `input` ‧ The text string on which to search.
    #[must_use]
    pub fn new(
        client: &'r crate::client::Client,
        input: impl Into<String>
    ) -> Self {
        // Instantiate struct and return it to caller:
        Self {
            // Required parameters:
            client,
            input: input.into(),
            // Optional parameters:
            components: vec![],
            language: None,
            location: None,
            offset: None,
            origin: None,
            radius: None,
            region: None,
            sessiontoken: None,
            strictbounds: None,
            types: vec![],
        } // struct
    } // fn
} // impl
