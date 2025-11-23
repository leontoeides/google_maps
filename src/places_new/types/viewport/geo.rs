//! Provides conversion traits between `Viewport` and `geo-types` geometry types.
//!
//! This module enables seamless interoperability with the [`geo`](https://crates.io/crates/geo)
//! crate ecosystem. Use these conversions when you need to perform geometric
//! operations (area calculations, intersection tests, etc.) on map viewports
//! using `geo`, or when converting geometric bounds back to Google Maps viewport
//! format.

use crate::places_new::autocomplete::{Error, request::{LatLng, Viewport}};
use geo_types::geometry::{Coord, Polygon, Rect};

// -------------------------------------------------------------------------------------------------
//
// Rect → Viewport conversions

impl TryFrom<&Rect> for Viewport {
    type Error = Error;

    /// Converts a borrowed `geo_types::Rect` to a `Viewport`.
    ///
    /// Extracts the minimum and maximum coordinates from the `Rect` and constructs the
    /// corresponding `low` (southwest) and `high` (northeast) corners of the viewport.
    ///
    /// Note that `geo` uses `min`/`max` nomenclature while Google Maps uses `low`/`high`.
    ///
    /// Use this when you have geometric calculation results from `geo` that you need to pass to
    /// Google Maps APIs.
    ///
    /// # Errors
    ///
    /// * Returns an error if the coordinate conversion fails.
    fn try_from(rect: &Rect) -> Result<Self, Self::Error> {
        let min_coordinate: Coord = rect.min();
        let max_coordinate: Coord = rect.max();
        let low: LatLng = LatLng::try_from_f64(min_coordinate.y, min_coordinate.x)?;
        let high: LatLng = LatLng::try_from_f64(max_coordinate.y, max_coordinate.x)?;
        Self::try_new(low, high)
    }
}

impl TryFrom<Rect> for Viewport {
    type Error = Error;

    /// Converts an owned `geo_types::Rect` to a `Viewport`.
    ///
    /// Delegates to the borrowed implementation. Use this when consuming a `Rect` value that's no
    /// longer needed after conversion.
    ///
    /// # Errors
    ///
    /// * Returns an error if the coordinate conversion fails.
    fn try_from(rect: Rect) -> Result<Self, Self::Error> {
        (&rect).try_into()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Viewport → Rect conversions

impl TryFrom<&Viewport> for Rect {
    type Error = Error;

    /// Converts a borrowed `Viewport` to a `geo_types::Rect`.
    ///
    /// Extracts the `low` and `high` corner coordinates and constructs a `Rect` suitable for
    /// geometric operations in the `geo` ecosystem.
    ///
    /// Use this when you need to perform operations like area calculations, intersection tests, or
    /// containment checks on Google Maps viewports.
    ///
    /// # Errors
    ///
    /// Returns an error if the coordinate conversion fails.
    fn try_from(viewport: &Viewport) -> Result<Self, Self::Error> {
        let c1: Coord = Coord::try_from(&viewport.low)?;
        let c2: Coord = Coord::try_from(&viewport.high)?;
        Ok(Self::new(c1, c2))
    }
}

impl TryFrom<Viewport> for Rect {
    type Error = Error;

    /// Converts an owned `Viewport` to a `geo_types::Rect`.
    ///
    /// Delegates to the borrowed implementation. Use this when consuming a `Viewport` value that's
    /// no longer needed after conversion.
    ///
    /// # Errors
    ///
    /// * Returns an error if the coordinate conversion fails.
    fn try_from(viewport: Viewport) -> Result<Self, Self::Error> {
        (&viewport).try_into()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Viewport → Polygon conversions

impl TryFrom<&Viewport> for Polygon {
    type Error = Error;

    /// Converts a borrowed `Viewport` to a `geo_types::Polygon`.
    ///
    /// First converts the viewport to a `Rect`, then converts that rectangle to a closed polygon
    /// with four vertices representing the corners of the viewport.
    ///
    /// Use this when you need to perform polygon-based operations like point-in-polygon tests,
    /// polygon unions, or spatial joins on map viewports.
    ///
    /// # Errors
    ///
    /// * Returns an error if the coordinate conversion fails.
    fn try_from(viewport: &Viewport) -> Result<Self, Self::Error> {
        Ok(Rect::try_from(viewport)?.to_polygon())
    }
}

impl TryFrom<Viewport> for Polygon {
    type Error = Error;

    /// Converts an owned `Viewport` to a `geo_types::Polygon`.
    ///
    /// Delegates to the borrowed implementation. Use this when consuming a `Viewport` value that's
    /// no longer needed after conversion.
    ///
    /// # Errors
    ///
    /// * Returns an error if the coordinate conversion fails.
    fn try_from(viewport: Viewport) -> Result<Self, Self::Error> {
        (&viewport).try_into()
    }
}