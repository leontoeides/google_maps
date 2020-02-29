//! # google_maps
//!
//! 🗺 An unofficial Google Maps Platform client library for the Rust
//! programming language. This client currently implements the Directions API,
//! Distance Matrix API, Elevation API, Geocoding API, and Time Zone API.
//!
//! ![alt text](https://camo.githubusercontent.com/5b3f625c9385e359f735cac1f23b5cd35332cd0a/68747470733a2f2f7777772e61726b697465712e63612f6372617465732f676f6f676c655f6d6170732f62616e6e65722e6a7067 "Unofficial Google Maps Platform Client for Rust")
//!
//! # Welcome
//!
//! This crate is expected to work well and have the more important Google Maps
//! features implemented. It should work well because Reqwest and Serde do most
//! of the heavy lifting! While it's an early release, this crate should work
//! fine as is for most people.
//!
//! I created this client library because I needed several Google Maps Platform
//! features for a project that I'm working on. So, I've decided to spin my
//! library off into a public crate. This is a very small token of gratitude and
//! an attempt to give back to the Rust community. I hope it saves someone out
//! there some work.
//!
//! ## Example Directions API Request
//!
//! ```rust
//! use google_maps::*;
//! let mut my_settings = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);
//!
//! // Example request:
//!
//! let directions = DirectionsRequest::new(
//!     &mut my_settings,
//!     // Origin: Canadian Museum of Nature
//!     Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
//!     // Destination: Canada Science and Technology Museum
//!     Location::LatLng(LatLng::try_from(45.403_509, -75.618_904).unwrap()),
//! )
//! .with_travel_mode(TravelMode::Transit)
//! .with_arrival_time(
//!     // Ensure this date is a weekday in the future or this query will return
//!     // zero results.
//!     NaiveDate::from_ymd(2020, 2, 25).and_hms(13, 00, 0)
//! )
//! .execute().unwrap();
//!
//! // Dump entire response:
//!
//! println!("{:#?}", directions);
//! ```
//!
//! ## Example Distance Matrix API Request
//!
//! ```rust
//! use google_maps::*;
//! let mut my_settings = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);
//!
//! // Example request:
//!
//! let distance_matrix = DistanceMatrixRequest::new(
//!     &mut my_settings,
//!     // Origins
//!     vec![
//!         // Microsoft
//!         Waypoint::Address(String::from("One Microsoft Way, Redmond, WA 98052, United States")),
//!         // Cloudflare
//!         Waypoint::Address(String::from("101 Townsend St, San Francisco, CA 94107, United States")),
//!     ],
//!     // Destinations
//!     vec![
//!         // Google
//!         Waypoint::PlaceId(String::from("ChIJj61dQgK6j4AR4GeTYWZsKWw")),
//!         // Mozilla
//!         Waypoint::LatLng(LatLng::try_from(37.387_316, -122.060_008).unwrap()),
//!     ],
//! )
//! .execute().unwrap();
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
//! let mut my_settings = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);
//!
//! // Example request:
//!
//! let elevation = ElevationRequest::new(&mut my_settings)
//!     // Denver, Colorado, the "Mile High City"
//!     .for_positional_request(LatLng::try_from(39.739_154, -104.984_703).unwrap())
//!     .execute().unwrap();
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
//! let mut my_settings = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);
//!
//! // Example request:
//!
//! let location = GeocodingRequest::new(&mut my_settings)
//!     .with_address("10 Downing Street London")
//!     .execute().unwrap();
//!
//! // Dump entire response:
//!
//! println!("{:#?}", location);
//!
//! // Parsing example:
//!
//! for result in &location.results {
//!     println!("{}", result.geometry.location)
//! }
//! ```
//!
//! ## Example Reverse Geocoding API Request
//!
//! ```rust
//! use google_maps::*;
//! let mut my_settings = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);
//!
//! // Example request:
//!
//! let location = GeocodingReverseRequest::new(
//!     &mut my_settings,
//!     // 10 Downing St, Westminster, London
//!     LatLng::try_from(51.503_364, -0.127_625).unwrap(),
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
//! let mut my_settings = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);
//!
// Example request:
//!
//! let time_zone = TimeZoneRequest::new(
//!      &mut my_settings,
//!      // St. Vitus Cathedral in Prague, Czechia
//!      LatLng::try_from(50.090_903, 14.400_512).unwrap(),
//!      // Tuesday February 23, 2020 @ 6:00:00 pm
//!      NaiveDate::from_ymd(2020, 2, 23).and_hms(18, 00, 0)
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
//! println!("Time in {:#?}: {}",
//!     time_zone.time_zone_id.unwrap(),
//!     unix_timestamp as i64 + time_zone.dst_offset.unwrap() as i64 +
//!         time_zone.raw_offset.unwrap() as i64
//! );
//! ```
//!
//! ## [Geolocation API](https://developers.google.com/maps/documentation/geolocation/intro)
//!
//! Google's Geolocation API seems to be offline. While the online documentation
//! is still available and the API appears configurable through the Google Cloud
//! Platform console, the Geolocation API responds Status code `404 Not Found`
//! with an empty body to all requests. This API cannot be implemented until the
//! server responds as expected.
//!
//! # Feedback
//!
//! I would like for you to be successful with your project! If this crate is
//! not working for you, doesn't work how you think it should, or if you have
//! requests, or suggestions - please [report them to
//! me](https://github.com/leontoeides/google_maps/issues)! I'm not always fast
//! at responding but I will respond. Thanks!
//!
//! # To do:
//!
//! 1. Track both _requests_ and request _elements_ for rate limiting.
//! 2. Make a generic get() function for that can be used by all APIs.
//! 3. Look into making APIs optional, i.e. features.
//! 4. Look into the Prelude::* convention.
//! 5. Look into integrating [yaiouom](https://crates.io/crates/yaiouom).
//! 6. Convert explicit query validation to session types wherever reasonable.
//! 7. [Places API](https://developers.google.com/places/web-service/intro).
//! There are no immediate plans for supporting this API. It's quite big and I
//! have no current need for it. If you would like to have to implemented,
//! please contact me.
//! 8. [Roads API](https://developers.google.com/maps/documentation/roads/intro).
//! There are no immediate plans for supporting this API. It's quite big and I
//! have no current need for it. If you would like to have to implemented,
//! please contact me.

