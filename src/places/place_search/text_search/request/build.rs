use crate::place_type::PlaceType;
use crate::places::place_search::text_search::request::Request;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {

    /// Builds the query string for the Google Maps Places API _Text Search_
    /// query based on the input provided by the client.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn build(&mut self) -> &'a mut Request {

        // This section builds the "required parameters" portion of the query
        // string:

        let mut query = format!(
            "key={}&query={}",
            self.client.key,
            utf8_percent_encode(&self.input, NON_ALPHANUMERIC),
        );

        // This section builds the "optional parameters" portion of the query
        // string:

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

        // Page Token key/value pair:
        if let Some(pagetoken) = &self.pagetoken {
            query.push_str("&pagetoken=");
            query.push_str(pagetoken)
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

        // Place Type key/value pair:
        if let Some(place_types) = &self.place_types {
            query.push_str("&type=");
            query.push_str(&PlaceType::vec_to_csv(place_types))
        }

        // Set query string in Request struct.
        self.query = Some(query);

        // Return modified Request struct to caller.
        self

    } // fn

} // impl