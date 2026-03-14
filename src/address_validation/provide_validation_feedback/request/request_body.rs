use crate::address_validation::provide_validation_feedback::RequestQuery;

// -----------------------------------------------------------------------------


impl crate::traits::RequestBody for crate::address_validation::provide_validation_feedback::Request<'_> {
    /// Implementation of the `RequestBody` trait, which tells the client how to
    /// convert the above defined `RequestBody` struct into JSON for submission
    /// to Google Maps.
    fn request_body(&self) -> Result<String, crate::Error> {
        Ok(serde_json::to_string(&RequestQuery::from(self))?)
    } // fn
} // impl