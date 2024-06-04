//! Resources (enums, structs) for processing the _Nearest Roads_ response from
//! the Google Maps Platform. Look in here for more information about the data
//! returned from Google's server and how to parse it with your program.

// -----------------------------------------------------------------------------

use crate::roads::error_response::ErrorResponse;
use crate::roads::snapped_point::SnappedPoint;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// The response from the Google Maps _Nearest Roads_ request will be stored in
/// this structure.
///
/// [Nearest Roads Responses](https://developers.google.com/maps/documentation/roads/nearest#NearestRoadsResponse)
/// ------------------------------------------------------------------------------------------------
/// For each valid request, the Roads API will return a response in the format
/// indicated within the request URL such as the following JSON response.

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// An array of snapped points. See
    /// [SnappedPoint](https://developers.google.com/maps/documentation/roads/nearest#SnappedPoint)
    /// for more information.
    #[serde(alias = "snappedPoints")]
    #[serde(default)]
    pub snapped_points: Vec<SnappedPoint>,

    /// A string containing a user-visible warning.
    #[serde(alias = "warningMessage")]
    pub warning_message: Option<String>,

    /// In the case of an error, a standard format error response body will be
    /// returned and the HTTP status code will be set to an error status.
    pub error: Option<ErrorResponse>,
} // struct

// -----------------------------------------------------------------------------

impl std::str::FromStr for Response {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps _Nearest Roads_ JSON `String` response into a
    /// usable `Response` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    } // fn
} // impl
