use crate::address_validation::validate_address::RequestQuery;

// -----------------------------------------------------------------------------

impl crate::traits::RequestBody for crate::address_validation::validate_address::Request<'_> {
    /// Implementation of the `RequestBody` trait, which tells the `Client` how
    /// to convert the above defined `RequestBody` struct into JSON for
    /// submission to Google Maps.
    ///
    /// In this case, we'll be converting the `RequestQuery` struct into a JSON
    /// object which will be submitted inside the HTTP request's body.
    fn request_body(&self) -> Result<String, crate::Error> {
        Ok(serde_json::to_string(&RequestQuery::from(self))?)
    } // fn
} // impl