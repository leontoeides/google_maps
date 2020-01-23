//! Contains the `Bounds` struct and its associated traits. `Bounds` is used
//! to specify a selection box over a geographic area using two
//! latitude/longitude pairs.

use crate::latlng::LatLng;
use serde::{Serialize, Deserialize};

/// Contains the recommended viewport for displaying the returned result,
/// specified as two latitude/longitude values defining the southwest and
/// northeast corner of the viewport bounding box. Generally the viewport is
/// used to frame a result when displaying it to a user.

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Bounds {
    pub southwest: LatLng,
    pub northeast: LatLng,
} // struct

impl std::convert::From<&Bounds> for String {
    /// Converts a `Bounds` struct to a `String` that contains two
    /// latitude/longitude pairs that represent a bounding box.
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