//! Contains the `LatLng` struct, its associated traits, and functions. The
//! latitude & longitude coorindate system is used to specify a position or
//! location on the Earth's surface.

use crate::error::Error;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

/// Latitude and longitude values must correspond to a valid location on the
/// face of the earth. Latitudes can take any value between -90 and 90 while
/// longitude values can take any value between -180 and 180. If you specify an
/// invalid latitude or longitude value, your request will be rejected as a bad
/// request.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LatLng {
    /// Latitude. A value between -90.0° and 90.0°.
    pub lat: Decimal,
    /// Longitude. A value between -180.0° and 180.0°.
    pub lng: Decimal,
} // struct

impl LatLng {
    /// Takes individual latitude & longitude floating-point coordinates and
    /// converts them into a `LatLng` structure. If either the latitude
    /// (-90.0 to +90.0) or longitude (-180.0 to +180.0) are out of range, this
    /// function will return an error.
    pub fn try_from(latitude: Decimal, longitude: Decimal) -> Result<LatLng, Error> {
        if latitude < dec!(-90.0) || latitude > dec!(90.0) {
            return Err(Error::InvalidLatitude(latitude, longitude));
        } // if
        if longitude < dec!(-180.0) || longitude > dec!(180.0) {
            return Err(Error::InvalidLongitude(latitude, longitude));
        } // if
        Ok(LatLng {
            lat: latitude,
            lng: longitude,
        }) // Lat Lng
    } // fn
} // impl

impl std::convert::From<&LatLng> for String {
    /// Converts a `LatLng` struct to a `String` that contains a
    /// latitude/longitude pair.
    fn from(latlng: &LatLng) -> String {
        format!("{},{}", latlng.lat.normalize(), latlng.lng.normalize())
    } // fn
} // impl

impl std::default::Default for LatLng {
    /// Returns a reasonable default values for the `LatLng` struct.
    fn default() -> Self {
        LatLng {
            lat: dec!(0.0),
            lng: dec!(0.0),
        } // struct
    } // fn
} // impl

impl std::fmt::Display for LatLng {
    /// Formats a `LatLng` struct into a string that is presentable to the end
    /// user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Indicate north-south hemisphere for latitude.
        let ns_hemisphere = if self.lat > dec!(0.0) {
            String::from(" N")
        } else if self.lat == dec!(0.0) {
            String::from("")
        } else {
            String::from(" S")
        };
        // Indicate east-west hemisphere for longitude.
        let ew_hemisphere = if self.lng > dec!(0.0) {
            String::from(" E")
        } else if self.lng == dec!(0.0) {
            String::from("")
        } else {
            String::from(" W")
        };
        // Display latitude and longitude as decimal degrees with some extra
        // fixins'.
        write!(
            f,
            "{}°{} {}°{}",
            self.lat.abs().normalize(),
            ns_hemisphere,
            self.lng.abs().normalize(),
            ew_hemisphere,
        )
    } // fn
} // impl
