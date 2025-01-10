impl crate::places::place_search::nearby_search::Request<'_> {
    /// Adds "open now" filter to the Places API _Nearby Search_ query.
    ///
    /// ## Arguments
    ///
    /// * `opennow` ‧ Returns only those places that are open for business at
    ///   the time the query is sent. Places that do not specify opening hours
    ///   in the Google Places database will not be returned if you include this
    ///   parameter in your query.
    #[must_use] pub fn is_open_now(
        mut self,
        opennow: impl Into<bool>,
    ) -> Self {
        // Set "open now" filter in Request struct.
        self.opennow = Some(opennow.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl