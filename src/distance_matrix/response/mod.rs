//! Resources (enums, structs) for processing the _Distance Matrix API_ response
//! from the Google Maps Platform. Look in here for more information about the
//! data returned from Google's server and how to parse it with your program.
//! The Distance Matrix API shares many enums and struct with the _Directions
//! API_. If you're not finding what you're looking for in this module, check
//! out the Directions modules also.

pub mod element;
pub mod element_status;
pub mod row;
pub mod status;

use crate::distance_matrix::response::{row::Row, status::Status};
use serde::{Deserialize, Serialize};

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

impl std::str::FromStr for Response {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Distance Matrix API JSON `String` response into a
    /// usable `Response` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    }
}
