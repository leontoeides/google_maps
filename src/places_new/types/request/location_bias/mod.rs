//! Contains location biasing type for Google Maps queries.
//!
//! Location bias allows you to influence search results to specific geographic areas, improving
//! relevance for location-based queries.

#[cfg(feature = "geo")]
mod geo;

use crate::places_new::types::request::Circle;
use crate::places_new::{Error, Viewport};
use rust_decimal::Decimal;

// -------------------------------------------------------------------------------------------------
//
/// Biases search results toward a specified geographic region.
///
/// Location bias influences the ranking of results to favor places within or near the specified
/// area, but does not exclude results outside that area.
///
/// Use this when you want to prioritize local results while still allowing relevant distant results
/// to appear.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LocationBias {
    /// Bias results toward a rectangular viewport.
    ///
    /// Results within or near this viewport will be ranked higher, but results outside the viewport
    /// may still be returned if they are relevant.
    ///
    /// Use this when you want to favor results visible in a map view.
    Rectangle(Viewport),

    /// Bias results toward a circular area.
    ///
    /// Results within or near this circle will be ranked higher, but results outside the circle may
    /// still be returned if they are relevant.
    ///
    /// Use this for radius-based searches where you want to favor results near a point without
    /// strictly limiting the search area.
    Circle(Circle),
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl LocationBias {
    /// Creates a rectangular location bias from a viewport.
    ///
    /// Use this to bias results toward a map viewport area without excluding results outside the
    /// viewport.
    #[must_use]
    pub fn rectangle(viewport: impl Into<Viewport>) -> Self {
        Self::Rectangle(viewport.into())
    }

    /// Creates a circular location bias from a circle.
    ///
    /// Use this to bias results toward a point and radius without excluding results outside the
    /// circle.
    #[must_use]
    pub fn circle(circle: impl Into<Circle>) -> Self {
        Self::Circle(circle.into())
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl From<Viewport> for LocationBias {
    /// Converts a `Viewport` into a rectangular location bias.
    ///
    /// Convenience conversion for when you have a viewport and want to use it as a location bias
    /// without calling the constructor explicitly.
    fn from(viewport: Viewport) -> Self {
        Self::Rectangle(viewport)
    }
}

impl From<Circle> for LocationBias {
    /// Converts a `Circle` into a circular location bias.
    ///
    /// Convenience conversion for when you have a circle and want to use it as a location bias
    /// without calling the constructor explicitly.
    fn from(circle: Circle) -> Self {
        Self::Circle(circle)
    }
}

// Circle tuple conversions (3-tuple: lat, lng, radius)

impl TryFrom<(f64, f64, f64)> for LocationBias {
    type Error = Error;

    /// Creates a circular location bias from a tuple of `(latitude, longitude, radius_meters)`.
    ///
    /// Convenience conversion that creates a circle bias from raw f64 coordinates and radius in a
    /// single step.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let bias = LocationBias::try_from((37.7749, -122.4194, 5000.0))?;
    /// ```
    fn try_from(tuple: (f64, f64, f64)) -> Result<Self, Self::Error> {
        Ok(Self::Circle(Circle::try_from(tuple)?))
    }
}

impl TryFrom<(f32, f32, f32)> for LocationBias {
    type Error = Error;

    /// Creates a circular location bias from a tuple of `(latitude, longitude, radius_meters)`.
    ///
    /// Convenience conversion using f32 precision. Useful when working with lower-precision data or
    /// memory-constrained environments.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let bias = LocationBias::try_from((37.7749f32, -122.4194f32, 5000.0f32))?;
    /// ```
    fn try_from(tuple: (f32, f32, f32)) -> Result<Self, Self::Error> {
        Ok(Self::Circle(Circle::try_from(tuple)?))
    }
}

impl TryFrom<(Decimal, Decimal, Decimal)> for LocationBias {
    type Error = Error;

    /// Creates a circular location bias from a tuple of `(latitude, longitude, radius_meters)`.
    ///
    /// Highest precision conversion using `Decimal` types throughout. Preferred when working
    /// directly with decimal values to avoid floating-point conversions.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let bias = LocationBias::try_from((
    ///     dec!(37.7749),
    ///     dec!(-122.4194),
    ///     dec!(5000.0)
    /// ))?;
    /// ```
    fn try_from(tuple: (Decimal, Decimal, Decimal)) -> Result<Self, Self::Error> {
        Ok(Self::Circle(Circle::try_from(tuple)?))
    }
}

// Viewport tuple conversions (4-tuple: low_lat, low_lng, high_lat, high_lng)

impl TryFrom<(f64, f64, f64, f64)> for LocationBias {
    type Error = Error;

    /// Creates a rectangular location bias from a tuple of
    /// `(low_lat, low_lng, high_lat, high_lng)`.
    ///
    /// Convenience conversion that creates a viewport bias from raw f64 coordinates in a single
    /// step.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let bias = LocationBias::try_from((37.0, -122.5, 38.0, -122.0))?;
    /// ```
    fn try_from(tuple: (f64, f64, f64, f64)) -> Result<Self, Self::Error> {
        Ok(Self::Rectangle(Viewport::try_from(tuple)?))
    }
}

impl TryFrom<(f32, f32, f32, f32)> for LocationBias {
    type Error = Error;

    /// Creates a rectangular location bias from a tuple of
    /// `(low_lat, low_lng, high_lat, high_lng)`.
    ///
    /// Convenience conversion using f32 precision. Useful when working with lower-precision data or
    /// memory-constrained environments.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let bias = LocationBias::try_from((37.0f32, -122.5f32, 38.0f32, -122.0f32))?;
    /// ```
    fn try_from(tuple: (f32, f32, f32, f32)) -> Result<Self, Self::Error> {
        Ok(Self::Rectangle(Viewport::try_from(tuple)?))
    }
}

impl TryFrom<(Decimal, Decimal, Decimal, Decimal)> for LocationBias {
    type Error = Error;

    /// Creates a rectangular location bias from a tuple of
    /// `(low_lat, low_lng, high_lat, high_lng)`.
    ///
    /// Highest precision conversion using `Decimal` types throughout. Preferred when working
    /// directly with decimal values to avoid floating-point conversions.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let bias = LocationBias::try_from((
    ///     dec!(37.0), dec!(-122.5),
    ///     dec!(38.0), dec!(-122.0)
    /// ))?;
    /// ```
    fn try_from(tuple: (Decimal, Decimal, Decimal, Decimal)) -> Result<Self, Self::Error> {
        Ok(Self::Rectangle(Viewport::try_from(tuple)?))
    }
}
