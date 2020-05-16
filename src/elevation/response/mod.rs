//! Resources (enums, structs) for processing the _Elevation API_ response
//! from the Google Maps Platform. Look in here for more information about the
//! data returned from Google's server and how to parse it with your program.

pub mod point;
pub mod status;

use crate::elevation::response::{point::Point, status::Status};
use serde::{Deserialize, Serialize};

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
    pub error_message: Option<String>,

    /// If there was only one `location` in the query, there will only be one
    /// sample point in the response. If there were multiple `locations` or a
    /// `path` in the query, there will be multiple sample points in the
    /// response.
    pub results: Option<Vec<Point>>,

    /// The status of the response.
    pub status: Status,

} // struct