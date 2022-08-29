//! Provides some `Bounds` conversion methods for the
//! [geo](https://crates.io/crates/geo) crate.

use crate::{Bounds, error::Error, LatLng};
use geo::geometry::{Coordinate, Polygon, Rect};
use rust_decimal::{Decimal, prelude::FromPrimitive};

// -----------------------------------------------------------------------------

impl TryFrom<&Rect> for Bounds {

    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a `geo::geometry::Rect` struct to a
    /// `google_maps::Bounds` struct.
    fn try_from(rect: &Rect) -> Result<Self, Self::Error> {

        let min_coordinate: Coordinate = rect.min();
        let max_coordinate: Coordinate = rect.max();

        // Due to a `try_from` name collision between the `LatLng` method and
        // the `TryFrom` trait, we must do the conversion in a longer way:

        let sw_lat: Decimal = Decimal::from_f64(max_coordinate.y)
            .ok_or_else(|| Error::FloatToDecimalConversionError(max_coordinate.y.to_string()))?;

        let sw_lng: Decimal = Decimal::from_f64(min_coordinate.x)
            .ok_or_else(|| Error::FloatToDecimalConversionError(min_coordinate.x.to_string()))?;

        let southwest: LatLng = LatLng::try_from(sw_lat, sw_lng)?;

        let ne_lat: Decimal = Decimal::from_f64(min_coordinate.y)
            .ok_or_else(|| Error::FloatToDecimalConversionError(min_coordinate.y.to_string()))?;

        let ne_lng: Decimal = Decimal::from_f64(max_coordinate.x)
            .ok_or_else(|| Error::FloatToDecimalConversionError(max_coordinate.x.to_string()))?;

        let northeast: LatLng = LatLng::try_from(ne_lat, ne_lng)?;

        Ok(Bounds { southwest, northeast })

    } // fn

} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&Bounds> for Rect {

    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a `google_maps::Bounds` struct to a
    /// `geo::geometry::Rect` struct.
    fn try_from(bounds: &Bounds) -> Result<Self, Self::Error> {

        let c1: Coordinate = Coordinate::try_from(&bounds.southwest)?;
        let c2: Coordinate = Coordinate::try_from(&bounds.northeast)?;

        Ok(Rect::new(c1, c2))

    } // fn

} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&Bounds> for Polygon {

    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a `google_maps::Bounds` struct to a
    /// `geo::geometry::Polygon` struct.
    fn try_from(bounds: &Bounds) -> Result<Self, Self::Error> {

        Ok(Rect::try_from(bounds)?.to_polygon())

    } // fn

} // impl