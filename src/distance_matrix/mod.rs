//! The **Distance Matrix API** is a service that provides travel distance and
//! time for a matrix of origins and destinations, based on the recommended
//! route between start and end points.
//!
//! # [Get Started](https://developers.google.com/maps/documentation/distance-matrix/start)
//!
//! * **New Users**: Before you can start using the Google Maps Platform APIs
//!   and SDKs, you must sign up and create a billing account. To learn more,
//!   see [Get Started with Google Maps
//!   Platform](https://developers.google.com/maps/gmp-get-started).
//!
//! * This service is also available as part of the client-side [Maps JavaScript
//!   API](https://developers.google.com/maps/documentation/javascript/distancematrix),
//!   or for server-side use with the [Java Client, Python Client, Go Client and
//!   Node.js Client for Google Maps
//!   Services](https://developers.google.com/maps/documentation/distance-matrix/client-library).
//!
//! # [Start coding with our client libraries](https://developers.google.com/maps/documentation/distance-matrix/start#client-library)
//!
//! Client libraries make developing with the Google Maps web service APIs
//! easier by providing simple, native implementations of common tasks, such as
//! authentication, request throttling and automatic retry. The Distance Matrix
//! API is available in the [Java Client, Python Client, Go Client and Node.js
//! Client for Google Maps
//! Services](https://developers.google.com/maps/documentation/distance-matrix/client-library).
//!
//! # [Authentication, quotas, pricing, and policies](https://developers.google.com/maps/documentation/distance-matrix/start#auth)
//!
//! ## [Activate the API and get an API key](https://developers.google.com/maps/documentation/distance-matrix/start#get-a-key)
//!
//! To use the Distance Matrix API, you must first activate the API in the
//! Google Cloud Platform Console and obtain the proper authentication
//! credentials. You need to provide an **API key** in each request (or a
//! [client ID if you have a Premium
//! Plan](https://developers.google.com/maps/documentation/distance-matrix/get-api-key#premium-auth).
//!
//! Click the button below to flow through a process where you will:
//! 1. Create or select a project
//! 2. Enable the API
//! 3. Get an API key
//!
//! [Get Started](https://cloud.google.com/maps-platform/#get-started)
//!
//! [Learn more about authentication
//! credentials](https://developers.google.com/maps/documentation/distance-matrix/get-api-key).
//!
//! ## [Quotas and pricing](https://developers.google.com/maps/documentation/distance-matrix/start#quotas)
//!
//! Review the [usage and
//! billing](https://developers.google.com/maps/documentation/distance-matrix/usage-and-billing)
//! page for details on the quotas and pricing set for the Distance Matrix API.
//!
//! ## [Policies](https://developers.google.com/maps/documentation/distance-matrix/start#policies)
//!
//! Use of the Distance Matrix API must be in accordance with the [API
//! policies](https://developers.google.com/maps/documentation/distance-matrix/policies).
//!
//! # [Learn more](https://developers.google.com/maps/documentation/distance-matrix/start#learn-more)
//!
//! There’s more you can do with the Distance Matrix API, like [requesting
//! distance data for different travel
//! modes](https://developers.google.com/maps/documentation/distance-matrix/intro#travel_modes),
//! [requesting distance data in different units (for example, kilometers or
//! miles)](https://developers.google.com/maps/documentation/distance-matrix/intro#unit_systems),
//! and [estimating travel time in
//! traffic](https://developers.google.com/maps/documentation/distance-matrix/intro#traffic-model).
//! See the [Distance Matrix API developer
//! guide](https://developers.google.com/maps/documentation/distance-matrix/intro)
//! for more examples and other details.
//!
//! The [Distance Matrix API developer
//! guide](https://developers.google.com/maps/documentation/distance-matrix/intro)
//! is intended for developers who wish to compute travel distance and time
//! between a number of points within maps provided by one of the Google Maps
//! APIs. It provides an introduction to using the API and reference material on
//! the available parameters.

pub mod error;
pub mod request;
pub mod response;

// -----------------------------------------------------------------------------

pub const SERVICE_URL: &str = "https://maps.googleapis.com/maps/api/distancematrix";
pub const OUTPUT_FORMAT: &str = "json"; // json or xml

// -----------------------------------------------------------------------------

pub use crate::directions::{
    request::{
        avoid::Avoid, departure_time::DepartureTime, location::Location,
        traffic_model::TrafficModel, transit_mode::TransitMode,
        transit_route_preference::TransitRoutePreference, unit_system::UnitSystem,
        waypoint::Waypoint,
    }, // request
    response::{
        directions_distance::DirectionsDistance, directions_duration::DirectionsDuration,
        driving_maneuver::DrivingManeuver, leg::Leg, overview_polyline::OverviewPolyline,
        polyline::Polyline, route::Route, step::Step, transit_agency::TransitAgency,
        transit_currency::TransitCurrency, transit_details::TransitDetails,
        transit_fare::TransitFare, transit_line::TransitLine, transit_stop::TransitStop,
        transit_time::TransitTime, transit_vehicle::TransitVehicle,
    }, // response
    travel_mode::TravelMode,
    vehicle_type::VehicleType,
}; // crate::directions

pub use crate::distance_matrix::{
    request::Request as DistanceMatrixRequest, response::status::Status as DistanceMatrixStatus,
    response::Response as DistanceMatrixResponse,
}; // crate::distance_matrix
