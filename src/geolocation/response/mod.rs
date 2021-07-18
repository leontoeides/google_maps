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
    pub location: Option<LatLng>,
    pub accuracy: Option<f64>,
    pub error: Option<ErrorObject>,
} // struct

impl std::str::FromStr for Response {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Geolocation API JSON `String` into a `Response`
    /// usable `Response` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    }
}