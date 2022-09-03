//! Provides some `Waypoint` conversion methods for the
//! [geo](https://crates.io/crates/geo) crate.

use crate::directions::Waypoint;
use crate::error::Error;
use crate::LatLng;
use geo::geometry::{Coordinate, Point};
use rust_decimal::{Decimal, prelude::FromPrimitive};

// -----------------------------------------------------------------------------

impl TryFrom<&Coordinate> for Waypoint {

    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a `geo::geometry::Coordinate` struct to a
    /// `google_maps::directions::Waypoint` struct.
    fn try_from(coordinate: &Coordinate) -> Result<Self, Self::Error> {

        let lat: Decimal = Decimal::from_f64(coordinate.y)
            .ok_or_else(|| Error::FloatToDecimalConversionError(coordinate.y.to_string()))?;

        let lng: Decimal = Decimal::from_f64(coordinate.x)
            .ok_or_else(|| Error::FloatToDecimalConversionError(coordinate.x.to_string()))?;

        let lat_lng: LatLng = LatLng::try_from(lat, lng)?;

        Ok(Waypoint::LatLng(lat_lng))

    } // fn

} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&Point> for Waypoint {

    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a `geo::geometry::Point` struct to a
    /// `google_maps::directions::Waypoint` struct.
    fn try_from(point: &Point) -> Result<Self, Self::Error> {

        let lat: Decimal = Decimal::from_f64(point.y())
            .ok_or_else(|| Error::FloatToDecimalConversionError(point.y().to_string()))?;

        let lng: Decimal = Decimal::from_f64(point.x())
            .ok_or_else(|| Error::FloatToDecimalConversionError(point.x().to_string()))?;

        let lat_lng: LatLng = LatLng::try_from(lat, lng)?;

        Ok(Waypoint::LatLng(lat_lng))

    } // fn

} // impl