use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

// -----------------------------------------------------------------------------

impl crate::traits::QueryString for crate::places::place_autocomplete::Request<'_> {
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

        let mut query = format!(
            "key={}&input={}",
            self.client.key,
            utf8_percent_encode(&self.input, NON_ALPHANUMERIC),
        );

        // This section builds the "optional parameters" portion of the query
        // string:

        // Components key/value pair:
        if !self.components.is_empty() {
            query.push_str("&components=");
            let components = self
                .components
                .iter()
                .map(|component| format!("country:{}", String::from(component).to_lowercase()))
                .collect::<Vec<String>>()
                .join("|");
            query.push_str(&utf8_percent_encode(&components, NON_ALPHANUMERIC).to_string());
        } // if

        // Language key/value pair:
        if let Some(language) = &self.language {
            query.push_str("&language=");
            query.push_str(&String::from(language));
        }

        // Location key/value pair:
        if let Some(location) = &self.location {
            query.push_str("&location=");
            query.push_str(&String::from(location));
        }

        // Offset key/value pair:
        if let Some(offset) = &self.offset {
            query.push_str("&offset=");
            query.push_str(&offset.to_string());
        }

        // Origin key/value pair:
        if let Some(origin) = &self.origin {
            query.push_str("&origin=");
            query.push_str(&String::from(origin));
        }

        // Radius key/value pair:
        if let Some(radius) = &self.radius {
            query.push_str("&radius=");
            query.push_str(&radius.to_string());
        }

        // Region key/value pair:
        if let Some(region) = &self.region {
            query.push_str("&region=");
            query.push_str(&String::from(region));
        }

        // Session Token key/value pair:
        if let Some(sessiontoken) = &self.sessiontoken {
            query.push_str("&sessiontoken=");
            query.push_str(&utf8_percent_encode(sessiontoken, NON_ALPHANUMERIC).to_string());
        }

        // Strict Bounds key/value pair:
        if let Some(strictbounds) = &self.strictbounds {
            query.push_str("&strictbounds=");
            query.push_str(&strictbounds.to_string());
        }

        // Types key/value pair:
        if !self.types.is_empty() {
            query.push_str("&types=");
            let types = self
                .types
                .iter()
                .map(String::from)
                .collect::<Vec<String>>()
                .join("|");
            query.push_str(&types);
        }

        // Return built query string to caller:
        query
    } // fn
} // impl
