//! The **Query Autocomplete** service is used to provide a query prediction for
//! text-based geographic searches, by returning suggested queries as you type.
//!
//! # [Query Autocomplete requests](https://developers.google.com/maps/documentation/places/web-service/query#query_autocomplete_requests)
//!
//! * The Query Autocomplete service is part of the Places API and shares an
//!   [API key](https://developers.google.com/maps/documentation/places/web-service/get-api-key)
//!   and quotas with the [Places API](https://developers.google.com/maps/documentation/places/web-service/overview).
//!
//! The Query Autocomplete service allows you to add on-the-fly geographic
//! query predictions to your application. Instead of searching for a specific
//! location, a user can type in a categorical search, such as "pizza near New
//! York" and the service responds with a list of suggested queries matching the
//! string. As the Query Autocomplete service can match on both full words and
//! substrings, applications can send queries as the user types to provide
//! on-the-fly predictions.

pub mod request;

// -----------------------------------------------------------------------------

const SERVICE_URL: &str = "https://maps.googleapis.com/maps/api/place/autocomplete";
const OUTPUT_FORMAT: &str = "json"; // json or xml

// -----------------------------------------------------------------------------

pub use crate::places::place_autocomplete::{
    error::Error,
    response::{
        matched_substring::MatchedSubstring, prediction::Prediction, status::Status,
        structured_format::StructuredFormat, term::Term, Response,
    }, // response
}; // place_autocomplete

pub use crate::places::query_autocomplete::request::Request; // place_autocomplete
