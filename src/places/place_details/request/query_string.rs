use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

// -----------------------------------------------------------------------------

impl crate::traits::QueryString for crate::places::place_details::Request<'_> {
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
            "key={}&place_id={}",
            self.client.key,
            utf8_percent_encode(&self.place_id, NON_ALPHANUMERIC),
        );

        // This section builds the "optional parameters" portion of the query
        // string:

        // Fields key/value pair:
        if !self.fields.is_empty() {
            query.push_str("&fields=");
            query.push_str(&crate::places::place_details::Field::vec_to_csv(&self.fields));
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

        // `reviews_no_translations` key/value pair:
        if let Some(reviews_no_translations) = &self.reviews_no_translations {
            query.push_str("&reviews_no_translations=");
            query.push_str(&reviews_no_translations.to_string());
        }

        // Reviews Sort key/value pair:
        if let Some(reviews_sort) = &self.reviews_sort {
            query.push_str("&reviews_sort=");
            query.push_str(&String::from(reviews_sort));
        }

        // Session Token key/value pair:
        if let Some(sessiontoken) = &self.sessiontoken {
            query.push_str("&sessiontoken=");
            query.push_str(&String::from(sessiontoken));
        }

        // Return built query string to caller:
        query
    } // fn
} // impl
