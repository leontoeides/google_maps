//! # google_maps
//!
//! An unofficial Google Maps Platform client for the Rust programming language.
//! This client currently implements the Directions API, Distance Matrix API,
//! Elevation API, Geocoding API, and Time Zone API.
//!
//! # Welcome
//!
//! This crate is expected to work well and have the more important Google Maps
//! features implemented. It should work well because Reqwest and Serde do most
//! of the heavy lifting! While it's an early release, this crate should work
//! fine as is for most people.
//!
//! I created this library because I needed several Google Maps Platform
//! features for a project that I'm working on. So, I've decided to spin my
//! library off into a public crate. This is a very small token of gratitude and
//! an attempt to give back to the Rust community. I hope it saves someone out
//! there some work.
//!
//! ## Example Directions API Request
//!
//! ```rust
//! use google_maps::*;
//!
//! let directions = DirectionsRequest::new(
//!     YOUR_GOOGLE_API_KEY_HERE,
//!     // Canadian Museum of Nature
//!     Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
//!     // Canada Science and Technology Museum
//!     Location::LatLng { lat: 45.403509, lng: -75.618904 },
//! )
//! .with_travel_mode(TravelMode::Transit)
//! .with_arrival_time(PrimitiveDateTime::new(
//!     Date::try_from_ymd(2020, 1, 20).unwrap(),
//!     Time::try_from_hms(13, 00, 0).unwrap()
//! ))
//! .execute().unwrap();
//!
//! println!("{:#?}", directions);
//! ```
//!
//! ## Example Distance Matrix API Request
//!
//! ```rust
//! use google_maps::*;
//!
//! // Example request:
//!
//! let distance_matrix = DistanceMatrixRequest::new(
//!     YOUR_GOOGLE_API_KEY_HERE,
//!     // Origins
//!     vec![
//!         // Microsoft
//!         Waypoint::Address(String::from("One Microsoft Way, Redmond, WA 98052, United States")),
//!         // Apple
//!         Waypoint::Address(String::from("Infinite Loop, Cupertino, CA 95014, United States")),
//!     ],
//!     // Destinations
//!     vec![
//!         // Google
//!         Waypoint::PlaceId(String::from("ChIJj61dQgK6j4AR4GeTYWZsKWw")),
//!         // Mozilla
//!         Waypoint::LatLng { lat: 37.387316, lng: -122.060008 },
//!     ],
//! ).execute().unwrap();
//!
//! // Dump entire response:
//!
//! println!("{:#?}", distance_matrix);
//! ```
//!
//! ## Example Elevation API Positional Request
//!
//! ```rust
//! use google_maps::*;
//!
//! // Example request:
//!
//! let elevation = ElevationRequest::new(YOUR_GOOGLE_API_KEY_HERE)
//! .positional_request(ElevationLocations::LatLngs(vec![
//!     // Denver, Colorado, the "Mile High City"
//!     LatLng { lat: 39.7391536, lng: -104.9847034 },
//! ]))
//! .execute().unwrap();
//!
//! // Dump entire response:
//!
//! println!("{:#?}", elevation);
//!
//! // Parsing example:
//!
//! println!("Elevation: {} meters", elevation.results.unwrap()[0].elevation);
//! ```
//!
//! ## Example Geocoding API Request
//!
//! ```rust
//! use google_maps::*;
//!
//! // Example request:
//!
//! let location = GeocodingRequest::new(YOUR_GOOGLE_API_KEY_HERE)
//! .with_address("10 Downing Street London")
//! .execute().unwrap();
//!
//! // Dump entire response:
//!
//! println!("{:#?}", location);
//!
//! // Parsing example:
//!
//! for result in &location.results {
//!     println!(
//!         "Latitude: {:.7}, Longitude: {:.7}",
//!         result.geometry.location.lat, result.geometry.location.lng
//!     );
//! }
//! ```
//!
//! ## Example Reverse Geocoding API Request
//!
//! ```rust
//! use google_maps::*;
//!
//! // Example request:
//!
//! let location = GeocodingReverseRequest::new(
//!     YOUR_GOOGLE_API_KEY_HERE,
//!     // 10 Downing St, Westminster, London
//!     LatLng { lat: 51.5033635, lng: -0.1276248 }
//! )
//! .with_result_type(PlaceType::StreetAddress)
//! .execute().unwrap();
//!
//! // Dump entire response:
//!
//! println!("{:#?}", location);
//!
//! // Parsing example:
//!
//! for result in &location.results {
//!     for address_component in &result.address_components {
//!         print!("{} ", address_component.short_name);
//!     }
//!     println!(""); // New line.
//! }
//! ```
//!
//! ## Example Time Zone API Request
//!
//! ```rust
//! use google_maps::*;
//!
//! // Example request:
//!
//! let time_zone = TimeZoneRequest::new(
//!     YOUR_GOOGLE_API_KEY_HERE,
//!     // St. Vitus Cathedral in Prague, Czechia
//!     LatLng { lat: 50.090903, lng: 14.400512 },
//!     PrimitiveDateTime::new(
//!         // Tuesday February 15, 2022
//!         Date::try_from_ymd(2022, 2, 15).unwrap(),
//!         // 6:00:00 pm
//!         Time::try_from_hms(18, 00, 0).unwrap(),
//!     ),
//! ).execute().unwrap();
//!
//! // Dump entire response:
//!
//! println!("{:#?}", time_zone);
//!
//! // Parsing example:
//!
//! use std::time::{SystemTime, UNIX_EPOCH};
//!
//! let unix_timestamp =
//!     SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
//!
//! println!("Time at your computer: {}", unix_timestamp);
//!
//! println!("Time in {}: {}",
//!     time_zone.time_zone_id.unwrap(),
//!     unix_timestamp as i64 + time_zone.dst_offset.unwrap() as i64 +
//!         time_zone.raw_offset.unwrap() as i64
//! );
//! ```
//!
//! To do:
//! 1. [Geolocation API](https://developers.google.com/maps/documentation/geolocation/intro)
//! 2. [Places API](https://developers.google.com/places/web-service/intro)
//! 3. [Roads API](https://developers.google.com/maps/documentation/roads/intro)
//! 4. Retry on Failure
//! 5. Automatic Rate Limiting

