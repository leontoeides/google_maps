//! Contains the `LatLng` struct, its associated traits, and functions. The
//! latitude & longitude coorindate system is used to specify a position or
//! location on the Earth's surface.

#[cfg(feature = "geo")]
mod geo;

// -----------------------------------------------------------------------------

use crate::error::Error as GoogleMapsError;
use crate::types::error::Error as TypeError;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering};

// -----------------------------------------------------------------------------
//
/// Represents a geographic coordinate on Earth's surface.
///
/// Stores latitude (north-south position) and longitude (east-west position) using `Decimal` types
/// for precise coordinate representation.
///
/// This struct is used throughout the Google Maps API to specify locations, waypoints, and
/// geographic bounds. Coordinates are validated to ensure they fall within valid ranges: latitude
/// must be between -90° and 90°, longitude between -180° and 180°.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LatLng {
    /// Latitude coordinate in decimal degrees.
    ///
    /// Valid range: -90.0° (South Pole) to 90.0° (North Pole).
    #[serde(alias = "y")]
    #[serde(alias = "lat")]
    #[serde(alias = "latitude")]
    #[serde(with = "rust_decimal::serde::float")]
    pub lat: Decimal,

    /// Longitude coordinate in decimal degrees.
    ///
    /// Valid range: -180.0° (western hemisphere) to 180.0° (eastern hemisphere).
    #[serde(alias = "x")]
    #[serde(alias = "lon")]
    #[serde(alias = "long")]
    #[serde(alias = "longitude")]
    #[serde(with = "rust_decimal::serde::float")]
    pub lng: Decimal,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl LatLng {
    /// Creates a `LatLng` from `Decimal` coordinates with validation.
    ///
    /// This is the canonical constructor for creating validated coordinates from `Decimal` values.
    ///
    /// Use this when you already have `Decimal` values, typically from parsing or calculations.
    ///
    /// # Errors
    ///
    /// Returns an error if latitude is outside [-90, 90] or longitude is outside [-180, 180].
    pub fn try_from_dec(latitude: Decimal, longitude: Decimal) -> Result<Self, GoogleMapsError> {
        let lat_valid = latitude >= dec!(-90.0) && latitude <= dec!(90.0);
        let lng_valid = longitude >= dec!(-180.0) && longitude <= dec!(180.0);

        if !lat_valid {
            Err(TypeError::InvalidLatitude(latitude, longitude).into())
        } else if !lng_valid {
            Err(TypeError::InvalidLongitude(latitude, longitude).into())
        } else {
            Ok(Self {
                lat: latitude,
                lng: longitude,
            })
        }
    }

    /// Creates a `LatLng` from `f32` coordinates with validation.
    ///
    /// Converts 32-bit floating point coordinates to `Decimal` representation and validates the
    /// ranges.
    ///
    /// # Errors
    ///
    /// Returns an error if latitude is outside [-90, 90] or longitude is outside [-180, 180].
    pub fn try_from_f32(latitude: f32, longitude: f32) -> Result<Self, GoogleMapsError> {
        let lat = Decimal::from_f32(latitude)
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(latitude.to_string()))?;

        let lng = Decimal::from_f32(longitude)
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(longitude.to_string()))?;

        Self::try_from_dec(lat, lng)
    }

    /// Creates a `LatLng` from `f64` coordinates with validation.
    ///
    /// Converts 64-bit floating point coordinates to `Decimal` representation and validates the
    /// ranges.
    ///
    /// # Errors
    ///
    /// Returns an error if latitude is outside [-90, 90] or longitude is outside [-180, 180].
    pub fn try_from_f64(latitude: f64, longitude: f64) -> Result<Self, GoogleMapsError> {
        let lat = Decimal::from_f64(latitude)
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(latitude.to_string()))?;

        let lng = Decimal::from_f64(longitude)
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(longitude.to_string()))?;

        Self::try_from_dec(lat, lng)
    }

    /// Formats coordinates as a human-readable string with hemisphere indicators.
    ///
    /// Converts to an absolute value format with N/S/E/W indicators, making coordinates easier to
    /// read and understand.
    ///
    /// For example, returns `37.7749° N 122.4194° W` instead of `37.7749,-122.4194`. Use this
    /// when displaying coordinates to end users in UI elements.
    #[must_use]
    pub fn display(&self) -> String {
        format!(
            "{lat}°{lat_hem} {lng}°{lng_hem}",
            lat = self.lat.abs().normalize(),
            lat_hem = match self.lat.cmp(&dec!(0.0)) {
                Ordering::Less => " S",
                Ordering::Greater => " N",
                Ordering::Equal => "",
            },
            lng = self.lng.abs().normalize(),
            lng_hem = match self.lng.cmp(&dec!(0.0)) {
                Ordering::Less => " W",
                Ordering::Greater => " E",
                Ordering::Equal => "",
            },
        )
    }

    /// Returns a reference to the latitude coordinate.
    ///
    /// Alias for vertical/y-axis geographic position. Prefer using this method name when working
    /// with Cartesian-style APIs.
    #[must_use]
    pub const fn y(&self) -> &Decimal {
        &self.lat
    }

    /// Returns a reference to the latitude coordinate.
    ///
    /// Short-form accessor for the north-south position.
    #[must_use]
    pub const fn lat(&self) -> &Decimal {
        &self.lat
    }

    /// Returns a reference to the latitude coordinate.
    ///
    /// Full-name accessor for the north-south position. Use this for clarity in contexts where
    /// abbreviated names might be ambiguous.
    #[must_use]
    pub const fn latitude(&self) -> &Decimal {
        &self.lat
    }

    /// Returns a reference to the longitude coordinate.
    ///
    /// Alias for horizontal/x-axis geographic position. Prefer using this method name when working
    /// with Cartesian-style APIs.
    #[must_use]
    pub const fn x(&self) -> &Decimal {
        &self.lng
    }

    /// Returns a reference to the longitude coordinate.
    ///
    /// Short-form accessor for the east-west position.
    #[must_use]
    pub const fn lng(&self) -> &Decimal {
        &self.lng
    }

    /// Returns a reference to the longitude coordinate.
    ///
    /// Alternative short-form accessor for the east-west position.
    #[must_use]
    pub const fn lon(&self) -> &Decimal {
        &self.lng
    }

    /// Returns a reference to the longitude coordinate.
    ///
    /// Alternative accessor for the east-west position.
    #[must_use]
    pub const fn long(&self) -> &Decimal {
        &self.lng
    }

    /// Returns a reference to the longitude coordinate.
    ///
    /// Full-name accessor for the east-west position. Use this for clarity in contexts where
    /// abbreviated names might be ambiguous.
    #[must_use]
    pub const fn longitude(&self) -> &Decimal {
        &self.lng
    }

    /// Returns both coordinates as a tuple `(latitude, longitude)`.
    ///
    /// Short-form accessor for both coordinates. Use this when you need to work with both values
    /// simultaneously or when destructuring.
    #[must_use]
    pub const fn coords(&self) -> (&Decimal, &Decimal) {
        (&self.lat, &self.lng)
    }

    /// Returns both coordinates as a tuple `(latitude, longitude)`.
    ///
    /// Full-name accessor for both coordinates. Prefer this for clarity in contexts where
    /// abbreviated names might be ambiguous.
    #[must_use]
    pub const fn coordinates(&self) -> (&Decimal, &Decimal) {
        (&self.lat, &self.lng)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::str::FromStr for LatLng {
    type Err = GoogleMapsError;

    /// Parses a comma-delimited string like "37.7749,-122.4194" into a `LatLng`.
    ///
    /// The expected format is "latitude,longitude" with whitespace allowed.
    ///
    /// This is commonly used when parsing coordinates from configuration files, URLs, or user
    /// input.
    ///
    /// # Errors
    ///
    /// Returns an error if the string format is invalid or coordinates are out of range.
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<&str> = str.trim().split(',').collect();

        let result = if coordinates.len() == 2 {
            let lat = Decimal::from_str(coordinates[0].trim());
            let lat = lat.map_err(|_| TypeError::InvalidLatLongString(str.to_owned()))?;
            let lon = Decimal::from_str(coordinates[1].trim());
            let lon = lon.map_err(|_| TypeError::InvalidLatLongString(str.to_owned()))?;
            Self::try_from_dec(lat, lon)
        } else {
            Err(TypeError::InvalidLatLongString(str.to_owned()).into())
        };

        result
    }
}

