//! # google_maps
//!
//! An unofficial Google Maps Platform client for the Rust programming language.
//! This client currently implements the Directions API, Elevation API,
//! Geocoding ApI, and Time Zone API.
//!
//! # Welcome
//!
//! As of version 0.1.0 this crate is expected to work well and have the more
//! important Google Maps features implemented. It should work well because
//! Reqwest and Serde do most of the heavy lifting! While it's an early release,
//! for most people this crate should work fine as is.
//!
//! I created this crate because I needed the Google Maps Platform for a project
//! that I'm working on. So, I've decided to spin my library off into a public
//! crate. This is a small token of gratitude and an attempt to give back to the
//! Rust community. I hope it saves someone out there some work.
//!
//! # Example Request
//!
//! ```
//! use google_maps::*;
//!
//! let directions = DirectionsRequest::new(
//!     // Canadian Museum of Nature
//!     Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
//!     // Canada Science and Technology Museum
//!     Location::Address(String::from("1867 St Laurent Blvd, Ottawa, ON K1G 5A3")),
//!     YOUR_GOOGLE_API_KEY_HERE
//! )
//! .with_travel_mode(TravelMode::Transit)
//! .with_arrival_time(PrimitiveDateTime::new(
//!     Date::try_from_ymd(2020, 1, 20).unwrap(),
//!     Time::try_from_hms(13, 00, 0).unwrap()
//! ))
//! .validate().unwrap()
//! .build()
//! .get().unwrap();
//!
//! println!("{:#?}", directions);
//! ```
//!
//! To do:
//! 1. [Distance Matrix API](https://developers.google.com/maps/documentation/distance-matrix/start)
//! 2. [Geolocation API](https://developers.google.com/maps/documentation/geolocation/intro)
//! 3. [Places API](https://developers.google.com/places/web-service/intro)
//! 4. [Roads API](https://developers.google.com/maps/documentation/roads/intro)
//! 5. Automatic Rate Limiting
//! 6. Retry on Failure

pub mod bounds;
pub mod directions;
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

pub use crate::elevation::{
    error::Error as ElevationError,
    request::{
        Locations as ElevationLocations,
        Request as ElevationRequest,
    }, // request
    response::{
        Response as Elevation,
        Status as ElevationStatus,
    }, // response
}; // use

pub use crate::geocoding::{
    error::Error as GeocodingError,
    forward::{
        component::Component as GeocodingComponent,
        ForwardRequest as GeocodingForwardRequest,
    }, // forward
    location_type::LocationType as LocationType,
    response::{
        Response as Geocoding,
        Status as GeocodingStatus,
    }, // response
    reverse::ReverseRequest as GeocodingReverseRequest,
}; // use

pub use crate::time_zone::{
    error::Error as TimeZoneError,
    request::Request as TimeZoneRequest,
    response::{
        Response as TimeZone,
        Status as TimeZoneStatus,
    }, // reponse
}; // use