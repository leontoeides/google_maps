//! All requests to Place Photos (New) must include a photo resource name, returned in the response
//! to a Nearby Search (New), Text Search (New), or Place Details (New) request.
//!
//! The response to these requests contains a photos[] array if the place has related photographic
//! content.
//!
//! # [Migrate to Place Photos (New)](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-photo)
//!
//! > **European Economic Area (EEA) developers** If your billing address is in the European
//! > Economic Area, effective on 8 July 2025, the
//! > [Google Maps Platform EEA Terms of Service](https://cloud.google.com/terms/maps-platform/eea)
//! > will apply to your use of the Services. Functionality varies by region.
//! > [Learn more](https://developers.google.com/maps/comms/eea/faq).
//!
//! ## [Introduction](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-photo#introduction)
//!
//! The Places API supports
//! [Place Photos (Legacy)](https://developers.google.com/maps/documentation/places/web-service/photos).
//! If you are familiar with Place Photos (Legacy),
//! [Place Photos (New)](https://developers.google.com/maps/documentation/places/web-service/place-photos)
//! makes the following changes:
//!
//! - **Place Photos (New) requires that you pass the resource name of the photo,** which includes
//!   the place ID in the request URL, instead of just the photo reference. For more information,
//!   see [Change to photo references](#change-to-photo-references).
//!
//! - **Place Photos (New) supports both
//!   [API keys](https://developers.google.com/maps/documentation/places/web-service/get-api-key) and
//!   [OAuth](https://developers.google.com/maps/documentation/places/web-service/oauth-token)
//!   tokens as the authentication mechanism.**
//!
//! ## Parameter Changes
//!
//! The following table lists parameters in Place Photos (Legacy) that have been renamed or modified
//! for Place Photos (New), or parameters that are no longer supported.
//!
//! | Current parameter | New parameter | Notes                                                         |
//! |-------------------|---------------|---------------------------------------------------------------|
//! | `maxheight`       | `maxHeightPx` | You must specify either `maxHeightPx`, `maxWidthPx`, or both. |
//! | `maxwidth`        | `maxWidthPx`  | You must specify either `maxHeightPx`, `maxWidthPx`, or both. |
//!
//! ## [Increased Photo Size](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-photo#increased_photo_size)
//!
//! Place Photos (Legacy) supported a maximum photo size of 1600 by 1600 pixels. Place Photos (New)
//! supports sizes up to **4800 by 4800 pixels**.
//!
//! ## [Change to Photo References](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-photo#photo-ref)
//!
//! [Place Photos (New)](https://developers.google.com/maps/documentation/places/web-service/place-photos)
//! API requires that you pass the resource name of the requested photo, which includes a place ID
//! and the photo reference, in the form:
//!
//! ```
//! https://places.googleapis.com/v1/places/PLACE_ID/photos/PHOTO_REFERENCE/media?maxWidthPx=400&key=API_KEY
//! ```
//!
//! [Nearby Search (New)](https://developers.google.com/maps/documentation/places/web-service/nearby-search),
//! [Text Search (New)](https://developers.google.com/maps/documentation/places/web-service/text-search), and
//! [Place Details (New)](https://developers.google.com/maps/documentation/places/web-service/place-details)
//! all return a `photos[]` array in the response that supports this format. Each element of
//! `photos[]` in the new response contains the following fields:
//!
//! - **`name`** — A string containing the resource name of the photo. This string is in the form:
//!   `places/PLACE_ID/photos/PHOTO_REFERENCE`
//!
//! - **`heightPx`** — The maximum height of the image, in pixels.
//!
//! - **`widthPx`** — The maximum width of the image, in pixels.
//!
//! - **`authorAttributions[]`** — Any required attributions. This field is always present, but
//!   might be empty.
//!
//! ### Legacy Format
//!
//! With the legacy API, you only have to pass the photo reference, in the form:
//!
//! ```
//! https://maps.googleapis.com/maps/api/place/photo?photo_reference=PHOTO_REFERENCE&maxwidth=400&key=API_KEY
//! ```
//!
//! [Find Place (Legacy)](https://developers.google.com/maps/documentation/places/web-service/search-find-place),
//! [Nearby Search (Legacy)](https://developers.google.com/maps/documentation/places/web-service/search-nearby),
//! [Text Search (Legacy)](https://developers.google.com/maps/documentation/places/web-service/search-text), and
//! [Place Details (Legacy)](https://developers.google.com/maps/documentation/places/web-service/details)
//! all return a `photos[]` array in the response that supports this format. Each photo element in
//! the legacy response contains the following fields:
//!
//! - **`photo_reference`** — A string used to identify the photo when you perform a Photo request.
//! - **`height`** — The maximum height of the image.
//! - **`width`** — The maximum width of the image.
//! - **`html_attributions[]`** — Any required attributions. This field is always present, but might be empty.

#[cfg(feature = "reqwest")]
mod request;
#[cfg(feature = "reqwest")]
pub use crate::places_new::place_photos::image::request::Request;

mod response;
pub use crate::places_new::place_photos::image::response::{
	Response,
	PhotoImage
};