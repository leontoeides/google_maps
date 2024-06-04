//! **Roads API** - Identify the roads a vehicle is traveling along, and get
//! metadata about those roads.
//!
//! # [Overview](https://developers.google.com/maps/documentation/roads/overview)
//!
//! * **Before you begin**: Before you start using the Places API, you need a
//!   project with a billing account and the Places API enabled. We recommend
//!   creating multiple Project Owners and Billing Administrators, so that you'll
//!   always have someone with these roles available to your team. To learn more,
//!   see [Set up in Cloud Console](https://developers.google.com/maps/documentation/places/web-service/cloud-setup).
//!
//! The Roads API identifies the roads a vehicle was traveling along and
//! provides additional metadata about those roads, such as speed limits.
//!
//! Before you start developing with the Roads API, review the authentication
//! requirements (you need an API key) and the API usage and billing
//! information.
//!
//! # [Introduction](https://developers.google.com/maps/documentation/roads/overview#introduction)
//!
//! [Watch this video for some examples of the kinds of apps that will find the
//! Roads API useful.](https://youtu.be/e5YDb-XnDVk)
//!
//! The Roads API allows you to map GPS coordinates to the geometry of the road,
//! and to determine the speed limit along those road segments. The API is
//! available via a simple HTTPS interface, and exposes the following services:
//!
//! * [Snap to roads](https://developers.google.com/maps/documentation/roads/snap)
//!   This service returns the best-fit road geometry for a given set of GPS
//!   coordinates. This service takes up to 100 GPS points collected along a
//!   route, and returns a similar set of data with the points snapped to the
//!   most likely roads the vehicle was traveling along. Optionally, you can
//!   request that the points be interpolated, resulting in a path that smoothly
//!   follows the geometry of the road.
//!
//! * [Nearest roads](https://developers.google.com/maps/documentation/roads/nearest)
//!   This service returns individual road segments for a given set of GPS
//!   coordinates. This services takes up to 100 GPS points and returns the
//!   closest road segment for each point. The points passed do not need to be
//!   part of a continuous path.
//!
//! * [Speed limits](https://developers.google.com/maps/documentation/roads/speed-limits)
//!   **(Not yet implemented in this client.)**
//!   This service returns the posted speed limit for a road segment. The Speed
//!   Limit service is available to all customers with an Asset Tracking license.
//!   For [Google Maps Platform Premium Plan customers](https://developers.google.com/maps/premium)
//!   who transitioned to pay-as-you-go pricing, the feature remains active.
//!
//! # [Client library](https://developers.google.com/maps/documentation/roads/overview#client_library)
//!
//! The Roads API is available with the [Java Client, Python Client, Go Client
//! and Node.js Client for Google Maps Services](https://developers.google.com/maps/web-services/client-library).
//! Client libraries make developing with the Roads API easier by providing
//! simple, native implementations of common tasks, such as authentication,
//! request throttling and automatic retry.

pub mod error;
pub mod error_response;
pub mod nearest_roads;
pub mod snap_to_roads;
pub mod snapped_point;
pub mod status;

// -----------------------------------------------------------------------------

pub use crate::roads::{
    error::Error as RoadsError, error_response::ErrorResponse as RoadsErrorResponse,
    snapped_point::SnappedPoint, status::Status as RoadsStatus,
}; // crate::roads

pub use crate::roads::snap_to_roads::{
    request::Request as SnapToRoadsRequest, response::Response as SnapToRoadsResponse,
}; // crate::roads::snap_to_roads

pub use crate::roads::nearest_roads::{
    request::Request as NearestRoadsRequest, response::Response as NearestRoadsResponse,
}; // crate::roads::nearest_roads
