//! Contains the `LatLng` struct and its associated traits. The `LatLng`
//! coorindate system is used to specify a position or location on the Earth's
//! surface.

use crate::error::Error;
use serde::{Serialize, Deserialize};

/// Latitude and longitude values must correspond to a valid location on the
/// face of the earth. Latitudes can take any value between -90 and 90 while
/// longitude values can take any value between -180 and 180. If you specify an
/// invalid latitude or longitude value, your request will be rejected as a bad
/// request.

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LatLng {
    /// Latitude. A value between -90 and 90.
    pub lat: f64,
    /// Longitude. A value between -180 and 180.
    pub lng: f64,
} // struct

impl std::convert::TryFrom<&LatLng> for String {

    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::error::Error;

    /// Converts a `LatLng` struct to a `String` that contains a
    /// latitude/longitude pair.
    fn try_from(latlng: &LatLng) -> Result<String, Error> {

        // Range check latitude & longitude:
        if latlng.lat < -90.0 || latlng.lat > 90.0 {
            return Err(Error::InvalidLatitude(latlng.lat))
        } else if latlng.lng < -180.0 || latlng.lng > 180.0 {
            return Err(Error::InvalidLongitude(latlng.lng))
        } // if

        // The seventh decimal place is worth up to 11 mm: this is good for much
        // surveying and is near the limit of what GPS-based techniques can
        // achieve.
        // https://gis.stackexchange.com/questions/8650/measuring-accuracy-of-latitude-and-longitude
        Ok(format!("{:.7},{:.7}", latlng.lat, latlng.lng))

    } // fn
} // impl