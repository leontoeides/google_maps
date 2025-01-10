//! # `google_maps`
//! ![Crates.io Version](https://img.shields.io/crates/v/google_maps)
//! ![Crates.io MSRV](https://img.shields.io/crates/msrv/google_maps)
//! ![Crates.io License](https://img.shields.io/crates/l/google_maps)
//! ![Crates.io Total Downloads](https://img.shields.io/crates/d/google_maps)
//!
//! An unofficial Google Maps Platform client library for the Rust programming
//! language.
//!
//! This client currently implements the Directions API, Distance Matrix API,
//! Elevation API, Geocoding API, Time Zone API, and parts of the Places and
//! Roads API.
//!
//! <img src="https://www.arkiteq.io/crates/google_maps/banner.jpg" alt="Unofficial Google Maps Platform Client for Rust" width="400"/>
//!
//! # Installation
//!
//! Configure the dependencies:
//!
//! ```toml
//! [dependencies]
//! google_maps = "3.8"
//! ```
//!
//! Optionally, add `rust_decimal = "1"` and `rust_decimal_macros = "1"` for
//! access to the `dec!` macro. This macro can be used to define decimal numbers
//! in your program.
//!
//! This is useful for hard-coding latitudes and longitudes into your code for
//! testing.
//!
//! ## Feature Flags
//!
//! The desired Google Maps APIs can be enabled individually via feature flags.
//!
//! Additionally, usage of rustls for Reqwest is supported.
//!
//! ### Google Maps Client Feature Flags:
//!
//! * `autocomplete` ‧ includes Google Maps Places autocomplete API
//! * `directions` ‧ includes Google Maps Directions API
//! * `distance_matrix` ‧ includes Google Maps Distance Matrix API
//! * `elevation` ‧ includes Google Maps Elevation API
//! * `geocoding` ‧ includes Google Maps Geocoding API
//! * `places` ‧ includes Google Maps Places API
//! * `roads` ‧ includes Google Maps Roads API
//! * `time_zone` ‧ includes Google Maps Time Zone API
//! * `reqwest` ‧ uses [reqwest](https://crates.io/crates/reqwest) for querying
//!   the Google Maps API
//! * `reqwest-middleware` ‧ uses [reqwest-middleware](https://crates.io/crates/reqwest-middleware)
//!   for querying the Google Maps API
//! * `geo` ‧ support for the rust [geo](https://crates.io/crates/geo) ecosystem
//! * `polyline` ‧ allows easy type conversions from a `Route` or `Step` to a geo
//!   [LineString](https://docs.rs/geo-types/0.7.13/geo_types/geometry/struct.LineString.html)
//!
//! Note: the `autocomplete` feature covers the Places API autocomplete-related
//! services:
//! [Place Autocomplete requests](https://docs.rs/google_maps/latest/google_maps/prelude/struct.ClientSettings.html#method.place_autocomplete)
//! and [Query Autocomplete requests](https://docs.rs/google_maps/latest/google_maps/prelude/struct.ClientSettings.html#method.query_autocomplete).
//! All other Places API services are covered by the `places` feature.
//!
//! ### Reqwest Feature Flags
//!
//! For use with `reqwest` only.
//!
//! * `reqwest-native-tls` ‧ Enables TLS functionality provided by `native-tls`.
//! * `reqwest-rustls` ‧ Enables TLS functionality provided by `rustls`.
//!
//! ### Default Feature Flags
//!
//! By default, the Google Maps client includes all implemented Google Maps
//! APIs. Reqwest will secure the connection using the system-native TLS
//! (`native-tls`), and has gzip compression enabled (`gzip`).
//!
//! ```toml
//! default = [
//!     # google_maps default features:
//!     "directions",
//!     "distance_matrix",
//!     "elevation",
//!     "geocoding",
//!     "time_zone",
//!     # `autocomplete` covers the Places API autocomplete-related services.
//!     # All other Places API services are under `places` feature.
//!     "autocomplete",
//!     "roads",
//!     "places",
//!
//!     # reqwest default features:
//!     "reqwest",
//!     "reqwest-default-tls",
//!     "reqwest-http2",
//!     "reqwest-brotli",
//!
//!     # rust_decimal default features:
//!     "decimal-serde",
//!
//!     # simd-json default features:
//!     "simd-json-beef",
//!     "simd-json-known-key",
//! ]
//! ```
//!
//! #### Feature flag usage example
//!
//! This example will only include the Google Maps Directions API. Reqwest will
//! secure the connection using the Rustls library, and has brotli compression
//! enabled.
//!
//! ```toml
//! google_maps = {
//!     version = "3.8",
//!     default-features = false,
//!     features = [
//!         "directions",
//!         "reqwest",
//!         "reqwest-rustls",
//!         "reqwest-brotli"
//!     ]
//! }
//! ```
//!
//! # Release Notes
//!
//! The [full changelog is available
//! here](https://github.com/leontoeides/google_maps/blob/master/CHANGELOG.md).
//!
//! Releases [are available on
//! GitHub](https://github.com/leontoeides/google_maps/releases).
//!
//! # Examples
//!
//! ## Directions API
//!
//! The Directions API is a service that calculates directions between
//! locations. You can search for directions for several modes of
//! transportation, including transit, driving, walking, or cycling.
//!
//! ```rust
//! use google_maps::prelude::*;
//!
//! let google_maps_client = Client::try_new("YOUR_GOOGLE_API_KEY_HERE");
//!
//! // Example request:
//!
//! let directions = google_maps_client.directions(
//!     // Origin: Canadian Museum of Nature
//!     Location::from_address("240 McLeod St, Ottawa, ON K2P 2R1"),
//!     // Destination: Canada Science and Technology Museum
//!     Location::try_from_f32(45.403_509, -75.618_904)?,
//! )
//! .with_travel_mode(TravelMode::Driving)
//! .execute()
//! .await?;
//!
//! // Dump entire response:
//!
//! println!("{:#?}", directions);
//! ```
//!
//! ## Distance Matrix API
//!
//! The Distance Matrix API is a service that provides travel distance and time
//! for a matrix of origins and destinations, based on the recommended route
//! between start and end points.
//!
//! ```rust
//! use google_maps::prelude::*;
//!
//! let google_maps_client = Client::try_new("YOUR_GOOGLE_API_KEY_HERE");
//!
//! // Example request:
//!
//! let distance_matrix = google_maps_client.distance_matrix(
//!     // Origins
//!     vec![
//!         // Microsoft
//!         Waypoint::from_address("One Microsoft Way, Redmond, WA 98052, United States"),
//!         // Cloudflare
//!         Waypoint::from_address("101 Townsend St, San Francisco, CA 94107, United States"),
//!     ],
//!     // Destinations
//!     vec![
//!         // Google
//!         Waypoint::from_place_id("ChIJj61dQgK6j4AR4GeTYWZsKWw"),
//!         // Mozilla
//!         Waypoint::try_from_f32(37.387_316, -122.060_008)?,
//!     ],
//! ).execute().await?;
//!
//! // Dump entire response:
//!
//! println!("{:#?}", distance_matrix);
//! ```
//!
//! ## Elevation API (Positional)
//!
//! The Elevation API provides elevation data for all locations on the surface
//! of the earth, including depth locations on the ocean floor (which return
//! negative values).
//!
//! ```rust
//! use google_maps::prelude::*;
//!
//! let google_maps_client = Client::try_new("YOUR_GOOGLE_API_KEY_HERE");
//!
//! // Example request:
//!
//! let elevation = google_maps_client.elevation()
//!     // Denver, Colorado, the "Mile High City"
//!     .for_positional_request(LatLng::try_from_dec(dec!(39.739_154), dec!(-104.984_703))?)
//!     .execute()
//!     .await?;
//!
//! // Dump entire response:
//!
//! println!("{:#?}", elevation);
//!
//! // Display all results:
//!
//! if let Some(results) = &elevation.results {
//!     for result in results {
//!         println!("Elevation: {} meters", result.elevation)
//!     }
//! }
//! ```
//!
//! ## Geocoding API
//!
//! The Geocoding API is a service that provides geocoding and reverse geocoding
//! of addresses. Geocoding is the process of converting addresses (like a
//! street address) into geographic coordinates (like latitude and longitude),
//! which you can use to place markers on a map, or position the map.
//!
//! ```rust
//! use google_maps::prelude::*;
//!
//! let google_maps_client = Client::try_new("YOUR_GOOGLE_API_KEY_HERE");
//!
//! // Example request:
//!
//! let location = google_maps_client.geocoding()
//!     .with_address("10 Downing Street London")
//!     .execute()
//! .await?;
//!
//! // Dump entire response:
//!
//! println!("{:#?}", location);
//!
//! // Print latitude & longitude coordinates:
//!
//! for result in location.results {
//!     println!("{}", result.geometry.location)
//! }
//! ```
//!
//! ## Reverse Geocoding API
//!
//! The Geocoding API is a service that provides geocoding and reverse geocoding
//! of addresses. Reverse geocoding is the process of converting geographic
//! coordinates into a human-readable address.
//!
//! ```rust
//! use google_maps::prelude::*;
//!
//! let google_maps_client = Client::try_new("YOUR_GOOGLE_API_KEY_HERE");
//!
//! // Example request:
//!
//! let location = google_maps_client.reverse_geocoding(
//!     // 10 Downing St, Westminster, London
//!     LatLng::try_from_dec(dec!(51.503_364), dec!(-0.127_625))?,
//! )
//! .with_result_type(PlaceType::StreetAddress)
//! .execute()
//! .await?;
//!
//! // Dump entire response:
//!
//! println!("{:#?}", location);
//!
//! // Display all results:
//!
//! for result in location.results {
//!     println!(
//!         "{}",
//!         result.address_components.iter()
//!             .map(|address_component| address_component.short_name.to_string())
//!             .collect::<Vec<String>>()
//!             .join(", ")
//!     );
//! }
//! ```
//!
//! ## Time Zone API
//!
//! The Time Zone API provides time offset data for locations on the surface of
//! the earth. You request the time zone information for a specific
//! latitude/longitude pair and date. The API returns the name of that time
//! zone, the time offset from UTC, and the daylight savings offset.
//!
//! ```rust
//! use google_maps::prelude::*;
//!
//! let google_maps_client = Client::try_new("YOUR_GOOGLE_API_KEY_HERE");
//!
//! // Example request:
//!
//! let time_zone = google_maps_client.time_zone(
//!     // St. Vitus Cathedral in Prague, Czechia
//!     LatLng::try_from_dec(dec!(50.090_903), dec!(14.400_512))?,
//!     // The time right now in UTC (Coordinated Universal Time)
//!     Utc::now()
//! ).execute().await?;
//!
//! // Dump entire response:
//!
//! println!("{:#?}", time_zone);
//!
//! // Usage example:
//!
//! println!("Time at your computer: {}", Local::now().to_rfc2822());
//!
//! if let Some(time_zone_id) = time_zone.time_zone_id {
//!     println!(
//!         "Time in {}: {}",
//!         time_zone_id.name(),
//!         Utc::now().with_timezone(&time_zone_id).to_rfc2822()
//!     );
//! }
//! ```
//!
//! ### [Geolocation API](https://developers.google.com/maps/documentation/geolocation/intro)
//!
//! Google's Geolocation API seems to be offline. While the online documentation
//! is still available and the API appears configurable through the Google Cloud
//! Platform console, the Geolocation API responds Status code `404 Not Found`
//! with an empty body to all requests. This API cannot be implemented until the
//! server responds as expected.
//!
//! ### Controlling Request Settings
//!
//! The Google Maps client settings can be used to change the request rate and
//! automatic retry parameters.
//!
//! ```rust
//! use google_maps::prelude::*;
//!
//! let google_maps_client = Client::try_new("YOUR_GOOGLE_API_KEY_HERE")
//!     // For all Google Maps Platform APIs, the client will limit 2 sucessful
//!     // requests for every 10 seconds:
//!     .with_rate(&Api::All, 2, std::time::Duration::from_secs(10))
//!     // Returns the `GoogleMapsClient` struct to the caller. This struct is used
//!     // to make Google Maps Platform requests.
//!     .build();
//! ```
//!
//! # Feedback
//!
//! I would like for you to be successful with your project! If this crate is
//! not working for you, doesn't work how you think it should, or if you have
//! requests, or suggestions - please [report them to
//! me](https://github.com/leontoeides/google_maps/issues)! I'm not always fast
//! at responding but I will respond. Thanks!
//!
//! # Roadmap
//!
//! - [ ] Track both _requests_ and request _elements_ for rate limiting.
//! - [ ] Convert explicit query validation to session types wherever
//!     reasonable.
//! - [ ] [Places API](https://developers.google.com/places/web-service/intro).
//!     Only partly implemented. If you would like to have any missing pieces
//!     implemented, please contact me.
//! - [ ] [Roads API](https://developers.google.com/maps/documentation/roads/intro).
//!     Only partly implemented. If you would like to have any missing pieces
//!     implemented, please contact me.
//!
//! # Author's Note
//!
//! This crate is expected to work well and have the more important Google Maps
//! features implemented. It should work well because
//! [serde](https://crates.io/crates/serde), [simd-json](https://crates.io/crates/simd-json)
//! and, by default, [reqwest](https://crates.io/crates/reqwest) do most of the
//! heavy lifting!
//!
//! I created this client library because I needed several Google Maps Platform
//! features for a project that I'm working on. So, I've decided to spin my
//! library off into a public crate. This is a very small token of gratitude and
//! an attempt to give back to the Rust community. I hope it saves someone out
//! there some work.

