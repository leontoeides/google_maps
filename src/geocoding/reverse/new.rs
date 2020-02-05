use crate::{
    geocoding::reverse::ReverseRequest,
    client_settings::ClientSettings,
    latlng::LatLng,
}; // use

impl<'a> ReverseRequest<'a> {

    /// Initializes the builder pattern for a Geolocation API query with the
    /// required, non-optional parameters.
    ///
    /// # Arguments:
    ///
    /// * `key` - Your application's Google Cloud API key.
    ///
    /// * `latlng` - The latitude and longitude values specifying the location
    /// for which you wish to obtain the closest, human-readable address.

    pub fn new(client_settings: &mut ClientSettings, latlng: LatLng) -> ReverseRequest {
        // Instantiate struct and return it to caller:
        ReverseRequest {
            // Required parameters:
            client_settings,
            latlng,
            // Optional parameters:
            language: None,
            location_types: None,
            result_types: None,
            // Internal use only:
            validated: false,
            query: None,
        } // struct
    } // fn

} // impl