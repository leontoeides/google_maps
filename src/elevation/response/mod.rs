//! Resources (enums, structs) for the client to process the Elevation API
//! response.

pub mod point;
pub mod status;

use crate::elevation::response::{
    point::Point,
    status::Status,
};
use serde::{Serialize, Deserialize};

/// The response from the Google Maps Elevation API is stored in this structure.

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Response {

    /// More detailed information about the reasons behind the given status
    /// code, if other than `OK`.
    pub error_message: Option<String>,

    /// If there was only one `location` in the query, there will only be one
    /// sample point in the response. If there were multiple `locations` or a
    /// `path` in the query, there will be multiple sample points in the
    /// response.
    pub results: Option<Vec<Point>>,

    /// The status of the response.
    pub status: Status,

} // struct