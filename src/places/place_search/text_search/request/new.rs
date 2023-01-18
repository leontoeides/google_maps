use crate::client::GoogleMapsClient;
use crate::places::place_search::text_search::request::Request;

// =============================================================================

impl<'a> Request<'a> {

    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Places API _Text Search_ query
    /// with the required, non-optional parameters.
    ///
    /// ## Arguments:
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    /// * `query` ‧ The text string on which to search, for example:
    /// "restaurant" or "123 Main Street". This must a place name, address, or
    /// category of establishments. Any other types of input can generate errors
    /// and are not guaranteed to return valid results. The Google Places
    /// service will return candidate matches based on this string and order the
    /// results based on their perceived relevance.

    pub fn new(
        client: &GoogleMapsClient,
        query: String,
    ) -> Request {

        // Instantiate struct and return it to caller:
        Request {
            // Required parameters:
            client,
            input: query,
            // Optional parameters:
            language: None,
            location: None,
            maxprice: None,
            minprice: None,
            opennow: None,
            pagetoken: None,
            radius: None,
            region: None,
            place_types: None,
            // Internal use only:
            query: None,
        } // struct

    } // fn

} // impl