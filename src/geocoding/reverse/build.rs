use crate::geocoding::reverse::ReverseRequest;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

impl<'a> ReverseRequest<'a> {

    /// Builds the query string for the Google Maps Geocoding API based on the
    /// input provided by the client.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn build(&mut self) -> &'a mut ReverseRequest {

        // This section builds the "required parameters" portion of the query
        // string:

        let mut query = format!(
            "key={}&latlng={}",
            self.client_settings.key,
            String::from(&self.latlng)
        ); // format!

        // This section builds the "optional parameters" portion of the query
        // string:

        // Language key/value pair:
        if let Some(language) = &self.language {
            query.push_str("&language=");
            query.push_str(&String::from(language))
        } // if

        // Location type(s) key/value pair:
        if let Some(location_type) = &self.location_type {
            query.push_str("&location_type=");
            query.push_str(&*utf8_percent_encode(
                &String::from(location_type.iter().map(|location_type| String::from(location_type) + "|").collect::<String>().trim_end_matches('|')),
                NON_ALPHANUMERIC
            ).to_string())
        } // if

        // Result type(s) key/value pair:
        if let Some(result_type) = &self.result_type {
            query.push_str("&result_type=");
            query.push_str(&*utf8_percent_encode(
                &String::from(result_type.iter().map(|result_type| String::from(result_type) + "|").collect::<String>().trim_end_matches('|')),
                NON_ALPHANUMERIC
            ).to_string())
        } // if

        // Set query string in ReverseRequest struct.
        self.query = Some(query);

        // Return modified ReverseRequest struct to caller.
        self

    } // fn

} // impl