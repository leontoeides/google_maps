//! # google_maps
//!
//! An unofficial Google Maps Platform API for the Rust programming language.
//!
//! # Welcome
//!
//! As of version 0.1.0 this crate is expected to work well, work reliably, and
//! have the most important features implemented. There are some creature
//! comforts and specialized APIs not implemented yet.
//!
//! While an early release, for most people this crate should work fine as is.
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
//!     GOOGLE_API_KEY
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
//! 7. Asynchronous

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
pub use time::{Date, PrimitiveDateTime, Time};

pub use crate::bounds::Bounds as Bounds;
pub use crate::language::Language as Language;
pub use crate::latlng::LatLng as LatLng;
pub use crate::place_type::PlaceType as PlaceType;
pub use crate::region::Region as Region;

pub use crate::directions::{
    request::avoid::Avoid as Avoid,
    request::departure_time::DepartureTime as DepartureTime,
    request::location::Location as Location,
    request::Request as DirectionsRequest,
    request::traffic_model::TrafficModel as TrafficModel,
    request::transit_mode::TransitMode as TransitMode,
    request::transit_route_preference::TransitRoutePreference as TransitRoutePreference,
    request::unit_system::UnitSystem as UnitSystem,
    request::waypoint::Waypoint as Waypoint,
    response::Response as Directions,
    response::status::Status as DirectionsStatus,
    travel_mode::TravelMode as TravelMode,
};

pub use crate::elevation::{
    error::Error as ElevationError,
    request::Locations as ElevationLocations,
    request::Request as ElevationRequest,
    response::Response as Elevation,
    response::Status as ElevationStatus,
};

pub use crate::geocoding::{
    error::Error as GeocodingError,
    forward::component::Component as GeocodingComponent,
    forward::ForwardRequest as GeocodingForwardRequest,
    location_type::LocationType as LocationType,
    response::Response as Geocoding,
    response::Status as GeocodingStatus,
    reverse::ReverseRequest as GeocodingReverseRequest,
};

pub use crate::time_zone::{
    error::Error as TimeZoneError,
    request::Request as TimeZoneRequest,
    response::Response as TimeZone,
    response::Status as TimeZoneStatus,
};