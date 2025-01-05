use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

// -----------------------------------------------------------------------------

impl crate::traits::QueryString for crate::roads::snap_to_roads::Request<'_> {
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
        let path: String = self
            .path
            .iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join("|");

        // URL encode path `String`:
        let path: String = utf8_percent_encode(&path, NON_ALPHANUMERIC).to_string();

        // Build "required parameters" portion of the query string:
        let mut query = format!("key={key}&path={path}", key = self.client.key);

        // This section builds the "optional parameters" portion of the query
        // string:

        // Language key/value pair:
        if let Some(interpolate) = &self.interpolate {
            query.push_str("&interpolate=");
            query.push_str(&interpolate.to_string());
        }

        // Return built query string to caller:
        query
    } // fn
} // impl
