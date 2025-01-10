impl crate::places::place_search::text_search::Request<'_> {
    /// Adds the maximum price to the Places API _Text Search_ query.
    ///
    /// ## Arguments
    ///
    /// * `maxprice` ‧ Restricts results to only those places within the
    ///   specified range. Valid values range between 0 (most affordable) to 4
    ///   (most expensive), inclusive. The exact amount indicated by a specific
    ///   value will vary from region to region.
    #[must_use] pub fn with_max_price(
        mut self,
        maxprice: impl Into<u8>
    ) -> Self {
        // Set maximum price in Request struct.
        self.maxprice = Some(maxprice.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
