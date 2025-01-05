impl crate::traits::QueryString for crate::elevation::Request<'_> {
    /// Builds the URL [query string](https://en.wikipedia.org/wiki/Query_string)
    /// for the HTTP [GET](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/GET)
    /// request. The query string is generated from the data found in this
    /// `Request` structure.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    ///
    /// ## Notes
    ///
    /// * This function does not validate the request before generating the
    ///   _query string_. However, the superior method that generates the _query
    ///   URL_ does perform validation.
    ///
    /// * The query string is the part of the URL after the `?` question mark.
    ///   For example, in the URL `https://example.com/over/there?name=ferret`
    ///   the query string is `name=ferret`
    ///
    /// * There's no benefit to working on an owned `Request` struct (i.e. an
    ///   owned `self` versus an borrowed `&self`).
    ///   [percent-encoding](https://crates.io/crates/percent-encoding)
    ///   works on borrowed UTF-8 strings. Other types, such as enums and
    ///   numeric values are converted into strings. Therefore no zero-copy
    ///   operations are possible with an owned `self`.
    fn query_string(&self) -> String {
        // This section builds the "required parameters" portion of the query
        // string:

        let mut query = String::from("key=");
        query.push_str(&self.client.key);

        // This section builds the "positional request" portion of the query
        // string:

        // Locations key/value pair:
        if let Some(locations) = &self.locations {
            query.push_str("&locations=");
            query.push_str(&String::from(locations));
        } // if

        // This section builds the "sampled path request" portion of the query
        // string:

        // Path key/value pair:
        if let Some(path) = &self.path {
            query.push_str("&path=");
            query.push_str(&String::from(path));
        } // if

        // Samples key/value pair:
        if let Some(samples) = &self.samples {
            query.push_str("&samples=");
            query.push_str(&samples.to_string());
        } // if

        // Return built query string to caller:
        query
    } // fn
} // impl
