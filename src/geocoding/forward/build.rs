impl<'r> crate::geocoding::ForwardRequest<'r> {
    /// Builds the URL [query string](https://en.wikipedia.org/wiki/Query_string)
    /// for the HTTP [GET](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/GET)
    /// request.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    ///
    /// ## Notes
    ///
    /// * The query string is the part of the URL after the `?` question mark.
    ///   For example, in the URL `https://example.com/over/there?name=ferret`
    ///   the query string is `name=ferret`
    ///
    /// * The `build` method has been removed. It would store the generated
    ///   query string inside of the request structure.
    ///
    ///   This way, the same query string would only have to be generated once
    ///   and could be used for any subsequent retries. This increased
    ///   implementation complexity but had very performance little benefit. It
    ///   has been removed.
    ///
    ///   If you want to generate a query string (without the preceding URL),
    ///   try the `query_string` method.
    #[deprecated(note = "try using the `query_string` method instead", since = "3.8.0")]
    pub fn build(
        &'r mut self
    ) -> Result<&'r mut Self, crate::geocoding::Error> {
        Ok(self)
    } // fn
} // impl
