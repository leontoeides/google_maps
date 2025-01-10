impl crate::places::place_search::text_search::Request<'_> {
    /// Adds the region parameter to the Places API _Text Search_ query.
    ///
    /// ## Arguments
    ///
    /// * `region` ‧ The region code, specified as a [ccTLD ("top-level
    ///   domain")](https://en.wikipedia.org/wiki/List_of_Internet_top-level_domains#Country_code_top-level_domains)
    ///   two-character value. Most ccTLD codes are identical to ISO 3166-1
    ///   codes, with some notable exceptions. For example, the United Kingdom's
    ///   ccTLD is "uk" (.co.uk) while its ISO 3166-1 code is "gb"
    ///   (technically for the entity of "The United Kingdom of Great Britain
    ///   and Northern Ireland").
    #[must_use] pub fn with_region(
        mut self,
        region: impl Into<crate::types::Region>
    ) -> Self {
        // Set region in Request struct.
        self.region = Some(region.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl