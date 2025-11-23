//! Contains the `Circle` struct and its associated traits.
//!
//! A circle is defined by a center point and a radius, used to specify geographic search areas and
//! boundaries in Google Maps APIs.

use crate::places_new::{Error, LatLng};
use rust_decimal::{Decimal, prelude::ToPrimitive};
use rust_decimal_macros::dec;

// -------------------------------------------------------------------------------------------------
//
/// Represents a geographic circle defined by a center point and radius.
///
/// Used to specify circular geographic areas for location searches, boundaries, and spatial queries
/// in Google Maps APIs.
///
/// The circle is defined in a 2D geographic coordinate system with the radius measured in meters.
///
/// Valid radius values range from 0.0 to 50,000.0 meters (50 kilometers), and the center must be a
/// valid location on Earth's surface.
#[derive(
    //std
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    // serde
    serde::Deserialize,
    serde::Serialize,
    // getset
    getset::Getters,
    getset::MutGetters,
    getset::Setters,
)]
pub struct Circle {
    /// The geographic center point of the circle.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub center: LatLng,
    
    /// The radius of the circle in meters.
    ///
    /// Valid range: 0.0 to 50,000.0 meters.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub radius: Decimal,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Circle {
    /// Creates a new `Circle` from a center point and radius with validation.
    ///
    /// Validates that the radius falls within the acceptable range of 0.0 to 50,000.0 meters as
    /// specified by the Google Maps API.
    ///
    /// Use this constructor when creating circles for search queries or geographic boundaries.
    ///
    /// # Errors
    ///
    /// Returns an error if the radius is negative or exceeds the maximum allowed value.
    pub fn try_new<C, R>(
        center: C,
        radius: R
    ) -> Result<Self, Error>
    where
        C: TryInto<LatLng>,
        C::Error: Into<Error>,
        R: TryInto<Decimal>,
        R::Error: Into<Error>,
    {
        let center: LatLng = center.try_into().map_err(std::convert::Into::into)?;
        let radius: Decimal = radius.try_into().map_err(std::convert::Into::into)?;

        let result = if radius < dec!(0.0) || radius > dec!(50000.0) {
            let debug = format!("radius: {radius} meters");
            let span = (8, 8 + radius.to_string().len());
            Err(Error::InvalidCircleRadius { radius, debug, span })
        } else {
            Ok(Self { center, radius })
        };

        result
    }

    /// Creates a new `Circle` from a center and radius in meters (f64).
    ///
    /// Convenience constructor that accepts an `f64` radius value and converts it to `Decimal`.
    ///
    /// Use this when working with floating-point distance calculations or when interfacing with
    /// APIs that use `f64` for distances.
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails or if the radius is out of the valid range.
    pub fn try_new_f64(center: LatLng, radius_meters: f64) -> Result<Self, Error> {
        let radius = Decimal::try_from(radius_meters).map_err(|_| {
            let debug = format!("radius: {radius_meters}");
            let span = (8, 8 + radius_meters.to_string().len());
            Error::FloatToDecimalConversionError {
                value: radius_meters.to_string(),
                debug,
                span,
            }
        })?;

        Self::try_new(center, radius)
    }

    /// Creates a new `Circle` from a center and radius in meters (f32).
    ///
    /// Convenience constructor that accepts an `f32` radius value and converts it to `Decimal`.
    ///
    /// Use this when working with lower-precision distance values or when memory efficiency is
    /// important.
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails or if the radius is out of the valid range.
    pub fn try_new_f32(center: LatLng, radius_meters: f32) -> Result<Self, Error> {
        let radius = Decimal::try_from(radius_meters).map_err(|_| {
            let debug = format!("radius: {radius_meters}");
            let span = (8, 8 + radius_meters.to_string().len());
            Error::FloatToDecimalConversionError {
                value: radius_meters.to_string(),
                debug,
                span,
            }
        })?;

        Self::try_new(center, radius)
    }

    /// Formats the circle with human-readable coordinates and radius.
    ///
    /// Uses hemisphere indicators for the center point and includes
    /// appropriate distance units. For example, returns
    /// `"Circle at 37.7749°N 122.4194°W, radius 5.0 km"` for large radii
    /// or `"Circle at 37.7749°N 122.4194°W, radius 500 m"` for smaller ones.
    #[must_use]
    pub fn display(&self) -> String {
        let radius_f64 = self.radius.to_f64().unwrap_or(0.0);

        let (radius_val, radius_unit) = if radius_f64 >= 1000.0 {
            (radius_f64 / 1000.0, "km")
        } else {
            (radius_f64, "m")
        };

        format!(
            "Circle at {}, radius {:.1} {}",
            self.center.display(),
            radius_val,
            radius_unit
        )
    }

    /// Returns the radius as an `f64` value in meters.
    ///
    /// Converts the internal `Decimal` radius to `f64` for use with floating-point calculations or
    /// APIs that expect `f64` distances. Returns `None` if the conversion fails (though this should
    /// be rare for valid radius values).
    pub fn radius_f64(&self) -> Result<f64, Error> {
        Ok(self.radius.try_into()?)
    }

    /// Returns the radius as an `f32` value in meters.
    ///
    /// Converts the internal `Decimal` radius to `f32` for use with lower-precision calculations or
    /// memory-constrained contexts. Returns `None` if the conversion fails.
    pub fn radius_f32(&self) -> Result<f32, Error> {
        Ok(self.radius.try_into()?)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl TryFrom<(f64, f64, f64)> for Circle {
    type Error = Error;

    /// Creates a circle from a tuple of `(latitude, longitude, radius_meters)`.
    ///
    /// Convenience conversion that allows creating a circle from raw f64 coordinates and radius
    /// without explicitly constructing a `LatLng` first.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let circle = Circle::try_from((37.7749, -122.4194, 5000.0))?;
    /// ```
    fn try_from((lat, lng, radius): (f64, f64, f64)) -> Result<Self, Self::Error> {
        let center = LatLng::try_from((lat, lng))?;
        Self::try_new_f64(center, radius)
    }
}

impl TryFrom<(f32, f32, f32)> for Circle {
    type Error = Error;

    /// Creates a circle from a tuple of `(latitude, longitude, radius_meters)`.
    ///
    /// Convenience conversion using f32 precision. Useful when working with lower-precision data or
    /// memory-constrained environments.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let circle = Circle::try_from((37.7749f32, -122.4194f32, 5000.0f32))?;
    /// ```
    fn try_from((lat, lng, radius): (f32, f32, f32)) -> Result<Self, Self::Error> {
        let center = LatLng::try_from((lat, lng))?;
        Self::try_new_f32(center, radius)
    }
}

impl TryFrom<(Decimal, Decimal, Decimal)> for Circle {
    type Error = Error;

    /// Creates a circle from a tuple of `(latitude, longitude, radius_meters)`.
    ///
    /// Highest precision conversion using `Decimal` types throughout. Preferred when working
    /// directly with decimal values to avoid floating-point conversions.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let circle = Circle::try_from((dec!(37.7749), dec!(-122.4194), dec!(5000.0)))?;
    /// ```
    fn try_from((lat, lng, radius): (Decimal, Decimal, Decimal)) -> Result<Self, Self::Error> {
        let center = LatLng::try_from_dec(lat, lng)?;
        Self::try_new(center, radius)
    }
}

impl std::fmt::Display for Circle {
    /// Formats the circle as "center: <lat>,<lng>, radius: <meters>m".
    ///
    /// Provides a human-readable representation suitable for logging and debugging. The center is
    /// formatted as comma-delimited coordinates and the radius is shown with a meter unit suffix.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "center: {}, radius: {}m",
            self.center,
            self.radius.normalize()
        )
    }
}

impl std::default::Default for Circle {
    /// Returns a default circle centered at Null Island with zero radius.
    ///
    /// Creates a circle at (0°N, 0°E) with a radius of 0 meters. This default is useful for
    /// initialization but typically shouldn't represent real-world search areas. Consider using
    /// `try_new` with meaningful values for actual use cases.
    fn default() -> Self {
        Self {
            center: LatLng::default(),
            radius: dec!(0.0),
        }
    }
}