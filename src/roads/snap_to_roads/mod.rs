//! The Roads API **Snap To Roads** service takes up to 100 GPS points collected
//! along a route, and returns a similar set of data, with the points snapped to
//! the most likely roads the vehicle was traveling along. Optionally, you can
//! request that the points be interpolated, resulting in a path that smoothly
//! follows the geometry of the road.
//!
//! # [Required parameters](https://developers.google.com/maps/documentation/roads/snap#required-parameters)
//!
//! * `path` - The path to be snapped. The path parameter accepts a list of
//!   latitude/longitude pairs. Latitude and longitude values should be
//!   separated by commas. Coordinates should be separated by the pipe
//!   character: "|". For example:
//!   `path=60.170880,24.942795|60.170879,24.942796|60.170877,24.942796`.
//!
//! Note: The snapping algorithm works best for points that are not too far
//! apart. If you observe odd snapping behavior, try creating paths that have
//! points closer together. To ensure the best snap-to-road quality, you should
//! aim to provide paths on which consecutive pairs of points are within 300m of
//! each other. This will also help in handling any isolated, long jumps between
//! consecutive points caused by GPS signal loss, or noise.
//!
//! # [Optional parameters](https://developers.google.com/maps/documentation/roads/snap#optional-parameters)
//!
//! * `interpolate` - Whether to interpolate a path to include all points
//!   forming the full road-geometry. When `true`, additional interpolated
//!   points will also be returned, resulting in a path that smoothly follows
//!   the geometry of the road, even around corners and through tunnels.
//!   Interpolated paths will most likely contain more points than the original
//!   path. Defaults to `false`.

pub mod request;
pub mod response;

// -----------------------------------------------------------------------------

const SERVICE_URL: &str = "https://roads.googleapis.com/v1/snapToRoads";

// -----------------------------------------------------------------------------

pub use crate::roads::snap_to_roads::request::Request as SnapToRoadsRequest;
pub use crate::roads::snap_to_roads::response::Response as SnapToRoadsResponse;
