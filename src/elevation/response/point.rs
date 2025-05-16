use crate::types::LatLng;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Structure for an elevation sample point.
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Point {
    /// Elevation of the location in meters.
    pub elevation: f64,

    /// Position for which elevation data is being computed. Note that for path
    /// requests, the set of `location` elements will contain the sampled points
    /// along the path.
    pub location: LatLng,

    /// Maximum distance between data points from which the elevation was
    /// interpolated, in meters. This property will be missing if the
    /// resolution is not known. Note that elevation data becomes more coarse
    /// (larger `resolution` values) when multiple points are passed. To obtain
    /// the most accurate elevation value for a point, it should be queried
    /// independently.
    #[serde(default)]
    pub resolution: Option<f64>,
} // struct
