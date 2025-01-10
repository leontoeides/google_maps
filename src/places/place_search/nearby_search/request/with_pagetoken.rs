impl crate::places::place_search::nearby_search::Request<'_> {
    /// Adds the page token parameter to the Places API _Nearby Search_ query.
    ///
    /// ## Arguments
    ///
    /// * `pagetoken` ‧ Returns up to 20 results from a previously run search.
    ///   Setting a `pagetoken` parameter will execute a search with the same
    ///   parameters used previously — all parameters other than pagetoken will
    ///   be ignored.
    #[must_use] pub fn with_pagetoken(
        mut self,
        pagetoken: impl Into<String>
    ) -> Self {
        // Set page token in Request struct.
        self.pagetoken = Some(pagetoken.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl