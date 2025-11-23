//! A Nearby Search (New) request takes one or more place types, and returns a list of matching
//! places within the specified area.
//!
//! # [Migrate to Nearby Search (New)](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-nearby)
//!
//! > **European Economic Area (EEA) developers** If your billing address is in the European
//! > Economic Area, effective on 8 July 2025, the
//! > [Google Maps Platform EEA Terms of Service](https://cloud.google.com/terms/maps-platform/eea)
//! > will apply to your use of the Services. Functionality varies by region.
//! > [Learn more](https://developers.google.com/maps/comms/eea/faq).
//!
//! ## [Introduction](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-nearby#introduction)
//!
//! Places API supports
//! [Nearby Search (Legacy)](https://developers.google.com/maps/documentation/places/web-service/search-nearby).
//! If you are familiar with Nearby Search (Legacy), the
//! [Nearby Search (New)](https://developers.google.com/maps/documentation/places/web-service/nearby-search)
//! makes the following changes:
//!
//! - **Nearby Search (New) uses HTTP POST requests.** Pass parameters in the request body or in
//!   headers as part of an HTTP POST request. In contrast, with Nearby Search (Legacy), you pass
//!   URL parameters using an HTTP GET request.
//!
//! - **Field masking is required.** You must specify which fields you want returned in the
//!   response. There is no default list of returned fields. If you omit this list, the methods
//!   return an error. For more information, see
//!   [FieldMask](https://developers.google.com/maps/documentation/places/web-service/nearby-search#fieldmask).
//!
//! - **Nearby Search (New) supports both
//!   [API keys](https://developers.google.com/maps/documentation/places/web-service/get-api-key) and
//!   [OAuth](https://developers.google.com/maps/documentation/places/web-service/oauth-token)
//!   tokens as the authentication mechanism.**
//!
//! - **Only JSON is supported as a response format in Nearby Search (New).**
//!
//! - **All requests that include a text query should now use [Text Search
//!   (New)](https://developers.google.com/maps/documentation/places/web-service/text-search)**
//!   because Nearby Search (New) does not support text input.
//!
//! - **The JSON response format for Nearby Search (New) has changed** from the format of the legacy
//!   APIs. For more details, see [Migrate the Places API
//!   response](https://developers.google.com/maps/documentation/places/web-service/migrate-response).
//!
//! ## Parameter Changes
//!
//! The following table lists parameters in Nearby Search (Legacy) that have been renamed or
//! modified for Nearby Search (New), or parameters that are no longer supported.
//!
//! | Current parameter   | New parameter                                                                 | Notes |
//! |---------------------|-------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------|
//! | `keyword`           |                                                                               | Not supported. Use [Text Search (New)](https://developers.google.com/maps/documentation/places/web-service/text-search) instead. |
//! | `language`          | `languageCode`                                                                |                                                                                                                                  |
//! | `location`          | `locationRestriction`                                                         | Required.                                                                                                                        |
//! | `maxprice/minprice` |                                                                               | Not supported.                                                                                                                   |
//! |                     | `maxResultCount`                                                              | New parameter.                                                                                                                   |
//! | `opennow`           |                                                                               | Not supported.                                                                                                                   |
//! | `pagetoken`         |                                                                               | Not supported.                                                                                                                   |
//! | `radius`            |                                                                               | Use `locationRestriction` now.                                                                                                   |
//! | `rankby`            | `rankPreference`                                                              |                                                                                                                                  |
//! |                     | `regionCode`                                                                  | New parameter.                                                                                                                   |
//! | `type`              | `includedTypes` `excludedTypes` `includedPrimaryTypes` `excludedPrimaryTypes` | The new parameters also accept multiple type values. The legacy API only accepts a single value.                                 |
//!
//! ## [Example Request](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-nearby#example_request)
//!
//! The following example GET request uses the Nearby Search (Legacy). In this example, you request a JSON response containing information about places of type `restaurant` and pass all parameters as URL parameters:
//!
//! ```curl
//! curl -L -X GET \
//! 'https://maps.googleapis.com/maps/api/place/nearbysearch/json?location=37.7937%2C-122.3965&radius=500&type=restaurant&key=API_KEY'
//! ```
//!
//! With Nearby Search (New), you make a POST request and pass all parameters in the JSON request body or in headers as part of the POST request. This example also uses a field mask so the response only includes the display name and formatted address of the place:
//!
//! ```curl
//! curl -X POST -d '{
//!  "includedTypes": ["restaurant"],
//!  "locationRestriction": {
//!    "circle": {
//!      "center": {
//!        "latitude": 37.7937,
//!        "longitude": -122.3965},
//!      "radius": 500.0
//!    }
//!  }
//! }' \
//! -H 'Content-Type: application/json' -H "X-Goog-Api-Key: API_KEY" \
//! -H "X-Goog-FieldMask: places.displayName,places.formattedAddress" \
//! https://places.googleapis.com/v1/places:searchNearby
//! ```

mod error;
pub use crate::places_new::nearby_search::error::Error;

mod request;
pub use crate::places_new::nearby_search::request::Request;

#[cfg(feature = "reqwest")]
mod request_with_client;
#[cfg(feature = "reqwest")]
pub use crate::places_new::nearby_search::request_with_client::RequestWithClient;

mod response;
pub use crate::places_new::nearby_search::response::Response;