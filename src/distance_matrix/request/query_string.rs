use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

// -----------------------------------------------------------------------------

impl crate::traits::QueryString for crate::distance_matrix::Request<'_> {
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
        // Builds the "required parameters" portion of the query string:

        let mut query = format!(
            "key={}&origins={}&destinations={}",
            // Key:
            self.client.key,
            // Origins:
            utf8_percent_encode(
                &self
                    .origins
                    .iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
                    .join("|"),
                NON_ALPHANUMERIC
            ),
            // Destinations:
            utf8_percent_encode(
                &self
                    .destinations
                    .iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
                    .join("|"),
                NON_ALPHANUMERIC
            ),
        ); // format!

        // Builds the "optional parameters" portion of the query string:

        // Arrival time key/value pair:
        if let Some(arrival_time) = &self.arrival_time {
            query.push_str("&arrival_time=");
            query.push_str(&arrival_time.and_utc().timestamp().to_string());
        } // if

        // Avoid key/value pair:
        if !self.restrictions.is_empty() {
            query.push_str("&avoid=");
            query.push_str(
                &utf8_percent_encode(
                    &self
                        .restrictions
                        .iter()
                        .map(String::from)
                        .collect::<Vec<String>>()
                        .join("|"),
                    NON_ALPHANUMERIC,
                )
                .to_string(),
            ); // push_str
        } // if

        // Departure time key/value pair:
        if let Some(departure_time) = &self.departure_time {
            query.push_str("&departure_time=");
            query.push_str(&String::from(departure_time));
        } // if

        // Language key/value pair:
        if let Some(language) = &self.language {
            query.push_str("&language=");
            query.push_str(&String::from(language));
        } // if

        // Travel mode key/value pair:
        if let Some(travel_mode) = &self.travel_mode {
            query.push_str("&mode=");
            query.push_str(&String::from(travel_mode).to_lowercase());
        } // if

        // Region key/value pair:
        if let Some(region) = &self.region {
            query.push_str("&region=");
            query.push_str(&String::from(region));
        } // if

        // Traffic model key/value pair:
        if let Some(traffic_model) = &self.traffic_model {
            query.push_str("&traffic_model=");
            query.push_str(&String::from(traffic_model));
        } // if

        // Transit mode key/value pair:
        if !self.transit_modes.is_empty() {
            query.push_str("&transit_mode=");
            query.push_str(
                &utf8_percent_encode(
                    &self
                        .transit_modes
                        .iter()
                        .map(String::from)
                        .collect::<Vec<String>>()
                        .join("|"),
                    NON_ALPHANUMERIC,
                )
                .to_string(),
            ); // push_str
        } // if

        // Transit route preference key/value pair:
        if let Some(transit_route_preference) = &self.transit_route_preference {
            query.push_str("&transit_routing_preference=");
            query.push_str(&String::from(transit_route_preference));
        } // if

        // Unit system key/value pair:
        if let Some(unit_system) = &self.unit_system {
            query.push_str("&units=");
            query.push_str(&String::from(unit_system));
        } // if

        // Return built query string to caller:
        query
    } // fn
} // impl
