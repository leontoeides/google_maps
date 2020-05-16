//! An [encoded polyline representation](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)
//! of the route.

use serde::{Deserialize, Serialize};

/// An [encoded polyline representation](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)
/// of the route.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Polyline {
    pub points: String,
} // struct