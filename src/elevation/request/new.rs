use crate::client_settings::ClientSettings;
use crate::elevation::request::Request;

impl<'a> Request<'a> {

    /// Initializes the builder pattern for a Elevation API query with the
    /// required, non-optional parameters.
    ///
    /// ## Arguments:
    ///
    /// * `key` ‧ Your application's Google Cloud API key.

    pub fn new(client_settings: &ClientSettings) -> Request {
        // Instantiate struct and return it to caller:
        Request {
            // Required parameters:
            client_settings,
            // Positional requests:
            locations: None,
            // Sampled path requests:
            path: None,
            samples: None,
            // Internal use only:
            query: None,
            validated: false,
        } // struct
    } // fn

} // impl