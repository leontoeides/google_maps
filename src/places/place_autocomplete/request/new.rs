use crate::client::GoogleMapsClient;
use crate::places::place_autocomplete::request::Request;

// =============================================================================

impl<'a> Request<'a> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Place API Place Autocomplete query
    /// with the required, non-optional parameters.
    ///
    /// ## Arguments:
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    ///
    /// * `input` ‧ The text string on which to search.

    pub fn new(client: &GoogleMapsClient, input: String) -> Request {
        // Instantiate struct and return it to caller:
        Request {
            // Required parameters:
            client,
            input,
            // Optional parameters:
            components: vec![],
            language: None,
            location: None,
            offset: None,
            origin: None,
            radius: None,
            region: None,
            sessiontoken: None,
            strictbounds: None,
            types: vec![],
            // Internal use only:
            query: None,
        } // struct
    } // fn
} // impl
