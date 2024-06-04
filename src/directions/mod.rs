//! The **Directions API** is a service that calculates directions between
//! locations. You can search for directions for several modes of
//! transportation, including transit, driving, walking, or cycling.
//!
//! # [Get Started](https://developers.google.com/maps/documentation/directions/start)
//!
//! * **New Users**: Before you can start using the Google Maps Platform APIs
//!   and SDKs, you must sign up and create a billing account. To learn more,
//!   see [Get Started with Google Maps
//!   Platform](https://developers.google.com/maps/gmp-get-started).
//!
//! * This service is also available as part of the client-side [Maps JavaScript
//!   API](https://developers.google.com/maps/documentation/javascript/directions),
//!   or for server-side use with the [Java Client, Python Client, Go Client and
//!   Node.js Client for Google Maps
//!   Services](https://developers.google.com/maps/documentation/directions/client-library).
//!
//! # [Start coding with our client libraries](https://developers.google.com/maps/documentation/directions/start#client-library)
//!
//! Client libraries make developing with the Google Maps web service APIs
//! easier by providing simple, native implementations of common tasks, such as
//! authentication, request throttling and automatic retry. The Directions API
//! is available in the [Java Client, Python Client, Go Client and Node.js
//! Client for Google Maps
//! Services](https://developers.google.com/maps/documentation/directions/client-library).
//!
//! # [Authentication, quotas, pricing, and policies](https://developers.google.com/maps/documentation/directions/start#auth)
//!
//! ## [Activate the API and get an API key](https://developers.google.com/maps/documentation/directions/start#get-a-key)
//!
//! To use the Directions API, you must first activate the API in the Google
//! Cloud Platform Console and obtain the proper authentication credentials. You
//! need to provide an **API key** in each request (or a [client ID if you have
//! a Premium
//! Plan](https://developers.google.com/maps/documentation/directions/get-api-key#premium-auth).
//!
//! Click the button below to flow through a process where you will:
//! 1. Create or select a project
//! 2. Enable the API
//! 3. Get an API key
//!
//! [Get Started](https://cloud.google.com/maps-platform/#get-started)
//!
//! [Learn more about authentication
//! credentials](https://developers.google.com/maps/documentation/directions/get-api-key).
//!
//! ## [Quotas and pricing](https://developers.google.com/maps/documentation/directions/start#quotas)
//!
//! Review the [usage and
//! billing](https://developers.google.com/maps/documentation/directions/usage-and-billing)
//! page for details on the quotas and pricing set for the Directions API.
//!
//! ## [Policies](https://developers.google.com/maps/documentation/directions/start#policies)
//!
//! Use of the Directions API must be in accordance with the [API
//! policies](https://developers.google.com/maps/documentation/directions/policies).
//!
//! From our Terms of Service: **Innovate, but don't duplicate.** Don't make a
//! substitute for Google Maps. If your app's primary purpose is navigation, a
//! business directory, or a general purpose "maps app", it's a substitute for
//! Google Maps. [Learn
//! more](https://cloud.google.com/maps-platform/terms/#3-license).
//!
//! # [Learn more](https://developers.google.com/maps/documentation/directions/start#learn-more)
//!
//! There’s more you can do with the Directions API, like [requesting directions
//! via different travel
//! modes](https://developers.google.com/maps/documentation/directions/intro#TravelModes),
//! [using waypoints to calculate routes through additional
//! locations](https://developers.google.com/maps/documentation/directions/intro#Waypoints),
//! and [estimating travel
//! time](https://developers.google.com/maps/documentation/directions/intro#traffic-model).
//! See the [Directions API developer
//! guide](https://developers.google.com/maps/documentation/directions/intro)
//! for more examples and other details.
//!
//! The [Directions API developer
//! guide](https://developers.google.com/maps/documentation/directions/intro) is
//! intended for website and mobile developers who want to compute direction
//! data within maps provided by one of the Google Maps APIs. It provides an
//! introduction to using the API and reference material on the available
//! parameters.

pub mod error;
pub mod request;
pub mod response;
pub mod travel_mode;
pub mod vehicle_type;

// -----------------------------------------------------------------------------

const SERVICE_URL: &str = "https://maps.googleapis.com/maps/api/directions";
const OUTPUT_FORMAT: &str = "json"; // json or xml

// -----------------------------------------------------------------------------

pub use crate::directions::{
    request::{
        avoid::Avoid, departure_time::DepartureTime, location::Location,
        traffic_model::TrafficModel, transit_mode::TransitMode,
        transit_route_preference::TransitRoutePreference, unit_system::UnitSystem,
        waypoint::Waypoint, Request as DirectionsRequest,
    }, // crate::directions::request
    response::{
        directions_distance::DirectionsDistance, directions_duration::DirectionsDuration,
        driving_maneuver::DrivingManeuver, leg::Leg, overview_polyline::OverviewPolyline,
        polyline::Polyline, route::Route, status::Status as DirectionsStatus, step::Step,
        transit_agency::TransitAgency, transit_currency::TransitCurrency,
        transit_details::TransitDetails, transit_fare::TransitFare, transit_line::TransitLine,
        transit_stop::TransitStop, transit_time::TransitTime, transit_vehicle::TransitVehicle,
        Response as DirectionsResponse,
    }, // crate::directions::response
    travel_mode::TravelMode,
    vehicle_type::VehicleType,
}; // use
