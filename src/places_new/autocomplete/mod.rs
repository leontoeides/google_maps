//! Autocomplete (New) is a web service that returns place predictions and query predictions in
//! response to an HTTP request.
//!
//! In the request, specify a text search string and geographic bounds that controls the search
//! area.
//!
//! Autocomplete (New) can match on full words and substrings of the input, resolving place names,
//! addresses, and plus codes. Applications can therefore send queries as the user types, to provide
//! on-the-fly place and query predictions.
//!
//! The response from Autocomplete (New) can contain two types of predictions:
//!
//! - **Place predictions**: Places, such as businesses, addresses and points of interest, based on
//!   the specified input text string and search area. Place predictions are returned by default.
//!
//! - **Query predictions**: Query strings matching the input text string and search area. Query
//!   predictions are not returned by default. Use the includeQueryPredictions request parameter to
//!   add query predictions to the response.
//!
//! For example, you call Autocomplete (New) using as input a string that contains a partial user
//! input, "Sicilian piz", with the search area limited to San Francisco, CA. The response then
//! contains a list of place predictions that match the search string and search area, such as the
//! restaurant named "Sicilian Pizza Kitchen", along with details about the place.
//!
//! The returned place predictions are designed to be presented to the user to aid them in selecting
//! the intended place. You can make a [Place Details
//! (New)](https://developers.google.com/maps/documentation/places/web-service/place-details)
//! request to get more information about any of the returned place predictions.
//!
//! The response can also contain a list of query predictions that match the search string and
//! search area, such as "Sicilian Pizza & Pasta". Each query prediction in the response includes
//! the text field containing a recommended text search string. Use that string as an input to [Text
//! Search (New)](https://developers.google.com/maps/documentation/places/web-service/text-search)
//! to perform a more detailed search.
//!
//! # [Migrate to Autocomplete (New)](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-autocomplete)
//!
//! > **European Economic Area (EEA) developers** If your billing address is in the European
//! > Economic Area, effective on 8 July 2025, the
//! > [Google Maps Platform EEA Terms of Service](https://cloud.google.com/terms/maps-platform/eea)
//! > will apply to your use of the Services. Functionality varies by region.
//! > [Learn more](https://developers.google.com/maps/comms/eea/faq).
//!
//! ## [Introduction](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-autocomplete#introduction)
//!
//! Places API supports the
//! [Place Autocomplete (Legacy)](https://developers.google.com/maps/documentation/places/web-service/autocomplete) and
//! [Query Autocomplete (Legacy)](https://developers.google.com/maps/documentation/places/web-service/query) APIs.
//! If you are familiar with these APIs, Autocomplete (New) makes the following changes:
//!
//! - **Autocomplete (New) uses HTTP POST requests.** Pass parameters in the request body or in
//!   headers as part of an HTTP POST request. In contrast, with the legacy APIs, you pass URL
//!   parameters using an HTTP GET request.
//!
//! - **Autocomplete (New) supports both
//!   [API keys](https://developers.google.com/maps/documentation/places/web-service/get-api-key)
//!   and [OAuth](https://developers.google.com/maps/documentation/places/web-service/oauth-token)
//!   tokens as the authentication mechanism.**
//!
//! - **Only JSON is supported as a response format in Autocomplete (New).**
//!
//! ## Parameter Changes
//!
//! The following table lists parameters in Place Autocomplete (Legacy) and Query Autocomplete
//! (Legacy) that have been renamed or modified for Autocomplete (New), or parameters that are no
//! longer supported.
//!
//! | Current parameter | New parameter                           | Notes |
//! |-------------------|-----------------------------------------|-------|
//! | `components`      | `includedRegionCodes`                   | |
//! | `language`        | `languageCode`                          | |
//! | `location`        | `locationBias`                          | |
//! | `ipbias`          |                                         | If you omit both `locationBias` and `locationRestriction`, then the API uses IP biasing by default. |
//! | `offset`          | `inputOffset`                           | |
//! | `radius`          | `locationBias` or `locationRestriction` | |
//! | `region`          | `regionCode`                            | |
//! | `strictbounds`    | `locationRestriction`                   | |
//! | `sessiontoken`    | `sessionToken`                          | |
//! | `types`           | `includedPrimaryTypes`                  | |
//!
//! ## [Example Request](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-autocomplete#example_request)
//!
//! The following example GET request uses Place Autocomplete (Legacy). In this example, you request
//! a JSON response containing information about places called "Amoeba" that are of type
//! `establishment` and pass all parameters as URL parameters:
//!
//! ```curl
//! curl -L -X GET \
//! 'https://maps.googleapis.com/maps/api/place/autocomplete/json?input=amoeba&types=establishment&location=37.76999%2C-122.44696&radius=500&key=YOUR_API_KEY'
//! ```
//!
//! With Autocomplete (New), you make a POST request and pass all parameters in the JSON request
//! body or in headers as part of the POST request.
//!
//! Field masks are not required for Autocomplete (New) requests, but they can be used to filter for
//! specific results in the response. For example, the following request uses a field mask so the
//! response only includes the `suggestions.placePrediction.text` of the suggestion:
//!
//! ```curl
//! curl -X POST -d '{
//!   "input": "Amoeba",
//!   "locationBias": {
//!     "circle": {
//!       "center": {
//!         "latitude": 37.76999,
//!         "longitude": -122.44696
//!       },
//!       "radius": 500.0
//!     }
//!   }
//! }' \
//! -H 'Content-Type: application/json' -H 'X-Goog-Api-Key: API_KEY' \
//! -H 'X-Goog-FieldMask: suggestions.placePrediction.text' \
//! https://places.googleapis.com/v1/places:autocomplete
//! ```
//!
//! To learn more about using field masks, see [Place Details
//! (New)](https://developers.google.com/maps/documentation/places/web-service/place-details#fieldmask).

mod error;
pub use crate::places_new::autocomplete::error::Error;

mod request;
pub use crate::places_new::autocomplete::request::Request;

#[cfg(feature = "reqwest")]
mod request_with_client;
#[cfg(feature = "reqwest")]
pub use crate::places_new::autocomplete::request_with_client::{
	RequestWithClient,
	RequestWithClientBuilder
};

mod response;
pub use crate::places_new::autocomplete::response::{
	FormattableText,
	PlacePrediction,
	QueryPrediction,
	Response,
	StringRange,
	StructuredFormat,
	Suggestion
};

mod response_with_context;
pub use crate::places_new::autocomplete::response_with_context::ResponseWithContext;