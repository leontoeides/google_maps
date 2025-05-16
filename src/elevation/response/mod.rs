//! Resources (enums, structs) for processing the _Elevation API_ response
//! from the Google Maps Platform. Look in here for more information about the
//! data returned from Google's server and how to parse it with your program.

pub mod point;
pub mod status;

// -----------------------------------------------------------------------------

use crate::elevation::{Error, response::{point::Point, status::Status}};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// The response from the Google Maps Elevation API is stored in this structure.
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Response {
    /// When the status code is other than `OK`, there may be an additional
    /// `error_message` field within the Elevation response object. This field
    /// contains more detailed information about the reasons behind the given
    /// status code.
    ///
    /// **Note**: This field is not guaranteed to be always present, and its
    /// content is subject to change.
    #[serde(default)]
    pub error_message: Option<String>,

    /// If there was only one `location` in the query, there will only be one
    /// sample point in the response. If there were multiple `locations` or a
    /// `path` in the query, there will be multiple sample points in the
    /// response.
    #[serde(default)]
    pub results: Vec<Point>,

    /// The status of the response.
    pub status: Status,
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

impl std::convert::From<Response> for Result<Response, crate::elevation::Error> {
    /// Converts a Google Maps API `Response` into a `Result<Response, Error>`
    /// by examining the `status` field inside of the response.
    ///
    /// If the status indicates a success, then an `Ok(response)` will be
    /// returned. If the status indicates an error, then an `Err(error)` will be
    /// returned.
    fn from(response: Response) -> Self {
        match response.status {
            Status::Ok => Ok(response),
            Status::InvalidRequest => Err(Error::InvalidRequest),
            Status::OverDailyLimit => Err(Error::OverDailyLimit),
            Status::OverQueryLimit => Err(Error::OverQueryLimit),
            Status::RequestDenied => Err(Error::RequestDenied),
            Status::UnknownError => Err(Error::UnknownError),
        } // match
    } // fn
} // impl