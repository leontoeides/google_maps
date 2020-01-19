use crate::elevation::request::Request;

impl Request {

    /// Initializes the builder pattern for a Time Zone API query with the
    /// required, non-optional parameters.
    ///
    /// # Arguments:
    ///
    /// * `key` - Your application's Google Cloud API key.

    pub fn new(key: String) -> Request {
        // Instantiate struct and return it to caller:
        Request {
            // Required parameters:
            key,
            // Positional requests:
            locations: None,
            // Sampled path requests:
            path: None,
            samples: None,
            // Internal use only:
            validated: false,
            query: None,
        } // struct
    } // fn

} // impl