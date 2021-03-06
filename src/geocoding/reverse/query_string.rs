use crate::geocoding::reverse::ReverseRequest;

impl<'a> ReverseRequest<'a> {

    /// Returns the URI query string that represents the query you've built.
    ///
    /// ## Description:
    ///
    /// Returns the query string that will be sent to the Google Maps API. It
    /// is the result of the builder pattern. This method could be useful for
    /// records or logging. It could also be used for passing to your HTTP
    /// client of choice and executing the HTTP GET request yourself.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn query_string(&'a mut self) -> (String, String) {
        (
            "https://maps.googleapis.com/maps/api/geocode/json?".to_string(),
            match &self.query {
                // If query string has already been built, return it:
                Some(query_string) => query_string.to_owned(),
                // If it hasn't been built, build it:
                None => self.build().query.as_ref().unwrap().clone(),
            } // match
        )
    } // fn

} // impl