impl TryFrom<&str> for LatLng {
    type Error = GoogleMapsError;

    /// Converts a borrowed string slice to a `LatLng`.
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.parse()
    }
}


impl TryFrom<&String> for LatLng {
    type Error = GoogleMapsError;

    /// Converts a borrowed `String` reference to a `LatLng`.
    fn try_from(string: &String) -> Result<Self, Self::Error> {
        string.parse()
    }
}


impl TryFrom<String> for LatLng {
    type Error = GoogleMapsError;

    /// Converts an owned `String` to a `LatLng`.
    fn try_from(string: String) -> Result<Self, Self::Error> {
        string.parse()
    }
}

impl<V: TryInto<Decimal>> TryFrom<(V, V)> for LatLng {
    type Error = GoogleMapsError;

    /// Converts a tuple of `(latitude, longitude)` to a `LatLng`.
    ///
    /// Accepts any tuple where both elements can convert to `Decimal`.
    ///
    /// This is useful when working with numeric tuples from calculations or when destructuring
    /// coordinate pairs. The order is latitude first, longitude second.
    fn try_from(coordinates: (V, V)) -> Result<Self, Self::Error> {
        let lat = coordinates.0
            .try_into()
            .map_err(|_| TypeError::InvalidLatLongTuple)?;

        let lng = coordinates.1
            .try_into()
            .map_err(|_| TypeError::InvalidLatLongTuple)?;

        Self::try_from_dec(lat, lng)
    }
}

