impl crate::elevation::Request<'_> {
    /// Executes the Google Maps Elevation API request.
    ///
    /// ## Description
    ///
    /// This method will:
    ///
    /// 1. Validate the request `struct` that has been built,
    ///
    /// 2. Build a [URL](https://en.wikipedia.org/wiki/Uniform_Resource_Locator)
    ///    and [query string](https://en.wikipedia.org/wiki/Query_string) for an
    ///    [HTTP](https://en.wikipedia.org/wiki/HTTP)
    ///    [GET](https://www.w3schools.com/tags/ref_httpmethods.asp) request,
    ///
    /// 3. Perform the [HTTP](https://en.wikipedia.org/wiki/HTTP)
    ///    [GET](https://www.w3schools.com/tags/ref_httpmethods.asp) request
    ///    using the [reqwest](https://crates.io/crates/reqwest) crate.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    ///
    /// * This can fail if the request `struct` fails validation. For example,
    ///   parameters in the request conflict with one another, or the request
    ///   parameters are set in a way that's incompatible.
    ///
    ///   For example, Google Maps Directions API cannot calculate alternative
    ///   routes if waypoints have been set. This will cause a validation
    ///   failure.
    ///
    /// * The HTTP client cannot make a connection to the Google Maps API
    ///   server, or successfully send the request or receive the resposne over
    ///   the network.
    ///
    /// * The Google Maps API server returns an unexpected response, or data in
    ///   a format that's not expected.
    pub async fn execute(
        self
    ) -> Result<crate::elevation::Response, crate::Error> {
        self.client.get_request(self).await
    } // fn
} // impl