use crate::places::place_autocomplete::request::Request;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {

    /// Builds the query string for the Google Maps Time Zone API based on the
    /// input provided by the client.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn build(&mut self) -> &'a mut Request {

        // This section builds the "required parameters" portion of the query
        // string:

        let mut query = format!(
            "key={}&input={}",
            self.client_settings.key,
            utf8_percent_encode(&self.input, NON_ALPHANUMERIC),
        );

        // This section builds the "optional parameters" portion of the query
        // string:

        // Components key/value pair:
        if !self.components.is_empty() {
            query.push_str("&components=");
            let components = self.components
                .iter()
                .map(|component| format!("country:{}", String::from(component).to_lowercase()))
                .collect::<Vec<String>>()
                .join("|");
            query.push_str(&utf8_percent_encode(&components, NON_ALPHANUMERIC).to_string())
        } // if

        // Language key/value pair:
        if let Some(language) = &self.language {
            query.push_str("&language=");
            query.push_str(&String::from(language))
        }

        // Location key/value pair:
        if let Some(location) = &self.location {
            query.push_str("&location=");
            query.push_str(&String::from(location))
        }

        // Offset key/value pair:
        if let Some(offset) = &self.offset {
            query.push_str("&offset=");
            query.push_str(&offset.to_string())
        }

        // Origin key/value pair:
        if let Some(origin) = &self.origin {
            query.push_str("&origin=");
            query.push_str(&String::from(origin))
        }

        // Radius key/value pair:
        if let Some(radius) = &self.radius {
            query.push_str("&radius=");
            query.push_str(&radius.to_string())
        }

        // Region key/value pair:
        if let Some(region) = &self.region {
            query.push_str("&region=");
            query.push_str(&String::from(region))
        }

        // Session Token key/value pair:
        if let Some(sessiontoken) = &self.sessiontoken {
            query.push_str("&sessiontoken=");
            query.push_str(&utf8_percent_encode(sessiontoken, NON_ALPHANUMERIC).to_string())
        }

        // Strict Bounds key/value pair:
        if let Some(strictbounds) = &self.strictbounds {
            query.push_str("&strictbounds=");
            query.push_str(&strictbounds.to_string())
        }

        // Strict Bounds key/value pair:
        if !self.types.is_empty() {
            query.push_str("&types=");
            let types = self.types
                .iter()
                .map(String::from)
                .collect::<Vec<String>>()
                .join("|");
            query.push_str(&types);
        }

        // Set query string in Request struct.
        self.query = Some(query);

        // Return modified Request struct to caller.
        self

    } // fn

} // impl