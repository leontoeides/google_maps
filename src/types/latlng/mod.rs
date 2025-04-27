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

/// Latitude and longitude values must correspond to a valid location on the
/// face of the earth. Latitudes can take any value between -90 and 90 while
/// longitude values can take any value between -180 and 180. If you specify an
/// invalid latitude or longitude value, your request will be rejected as a bad
/// request.

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LatLng {
    /// Latitude. A value between -90.0° and 90.0°.
    #[serde(alias = "y")]
    #[serde(alias = "lat")]
    #[serde(alias = "latitude")]
    #[serde(with = "rust_decimal::serde::float")]
    pub lat: Decimal,
    /// Longitude. A value between -180.0° and 180.0°.
    #[serde(alias = "x")]
    #[serde(alias = "lon")]
    #[serde(alias = "long")]
    #[serde(alias = "longitude")]
    #[serde(with = "rust_decimal::serde::float")]
    pub lng: Decimal,
} // struct

// -----------------------------------------------------------------------------

impl LatLng {
    /// Takes individual latitude & longitude `Decimal` coordinates and
    /// converts them into a `LatLng` structure. If either the latitude
    /// (-90.0 to +90.0) or longitude (-180.0 to +180.0) are out of range, this
    /// function will return an error.
    pub fn try_from_dec(latitude: Decimal, longitude: Decimal) -> Result<Self, GoogleMapsError> {
        if latitude < dec!(-90.0) || latitude > dec!(90.0) {
            Err(TypeError::InvalidLatitude(latitude, longitude))?;
        } // if

        if longitude < dec!(-180.0) || longitude > dec!(180.0) {
            Err(TypeError::InvalidLongitude(latitude, longitude))?;
        } // if

        Ok(Self {
            lat: latitude,
            lng: longitude,
        })
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl LatLng {
    /// Takes individual latitude & longitude `f32` coordinates and
    /// converts them into a `LatLng` structure. If either the latitude
    /// (-90.0 to +90.0) or longitude (-180.0 to +180.0) are out of range, this
    /// function will return an error.
    pub fn try_from_f32(latitude: f32, longitude: f32) -> Result<Self, GoogleMapsError> {
        let lat: Decimal = Decimal::from_f32(latitude)
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(latitude.to_string()))?;

        let lng: Decimal = Decimal::from_f32(longitude)
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(longitude.to_string()))?;

        if lat < dec!(-90.0) || lat > dec!(90.0) {
            Err(TypeError::InvalidLatitude(lat, lng))?;
        } // if

        if lng < dec!(-180.0) || lng > dec!(180.0) {
            Err(TypeError::InvalidLongitude(lat, lng))?;
        } // if

        Ok(Self { lat, lng })
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl LatLng {
    /// Takes individual latitude & longitude `f64` coordinates and
    /// converts them into a `LatLng` structure. If either the latitude
    /// (-90.0 to +90.0) or longitude (-180.0 to +180.0) are out of range, this
    /// function will return an error.
    pub fn try_from_f64(latitude: f64, longitude: f64) -> Result<Self, GoogleMapsError> {
        let lat: Decimal = Decimal::from_f64(latitude)
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(latitude.to_string()))?;

        let lng: Decimal = Decimal::from_f64(longitude)
            .ok_or_else(|| TypeError::FloatToDecimalConversionError(longitude.to_string()))?;

        if lat < dec!(-90.0) || lat > dec!(90.0) {
            Err(TypeError::InvalidLatitude(lat, lng))?;
        } // if

        if lng < dec!(-180.0) || lng > dec!(180.0) {
            Err(TypeError::InvalidLongitude(lat, lng))?;
        } // if

        Ok(Self { lat, lng })
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for LatLng {
    // Error definitions are contained in the
    // `google_maps\src\geocoding\error.rs` module.
    type Err = GoogleMapsError;

    /// Attempts to get a `LatLng` struct from a borrowed `&str` that contains a
    /// comma-delimited latitude & longitude pair.
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<&str> = str.trim().split(',').collect();

        if coordinates.len() == 2 {
            let lat = Decimal::from_str(coordinates[0].trim());
            let lat = lat.map_err(|_| TypeError::InvalidLatLongString(str.to_owned()))?;
            let lon = Decimal::from_str(coordinates[1].trim());
            let lon = lon.map_err(|_| TypeError::InvalidLatLongString(str.to_owned()))?;
            Self::try_from_dec(lat, lon)
        } else {
            Err(TypeError::InvalidLatLongString(str.to_owned()))?
        } // if
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&str> for LatLng {
    type Error = GoogleMapsError;
    /// Attempts to get a `LatLng` struct from a borrowed `&str` that contains a
    /// comma-delimited latitude & longitude pair.
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        str.parse()
    }
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&String> for LatLng {
    type Error = GoogleMapsError;
    /// Attempts to get a `LatLng` struct from a borrowed `&String` that
    /// contains a comma-delimited latitude & longitude pair.
    fn try_from(string: &String) -> Result<Self, Self::Error> {
        string.parse()
    }
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<String> for LatLng {
    type Error = GoogleMapsError;
    /// Attempts to get a `LatLng` struct from an owned `String` that contains a
    /// comma-delimited latitude & longitude pair.
    fn try_from(string: String) -> Result<Self, Self::Error> {
        string.parse()
    }
} // impl

// -----------------------------------------------------------------------------

impl<V: TryInto<Decimal>> TryFrom<(V, V)> for LatLng {
    type Error = GoogleMapsError;
    /// Attempts to get a `LatLng` struct from a `(lat, lng)` tuple that
    /// contains a `0` latitude and a `1` longitude (in that order).
    fn try_from(coordinates: (V, V)) -> Result<Self, Self::Error> {
        Self::try_from_dec(
            coordinates.0.try_into().map_err(|_| TypeError::InvalidLatLongTuple)?,
            coordinates.1.try_into().map_err(|_| TypeError::InvalidLatLongTuple)?
        )
    }
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Self> for LatLng {
    /// Converts a borrowed `&LatLng` enum into an owned `LatLng` enum by
    /// copying it.
    fn from(lat_lng: &Self) -> Self {
        *lat_lng
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&LatLng> for String {
    /// Converts a borrowed `&LatLng` struct to a `String` that contains a
    /// latitude/longitude pair.
    fn from(lat_lng: &LatLng) -> Self {
        format!(
            "{latitude},{longitude}",
            latitude = lat_lng.lat.normalize(),
            longitude = lat_lng.lng.normalize(),
        ) // format!
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<LatLng> for String {
    /// Converts an owned `LatLng` struct to a `String` that contains a
    /// latitude/longitude pair.
    fn from(lat_lng: LatLng) -> Self {
        Self::from(&lat_lng)
    }
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for LatLng {
    /// Converts a `LatLng` struct to a string that contains a
    /// latitude/longitude pair.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for LatLng {
    /// Returns a reasonable default value for the `LatLng` struct.
    fn default() -> Self {
        Self {
            lat: dec!(0.0),
            lng: dec!(0.0),
        }
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl LatLng {
    /// Formats a `LatLng` struct into a string that is presentable to the end
    /// user.
    #[must_use]
    pub fn display(&self) -> String {
        // Display latitude and longitude as decimal degrees with some extra
        // fixins'.
        format!(
            "{lat}°{lat_hem} {lng}°{lng_hem}",
            lat = self.lat.abs().normalize(),
            lat_hem = match self.lat.cmp(&dec!(0.0)) {
                Ordering::Less => " S".to_string(),
                Ordering::Greater => " N".to_string(),
                Ordering::Equal => String::new(),
            }, // match
            lng = self.lng.abs().normalize(),
            lng_hem = match self.lng.cmp(&dec!(0.0)) {
                Ordering::Less => " W".to_string(),
                Ordering::Greater => " E".to_string(),
                Ordering::Equal => String::new(),
            }, // match
        ) // write!
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl LatLng {
    /// Returns the north-south latitudinal (or vertical) coordinate.
    #[must_use]
    pub const fn y(&self) -> &Decimal {
        &self.lat
    }
    /// Returns the north-south latitudinal (or vertical) coordinate.
    #[must_use]
    pub const fn lat(&self) -> &Decimal {
        &self.lat
    }
    /// Returns the north-south latitudinal (or vertical) coordinate.
    #[must_use]
    pub const fn latitude(&self) -> &Decimal {
        &self.lat
    }

    /// Returns the east-west longitudinal (or horizontal) coordinate.
    #[must_use]
    pub const fn x(&self) -> &Decimal {
        &self.lng
    }
    /// Returns the east-west longitudinal (or horizontal) coordinate.
    #[must_use]
    pub const fn lng(&self) -> &Decimal {
        &self.lng
    }
    /// Returns the east-west longitudinal (or horizontal) coordinate.
    #[must_use]
    pub const fn lon(&self) -> &Decimal {
        &self.lng
    }
    /// Returns the east-west longitudinal (or horizontal) coordinate.
    #[must_use]
    pub const fn long(&self) -> &Decimal {
        &self.lng
    }
    /// Returns the east-west longitudinal (or horizontal) coordinate.
    #[must_use]
    pub const fn longitude(&self) -> &Decimal {
        &self.lng
    }

    /// Returns a tuple containing 1. the latitude (y) coordinate, and then 2.
    /// the longitude (x) coordinate, in that order.
    #[must_use]
    pub const fn coords(&self) -> (&Decimal, &Decimal) {
        (&self.lat, &self.lng)
    }
    /// Returns a tuple containing 1. the latitude (y) coordinate, and then 2.
    /// the longitude (x) coordinate, in that order.
    #[must_use]
    pub const fn coordinates(&self) -> (&Decimal, &Decimal) {
        (&self.lat, &self.lng)
    }
} // impl