#![doc(html_favicon_url = "https://www.arkiteq.ca/crates/google_maps/icon.png")]
#![doc(html_logo_url = "https://www.arkiteq.ca/crates/google_maps/logo.png")]

mod bounds;
mod client_settings;
mod directions;
mod distance_matrix;
mod elevation;
mod error;
mod geocoding;
mod language;
mod latlng;
mod place_type;
mod region;
mod request_rate;
mod serde;
mod time_zone;

pub use chrono::{NaiveDate, NaiveDateTime};
pub use chrono_tz::Tz;

pub use crate::{
    bounds::Bounds as Bounds,
    client_settings::ClientSettings as ClientSettings,
    language::Language as Language,
    latlng::LatLng as LatLng,
    place_type::PlaceType as PlaceType,
    region::Region as Region,
    request_rate::api::Api as Api,
}; // use

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
        directions_distance::DirectionsDistance,
        directions_duration::DirectionsDuration,
        driving_maneuver::DrivingManeuver as DrivingManeuver,
        leg::Leg as Leg,
        overview_polyline::OverviewPolyline as OverviewPolyline,
        polyline::Polyline as Polyline,
        Response as DirectionsResponse,
        route::Route as Route,
        status::Status as DirectionsStatus,
        step::Step as Step,
        transit_agency::TransitAgency as TransitAgency,
        transit_currency::TransitCurrency as TransitCurrency,
        transit_details::TransitDetails as TransitDetails,
        transit_fare::TransitFare as TransitFare,
        transit_line::TransitLine as TransitLine,
        transit_stop::TransitStop as TransitStop,
        transit_time::TransitTime as TransitTime,
        transit_vehicle::TransitVehicle as TransitVehicle,
    }, // response
    travel_mode::TravelMode as TravelMode,
    vehicle_type::VehicleType as VehicleType,
}; // use

pub use crate::distance_matrix::{
    request::Request as DistanceMatrixRequest,
    response::Response as DistanceMatrixResponse,
    response::status::Status as DistanceMatrixStatus,
}; // use

pub use crate::elevation::{
    error::Error as ElevationError,
    request::{
        locations::Locations as ElevationLocations,
        Request as ElevationRequest,
    }, // request
    response::{
        point::Point as Point,
        Response as ElevationResponse,
        status::Status as ElevationStatus,
    }, // response
}; // use

pub use crate::geocoding::{
    error::Error as GeocodingError,
    forward::{
        component::Component as GeocodingComponent,
        ForwardRequest as GeocodingRequest,
    }, // forward
    location_type::LocationType as LocationType,
    response::{
        address_component::AddressComponent,
        geocoding::Geocoding as Geocoding,
        geometry::Geometry,
        plus_code::PlusCode,
        Response as GeocodingResponse,
        status::Status as GeocodingStatus,
    }, // response
    reverse::ReverseRequest as GeocodingReverseRequest,
}; // use

pub use crate::time_zone::{
    error::Error as TimeZoneError,
    request::Request as TimeZoneRequest,
    response::{
        Response as TimeZoneResponse,
        status::Status as TimeZoneStatus,
    }, // reponse
}; // use