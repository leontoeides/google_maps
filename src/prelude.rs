//! Put `use google_maps::prelude::*;` in your code will to get more convenient
//! access to everything you need. If you're not concerned with name space
//! collisions or conflicts, you can glob import all google_maps structs and
//! enums by using this module.

// Re-export dependencies

pub use rust_decimal::Decimal;
pub use rust_decimal_macros::dec;
#[cfg(any(feature = "directions", feature = "distance_matrix", feature = "time_zone"))]
pub use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, offset::TimeZone, Utc};
#[cfg(any(feature = "directions", feature = "time_zone"))]
pub use chrono_tz::Tz;

// =============================================================================
//
// Common structures:

pub use crate::{
    bounds::Bounds,
    client_settings::ClientSettings,
    country::Country,
    language::Language,
    latlng::LatLng,
    place_type::PlaceType,
    region::Region,
}; // crate

// =============================================================================
//
// Optional structures:

// -----------------------------------------------------------------------------

#[cfg(feature = "enable-reqwest")]
pub use crate::request_rate::api::Api;

// -----------------------------------------------------------------------------

#[cfg(any(feature = "directions", feature = "distance_matrix"))]
pub use crate::directions::{
    request::{
        avoid::Avoid,
        departure_time::DepartureTime,
        location::Location,
        traffic_model::TrafficModel,
        transit_mode::TransitMode,
        transit_route_preference::TransitRoutePreference,
        unit_system::UnitSystem,
        waypoint::Waypoint,
    }, // request
    response::{
        driving_maneuver::DrivingManeuver,
        leg::Leg,
        overview_polyline::OverviewPolyline,
        polyline::Polyline,
        Response as DirectionsResponse,
        route::Route,
        status::Status as DirectionsStatus,
        step::Step,
        transit_agency::TransitAgency,
        transit_currency::TransitCurrency,
        transit_details::TransitDetails,
        transit_fare::TransitFare,
        transit_line::TransitLine,
        transit_stop::TransitStop,
        transit_time::TransitTime,
        transit_vehicle::TransitVehicle,
    }, // response
    travel_mode::TravelMode,
    vehicle_type::VehicleType,
}; // crate::directions

// -----------------------------------------------------------------------------

#[cfg(feature = "directions")]
pub use crate::directions::{
    request::Request as DirectionsRequest,
    response::{
        directions_distance::DirectionsDistance,
        directions_duration::DirectionsDuration,
    }, // response
}; // crate::directions

// -----------------------------------------------------------------------------

#[cfg(feature = "distance_matrix")]
pub use crate::distance_matrix::{
    request::Request as DistanceMatrixRequest,
    response::Response as DistanceMatrixResponse,
    response::status::Status as DistanceMatrixStatus,
}; // use crate::distance_matrix

// -----------------------------------------------------------------------------

#[cfg(feature = "elevation")]
pub use crate::elevation::{
    error::Error as ElevationError,
    request::{
        locations::Locations as ElevationLocations,
        Request as ElevationRequest
    }, // request
    response::{
        point::Point,
        Response as ElevationResponse,
        status::Status as ElevationStatus,
    }, // response
}; // crate::elevation

// -----------------------------------------------------------------------------

#[cfg(feature = "geocoding")]
pub use crate::geocoding::{
    error::Error as GeocodingError,
    forward::{
        component::Component as GeocodingComponent,
        ForwardRequest as GeocodingRequest,
    }, // forward
    location_type::LocationType,
    response::{
        address_component::AddressComponent,
        geocoding::Geocoding,
        geometry::Geometry,
        plus_code::PlusCode,
        Response as GeocodingResponse,
        status::Status as GeocodingStatus,
    }, // response
    reverse::ReverseRequest as GeocodingReverseRequest,
}; // crate::geocoding

// -----------------------------------------------------------------------------

#[cfg(feature = "time_zone")]
pub use crate::time_zone::{
    error::Error as TimeZoneError,
    request::Request as TimeZoneRequest,
    response::{
        Response as TimeZoneResponse,
        status::Status as TimeZoneStatus,
    }, // reponse
}; // crate::time_zone

// -----------------------------------------------------------------------------

#[cfg(feature = "place_autocomplete")]
pub use crate::places::place_autocomplete::{
    error::Error as PlaceAutocompleteError,
    response::{
        matched_substring::MatchedSubstring,
        prediction::Prediction,
        Response as PlaceAutocompleteResponse,
        status::Status as PlaceAutocompleteStatus,
        structured_format::StructuredFormat,
        term::Term,
    }, // response
    request::{
        autocomplete_type::AutocompleteType,
        Request as PlaceAutocompleteRequest,
    }, // request
}; // crate::places::place_autocomplete