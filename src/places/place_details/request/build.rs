use crate::places::place_details::request::Request;
use crate::places::place_details::Field;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Builds the query string for the Google Maps Places API _Place
    /// Details_ query based on the input provided by the client.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn build(&mut self) -> &'a mut Request {
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
        if let Some(fields) = &self.fields {
            query.push_str("&fields=");
            query.push_str(&Field::vec_to_csv(fields));
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

        // Set query string in Request struct.
        self.query = Some(query);

        // Return modified Request struct to caller.
        self
    } // fn
} // impl
