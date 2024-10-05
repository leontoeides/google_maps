use crate::client::GoogleMapsClient;
use crate::elevation::request::Request;

// =============================================================================

impl Request<'_> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Elevation API query with the
    /// required, non-optional parameters.
    ///
    /// ## Arguments
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.

    #[must_use]
    pub const fn new(client: &GoogleMapsClient) -> Request {
        // Instantiate struct and return it to caller:
        Request {
            // Required parameters:
            client,
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
