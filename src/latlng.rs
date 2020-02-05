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
    /// Latitude. A value between -90° and 90°.
    pub lat: f64,
    /// Longitude. A value between -180° and 180°.
    pub lng: f64,
} // struct

impl LatLng {
    pub fn try_from(latitude: f64, longitude: f64) -> Result<LatLng, Error> {
        if latitude < -90.0 || latitude > 90.0 {
            return Err(Error::InvalidLatitude(latitude, longitude))
        } // if
        if longitude < -180.0 || longitude > 180.0 {
            return Err(Error::InvalidLongitude(latitude, longitude))
        } // if
        Ok(LatLng {
            lat: latitude,
            lng: longitude,
        }) // Lat Lng
    } // fn
} // impl

/// Contrains floating-point number to seventh decimal place precision, and
/// removes insignificant digits to save bandwidth & improve readability.
fn fmt_decdeg(float: f64) -> String {
    // The seventh decimal place is worth up to 11 mm: this is good for much
    // surveying and is near the limit of what GPS-based techniques can
    // achieve.
    // https://gis.stackexchange.com/questions/8650/measuring-accuracy-of-latitude-and-longitude
    let mut decdeg_str = format!("{:.7}", float);
    // Remove insignificant digits and possibly the fractional portion of the
    // string altogether.
    if decdeg_str.contains('.') {
        decdeg_str = decdeg_str.trim_end_matches('0').to_string();
        decdeg_str = decdeg_str.trim_end_matches('.').to_string();
    } // if
    decdeg_str
} // fn

impl std::convert::From<&LatLng> for String {
    /// Converts a `LatLng` struct to a `String` that contains a
    /// latitude/longitude pair.
    fn from(latlng: &LatLng) -> String {
        format!("{},{}", fmt_decdeg(latlng.lat), fmt_decdeg(latlng.lng))
    } // fn
} // impl

impl std::default::Default for LatLng {
    /// Returns a reasonable default values for the `LatLng` struct.
    fn default() -> Self {
        LatLng {
            lat: 0.0,
            lng: 0.0,
        } // struct
    } // fn
} // impl

impl std::fmt::Display for LatLng {
    /// Formats a `LatLng` struct into a string that is presentable to the end
    /// user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Indicate north-south hemisphere for latitude.
        let ns_hemisphere = if self.lat > 0.0 {
            String::from(" N")
        } else if self.lat == 0.0 {
            String::from("")
        } else {
            String::from(" S")
        };
        // Indicate east-west hemisphere for longitude.
        let ew_hemisphere = if self.lng > 0.0 {
            String::from(" E")
        } else if self.lng == 0.0 {
            String::from("")
        } else {
            String::from(" W")
        };
        // Display latitude and longitude as decimal degrees with some extra
        // fixins'.
        write!(f, "{}°{} {}°{}", fmt_decdeg(self.lat.abs()), ns_hemisphere, fmt_decdeg(self.lng.abs()), ew_hemisphere)
    } // fn
} // impl