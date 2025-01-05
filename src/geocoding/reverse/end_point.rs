#[cfg(feature = "reqwest")]
use crate::request_rate::api::Api;

// -----------------------------------------------------------------------------
//
/// Defines a Google Maps API HTTP end-point for a request.
///
/// This trait returns information needed to make connect HTTP `GET` requests to
/// their API end-point. It also includes some extra information for debugging
/// and rating-limiting.
impl crate::traits::EndPoint for crate::geocoding::ReverseRequest<'_> {
    fn service_url() -> &'static str { "https://maps.googleapis.com/maps/api/geocode" }
    #[cfg(feature = "reqwest")]
    fn title() -> &'static str { "Geocoding API Reverse" }
    #[cfg(feature = "reqwest")]
    fn apis() -> &'static [Api] { &[Api::All, Api::Geocoding] }
} // impl