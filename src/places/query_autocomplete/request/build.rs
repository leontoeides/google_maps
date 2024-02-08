use crate::places::query_autocomplete::request::Request;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Builds the query string for the Google Maps Places API _Query
    /// Autocomplete_ query based on the input provided by the client.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn build(&mut self) -> &'a mut Request {
        // This section builds the "required parameters" portion of the query
        // string:

        let mut query = format!(
            "key={}&input={}",
            self.client.key,
            utf8_percent_encode(&self.input, NON_ALPHANUMERIC),
        );

        // This section builds the "optional parameters" portion of the query
        // string:

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

        // Radius key/value pair:
        if let Some(radius) = &self.radius {
            query.push_str("&radius=");
            query.push_str(&radius.to_string());
        }

        // Set query string in Request struct.
        self.query = Some(query);

        // Return modified Request struct to caller.
        self
    } // fn
} // impl
