//! Resources (enums, structs) for processing the _Geolocation API_ response
//! from the Google Maps Platform. Look in here for more information about the
//! data returned from Google's server and how to parse it with your program.

mod error_object;
mod error;

use crate::{
    geolocation::response::error_object::ErrorObject,
    latlng::LatLng,
}; // use
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LatLng>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,
} // struct

impl std::str::FromStr for Response {
    type Err = simd_json::Error;
    /// Parse a Google Maps Geolocation API JSON `String` into a `Response`
    /// usable `Response` struct.
    fn from_str(s: &str) -> Result<Self, simd_json::Error> {
        let mut bytes = s.to_string().into_bytes();
        simd_json::serde::from_slice(&mut bytes)
    }
}