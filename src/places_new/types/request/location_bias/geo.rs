//! Provides conversion traits between `LocationBias` and `geo-types` geometry types.
//!
//! This module enables seamless interoperability with the [`geo`](https://crates.io/crates/geo)
//! crate ecosystem.

use crate::GoogleMapsError;
use crate::places_new::{LocationBias, Viewport};
use geo_types::geometry::Rect;

// -------------------------------------------------------------------------------------------------
//
// Rect → LocationBias conversions

impl TryFrom<Rect> for LocationBias {
    type Error = GoogleMapsError;

    /// Converts an owned `geo_types::Rect` into a rectangular location bias.
    ///
    /// First converts the `Rect` to a `Viewport`, then wraps it in a location bias.
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

impl TryFrom<&Rect> for LocationBias {
    type Error = GoogleMapsError;

    /// Converts a borrowed `geo_types::Rect` into a rectangular location bias.
    ///
    /// First converts the `Rect` to a `Viewport`, then wraps it in a location bias.
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