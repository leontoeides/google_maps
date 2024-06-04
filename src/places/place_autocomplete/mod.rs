//! The **Place Autocomplete** service is a web service that returns place
//! predictions. The request specifies a textual search string and optional
//! geographic bounds. The service can be used to provide autocomplete
//! functionality for text-based geographic searches, by returning places such
//! as businesses, addresses and points of interest as a user types.
//!
//! ## Note: Server-side and client-side libraries
//!
//! * The Places API is also available with the [Java Client, Python Client, Go
//!   Client and Node.js Client for Google Maps Services](https://developers.google.com/maps/documentation/places/web-service/client-library).
//!   The Places API and the client libraries are for use in server applications.
//!
//! * If you're building a client-side application, take a look at the [Places
//!   SDK for Android](https://developers.google.com/maps/documentation/places/android-sdk),
//!   the [Places SDK for iOS](https://developers.google.com/maps/documentation/places/ios-sdk),
//!   and the [Places Library, Maps JavaScript API](https://developers.google.com/maps/documentation/javascript/places).
//!
//! # [Place Autocomplete requests](https://developers.google.com/maps/documentation/places/web-service/autocomplete#place_autocomplete_requests)
//!
//! * The Place Autocomplete service is part of the Places API and shares an
//!   [API key](https://developers.google.com/maps/documentation/places/web-service/get-api-key)
//!   and quotas with the [Places API](https://developers.google.com/maps/documentation/places/web-service/overview).
//!
//! * **Note**: You can use Place Autocomplete even without a map. If you do
//!   show a map, it must be a Google map. When you display predictions from the
//!   Place Autocomplete service without a map, you must include the
//!   '[Powered by Google](https://developers.google.com/maps/documentation/places/web-service/policies#logo_requirements)'
//!   logo displayed inline with the search field/results.
//!
//! The Place Autocomplete service can match on full words and substrings,
//! resolving place names, addresses, and [plus codes](https://plus.codes/).
//! Applications can therefore send queries as the user types, to provide
//! on-the-fly place predictions.
//!
//! The returned predictions are designed to be presented to the user to aid
//! them in selecting the desired place. You can send a [Place Details
//! request](https://developers.google.com/maps/documentation/places/web-service/details#PlaceDetailsRequests)
//! for more information about any of the places which are returned.

pub mod error;
pub mod request;
pub mod response;

// -----------------------------------------------------------------------------

const SERVICE_URL: &str = "https://maps.googleapis.com/maps/api/place/autocomplete";
const OUTPUT_FORMAT: &str = "json"; // json or xml

// -----------------------------------------------------------------------------

pub use crate::places::place_autocomplete::{
    error::Error,
    request::{autocomplete_type::AutocompleteType, Request}, // request
    response::{
        matched_substring::MatchedSubstring, prediction::Prediction, status::Status,
        structured_format::StructuredFormat, term::Term, Response,
    }, // response
}; // place_autocomplete