mod error;
pub mod bounds;
pub mod directions;
pub mod distance_matrix;
pub mod elevation;
pub mod geocoding;
pub mod language;
pub mod latlng;
pub mod place_type;
pub mod region;
pub mod time_zone;

pub extern crate time;
pub use crate::bounds::Bounds as Bounds;
pub use crate::language::Language as Language;
pub use crate::latlng::LatLng as LatLng;
pub use crate::place_type::PlaceType as PlaceType;
pub use crate::region::Region as Region;
pub use time::{Date, PrimitiveDateTime, Time};

pub use crate::directions::{
    request::{
        avoid::Avoid as Avoid,
        departure_time::DepartureTime as DepartureTime,
        location::Location as Location,
        Request as DirectionsRequest,
        traffic_model::TrafficModel as TrafficModel,
        transit_mode::TransitMode as TransitMode,
        transit_route_preference::TransitRoutePreference as TransitRoutePreference,
        unit_system::UnitSystem as UnitSystem,
        waypoint::Waypoint as Waypoint,
    }, // request
    response::{
        Response as Directions,
        status::Status as DirectionsStatus,
    }, // response
    travel_mode::TravelMode as TravelMode,
}; // use

pub use crate::distance_matrix::{
    request::Request as DistanceMatrixRequest,
    response::Response as DistanceMatrix,
    response::status::Status as DistanceMatrixStatus,
}; // use

pub use crate::elevation::{
    error::Error as ElevationError,
    request::{
        locations::Locations as ElevationLocations,
        Request as ElevationRequest,
    }, // request
    response::{
        Response as Elevation,
        status::Status as ElevationStatus,
    }, // response
}; // use

pub use crate::geocoding::{
    error::Error as GeocodingError,
    forward::{
        component::Component as GeocodingComponent,
        ForwardRequest as GeocodingForwardRequest,
        ForwardRequest as GeocodingRequest,
    }, // forward
    location_type::LocationType as LocationType,
    response::{
        Response as Geocoding,
        status::Status as GeocodingStatus,
    }, // response
    reverse::ReverseRequest as GeocodingReverseRequest,
}; // use

pub use crate::time_zone::{
    error::Error as TimeZoneError,
    request::Request as TimeZoneRequest,
    response::{
        Response as TimeZone,
        status::Status as TimeZoneStatus,
    }, // reponse
}; // use