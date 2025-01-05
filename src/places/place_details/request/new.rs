// =============================================================================

impl<'r> crate::places::place_details::request::Request<'r> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Places API _Place Details_ query
    /// with the required, non-optional parameters.
    ///
    /// ## Arguments
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    ///
    /// * `place_id` ‧ A textual identifier that uniquely identifies a place,
    ///   returned from a
    ///   [Place Search](https://developers.google.com/maps/documentation/places/web-service/search).
    ///   For more information about place IDs, see the
    ///   [place ID overview](https://developers.google.com/maps/documentation/places/web-service/place-id).
    #[must_use]
    pub fn new(
        client: &'r crate::client::Client,
        place_id: impl Into<String>
    ) -> Self {
        // Instantiate struct and return it to caller:
        Self {
            // Required parameters:
            client,
            place_id: place_id.into(),
            // Optional parameters:
            fields: Vec::new(),
            language: None,
            region: None,
            reviews_no_translations: None,
            reviews_sort: None,
            sessiontoken: None,
        } // struct
    } // fn
} // impl
