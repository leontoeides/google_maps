#[cfg(feature = "reqwest")]
use crate::request_rate::api::Api;

// -----------------------------------------------------------------------------
//
/// Defines a Google Maps API HTTP end-point for a request.
///
/// This trait returns information needed to make connect HTTP `GET` requests to
/// their API end-point. It also includes some extra information for debugging
/// and rating-limiting.
impl crate::traits::EndPoint for crate::places::place_search::text_search::Request<'_> {
    fn service_url() -> &'static str { "https://maps.googleapis.com/maps/api/place/textsearch" }
    #[cfg(feature = "reqwest")]
    fn title() -> &'static str { "Places API Text Search" }
    #[cfg(feature = "reqwest")]
    fn apis() -> &'static [Api] { &[Api::All, Api::Places] }
} // impl