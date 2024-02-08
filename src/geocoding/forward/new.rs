use crate::{client::GoogleMapsClient, geocoding::forward::ForwardRequest};

// =============================================================================

impl<'a> ForwardRequest<'a> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Geolocation API query with the
    /// required, non-optional parameters.
    ///
    /// ## Arguments:
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.

    #[must_use]
    pub const fn new(client: &GoogleMapsClient) -> ForwardRequest {
        // Instantiate struct and return it to caller:
        ForwardRequest {
            // Required parameters:
            client,
            // Optional parameters:
            address: None,
            place_id: None,
            bounds: None,
            components: None,
            language: None,
            region: None,
            // Internal use only:
            validated: false,
            query: None,
        } // struct
    } // fn
} // impl
