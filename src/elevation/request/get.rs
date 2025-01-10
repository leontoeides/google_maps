impl crate::elevation::Request<'_> {
    /// Performs the HTTP get request and returns the response.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    ///
    /// ## Notes
    ///
    /// * The `get` method for this request has been moved to a `get` method
    ///   that's been generically implemented for all APIs and services,
    ///   implemented on the `google_maps::Client` struct.
    ///
    ///   Try using `execute` method for a somewhat similar result. The main
    ///   difference is that the execute method will validate the request and
    ///   build the URL string, whereas the previous `get` implementation
    ///   (that's been deprecated) would blindly submit the request, if any.
    #[deprecated(note = "try using `execute` instead", since = "3.8.0")]
    pub async fn get(
        self
    ) -> Result<crate::elevation::Response, crate::Error> {
        self.client.get_request(self).await
    } // fn
} // impl