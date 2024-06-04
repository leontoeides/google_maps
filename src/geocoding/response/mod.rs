//! Resources (enums, structs) for processing the _Geocoding API_ response
//! from the Google Maps Platform. Look in here for more information about the
//! data returned from Google's server and how to parse it with your program.

pub mod geocoding;
pub mod plus_code;
pub mod status;

// -----------------------------------------------------------------------------

use crate::geocoding::response::{geocoding::Geocoding, status::Status};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// The response from the Google Maps Geolocation API will be stored in this
/// structure.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Response {
    /// When the geocoder returns a status code other than `OK`, there may be an
    /// additional `error_message` field within the Geocoding response object.
    /// This field contains more detailed information about the reasons behind
    /// the given status code.
    ///
    /// *Note: This field is not guaranteed to be always present, and its
    /// content is subject to change.*
    pub error_message: Option<String>,

    /// When the geocoder returns results, it places them within a results
    /// array. Even if the geocoder returns no results (such as if the address
    /// doesn't exist) it still returns an empty results array.
    #[serde(default)]
    pub results: Vec<Geocoding>,

    /// The `status` field within the Geocoding response object contains the
    /// status of the request, and may contain debugging information to help you
    /// track down why geocoding is not working.
    pub status: Status,
} // struct

// -----------------------------------------------------------------------------

impl std::str::FromStr for Response {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Geocoding API JSON `String` into a `Response`
    /// usable `Response` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    }
}
