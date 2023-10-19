use crate::places::place_search::nearby_search::request::Request;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Builds the query string for the Google Maps Places API _Nearby Search_
    /// query based on the input provided by the client.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn build(&mut self) -> &'a mut Request {
        // This section builds the "required parameters" portion of the query
        // string:

        let mut query = format!(
            "key={}&location={}&radius={}",
            self.client.key,
            String::from(&self.location),
            self.radius,
        );

        // This section builds the "optional parameters" portion of the query
        // string:

        if let Some(keyword) = &self.keyword {
            query.push_str("&keyword={}");
            query.push_str(&utf8_percent_encode(keyword, NON_ALPHANUMERIC).to_string())
        }

        if let Some(language) = &self.language {
            query.push_str("&language=");
            query.push_str(&String::from(language))
        }

        if let Some(maxprice) = &self.maxprice {
            query.push_str("&maxprice=");
            query.push_str(&maxprice.to_string())
        }

        if let Some(minprice) = &self.minprice {
            query.push_str("&minprice=");
            query.push_str(&minprice.to_string())
        }

        if let Some(opennow) = &self.opennow {
            if *opennow {
                query.push_str("&opennow")
            }
        }

        if let Some(pagetoken) = &self.pagetoken {
            query.push_str("&pagetoken=");
            query.push_str(pagetoken)
        }

        if let Some(rankby) = &self.rankby {
            query.push_str("&rankby=");
            query.push_str(&String::from(rankby))
        }

        if let Some(place_type) = &self.place_type {
            query.push_str("&type=");
            query.push_str(&String::from(place_type))
        }

        // Set query string in Request struct.
        self.query = Some(query);

        // Return modified Request struct to caller.
        self
    } // fn
} // impl
