//! Once you have a [place ID](https://developers.google.com/maps/documentation/places/web-service/place-id),
//! you can request more details about a particular establishment or point of interest by initiating
//! a Place Details (New) request.
//!
//! A Place Details (New) request returns more comprehensive information about the indicated place
//! such as its complete address, phone number, user rating and reviews.
//!
//! There are many ways to obtain a place ID. You can use:
//!
//! * Text Search (New) or Nearby Search (New)
//! * Geocoding API
//! * Routes API
//! * Address Validation API
//! * Autocomplete (New)
//!
//! # [Migrate to Place Details (New)](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-details)
//!
//! > **European Economic Area (EEA) developers** If your billing address is in the European
//! > Economic Area, effective on 8 July 2025, the
//! > [Google Maps Platform EEA Terms of Service](https://cloud.google.com/terms/maps-platform/eea)
//! > will apply to your use of the Services. Functionality varies by region.
//! > [Learn more](https://developers.google.com/maps/comms/eea/faq).
//!
//! ## [Introduction](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-details#introduction)
//!
//! Places API supports
//! [Place Details (Legacy)](https://developers.google.com/maps/documentation/places/web-service/details).
//! If you are familiar with Places API (Legacy),
//! [Place Details (New)](https://developers.google.com/maps/documentation/places/web-service/place-details)
//! makes the following changes:
//!
//! - **Field masking is required.** You must specify which fields you want returned in the
//!   response. There is no default list of returned fields. If you omit this list, the methods
//!   return an error. For more information, see
//!   [FieldMask](https://developers.google.com/maps/documentation/places/web-service/place-details#fieldmask).
//!
//! - **Place Details (New) supports both
//!   [API keys](https://developers.google.com/maps/documentation/places/web-service/get-api-key) and
//!   [OAuth](https://developers.google.com/maps/documentation/places/web-service/oauth-token)
//!   tokens as the authentication mechanism.**
//!
//! - **Only JSON is supported as a response format in Place Details (New).**
//!
//! - **The JSON response format for Places API (New) has changed** from the format of the legacy
//!   APIs. For more details, see
//!   [Migrate the Places API response](https://developers.google.com/maps/documentation/places/web-service/migrate-response).
//!
//! ## Parameter Changes
//!
//! The following table lists parameters in Place Details (Legacy) that have been renamed or modified for Place Details (New), or parameters that are no longer supported.
//!
//! | Current parameter | New parameter     | Notes |
//! |-------------------|-------------------|-------|
//! | `place_id`        | `places/PLACE_ID` | The string `places/PLACE_ID` is also called the place resource name. In the response from a Place Details (New), Nearby Search (New), and Text Search (New) request, this string is contained in the `name` field of the response. The standalone place ID is contained in the `id` field of the response. |
//! | `language`        | `languageCode`    | |
//! | `region`          | `regionCode`      | |
//!
//! ## [Example Request](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-details#example_request)
//!
//! The following example GET request uses the Place Details (Legacy). In this example, you request a JSON response containing information about a place ID and pass all parameters as URL parameters. This example also uses `fields` so the response only includes the display name and formatted address of the place:
//!
//! ```curl
//! curl -L -X GET \
//! 'https://maps.googleapis.com/maps/api/place/details/json?place_id=ChIJj61dQgK6j4AR4GeTYWZsKWw&fields=name%2Cformatted_address&key=API_KEY'
//! ```
//!
//! With Place Details (New), you make a GET request and pass all parameters in URL parameters and headers as part of the request. This example also uses a field mask so the response only includes the display name and formatted address of the place:
//!
//! ```curl
//! curl -X GET -H 'Content-Type: application/json' \
//! -H "X-Goog-Api-Key: API_KEY" \
//! -H "X-Goog-FieldMask: displayName,formattedAddress" \
//! https://places.googleapis.com/v1/places/ChIJj61dQgK6j4AR4GeTYWZsKWw
//! ```
//!
//! > **Note:** The string `places/PLACE_ID` is also called the place resource name. In the response
//! > from a Place Details (New), Nearby Search (New), and Text Search (New) request, this string
//! > is contained in the `name` field of the response. The standalone place ID is contained in the
//! > `id` field of the response.

#[cfg(feature = "reqwest")]
mod request;
#[cfg(feature = "reqwest")]
pub use crate::places_new::place_details::request::Request;

mod response;
pub use crate::places_new::place_details::response::Response;