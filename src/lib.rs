//! As of version 0.2.0 this crate is expected to work well, work reliably, and
//! have the most important features implemented. There are some creature
//! comforts and niche APIs not implemented yet. While an early release, for
//! most people this crate should work fine as is.
//!
//! To do:
//! 1. [Places API](https://developers.google.com/places/web-service/intro)
//! 2. [Roads API](https://developers.google.com/maps/documentation/roads/intro)
//! 3. Automatic Rate Limiting
//! 4. Retry on Failure
//! 5. Asynchronous

pub mod bounds;
pub mod directions;
pub mod elevation;
pub mod geocoding;
pub mod language;
pub mod latlng;
pub mod place_type;
pub mod region;
pub mod time_zone;

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

pub use crate::{
    bounds::Bounds as Bounds,
};

pub use crate::{
    language::Language as Language,
};

pub use crate::{
    latlng::LatLng as LatLng,
};

pub use crate::{
    place_type::PlaceType as PlaceType,
};

pub use crate::{
    region::Region as Region,
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