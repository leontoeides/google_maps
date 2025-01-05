use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

// -----------------------------------------------------------------------------

impl crate::traits::QueryString for crate::roads::nearest_roads::Request<'_> {
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

        // Convert `Vec<LatLng>` to `String`:
        let points: String = self
            .points
            .iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join("|");

        // URL encode path `String`:
        let points: String = utf8_percent_encode(&points, NON_ALPHANUMERIC).to_string();

        // Build "required parameters" portion of the query string:
        let query = format!("key={key}&points={points}", key = self.client.key);

        // Return built query string to caller:
        query
    } // fn
} // impl
