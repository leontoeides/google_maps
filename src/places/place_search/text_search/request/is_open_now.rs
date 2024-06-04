use crate::places::place_search::text_search::request::Request;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Adds "open now" filter to the Places API _Text Search_ query.
    ///
    /// ## Arguments
    ///
    /// * `opennow` ‧ Returns only those places that are open for business at
    ///   the time the query is sent. Places that do not specify opening hours
    ///   in the Google Places database will not be returned if you include this
    ///   parameter in your query.

    pub fn is_open_now(&'a mut self, opennow: bool) -> &'a mut Self {
        // Set "open now" filter in Request struct.
        self.opennow = Some(opennow);
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
