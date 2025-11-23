//! Provides conversion traits between `LatLng` and `geo-types` geometry types.
//!
//! This module enables interoperability with the [`geo`](https://crates.io/crates/geo) crate
//! ecosystem. Use these conversions when you need to perform geometric operations (distance
//! calculations, polygon containment, etc.) using `geo` and then pass the results to Google Maps
//! APIs, or vice versa.
//!
//! Note: both `Coord` and `Point` are 2D types. `Coord` is the raw (x, y) building block that all
//! geometries are composed of, while `Point` is a geometry type that wraps a `Coord` and
//! participates in geometric algorithms.

use crate::{error::Error as GoogleMapsError, types::Error as TypeError, types::LatLng};
use geo_types::geometry::{Coord, Point};
use rust_decimal::{prelude::FromPrimitive, prelude::ToPrimitive, Decimal};

// -------------------------------------------------------------------------------------------------
//
// Coord → LatLng conversions

impl TryFrom<&Coord> for LatLng {
    type Error = GoogleMapsError;

    /// Converts a borrowed `geo_types::Coord` to a `LatLng`.
    ///
    /// Extracts the `x` (longitude) and `y` (latitude) values from the `Coord`, converts them from
    /// `f64` to `Decimal`, and validates they fall within valid geographic ranges. Use this when
    /// you have raw coordinate values from `geo` geometry construction (like polygon vertices or
    /// line segments) that you need to pass to Google Maps APIs.
    fn try_from(coordinate: &Coord) -> Result<Self, Self::Error> {
        let lat = Decimal::from_f64(coordinate.y)
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(coordinate.y.to_string()))?;

        let lng = Decimal::from_f64(coordinate.x)
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(coordinate.x.to_string()))?;

        Self::try_from_dec(lat, lng)
    }
}

impl TryFrom<Coord> for LatLng {
    type Error = GoogleMapsError;

    /// Converts an owned `geo_types::Coord` to a `LatLng`.
    ///
    /// Delegates to the borrowed implementation. Use this when consuming a `Coord` value that's no
    /// longer needed after conversion.
    fn try_from(coordinate: Coord) -> Result<Self, Self::Error> {
        (&coordinate).try_into()
    }
}

// -------------------------------------------------------------------------------------------------
//
// LatLng → Coord conversions

impl TryFrom<&LatLng> for Coord {
    type Error = GoogleMapsError;

    /// Converts a borrowed `LatLng` to a `geo_types::Coord`.
    ///
    /// Extracts the latitude and longitude values, converts them from `Decimal` to `f64`, and
    /// constructs a `Coord` with x=longitude and y=latitude.
    ///
    /// Use this when you need to build `geo` geometry types (like `LineString` or `Polygon`) from
    /// Google Maps coordinates.
    ///
    /// `Coord` is the fundamental building block for all `geo` geometries. Returns an error if the
    /// `Decimal` values cannot be represented as `f64`.
    fn try_from(lat_lng: &LatLng) -> Result<Self, Self::Error> {
        let x = lat_lng
            .lng
            .to_f64()
            .ok_or(TypeError::InvalidLongitude(lat_lng.lat, lat_lng.lng))?;

        let y = lat_lng
            .lat
            .to_f64()
            .ok_or(TypeError::InvalidLatitude(lat_lng.lat, lat_lng.lng))?;

        Ok(Self { x, y })
    }
}

impl TryFrom<LatLng> for Coord {
    type Error = GoogleMapsError;

    /// Converts an owned `LatLng` to a `geo_types::Coord`.
    ///
    /// Delegates to the borrowed implementation. Use this when consuming a `LatLng` value that's no
    /// longer needed after conversion.
    fn try_from(lat_lng: LatLng) -> Result<Self, Self::Error> {
        (&lat_lng).try_into()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Point → LatLng conversions

impl TryFrom<&Point> for LatLng {
    type Error = GoogleMapsError;

    /// Converts a borrowed `geo_types::Point` to a `LatLng`.
    ///
    /// Extracts the x (longitude) and y (latitude) coordinates from the `Point`, converts them from
    /// `f64` to `Decimal`, and validates they fall within valid geographic ranges.
    ///
    /// Use this when you have point geometry results from `geo` algorithms (like centroids,
    /// intersections, or closest point calculations) that you need to pass to Google Maps APIs.
    /// `Point` is a geometry type that participates in geometric operations, unlike the raw `Coord`
    /// building block.
    fn try_from(point: &Point) -> Result<Self, Self::Error> {
        let lat = Decimal::from_f64(point.y())
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(point.y().to_string()))?;

        let lng = Decimal::from_f64(point.x())
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(point.x().to_string()))?;

        Self::try_from_dec(lat, lng)
    }
}

impl TryFrom<Point> for LatLng {
    type Error = GoogleMapsError;

    /// Converts an owned `geo_types::Point` to a `LatLng`.
    ///
    /// Delegates to the borrowed implementation. Use this when consuming a `Point` value that's no
    /// longer needed after conversion.
    fn try_from(point: Point) -> Result<Self, Self::Error> {
        (&point).try_into()
    }
}

// -----------------------------------------------------------------------------
//
// LatLng → Point conversions

impl TryFrom<&LatLng> for Point {
    type Error = GoogleMapsError;

    /// Converts a borrowed `LatLng` to a `geo_types::Point`.
    ///
    /// Extracts the latitude and longitude values, converts them from `Decimal` to `f64`, and
    /// constructs a `Point` with x=longitude and y=latitude.
    ///
    /// Use this when you need to perform point-based geometric algorithms on Google Maps
    /// coordinates (like distance calculations via Haversine, containment checks, or finding
    /// closest points).
    ///
    /// `Point` is a geometry type that participates in the full `geo` algorithm ecosystem, unlike
    /// the raw `Coord` building block. Returns an error if the `Decimal` values cannot be
    /// represented as `f64`.
    fn try_from(lat_lng: &LatLng) -> Result<Self, Self::Error> {
        let x = lat_lng
            .lng
            .to_f64()
            .ok_or(TypeError::InvalidLongitude(lat_lng.lat, lat_lng.lng))?;

        let y = lat_lng
            .lat
            .to_f64()
            .ok_or(TypeError::InvalidLatitude(lat_lng.lat, lat_lng.lng))?;

        Ok(Self::new(x, y))
    }
}

impl TryFrom<LatLng> for Point {
    type Error = GoogleMapsError;

    /// Converts an owned `LatLng` to a `geo_types::Point`.
    ///
    /// Delegates to the borrowed implementation. Use this when consuming a `LatLng` value that's no
    /// longer needed after conversion.
    fn try_from(lat_lng: LatLng) -> Result<Self, Self::Error> {
        (&lat_lng).try_into()
    }
}