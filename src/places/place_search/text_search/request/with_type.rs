impl crate::places::place_search::text_search::Request<'_> {
    /// Adds the types parameter to the Places API _Text Search_ query.
    ///
    /// ## Arguments
    ///
    /// * `type` ‧ Restricts the results to places matching the specified type.
    ///   Only one type may be specified. If more than one type is provided, all
    ///   types following the first entry are ignored.
    ///
    /// `type=hospital|pharmacy|doctor` becomes `type=hospital`
    /// `type=hospital,pharmacy,doctor` is ignored entirely
    ///
    /// See the list of [supported types](https://developers.google.com/maps/documentation/places/web-service/supported_types).
    ///
    /// Note: Adding both `keyword` and `type` with the same value
    /// (`keyword=cafe&type=cafe` or `keyword=parking&type=parking`) can yield
    /// `ZERO_RESULTS`.
    #[must_use] pub fn with_type(
        mut self,
        place_type: impl Into<crate::types::PlaceType>
    ) -> Self {
        // Set location in Request struct.
        self.place_type = Some(place_type.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl