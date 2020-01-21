use crate::directions::error::Error;
use serde::{Serialize, Deserialize};

/// The `status` field within the Directions response object contains the
/// [status](https://developers.google.com/maps/documentation/directions/intro#StatusCodes)
/// of the request, and may contain debugging information to help you track down
/// why the Directions service failed.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Status {
    /// Indicates that the provided request was invalid. Common causes of this
    /// status include an invalid parameter or parameter value.
    #[serde(alias = "INVALID_REQUEST")]
    InvalidRequest,
    /// Indicates the requested route is too long and cannot be processed. This
    /// error occurs when more complex directions are returned. Try reducing the
    /// number of waypoints, turns, or instructions.
    #[serde(alias = "MAX_ROUTE_LENGTH_EXCEEDED")]
    MaxRouteLengthExceeded,
    /// Indicates that too many `waypoints` were provided in the request. For
    /// applications using the Directions API as a web service, or the
    /// [directions service in the Maps JavaScript API](https://developers.google.com/maps/documentation/javascript/directions),
    /// the maximum allowed number of `waypoints` is 25, plus the origin and
    /// destination.
    #[serde(alias = "MAX_WAYPOINTS_EXCEEDED")]
    MaxWaypointsExceeded,
    /// Indicates at least one of the locations specified in the request's
    /// origin, destination, or waypoints could not be geocoded.
    #[serde(alias = "NOT_FOUND")]
    NotFound,
    /// Indicates the response contains a valid `result`.
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
    /// Indicates the service has received too many requests from your
    /// application within the allowed time period.
    #[serde(alias = "OVER_QUERY_LIMIT")]
    OverQueryLimit,
    /// Indicates that the service denied use of the directions service by your
    /// application.
    #[serde(alias = "REQUEST_DENIED")]
    RequestDenied,
    /// Indicates a directions request could not be processed due to a server
    /// error. The request may succeed if you try again.
    #[serde(alias = "UNKNOWN_ERROR")]
    UnknownError,
    /// Indicates no route could be found between the origin and destination.
    #[serde(alias = "ZERO_RESULTS")]
    ZeroResults,
} // struct

impl std::convert::From<&Status> for String {
    /// Converts a `Status` enum to a `String` that contains a
    /// [status](https://developers.google.com/maps/documentation/directions/intro#StatusCodes)
    /// code.
    fn from(status: &Status) -> String {
        match status {
            Status::InvalidRequest => String::from("INVALID_REQUEST"),
            Status::MaxRouteLengthExceeded => String::from("MAX_ROUTE_LENGTH_EXCEEDED"),
            Status::MaxWaypointsExceeded => String::from("MAX_WAYPOINTS_EXCEEDED"),
            Status::NotFound => String::from("NOT_FOUND"),
            Status::Ok => String::from("OK"),
            Status::OverDailyLimit => String::from("OVER_DAILY_LIMIT"),
            Status::OverQueryLimit => String::from("OVER_QUERY_LIMIT"),
            Status::RequestDenied => String::from("REQUEST_DENIED"),
            Status::UnknownError => String::from("UNKNOWN_ERROR"),
            Status::ZeroResults => String::from("ZERO_RESULTS"),
        } // match
    } // fn
} // impl

impl std::convert::TryFrom<String> for Status {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;
    /// Gets a `Status` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/directions/intro#StatusCodes)
    /// code.
    fn try_from(status: String) -> Result<Status, Error> {
        match status.as_ref() {
            "INVALID_REQUEST" => Ok(Status::InvalidRequest),
            "MAX_ROUTE_LENGTH_EXCEEDED" => Ok(Status::MaxRouteLengthExceeded),
            "MAX_WAYPOINTS_EXCEEDED" => Ok(Status::MaxWaypointsExceeded),
            "NOT_FOUND" => Ok(Status::NotFound),
            "OK" => Ok(Status::Ok),
            "OVER_DAILY_LIMIT" => Ok(Status::OverDailyLimit),
            "OVER_QUERY_LIMIT" => Ok(Status::OverQueryLimit),
            "REQUEST_DENIED" => Ok(Status::RequestDenied),
            "UNKNOWN_ERROR" => Ok(Status::UnknownError),
            "ZERO_RESULTS" => Ok(Status::ZeroResults),
            _ => Err(Error::InvalidStatusCode(status)),
        } // match
    } // fn
} // impl

impl std::default::Default for Status {
    /// Returns a reasonable default variant for the `Status` enum type.
    fn default() -> Self {
        Status::Ok
    } // fn
} // impl

impl std::fmt::Display for Status {
    /// Formats a `Status` enum into a string that is presentable to the end
    /// user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::InvalidRequest => write!(f, "Invalid Request"),
            Status::MaxRouteLengthExceeded => write!(f, "Max Route Length Exceeded"),
            Status::MaxWaypointsExceeded => write!(f, "Max Waypoints Exceeded"),
            Status::NotFound => write!(f, "Not Found"),
            Status::Ok => write!(f, "OK"),
            Status::OverDailyLimit => write!(f, "Over Daily Limit"),
            Status::OverQueryLimit => write!(f, "Over Query Limit"),
            Status::RequestDenied => write!(f, "Request Denied"),
            Status::UnknownError => write!(f, "Unknown Error"),
            Status::ZeroResults => write!(f, "Zero Results"),
        } // match
    } // fn
} // impl