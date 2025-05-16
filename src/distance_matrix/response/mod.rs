//! Resources (enums, structs) for processing the _Distance Matrix API_ response
//! from the Google Maps Platform. Look in here for more information about the
//! data returned from Google's server and how to parse it with your program.
//! The Distance Matrix API shares many enums and struct with the _Directions
//! API_. If you're not finding what you're looking for in this module, check
//! out the Directions modules also.

// -----------------------------------------------------------------------------

pub mod element;
pub mod element_status;
pub mod row;
pub mod status;

// -----------------------------------------------------------------------------

use crate::distance_matrix::{Error, response::{row::Row, status::Status}};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Distance Matrix responses contain the following root elements.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Response {
    /// Contains an array of addresses as returned by the API from your original
    /// request. As with `origin_addresses`, these are localized if appropriate.
    #[serde(default)]
    pub destination_addresses: Vec<String>,

    /// When the status code is other than `OK`, there may be an additional
    /// `error_message` field within the Directions response object. This field
    /// contains more detailed information about the reasons behind the given
    /// status code.
    ///
    /// **Note**: This field is not guaranteed to be always present, and its
    /// content is subject to change.
    pub error_message: Option<String>,

    /// Contains an array of addresses as returned by the API from your original
    /// request. These are formatted by the
    /// [geocoder](https://developers.google.com/maps/documentation/geocoding/)
    /// and localized according to the `language` parameter passed with the
    /// request.
    #[serde(default)]
    pub origin_addresses: Vec<String>,

    /// Contains an array of elements, which in turn each contain a `status`,
    /// `duration`, and `distance` element.
    #[serde(default)]
    pub rows: Vec<Row>,

    /// Contains metadata on the request.
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

impl std::convert::From<Response> for Result<Response, crate::distance_matrix::Error> {
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
            Status::MaxElementsExceeded => Err(Error::MaxElementsExceeded),
            Status::MaxDimensionsExceeded => Err(Error::MaxDimensionsExceeded),
            Status::OverDailyLimit => Err(Error::OverDailyLimit),
            Status::OverQueryLimit => Err(Error::OverQueryLimit),
            Status::RequestDenied => Err(Error::RequestDenied),
            Status::UnknownError => Err(Error::UnknownError),
        } // match
    } // fn
} // impl