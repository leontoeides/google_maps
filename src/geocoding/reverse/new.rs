use crate::{
    geocoding::reverse::ReverseRequest,
    latlng::LatLng,
}; // use

impl ReverseRequest {

    /// Initializes the builder pattern for a Geolocation API query with the
    /// required, non-optional parameters.
    ///
    /// # Arguments:
    ///
    /// * `key` - Your application's Google Cloud API key.
    ///
    /// * `latlng` - The latitude and longitude values specifying the location
    /// for which you wish to obtain the closest, human-readable address.

    pub fn new(key: String, latlng: LatLng) -> ReverseRequest {
        // Instantiate struct and return it to caller:
        ReverseRequest {
            // Required parameters:
            key,
            latlng,
            // Optional parameters:
            language: None,
            location_type: None,
            result_type: None,
            // Internal use only:
            query: None,
        } // struct
    } // fn

} // impl