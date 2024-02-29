//! An [encoded polyline representation](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)
//! of the route.

use serde::{Deserialize, Serialize};

/// An [encoded polyline representation](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)
/// of the route.
///
/// See also: the Google Encoded Polyline encoding & decoding crate called
/// [polyline](https://crates.io/crates/polyline).

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Polyline {
    pub points: String,
} // struct
