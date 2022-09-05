//! Contains the `Bounds` struct and its associated traits. It is used to
//! specify a selection or bounding box over a geographic area using two
//! latitude & longitude pairs.

#[cfg(feature = "geo")]
mod geo_conversions;

// -----------------------------------------------------------------------------

use crate::latlng::LatLng;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------

/// Contains the recommended viewport for displaying the returned result,
/// specified as two latitude & longitude pairs defining the southwest and
/// northeast corner of the viewport bounding box. Generally the viewport is
/// used to frame a result when displaying it to a user.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Bounds {
    /// South-west or bottom-left corner of the bounding box.
    pub southwest: LatLng,
    /// North-east or top-right corner of the bounding box.
    pub northeast: LatLng,
} // struct

// -----------------------------------------------------------------------------

impl std::convert::From<&Bounds> for String {
    /// Converts a `Bounds` struct to a `String` that contains two
    /// latitude & longitude pairs that represent a bounding box.
    fn from(bounds: &Bounds) -> String {
        format!(
            "{},{}|{},{}",
            bounds.southwest.lat,
            bounds.southwest.lng,
            bounds.northeast.lat,
            bounds.northeast.lng,
        ) // format!
    } // fn
} // impl