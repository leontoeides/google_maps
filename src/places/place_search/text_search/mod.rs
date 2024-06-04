//! A **Text Search** returns information about a set of places based on a
//! string — for example "pizza in New York" or "shoe stores near Ottawa" or
//! "123 Main Street". The service responds with a list of places matching the
//! text string and any location bias that has been set.
//!
//! The service is especially useful for making ambiguous address queries in an
//! automated system, and non-address components of the string may match
//! businesses as well as addresses. Examples of ambiguous address queries are
//! poorly-formatted addresses or requests that include non-address components
//! such as business names. Requests like the first two examples below may
//! return `ZERO_RESULTS` unless a location bias - such as Region, Location, or
//! Bounds - is set.
//!
//! | "10 High Street, UK" or "123 Main Street, US" | multiple "High Street"s in the UK; multiple "Main Street"s in the US. Query will not return desirable results unless a location restriction is set. |
//! |---|---|
//! | "`ChainRestaurant` New York" | multiple "`ChainRestaurant`" locations in New York; no street address or even street name. Query will not return desirable results unless a location restriction is set. |
//! | "10 High Street, Escher UK" or "123 Main Street, Pleasanton US" | only one "High Street" in the UK city of Escher; only one "Main Street" in the US city of Pleasanton CA. |
//! | "`UniqueRestaurantName` New York" | only one establishment with this name in New York; no street address needed to differentiate. |
//! | "pizza restaurants in New York" | this query contains its location restriction, and "pizza restaurants" is a well-defined place type. Will yield multiple results, as is expected. |
//!
//! The search response will include a list of places. You can send a Place
//! Details request for more information about any of the places in the
//! response.
//!
//! * Nearby Search and Text Search return all of the available data fields for
//!   the selected place (a [subset of the supported fields](https://developers.google.com/maps/documentation/places/web-service/place-data-fields#places-api-fields-support)),
//!   and you will be [billed accordingly](https://developers.google.com/maps/billing/understanding-cost-of-use#nearby-search)
//!   There is no way to constrain Nearby Search or Text Search to only return
//!   specific fields. To keep from requesting (and paying for) data that you
//!   don't need, use a [Find Place request](https://developers.google.com/maps/documentation/places/web-service/search#FindPlaceRequests)
//!   instead.

pub mod request;
pub mod response;

// -----------------------------------------------------------------------------

const SERVICE_URL: &str = "https://maps.googleapis.com/maps/api/place/textsearch";
const OUTPUT_FORMAT: &str = "json"; // json or xml

// -----------------------------------------------------------------------------

pub use crate::places::{error::Error, status::Status}; // place_search

pub use crate::places::place_search::text_search::{request::Request, response::Response}; // text_search
