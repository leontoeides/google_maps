//! Provides some `LatLng` conversion `TryFrom` traits for the
//! [geo](https://crates.io/crates/geo) crate.

use crate::{error::Error as GoogleMapsError, types::Error as TypeError, LatLng};
use geo_types::geometry::{Coord, Point};
use rust_decimal::{prelude::FromPrimitive, prelude::ToPrimitive, Decimal};

// -----------------------------------------------------------------------------

impl TryFrom<&Coord> for LatLng {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = GoogleMapsError;

    /// Attempts to convert a borrowed `&geo_types::geometry::Coord` struct to a
    /// `google_maps::LatLng` struct.
    fn try_from(coordinate: &Coord) -> Result<Self, Self::Error> {
        let lat: Decimal = Decimal::from_f64(coordinate.y)
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(coordinate.y.to_string()))?;

        let lng: Decimal = Decimal::from_f64(coordinate.x)
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(coordinate.x.to_string()))?;

        Self::try_from_dec(lat, lng)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<Coord> for LatLng {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = GoogleMapsError;

    /// Attempts to convert an owned `geo_types::geometry::Coord` struct into a
    /// `google_maps::LatLng` struct.
    fn try_from(coordinate: Coord) -> Result<Self, Self::Error> {
        (&coordinate).try_into()
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&LatLng> for Coord {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = GoogleMapsError;

    /// Attempts to convert a borrowed `&google_maps::LatLng` struct to a
    /// `geo_types::geometry::Coord` struct.
    fn try_from(lat_lng: &LatLng) -> Result<Self, Self::Error> {
        let x: f64 = lat_lng
            .lng
            .to_f64()
            .ok_or(TypeError::InvalidLongitude(lat_lng.lat, lat_lng.lng))?;

        let y: f64 = lat_lng
            .lat
            .to_f64()
            .ok_or(TypeError::InvalidLatitude(lat_lng.lat, lat_lng.lng))?;

        Ok(Self { x, y })
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<LatLng> for Coord {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = GoogleMapsError;

    /// Attempts to convert an owned `google_maps::LatLng` struct into a
    /// `geo_types::geometry::Coord` struct.
    fn try_from(lat_lng: LatLng) -> Result<Self, Self::Error> {
        (&lat_lng).try_into()
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&Point> for LatLng {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = GoogleMapsError;

    /// Attempts to convert a borrowed `&geo_types::geometry::Point` struct to a
    /// `google_maps::LatLng` struct.
    fn try_from(point: &Point) -> Result<Self, Self::Error> {
        let lat: Decimal = Decimal::from_f64(point.y())
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(point.y().to_string()))?;

        let lng: Decimal = Decimal::from_f64(point.x())
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(point.x().to_string()))?;

        Self::try_from_dec(lat, lng)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<Point> for LatLng {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = GoogleMapsError;

    /// Attempts to convert an owned `geo_types::geometry::Point` struct into a
    /// `google_maps::LatLng` struct.
    fn try_from(point: Point) -> Result<Self, Self::Error> {
        (&point).try_into()
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&LatLng> for Point {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = GoogleMapsError;

    /// Attempts to convert a borrowed `&google_maps::LatLng` struct to a
    /// `geo_types::geometry::Point` struct.
    fn try_from(lat_lng: &LatLng) -> Result<Self, Self::Error> {
        let x: f64 = lat_lng
            .lng
            .to_f64()
            .ok_or(TypeError::InvalidLongitude(lat_lng.lat, lat_lng.lng))?;

        let y: f64 = lat_lng
            .lat
            .to_f64()
            .ok_or(TypeError::InvalidLatitude(lat_lng.lat, lat_lng.lng))?;

        Ok(Self::new(x, y))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<LatLng> for Point {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = GoogleMapsError;

    /// Attempts to convert an owned `google_maps::LatLng` struct into a
    /// `geo_types::geometry::Point` struct.
    fn try_from(lat_lng: LatLng) -> Result<Self, Self::Error> {
        (&lat_lng).try_into()
    } // fn
} // impl