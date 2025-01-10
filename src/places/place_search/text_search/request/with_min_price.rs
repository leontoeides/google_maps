impl crate::places::place_search::text_search::Request<'_> {
    /// Adds the minimum price to the Places API _Text Search_ query.
    ///
    /// ## Arguments
    ///
    /// * `minprice` ‧ Restricts results to only those places within the
    ///   specified range. Valid values range between 0 (most affordable) to 4
    ///   (most expensive), inclusive. The exact amount indicated by a specific
    ///   value will vary from region to region.
    #[must_use] pub fn with_min_price(
        mut self,
        minprice: impl Into<u8>
    ) -> Self {
        // Set minimum price in Request struct.
        self.minprice = Some(minprice.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl