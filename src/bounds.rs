use crate::latlng::LatLng;
use serde::{Serialize, Deserialize};

/// Contains the recommended viewport for displaying the returned result,
/// specified as two latitude/longitude values defining the southwest and
/// northeast corner of the viewport bounding box. Generally the viewport is
/// used to frame a result when displaying it to a user.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bounds {
    pub southwest: LatLng,
    pub northeast: LatLng,
} // struct

impl From<&Bounds> for String {
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