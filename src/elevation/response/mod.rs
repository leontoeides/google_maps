//! Resources (enums, structs) for the client to process the Elevation API
//! response.

use crate::latlng::LatLng;
use serde::{Serialize, Deserialize};

/// The response from the Google Maps Elevation API is stored in this structure.

#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// Structure for an elevation sample point.

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Point {

    /// Elevation of the location in meters.
    pub elevation: f64,

    /// Position for which elevation data is being computed. Note that for path
    /// requests, the set of `location` elements will contain the sampled points
    /// along the path.
    pub location: LatLng,

    /// Maximum distance between data points from which the elevation was
    /// interpolated, in meters. This property will be missing if the
    /// resolution is not known. Note that elevation data becomes more coarse
    /// (larger `resolution` values) when multiple points are passed. To obtain
    /// the most accurate elevation value for a point, it should be queried
    /// independently.
    pub resolution: Option<f64>,

} // struct

/// Indicates the status of the response.

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Status {

    /// Indicates that the request was malformed.
    #[serde(alias = "INVALID_REQUEST")]
    InvalidRequest,

    /// Indicates that the request was successful.
    #[serde(alias = "OK")]
    Ok,

    /// Indicates any of the following:
    /// * The API key is missing or invalid.
    /// * Billing has not been enabled on your account.
    /// * A self-imposed usage cap has been exceeded.
    /// * The provided method of payment is no longer valid (for example, a
    /// credit card has expired).
    ///
    /// See the [Maps
    /// FAQ](https://developers.google.com/maps/faq#over-limit-key-error) to
    /// learn how to fix this.
    #[serde(alias = "OVER_DAILY_LIMIT")]
    OverDailyLimit,

    /// Indicates the requestor has exceeded quota.
    #[serde(alias = "OVER_QUERY_LIMIT")]
    OverQueryLimit,

    /// Indicates that the API did not complete the request.
    #[serde(alias = "REQUEST_DENIED")]
    RequestDenied,

    /// Indicates an unknown error.
    #[serde(alias = "UNKNOWN_ERROR")]
    UnknownError,

} // struct