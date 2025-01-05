//! **Look in this module for documentation on building your _Distance Matrix
//! API_ query**. In particular, look at the _Request_ struct for examples of the
//! builder pattern. This module contains the tools (enums, structs, methods)
//! for building your Google Maps Platform request.

mod build;
mod end_point;
mod new;
mod query_string;
mod validatable;
mod with_arrival_time;
mod with_departure_time;
mod with_language;
mod with_region;
mod with_restrictions;
mod with_traffic_model;
mod with_transit_modes;
mod with_transit_route_preference;
mod with_travel_mode;
mod with_unit_system;

#[cfg(feature = "reqwest")]
mod execute;

#[cfg(feature = "reqwest")]
mod get;

// -----------------------------------------------------------------------------

use crate::directions::request::{
    avoid::Avoid,
    departure_time::DepartureTime,
    traffic_model::TrafficModel,
    transit_mode::TransitMode,
    transit_route_preference::TransitRoutePreference,
    unit_system::UnitSystem,
    waypoint::Waypoint,
};

// -----------------------------------------------------------------------------
//
/// **Look at this `Request` struct for documentation on how to build your
/// _Distance Matrix API_ query**. The methods implemented for this struct are
/// what's used to build your request.
#[derive(Debug)]
pub struct Request<'r> {
    // Required parameters:
    // --------------------
    /// This structure contains the application's API key and other
    /// user-definable settings such as "maximum retries."
    client: &'r crate::client::Client,

    /// One or more locations to use as the finishing point for calculating
    /// travel distance and time. You can pass an address, latitude/longitude,
    /// place ID, or encoded polyline.
    destinations: Vec<Waypoint>,

    /// The starting point for calculating travel distance and time. You can
    /// pass an address, latitude/longitude, place ID, or encoded polyline.
    origins: Vec<Waypoint>,

    // Optional parameters:
    // --------------------
    /// Desired arrival time. See method `with_arrival_time()` for more
    /// information.
    arrival_time: Option<chrono::NaiveDateTime>,

    /// Desired departure time. See files `departure_time.rs` and method
    /// `with_departure_time()` for more information.
    departure_time: Option<DepartureTime>,

    /// Language in which to return results. See file `language.rs` and method
    /// `with_language()` for more information.
    language: Option<crate::types::Language>,

    /// Region bias. See file `region.rs` and method `with_region()` for more
    /// information.
    region: Option<crate::types::Region>,

    /// Features that routes should avoid. See file `avoid.rs` and method
    /// `with_restrictions()` for more information.
    restrictions: Vec<Avoid>,

    /// Assumptions to use when calculating time in traffic. See file
    /// `traffic_model.rs` and method `with_traffic_model()` for more
    /// information.
    traffic_model: Option<TrafficModel>,

    /// Preferred modes of transit. See file `transit_mode.rs` and method
    /// `with_transit_modes()` for more information.
    transit_modes: Vec<TransitMode>,

    /// Preferences for transit routes. See file `transit_route_preference.rs`
    /// and method `with_transit_route_preference()` for more information.
    transit_route_preference: Option<TransitRoutePreference>,

    /// Mode of transportation. See file `travel_mode.rs` and method
    /// `with_travel_mode()` for more information.
    travel_mode: Option<crate::directions::travel_mode::TravelMode>,

    /// Unit system to use when displaying results. See file `unit_system.rs`
    /// and method `with_unit_system()` for more information.
    unit_system: Option<UnitSystem>,
} // struct