//! The Roads API **Nearest Roads** service returns individual road segments for
//! a given set of GPS coordinates. This services takes up to 100 GPS points and
//! returns the closest road segment for each point. The points passed do not
//! need to be part of a continuous path.
//!
//! **If you are working with sequential GPS points, use Nearest Roads.**
//!
//! # [Required parameters](https://developers.google.com/maps/documentation/roads/nearest#required-parameters)
//!
//! * `points` - The path to be snapped. The path parameter accepts a list of
//!   latitude/longitude pairs.
//!
//! Note: The snapping algorithm works best for points that are not too far
//! apart. If you observe odd snapping behavior, try creating paths that have
//! points closer together. To ensure the best snap-to-road quality, you should
//! aim to provide paths on which consecutive pairs of points are within 300m of
//! each other. This will also help in handling any isolated, long jumps between
//! consecutive points caused by GPS signal loss, or noise.

pub mod request;
pub mod response;

// -----------------------------------------------------------------------------

const SERVICE_URL: &str = "https://roads.googleapis.com/v1/nearestRoads";

// -----------------------------------------------------------------------------

pub use crate::roads::nearest_roads::request::Request as NearestRoadsRequest;
pub use crate::roads::nearest_roads::response::Response as NearestRoadsResponse;
