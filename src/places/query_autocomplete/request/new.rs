use crate::client::GoogleMapsClient;
use crate::places::query_autocomplete::request::Request;

// =============================================================================

impl<'a> Request<'a> {

    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Place API _Query Autocomplete_
    /// query with the required, non-optional parameters.
    ///
    /// ## Arguments:
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    ///
    /// * `input` ‧ The text string on which to search.

    pub fn new(
        client: &GoogleMapsClient,
        input: String,
    ) -> Request {

        // Instantiate struct and return it to caller:
        Request {
            // Required parameters:
            client,
            input,
            // Optional parameters:
            language: None,
            location: None,
            offset: None,
            radius: None,
            // Internal use only:
            query: None,
        } // struct

    } // fn

} // impl