impl std::convert::From<&Self> for LatLng {
    /// Copies a borrowed `LatLng` reference to an owned value.
    ///
    /// Since `LatLng` is `Copy`, this simply dereferences the pointer. This trait impl enables
    /// seamless conversion in generic contexts.
    fn from(lat_lng: &Self) -> Self {
        *lat_lng
    }
}

impl std::convert::From<&LatLng> for String {
    /// Converts a `LatLng` to a comma-delimited string representation.
    ///
    /// Formats as "latitude,longitude" with normalized decimal values.
    ////
    /// This format is suitable for Google Maps API queries and for serializing coordinates to
    /// configuration files or databases.
    fn from(lat_lng: &LatLng) -> Self {
        format!(
            "{latitude},{longitude}",
            latitude = lat_lng.lat.normalize(),
            longitude = lat_lng.lng.normalize(),
        )
    }
}

impl std::convert::From<LatLng> for String {
    /// Converts an owned `LatLng` to a comma-delimited string.
    fn from(lat_lng: LatLng) -> Self {
        Self::from(&lat_lng)
    }
}

impl std::fmt::Display for LatLng {
    /// Formats a `LatLng` for display using comma-delimited coordinates.
    ///
    /// Enables using `LatLng` with format macros and string conversion.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl std::default::Default for LatLng {
    /// Returns coordinates for [Null Island](https://en.wikipedia.org/wiki/Null_Island) (0°N, 0°E).
    ///
    /// Null Island is the point where the prime meridian crosses the equator in the Gulf of Guinea.
    fn default() -> Self {
        Self {
            lat: dec!(0.0),
            lng: dec!(0.0),
        }
    }
}