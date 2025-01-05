// =============================================================================

impl<'r> crate::places::query_autocomplete::request::Request<'r> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Place API _Query Autocomplete_
    /// query with the required, non-optional parameters.
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
            language: None,
            location: None,
            offset: None,
            radius: None,
        } // struct
    } // fn
} // impl