//! A Nearby Search lets you search for places within a specified area. You can
//! refine your search request by supplying keywords or specifying the type of
//! place you are searching for.
//!
//! * Nearby Search and Nearby Search return all of the available data fields
//!   for the selected place (a
//!   [subset of the supported fields](https://developers.google.com/maps/documentation/places/web-service/place-data-fields#places-api-fields-support)),
//!   and you will be [billed accordingly](https://developers.google.com/maps/billing/understanding-cost-of-use#nearby-search)
//!   There is no way to constrain Nearby Search or Nearby Search to only return
//!   specific fields. To keep from requesting (and paying for) data that you
//!   don't need, use a [Find Place request](https://developers.google.com/maps/documentation/places/web-service/search#FindPlaceRequests)
//!   instead.

pub mod request;
pub mod response;

// -----------------------------------------------------------------------------

const SERVICE_URL: &str = "https://maps.googleapis.com/maps/api/place/nearbysearch";
const OUTPUT_FORMAT: &str = "json"; // json or xml

// -----------------------------------------------------------------------------

pub use crate::places::{error::Error, status::Status}; // place_search

pub use crate::places::place_search::nearby_search::{request::Request, response::Response}; // nearby_search
