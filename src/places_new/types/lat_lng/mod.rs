//! Contains the `LatLng` struct, its associated traits, and functions. The latitude & longitude
//! coorindate system is used to specify a position or location on the Earth's surface.

#[cfg(feature = "geo")]
mod geo;

use crate::places_new::Error;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use rust_decimal_macros::dec;
use std::cmp::{Ord, Ordering};

// -------------------------------------------------------------------------------------------------
//
/// Represents a geographic coordinate on Earth's surface.
///
/// Stores latitude (north-south position) and longitude (east-west position) using `Decimal` types
/// for precise coordinate representation.
///
/// This struct is used throughout the Google Maps API to specify locations, waypoints, and
/// geographic bounds. Coordinates are validated to ensure they fall within valid ranges: latitude
/// must be between -90° and 90°, longitude between -180° and 180°.
#[derive(
    //std
    Clone,
    Copy,
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
pub struct LatLng {
    /// Latitude coordinate in decimal degrees.
    ///
    /// Valid range: -90.0° (South Pole) to 90.0° (North Pole).
    #[serde(alias = "y")]
    #[serde(alias = "lat")]
    #[serde(with = "rust_decimal::serde::float")]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub latitude: Decimal,

    /// Longitude coordinate in decimal degrees.
    ///
    /// Valid range: -180.0° (western hemisphere) to 180.0° (eastern hemisphere).
    #[serde(alias = "x")]
    #[serde(alias = "lon")]
    #[serde(alias = "long")]
    #[serde(with = "rust_decimal::serde::float")]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub longitude: Decimal,
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
    pub fn try_from_dec(latitude: Decimal, longitude: Decimal) -> Result<Self, Error> {
        let lat_valid = latitude >= dec!(-90.0) && latitude <= dec!(90.0);
        let lng_valid = longitude >= dec!(-180.0) && longitude <= dec!(180.0);

        if !lat_valid {
            Err(Error::from_invalid_latitude(latitude, longitude))
        } else if !lng_valid {
            Err(Error::from_invalid_longitude(latitude, longitude))
        } else {
            Ok(Self { latitude, longitude })
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
    pub fn try_from_f32(latitude: f32, longitude: f32) -> Result<Self, Error> {
        let lat = Decimal::from_f32(latitude)
            .ok_or_else(|| Error::from_float_conversion(latitude, "latitude"))?;

        let lng = Decimal::from_f32(longitude)
            .ok_or_else(|| Error::from_float_conversion(longitude, "longitude"))?;

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
    pub fn try_from_f64(latitude: f64, longitude: f64) -> Result<Self, Error> {
        let lat = Decimal::from_f64(latitude)
            .ok_or_else(|| Error::from_float_conversion(latitude, "latitude"))?;

        let lng = Decimal::from_f64(longitude)
            .ok_or_else(|| Error::from_float_conversion(longitude, "longitude"))?;

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
            lat = self.latitude.abs().normalize(),
            lat_hem = match self.latitude.cmp(&dec!(0.0)) {
                Ordering::Less => "S",
                Ordering::Greater => "N",
                Ordering::Equal => "",
            },
            lng = self.longitude.abs().normalize(),
            lng_hem = match self.longitude.cmp(&dec!(0.0)) {
                Ordering::Less => "W",
                Ordering::Greater => "E",  // Fixed: positive = East!
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
        &self.latitude
    }

    /// Returns a reference to the latitude coordinate.
    ///
    /// Short-form accessor for the north-south position.
    #[must_use]
    pub const fn lat(&self) -> &Decimal {
        &self.latitude
    }

    /// Returns a reference to the longitude coordinate.
    ///
    /// Alias for horizontal/x-axis geographic position. Prefer using this method name when working
    /// with Cartesian-style APIs.
    #[must_use]
    pub const fn x(&self) -> &Decimal {
        &self.longitude
    }

    /// Returns a reference to the longitude coordinate.
    ///
    /// Short-form accessor for the east-west position.
    #[must_use]
    pub const fn lng(&self) -> &Decimal {
        &self.longitude
    }

    /// Returns a reference to the longitude coordinate.
    ///
    /// Alternative short-form accessor for the east-west position.
    #[must_use]
    pub const fn lon(&self) -> &Decimal {
        &self.longitude
    }

    /// Returns a reference to the longitude coordinate.
    ///
    /// Alternative accessor for the east-west position.
    #[must_use]
    pub const fn long(&self) -> &Decimal {
        &self.longitude
    }

    /// Returns both coordinates as a tuple `(latitude, longitude)`.
    ///
    /// Short-form accessor for both coordinates. Use this when you need to work with both values
    /// simultaneously or when destructuring.
    #[must_use]
    pub const fn coords(&self) -> (&Decimal, &Decimal) {
        (&self.latitude, &self.longitude)
    }

    /// Returns both coordinates as a tuple `(latitude, longitude)`.
    ///
    /// Full-name accessor for both coordinates. Prefer this for clarity in contexts where
    /// abbreviated names might be ambiguous.
    #[must_use]
    pub const fn coordinates(&self) -> (&Decimal, &Decimal) {
        (&self.latitude, &self.longitude)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::str::FromStr for LatLng {
    type Err = Error;

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
            let lat = Decimal::from_str(coordinates[0].trim())
                .map_err(|_| Error::from_invalid_latlng_string(str.to_owned()))?;
            let lon = Decimal::from_str(coordinates[1].trim())
                .map_err(|_| Error::from_invalid_latlng_string(str.to_owned()))?;
            Self::try_from_dec(lat, lon)
        } else {
            Err(Error::from_invalid_latlng_string(str.to_owned()))
        };

        result
    }
}

impl TryFrom<&str> for LatLng {
    type Error = Error;

    /// Converts a borrowed string slice to a `LatLng`.
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.parse()
    }
}


impl TryFrom<&String> for LatLng {
    type Error = Error;

    /// Converts a borrowed `String` reference to a `LatLng`.
    fn try_from(string: &String) -> Result<Self, Self::Error> {
        string.parse()
    }
}


impl TryFrom<String> for LatLng {
    type Error = Error;

    /// Converts an owned `String` to a `LatLng`.
    fn try_from(string: String) -> Result<Self, Self::Error> {
        string.parse()
    }
}

impl<V: TryInto<Decimal>> TryFrom<(V, V)> for LatLng {
    type Error = Error;

    /// Converts a tuple of `(latitude, longitude)` to a `LatLng`.
    ///
    /// Accepts any tuple where both elements can convert to `Decimal`.
    ///
    /// This is useful when working with numeric tuples from calculations or when destructuring
    /// coordinate pairs. The order is latitude first, longitude second.
    fn try_from(coordinates: (V, V)) -> Result<Self, Self::Error> {
        let lat = coordinates.0
            .try_into()
            .map_err(|_| Error::InvalidLatLongTuple)?;

        let lng = coordinates.1
            .try_into()
            .map_err(|_| Error::InvalidLatLongTuple)?;

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
            latitude = lat_lng.latitude.normalize(),
            longitude = lat_lng.longitude.normalize(),
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
            latitude: dec!(0.0),
            longitude: dec!(0.0),
        }
    }
}