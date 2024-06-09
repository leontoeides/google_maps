use crate::places::place_search::nearby_search::request::Request;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Adds the searched text string to the Places API _Nearby Search_ query.
    ///
    /// ## Arguments
    ///
    /// * `keyword` ‧ The text string on which to search, for example:
    ///   "restaurant" or "123 Main Street". This must be a place name, address,
    ///   or category of establishments. Any other types of input can generate
    ///   errors and are not guaranteed to return valid results. The Google
    ///   Places service will return candidate matches based on this string and
    ///   order the results based on their perceived relevance.
    ///
    /// Explicitly including location information using this parameter may
    /// conflict with the location, radius, and rankby parameters, causing
    /// unexpected results.
    ///
    /// If this parameter is omitted, places with a `business_status` of
    /// `CLOSED_TEMPORARILY` or `CLOSED_PERMANENTLY` will not be returned.

    pub fn with_keyword(
        &'a mut self,
        keyword: impl Into<String>
    ) -> &'a mut Self {
        // Set maximum price in Request struct.
        self.keyword = Some(keyword.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
