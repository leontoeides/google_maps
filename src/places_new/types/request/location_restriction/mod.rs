//! Contains location restriction type for Google Maps queries.
//!
//! Location restriction allows you to constrain search results to specific geographic areas,
//! improving relevance for location-based queries.

#[cfg(feature = "geo")]
mod geo;

use crate::places_new::{Error, Viewport};
use crate::places_new::types::request::Circle;
use rust_decimal::Decimal;

// -------------------------------------------------------------------------------------------------
//
/// Restricts search results to a specified geographic region.
///
/// Location restriction hard-limits results to only include places within the specified area,
/// completely excluding results outside that area.
///
/// Use this when you need strict geographic boundaries, such as limiting searches to a specific
/// city or service area.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LocationRestriction {
    /// Restrict results to a rectangular viewport.
    ///
    /// Only results within this viewport will be returned.
    ///
    /// Use this when you need to strictly limit results to what's visible in a map view or to a
    /// specific rectangular region.
    Rectangle(Viewport),

    /// Restrict results to a circular area.
    ///
    /// Only results within this circle will be returned.
    ///
    /// Use this for strict radius-based searches where you need to exclude all results beyond a
    /// certain distance from a point.
    Circle(Circle),
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl LocationRestriction {
    /// Creates a rectangular location restriction from a viewport.
    ///
    /// Use this to strictly limit results to a viewport area, excluding all results outside the
    /// viewport.
    #[must_use]
    pub const fn rectangle(viewport: Viewport) -> Self {
        Self::Rectangle(viewport)
    }

    /// Creates a circular location restriction from a circle.
    ///
    /// Use this to strictly limit results to a point and radius, excluding all results outside the
    /// circle.
    #[must_use]
    pub const fn circle(circle: Circle) -> Self {
        Self::Circle(circle)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl From<Viewport> for LocationRestriction {
    /// Converts a `Viewport` into a rectangular location restriction.
    ///
    /// Convenience conversion for when you have a viewport and want to use it as a location
    /// restriction without calling the constructor explicitly.
    fn from(viewport: Viewport) -> Self {
        Self::Rectangle(viewport)
    }
}

impl From<Circle> for LocationRestriction {
    /// Converts a `Circle` into a circular location restriction.
    ///
    /// Convenience conversion for when you have a circle and want to use it as a location
    /// restriction without calling the constructor explicitly.
    fn from(circle: Circle) -> Self {
        Self::Circle(circle)
    }
}

// Circle tuple conversions (3-tuple: lat, lng, radius)

impl TryFrom<(f64, f64, f64)> for LocationRestriction {
    type Error = Error;

    /// Creates a circular location restriction from a tuple of
    /// `(latitude, longitude, radius_meters)`.
    ///
    /// Convenience conversion that creates a circle restriction from raw f64 coordinates and radius
    /// in a single step.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let restriction = LocationRestriction::try_from((37.7749, -122.4194, 5000.0))?;
    /// ```
    fn try_from(tuple: (f64, f64, f64)) -> Result<Self, Self::Error> {
        Ok(Self::Circle(Circle::try_from(tuple)?))
    }
}

impl TryFrom<(f32, f32, f32)> for LocationRestriction {
    type Error = Error;

    /// Creates a circular location restriction from a tuple of
    /// `(latitude, longitude, radius_meters)`.
    ///
    /// Convenience conversion using f32 precision. Useful when working with lower-precision data or
    /// memory-constrained environments.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let restriction = LocationRestriction::try_from((37.7749f32, -122.4194f32, 5000.0f32))?;
    /// ```
    fn try_from(tuple: (f32, f32, f32)) -> Result<Self, Self::Error> {
        Ok(Self::Circle(Circle::try_from(tuple)?))
    }
}

impl TryFrom<(Decimal, Decimal, Decimal)> for LocationRestriction {
    type Error = Error;

    /// Creates a circular location restriction from a tuple of
    /// `(latitude, longitude, radius_meters)`.
    ///
    /// Highest precision conversion using `Decimal` types throughout. Preferred when working
    /// directly with decimal values to avoid floating-point conversions.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let restriction = LocationRestriction::try_from((
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

impl TryFrom<(f64, f64, f64, f64)> for LocationRestriction {
    type Error = Error;

    /// Creates a rectangular location restriction from a tuple of
    /// `(low_lat, low_lng, high_lat, high_lng)`.
    ///
    /// Convenience conversion that creates a viewport restriction from raw f64 coordinates in a
    /// single step.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let restriction = LocationRestriction::try_from((37.0, -122.5, 38.0, -122.0))?;
    /// ```
    fn try_from(tuple: (f64, f64, f64, f64)) -> Result<Self, Self::Error> {
        Ok(Self::Rectangle(Viewport::try_from(tuple)?))
    }
}

impl TryFrom<(f32, f32, f32, f32)> for LocationRestriction {
    type Error = Error;

    /// Creates a rectangular location restriction from a tuple of
    /// `(low_lat, low_lng, high_lat, high_lng)`.
    ///
    /// Convenience conversion using f32 precision. Useful when working with lower-precision data or
    /// memory-constrained environments.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let restriction = LocationRestriction::try_from((37.0f32, -122.5f32, 38.0f32, -122.0f32))?;
    /// ```
    fn try_from(tuple: (f32, f32, f32, f32)) -> Result<Self, Self::Error> {
        Ok(Self::Rectangle(Viewport::try_from(tuple)?))
    }
}

impl TryFrom<(Decimal, Decimal, Decimal, Decimal)> for LocationRestriction {
    type Error = Error;

    /// Creates a rectangular location restriction from a tuple of
    /// `(low_lat, low_lng, high_lat, high_lng)`.
    ///
    /// Highest precision conversion using `Decimal` types throughout. Preferred when working
    /// directly with decimal values to avoid floating-point conversions.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let restriction = LocationRestriction::try_from((
    ///     dec!(37.0), dec!(-122.5),
    ///     dec!(38.0), dec!(-122.0)
    /// ))?;
    /// ```
    fn try_from(tuple: (Decimal, Decimal, Decimal, Decimal)) -> Result<Self, Self::Error> {
        Ok(Self::Rectangle(Viewport::try_from(tuple)?))
    }
}