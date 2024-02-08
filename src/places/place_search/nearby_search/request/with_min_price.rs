use crate::places::place_search::nearby_search::request::Request;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Adds the minimum price to the Places API _Nearby Search_ query.
    ///
    /// ## Arguments
    ///
    /// * `minprice` ‧ Restricts results to only those places within the
    /// specified range. Valid values range between 0 (most affordable) to 4
    /// (most expensive), inclusive. The exact amount indicated by a specific
    /// value will vary from region to region.

    pub fn with_min_price(&'a mut self, minprice: u8) -> &'a mut Request {
        // Set minimum price in Request struct.
        self.minprice = Some(minprice);
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
