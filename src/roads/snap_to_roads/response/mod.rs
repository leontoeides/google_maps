//! Resources (enums, structs) for processing the _Snap To Roads_ response from
//! the Google Maps Platform. Look in here for more information about the data
//! returned from Google's server and how to parse it with your program.

// -----------------------------------------------------------------------------

use crate::roads::{
    Error,
    error_response::ErrorResponse,
    snapped_point::SnappedPoint,
    Status,
};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// The response from the Google Maps _Snap To Roads_ request will be stored in
/// this structure.
///
/// [Snap To Roads Responses](https://developers.google.com/maps/documentation/roads/snap#responses)
/// ------------------------------------------------------------------------------------------------
/// For each valid request, the Roads API will return a response in the format
/// indicated within the request URL such as the following JSON response.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// An array of snapped points. See
    /// [SnappedPoint](https://developers.google.com/maps/documentation/roads/snap#SnappedPoint)
    /// for more information.
    #[serde(rename = "snappedPoints")]
    #[serde(alias = "snapped_points")]
    #[serde(default)]
    pub snapped_points: Vec<SnappedPoint>,

    /// A string containing a user-visible warning.
    #[serde(rename = "warningMessage")]
    #[serde(alias = "warning_message")]
    #[serde(default)]
    pub warning_message: Option<String>,

    /// In the case of an error, a standard format error response body will be
    /// returned and the HTTP status code will be set to an error status.
    #[serde(default)]
    pub error: Option<ErrorResponse>,
} // struct

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<String> for Response {
    type Error = serde_json::Error;
    /// Convert a Google Maps API [JSON](https://en.wikipedia.org/wiki/JSON)
    /// `String` response into a `Response` struct.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        serde_json::from_slice(&s.into_bytes())
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for Response {
    type Err = serde_json::Error;
    /// Converts a Google Maps API [JSON](https://en.wikipedia.org/wiki/JSON)
    /// `&str` response into a `Response` struct.
    ///
    /// # Notes
    ///
    /// * It's recommended to use the implemented `TryFrom` trait instead.
    ///
    /// * The [serde_json](https://crates.io/crates/simd-json)'s `from_str`
    ///   function implementation is unsafe and it's `from_slice` function
    ///   requires a mutable reference. Therefore this trait clones the `&str`
    ///   into a `String` to give `from_slice` mutable access to the string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.to_string().into_bytes();
        serde_json::from_slice(&bytes)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<Response> for Result<Response, Error> {
    /// Converts a Google Maps API `Response` into a `Result<Response, Error>`
    /// by examining the `status` field inside of the response.
    ///
    /// If the status indicates a success, then an `Ok(response)` will be
    /// returned. If the status indicates an error, then an `Err(error)` will be
    /// returned.
    fn from(response: Response) -> Self {
        match response.error {
            None => Ok(response),
            Some(error_response) => match error_response.status {
                Status::InvalidArgument => Err(Error::InvalidArgument(error_response.message)),
                Status::PermissionDenied => Err(Error::PermissionDenied(error_response.message)),
                Status::NotFound => Err(Error::NotFound(error_response.message)),
                Status::ResourceExhausted => Err(Error::ResourceExhausted(error_response.message)),
            }, // match
        } // match
    } // fn
} // impl