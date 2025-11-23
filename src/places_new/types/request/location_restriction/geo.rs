//! Provides conversion traits between `LocationRestriction` and `geo-types` geometry types.
//!
//! This module enables seamless interoperability with the [`geo`](https://crates.io/crates/geo)
//! crate ecosystem.

use crate::places_new::autocomplete::{Error, request::{LatLng, Viewport}};
use geo_types::geometry::{Coord, Polygon, Rect};

// -------------------------------------------------------------------------------------------------
//
// Rect → LocationRestriction conversions

impl TryFrom<Rect> for LocationRestriction {
    type Error = Error;

    /// Converts an owned `geo_types::Rect` into a rectangular location restriction.
    ///
    /// First converts the `Rect` to a `Viewport`, then wraps it in a location restriction.
    ///
    /// Use this when you have geometric bounds from `geo` operations that you want to use for
    /// strictly limiting search results.
    ///
    /// # Errors
    ///
    /// Returns an error if the coordinate conversion fails.
    fn try_from(rect: Rect) -> Result<Self, Self::Error> {
        Ok(Self::Rectangle(Viewport::try_from(rect)?))
    }
}

impl TryFrom<&Rect> for LocationRestriction {
    type Error = Error;

    /// Converts a borrowed `geo_types::Rect` into a rectangular location restriction.
    ///
    /// First converts the `Rect` to a `Viewport`, then wraps it in a location restriction.
    ///
    /// Use this when you have geometric bounds from `geo` operations that you want to use for
    /// strictly limiting search results without consuming the original `Rect`.
    ///
    /// # Errors
    ///
    /// Returns an error if the coordinate conversion fails.
    fn try_from(rect: &Rect) -> Result<Self, Self::Error> {
        Ok(Self::Rectangle(Viewport::try_from(rect)?))
    }
}