#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::cast_precision_loss,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions,
    clippy::negative_feature_names, // suppress clippy warning re: `simd_json` crate
    clippy::too_long_first_doc_paragraph, // bah!
    clippy::too_many_lines
)]
#![doc(html_favicon_url = "https://www.arkiteq.io/crates/google_maps/icon.png")]
#![doc(html_logo_url = "https://www.arkiteq.io/crates/google_maps/logo.png")]

// Common / global modules:

mod client;
mod serde;

pub mod error;
pub mod prelude;
pub mod types;

pub mod traits;

// Optional Google Maps API modules. Their inclusion can be changed with
// feature flags:

#[cfg(any(feature = "directions", feature = "distance_matrix"))]
pub mod directions;

#[cfg(feature = "distance_matrix")]
pub mod distance_matrix;

#[cfg(feature = "elevation")]
pub mod elevation;

#[cfg(feature = "geocoding")]
pub mod geocoding;

#[cfg(any(feature = "autocomplete", feature = "places"))]
pub mod places;

#[cfg(feature = "reqwest-middleware")]
pub mod reqwest_maybe_middleware;

#[cfg(feature = "roads")]
pub mod roads;

#[cfg(feature = "time_zone")]
pub mod time_zone;

#[cfg(feature = "address_validation")]
pub mod address_validation;

