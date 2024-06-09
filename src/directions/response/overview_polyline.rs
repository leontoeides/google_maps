//! An [encoded polyline representation](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)
//! of the route.

use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// An [encoded polyline representation](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)
/// of the route.
///
/// See also: the Google Encoded Polyline encoding & decoding crate called
/// [polyline](https://crates.io/crates/polyline).

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct OverviewPolyline {
    pub points: String,
} // struct

// -----------------------------------------------------------------------------

#[cfg(all(feature = "polyline", feature = "geo"))]
impl OverviewPolyline {
    /// Attempts to convert a borrowed `&OverviewPolyline` struct to a
    /// `geo_types::geometry::LineString<f64>` struct.
    ///
    /// # Errors
    ///
    /// * Returns an error if the polyline is invalid or if the decoded
    ///   coordinates are out of bounds.
    pub fn decode(
        &self,
        precision: u32
    ) -> Result<geo_types::geometry::LineString<f64>, crate::error::Error> {
        Ok(polyline::decode_polyline(&self.points, precision)?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

#[cfg(all(feature = "polyline", feature = "geo"))]
impl TryFrom<&OverviewPolyline> for geo_types::geometry::LineString<f64> {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a borrowed `&OverviewPolyline` struct to a
    /// `geo_types::geometry::LineString<f64>` struct.
    ///
    /// # Notes
    ///
    /// * Uses a hard-coded precision of `5`. For control over the precision,
    ///   use the `decode` method on the `OverviewPolyline` struct.
    ///
    /// # Errors
    ///
    /// * Returns an error if the polyline is invalid or if the decoded
    ///   coordinates are out of bounds.
    fn try_from(polyline: &OverviewPolyline) -> Result<Self, Self::Error> {
        polyline.decode(5)
    } // fn
} // impl

// -----------------------------------------------------------------------------

#[cfg(all(feature = "polyline", feature = "geo"))]
impl TryFrom<OverviewPolyline> for geo_types::geometry::LineString<f64> {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert an owned `OverviewPolyline` struct into a
    /// `geo_types::geometry::LineString<f64>` struct.
    ///
    /// # Notes
    ///
    /// * Uses a hard-coded precision of `5`. For control over the precision,
    ///   use the `decode` method on the `OverviewPolyline` struct.
    ///
    /// # Errors
    ///
    /// * Returns an error if the polyline is invalid or if the decoded
    ///   coordinates are out of bounds.
    fn try_from(polyline: OverviewPolyline) -> Result<Self, Self::Error> {
        polyline.decode(5)
    } // fn
} // impl