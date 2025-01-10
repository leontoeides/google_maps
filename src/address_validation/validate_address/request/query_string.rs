impl crate::traits::QueryString for crate::address_validation::validate_address::Request<'_> {
    /// Builds the URL [query string](https://en.wikipedia.org/wiki/Query_string)
    /// for the HTTP request. The query string is generated from the data found
    /// in this `Request` structure.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    ///
    /// ## Notes
    ///
    /// * The Address Validation API mostly uses the HTTP body for the request.
    fn query_string(&self) -> String {
        format!("key={}", self.client.key)
    } // fn
} // impl