// -----------------------------------------------------------------------------
//
// Re-exports for the main event

pub use crate::{
    client::Client,
    error::Error,
    types::error::Error as TypeError,
};

#[deprecated(note = "use `google_maps::Client` instead", since = "3.8.0")]
pub use crate::client::Client as ClientSettings;

#[deprecated(note = "use `google_maps::Client` instead", since = "3.8.0")]
pub use crate::client::Client as GoogleMapsClient;

#[deprecated(note = "use `google_maps::Error` instead", since = "3.8.0")]
pub use crate::error::Error as GoogleMapsError;

// -----------------------------------------------------------------------------
//
// Re-exports for common shared types

#[cfg(any(feature = "geocoding", feature = "places"))]
pub use crate::types::address_component::AddressComponent;

#[cfg(any(
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
pub use crate::types::bounds::Bounds;

pub use crate::types::classified_error::ClassifiedError;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "geocoding"
))]
pub use crate::types::country::Country;

#[cfg(any(feature = "geocoding", feature = "places"))]
pub use crate::types::geometry::Geometry;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places",
    feature = "time_zone"
))]
pub use crate::types::language::Language;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "elevation",
    feature = "geocoding",
    feature = "places",
    feature = "roads",
    feature = "time_zone"
))]
pub use crate::types::latlng::LatLng;

#[cfg(any(feature = "geocoding", feature = "places"))]
pub use crate::types::location_type::LocationType;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
pub use crate::types::place_type::PlaceType;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
pub use crate::types::region::Region;

// -----------------------------------------------------------------------------
//
// Re-exports for optional dependencies

#[cfg(feature = "reqwest")]
mod request_rate;

#[cfg(feature = "reqwest")]
pub use crate::request_rate::api::Api;

#[cfg(all(feature = "reqwest", feature = "reqwest-middleware"))]
type ReqError = reqwest_maybe_middleware::Error;

#[cfg(all(feature = "reqwest", not(feature = "reqwest-middleware")))]
type ReqError = reqwest::Error;