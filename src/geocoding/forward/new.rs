use crate::{
    client_settings::ClientSettings, geocoding::forward::ForwardRequest
}; // use crate

impl<'a> ForwardRequest<'a> {

    /// Initializes the builder pattern for a Geolocation API query with the
    /// required, non-optional parameters.
    ///
    /// ## Arguments:
    ///
    /// * `key` ‧ Your application's Google Cloud API key.

    pub fn new(client_settings: &ClientSettings) -> ForwardRequest {
        // Instantiate struct and return it to caller:
        ForwardRequest {
            // Required parameters:
            client_settings,
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