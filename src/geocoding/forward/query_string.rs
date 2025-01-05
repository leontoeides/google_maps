use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

// -----------------------------------------------------------------------------

impl crate::traits::QueryString for crate::geocoding::ForwardRequest<'_> {
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

        let mut query = format!("key={}", self.client.key);

        // This section builds the "optional parameters" portion of the query
        // string:

        // Address key/value pair:
        if let Some(address) = &self.address {
            query.push_str("&address=");
            query.push_str(
                &utf8_percent_encode(&String::from(address), NON_ALPHANUMERIC).to_string(),
            );
        }

        // Place Id key/value pair:
        if let Some(place_id) = &self.place_id {
            query.push_str("&place_id=");
            query.push_str(
                &utf8_percent_encode(&String::from(place_id), NON_ALPHANUMERIC).to_string(),
            );
        }

        // Bounds key/value pair:
        if let Some(bounds) = &self.bounds {
            query.push_str("&bounds=");
            query.push_str(
                &utf8_percent_encode(&String::from(bounds), NON_ALPHANUMERIC).to_string(),
            );
        }

        // Components key/value pair:
        if !self.components.is_empty() {
            query.push_str("&components=");
            query.push_str(
                &utf8_percent_encode(
                    &self
                        .components
                        .iter()
                        .map(String::from)
                        .collect::<Vec<String>>()
                        .join("|"),
                    NON_ALPHANUMERIC,
                )
                .to_string(),
            ); // push_str
        }

        // Language key/value pair:
        if let Some(language) = &self.language {
            query.push_str("&language=");
            query.push_str(&String::from(language));
        }

        // Region key/value pair:
        if let Some(region) = &self.region {
            query.push_str("&region=");
            query.push_str(&String::from(region));
        }

        // Return built query string to caller:
        query
    } // fn
} // impl
