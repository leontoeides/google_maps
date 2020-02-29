use chrono::NaiveDateTime;
use crate::{
    client_settings::ClientSettings,
    directions::request::location::Location,
    directions::request::waypoint::Waypoint,
    latlng::LatLng,
    request_rate::RequestRate,
}; // use

impl ClientSettings {

    /// Initialize the settings needed for a Google Cloud Maps API transaction.
    pub fn new(key: &str) -> ClientSettings {
        ClientSettings {
            key: String::from(key),
            max_retries: 20,
            max_backoff: 32000,
            rate_limit: RequestRate::default(),
        } // ClientSettings
    } // fn

    pub fn directions(&mut self, origin: Location, destination: Location) -> crate::directions::request::Request {
        crate::directions::request::Request::new(self, origin, destination)
    } // fn

    pub fn distance_matrix(&mut self, origins: Vec<Waypoint>, destinations: Vec<Waypoint>) -> crate::distance_matrix::request::Request {
        crate::distance_matrix::request::Request::new(self, origins, destinations)
    } // fn

    pub fn elevation(&mut self) -> crate::elevation::request::Request {
        crate::elevation::request::Request::new(self)
    } // fn

    pub fn geocoding(&mut self) -> crate::geocoding::forward::ForwardRequest {
        crate::geocoding::forward::ForwardRequest::new(self)
    } // fn

    pub fn reverse_geocoding(&mut self, latlng: LatLng) -> crate::geocoding::reverse::ReverseRequest {
        crate::geocoding::reverse::ReverseRequest::new(self, latlng)
    } // fn

    pub fn time_zone(&mut self, location: LatLng, time: NaiveDateTime) -> crate::time_zone::request::Request {
        crate::time_zone::request::Request::new(self, location, time)
    } // fn

} // impl