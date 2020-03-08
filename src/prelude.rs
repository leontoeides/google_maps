//! Put `use google_maps::prelude::*;` in your code will to get more convenient
//! access to everything you need. If you're not concerned with name space
//! collisions or conflicts, you can glob import all google_maps structs and
//! enums by using this module.

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