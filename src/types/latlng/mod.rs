//! Contains the `LatLng` struct, its associated traits, and functions. The
//! latitude & longitude coorindate system is used to specify a position or
//! location on the Earth's surface.

#[cfg(feature = "geo")]
mod geo;

// -----------------------------------------------------------------------------

use crate::types::error::Error;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::FromStr;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering};

// -----------------------------------------------------------------------------

/// Latitude and longitude values must correspond to a valid location on the
/// face of the earth. Latitudes can take any value between -90 and 90 while
/// longitude values can take any value between -180 and 180. If you specify an
/// invalid latitude or longitude value, your request will be rejected as a bad
/// request.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LatLng {
    /// Latitude. A value between -90.0° and 90.0°.
    #[serde(alias = "lat")]
    #[serde(alias = "latitude")]
    pub lat: Decimal,
    /// Longitude. A value between -180.0° and 180.0°.
    #[serde(alias = "lon")]
    #[serde(alias = "long")]
    #[serde(alias = "longitude")]
    pub lng: Decimal,
} // struct

// -----------------------------------------------------------------------------

impl LatLng {
    /// Takes individual latitude & longitude `Decimal` coordinates and
    /// converts them into a `LatLng` structure. If either the latitude
    /// (-90.0 to +90.0) or longitude (-180.0 to +180.0) are out of range, this
    /// function will return an error.
    pub fn try_from_dec(lat: Decimal, lng: Decimal) -> Result<LatLng, Error> {

        if lat < dec!(-90.0) || lat > dec!(90.0) {
            return Err(Error::InvalidLatitude(lat, lng));
        } // if

        if lng < dec!(-180.0) || lng > dec!(180.0) {
            return Err(Error::InvalidLongitude(lat, lng));
        } // if

        Ok(LatLng {
            lat,
            lng,
        }) // LatLng

    } // fn
} // impl

// -----------------------------------------------------------------------------

impl LatLng {
    /// Takes individual latitude & longitude `f32` coordinates and
    /// converts them into a `LatLng` structure. If either the latitude
    /// (-90.0 to +90.0) or longitude (-180.0 to +180.0) are out of range, this
    /// function will return an error.
    pub fn try_from_f32(lat: f32, lng: f32) -> Result<LatLng, Error> {

        let lat: Decimal = Decimal::from_f32(lat)
            .ok_or_else(|| Error::FloatToDecimalConversionError(lat.to_string()))?;

        let lng: Decimal = Decimal::from_f32(lng)
            .ok_or_else(|| Error::FloatToDecimalConversionError(lng.to_string()))?;

        if lat < dec!(-90.0) || lat > dec!(90.0) {
            return Err(Error::InvalidLatitude(lat, lng));
        } // if

        if lng < dec!(-180.0) || lng > dec!(180.0) {
            return Err(Error::InvalidLongitude(lat, lng));
        } // if

        Ok(LatLng {
            lat,
            lng,
        }) // LatLng

    } // fn
} // impl

// -----------------------------------------------------------------------------

impl LatLng {
    /// Takes individual latitude & longitude `f64` coordinates and
    /// converts them into a `LatLng` structure. If either the latitude
    /// (-90.0 to +90.0) or longitude (-180.0 to +180.0) are out of range, this
    /// function will return an error.
    pub fn try_from_f64(lat: f64, lng: f64) -> Result<LatLng, Error> {

        let lat: Decimal = Decimal::from_f64(lat)
            .ok_or_else(|| Error::FloatToDecimalConversionError(lat.to_string()))?;

        let lng: Decimal = Decimal::from_f64(lng)
            .ok_or_else(|| Error::FloatToDecimalConversionError(lng.to_string()))?;

        if lat < dec!(-90.0) || lat > dec!(90.0) {
            return Err(Error::InvalidLatitude(lat, lng));
        } // if

        if lng < dec!(-180.0) || lng > dec!(180.0) {
            return Err(Error::InvalidLongitude(lat, lng));
        } // if

        Ok(LatLng {
            lat,
            lng,
        }) // LatLng

    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&str> for LatLng {
    type Error = Error;
    ///
    /// Convert string to lat lng
    ///
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let coords: Vec<&str> = value.trim()
            .split(',')
            .collect();
        if coords.len() != 2 {
            Err(Error::InvalidLatLongString(value.to_owned()))
        } else {
            let lat = Decimal::from_str(coords[0].trim());
            let lat = lat.map_err(|_| Error::InvalidLatLongString(value.to_owned()))?;
            let lon = Decimal::from_str(coords[1].trim());
            let lon = lon.map_err(|_| Error::InvalidLatLongString(value.to_owned()))?;
            LatLng::try_from_dec(lat, lon)
        }
    }
}

// -----------------------------------------------------------------------------

impl std::str::FromStr for LatLng {
    // Error definitions are contained in the
    // `google_maps\src\geocoding\error.rs` module.
    type Err = crate::types::Error;
    /// Gets a `LatLng` struct from a `String` that contains a comma-delimited
    /// latitude & longitude pair.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = value.trim()
            .split(',')
            .collect();
        if coords.len() != 2 {
            Err(Error::InvalidLatLongString(value.to_owned()))
        } else {
            let lat = Decimal::from_str(coords[0].trim());
            let lat = lat.map_err(|_| Error::InvalidLatLongString(value.to_owned()))?;
            let lon = Decimal::from_str(coords[1].trim());
            let lon = lon.map_err(|_| Error::InvalidLatLongString(value.to_owned()))?;
            LatLng::try_from_dec(lat, lon)
        } // if
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for LatLng {
    /// Converts a `LatLng` struct to a `String` that contains a
    /// latitude/longitude pair.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "{latitude},{longitude}",
            latitude=self.lat.normalize(),
            longitude=self.lng.normalize(),
        ) // write!
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&LatLng> for String {
    /// Converts a `LatLng` struct to a `String` that contains a
    /// latitude/longitude pair.
    fn from(lat_lng: &LatLng) -> Self {
        lat_lng.to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for LatLng {
    /// Returns a reasonable default value for the `LatLng` struct.
    fn default() -> Self {
        LatLng {
            lat: dec!(0.0),
            lng: dec!(0.0),
        } // struct
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl LatLng {
    /// Formats a `LatLng` struct into a string that is presentable to the end
    /// user.
    pub fn display(&self) -> String {
        // Display latitude and longitude as decimal degrees with some extra
        // fixins'.
        format!(
            "{lat}°{lat_hem} {lng}°{lng_hem}",
            lat=self.lat.abs().normalize(),
            lat_hem=match self.lat.cmp(&dec!(0.0)) {
                Ordering::Less => " S".to_string(),
                Ordering::Greater => " N".to_string(),
                Ordering::Equal => "".to_string(),
            }, // match
            lng=self.lng.abs().normalize(),
            lng_hem=match self.lng.cmp(&dec!(0.0)) {
                Ordering::Less => " W".to_string(),
                Ordering::Greater => " E".to_string(),
                Ordering::Equal => "".to_string(),
            }, // match
        ) // write!
    } // fn
} // impl