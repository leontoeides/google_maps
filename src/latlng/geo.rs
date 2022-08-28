//! Provides some `LatLng` conversion methods for the
//! [geo](https://crates.io/crates/geo) crate.

use crate::{error::Error, LatLng};
use geo::geometry::{Coordinate, Point};
use rust_decimal::{Decimal, prelude::FromPrimitive, prelude::ToPrimitive};

// -----------------------------------------------------------------------------

impl TryFrom<&Coordinate> for LatLng {

    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a `geo::geometry::Coordinate` struct to a
    /// `google_maps::LatLng` struct.
    fn try_from(coordinate: &Coordinate) -> Result<Self, Self::Error> {

        let lat: Decimal = Decimal::from_f64(coordinate.y)
            .ok_or_else(|| Error::FloatToDecimalConversionError(coordinate.y.to_string()))?;

        let lng: Decimal = Decimal::from_f64(coordinate.x)
            .ok_or_else(|| Error::FloatToDecimalConversionError(coordinate.x.to_string()))?;

        LatLng::try_from(lat, lng)

    } // fn

} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&LatLng> for Coordinate {

    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a `google_maps::LatLng` struct to a
    /// `geo::geometry::Coordinate` struct.
    fn try_from(lat_lng: &LatLng) -> Result<Self, Self::Error> {

        let x: f64 = lat_lng.lng.to_f64()
            .ok_or(Error::InvalidLongitude(lat_lng.lat, lat_lng.lng))?;

        let y: f64 = lat_lng.lat.to_f64()
            .ok_or(Error::InvalidLatitude(lat_lng.lat, lat_lng.lng))?;

        Ok(Coordinate { x, y })

    } // fn

} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&Point> for LatLng {

    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a `geo::geometry::Point` struct to a
    /// `google_maps::LatLng` struct.
    fn try_from(point: &Point) -> Result<Self, Self::Error> {

        let lat: Decimal = Decimal::from_f64(point.y())
            .ok_or_else(|| Error::FloatToDecimalConversionError(point.y().to_string()))?;

        let lng: Decimal = Decimal::from_f64(point.x())
            .ok_or_else(|| Error::FloatToDecimalConversionError(point.x().to_string()))?;

        LatLng::try_from(lat, lng)

    } // fn

} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&LatLng> for Point {

    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a `google_maps::LatLng` struct to a
    /// `geo::geometry::Point` struct.
    fn try_from(lat_lng: &LatLng) -> Result<Self, Self::Error> {

        let x: f64 = lat_lng.lng.to_f64()
            .ok_or(Error::InvalidLongitude(lat_lng.lat, lat_lng.lng))?;

        let y: f64 = lat_lng.lat.to_f64()
            .ok_or(Error::InvalidLatitude(lat_lng.lat, lat_lng.lng))?;

        Ok(Point::new(x, y))

    } // fn

} // impl