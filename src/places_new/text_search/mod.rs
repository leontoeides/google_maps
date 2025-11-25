//! Text Search (New) returns information about a set of places based on a string (for example,
//! "pizza in New York" or "shoe stores near Ottawa" or "123 Main Street").
//!
//! # [Migrate to Text Search (New)](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-text)
//!
//! > **European Economic Area (EEA) developers** If your billing address is in the European
//! > Economic Area, effective on 8 July 2025, the
//! > [Google Maps Platform EEA Terms of Service](https://cloud.google.com/terms/maps-platform/eea)
//! > will apply to your use of the Services. Functionality varies by region.
//! > [Learn more](https://developers.google.com/maps/comms/eea/faq).
//!
//! ## [Introduction](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-text#introduction)
//!
//! Places API (Legacy) supports
//! [Text Search (Legacy)](https://developers.google.com/maps/documentation/places/web-service/legacy/search-text).
//! If you are familiar with Text Search (Legacy),
//! [Text Search (New)](https://developers.google.com/maps/documentation/places/web-service/text-search)
//! makes the following changes:
//!
//! - **Text Search (New) uses HTTP POST requests.** Pass parameters in the request body or in
//!   headers as part of an HTTP POST request. In contrast, with Text Search (Legacy), you pass URL
//!   parameters using an HTTP GET request.
//!
//! - **Field masking is required.** You must specify which fields you want returned in the
//!   response. There is no default list of returned fields. If you omit this list, the methods
//!   return an error. For more information, see
//!   [FieldMask](https://developers.google.com/maps/documentation/places/web-service/text-search#fieldmask).
//!
//! - **Text Search (New) supports both
//!   [API keys](https://developers.google.com/maps/documentation/places/web-service/legacy/get-api-key) and
//!   [OAuth](https://developers.google.com/maps/documentation/places/web-service/legacy/oauth-token)
//!   tokens as the authentication mechanism.**
//!
//! - **Only JSON is supported as a response format in Text Search (New).**
//!
//! - **Text Search (Legacy) lets you specify latitude and longitude coordinates in the query
//!   string.** For example: `query=37.4239,-122.0925`. That option is not supported in Text Search
//!   (New). Use
//!   [Reverse Geocoding](https://developers.google.com/maps/documentation/geocoding/requests-reverse-geocoding)
//!   to search on coordinates and return an address, or
//!   [Nearby Search (New)](https://developers.google.com/maps/documentation/places/web-service/nearby-search)
//!   to find places around a certain location.
//!
//! - **The JSON response format for Text Search (New) has changed** from the format of the legacy
//!   APIs. For more details, see [Migrate the Places API
//!   response](https://developers.google.com/maps/documentation/places/web-service/migrate-response).
//!
//! ## Parameter Changes
//!
//! The following table lists parameters in Text Search (Legacy) that have been renamed or modified
//! for Text Search (New), or parameters that are no longer supported.
//!
//! | Current parameter   | New parameter         | Notes                                            |
//! |---------------------|-----------------------|--------------------------------------------------|
//! |                     | `evOptions`           | New parameter.                                   |
//! | `language`          | `languageCode`        |                                                  |
//! | `location`          | `locationBias`        |                                                  |
//! | `maxprice/minprice` | `priceLevels`         |                                                  |
//! | `opennow`           | `openNow`             |                                                  |
//! | `pagetoken`         | `pageToken`           |                                                  |
//! | `query`             | `textQuery`           | Required in all requests.                        |
//! | `radius`            | `locationBias`        | Specify the radius when defining a locationBias. |
//! | `region`            | `regionCode`          |                                                  |
//! | `type`              | `includedType`        | Only takes a single value.                       |
//! |                     | `strictTypeFiltering` | New parameter.                                   |
//!
//! ## [Example Request](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-text#example_request)
//!
//! The following example GET request uses Text Search (Legacy). In this example, you request a JSON response containing information about "Spicy Vegetarian Food in Sydney, Australia" and pass all parameters as URL parameters:
//!
//! ```curl
//! curl -L -X GET \
//! 'https://maps.googleapis.com/maps/api/place/textsearch/json?query=Spicy%20Vegetarian%20Food%20in%20Sydney%20Australia&key=API_KEY'
//! ```
//!
//! With Text Search (New), you make a POST request and pass all parameters in the JSON request body or in headers as part of the POST request. This example also uses a field mask so the response only includes the display name and formatted address of the place:
//!
//! ```curl
//! curl -X POST -d '{
//!   "textQuery" : "Spicy Vegetarian Food in Sydney, Australia"
//! }' \
//! -H 'Content-Type: application/json' -H 'X-Goog-Api-Key: API_KEY' \
//! -H 'X-Goog-FieldMask: places.displayName,places.formattedAddress' \
//! 'https://places.googleapis.com/v1/places:searchText'
//! ```

mod error;
pub use crate::places_new::text_search::error::Error;

mod request;
pub use crate::places_new::text_search::request::{
	EvOptions,
	Request
};

#[cfg(feature = "reqwest")]
mod request_with_client;
#[cfg(feature = "reqwest")]
pub use crate::places_new::text_search::request_with_client::RequestWithClient;

mod response;
pub use crate::places_new::text_search::response::Response;

mod response_with_context;
pub use crate::places_new::text_search::response_with_context::ResponseWithContext;