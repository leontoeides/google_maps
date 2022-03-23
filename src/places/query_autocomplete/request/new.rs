use crate::client_settings::ClientSettings;
use crate::places::query_autocomplete::request::Request;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {

    /// Initializes the builder pattern for a Place API _Query Autocomplete_
    /// query with the required, non-optional parameters.
    ///
    /// ## Arguments:
    ///
    /// * `key` - Your application's Google Cloud API key.
    /// * `input` - The text string on which to search.

    pub fn new(
        client_settings: &ClientSettings,
        input: String,
    ) -> Request {
        // Instantiate struct and return it to caller:
        Request {
            // Required parameters:
            client_settings,
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