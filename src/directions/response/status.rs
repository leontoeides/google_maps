//! Contains the `Status` enum and its associated traits. It may contain
//! debugging information to help you track down why the service request failed.

use crate::directions::error::Error;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize};

// -----------------------------------------------------------------------------

/// The `status` field within the Directions response object contains the
/// [status](https://developers.google.com/maps/documentation/directions/intro#StatusCodes)
/// of the request, and may contain debugging information to help you track down
/// why the Directions service failed.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
    ///   credit card has expired).
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
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for Status {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match Self::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string())),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Status> for String {
    /// Converts a `Status` enum to a `String` that contains a
    /// [status](https://developers.google.com/maps/documentation/directions/intro#StatusCodes)
    /// code.
    fn from(status: &Status) -> Self {
        match status {
            Status::InvalidRequest => Self::from("INVALID_REQUEST"),
            Status::MaxRouteLengthExceeded => Self::from("MAX_ROUTE_LENGTH_EXCEEDED"),
            Status::MaxWaypointsExceeded => Self::from("MAX_WAYPOINTS_EXCEEDED"),
            Status::NotFound => Self::from("NOT_FOUND"),
            Status::Ok => Self::from("OK"),
            Status::OverDailyLimit => Self::from("OVER_DAILY_LIMIT"),
            Status::OverQueryLimit => Self::from("OVER_QUERY_LIMIT"),
            Status::RequestDenied => Self::from("REQUEST_DENIED"),
            Status::UnknownError => Self::from("UNKNOWN_ERROR"),
            Status::ZeroResults => Self::from("ZERO_RESULTS"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static STATUSES_BY_CODE: phf::Map<&'static str, Status> = phf_map! {
    "INVALID_REQUEST" => Status::InvalidRequest,
    "MAX_ROUTE_LENGTH_EXCEEDED" => Status::MaxRouteLengthExceeded,
    "MAX_WAYPOINTS_EXCEEDED" => Status::MaxWaypointsExceeded,
    "NOT_FOUND" => Status::NotFound,
    "OK" => Status::Ok,
    "OVER_DAILY_LIMIT" => Status::OverDailyLimit,
    "OVER_QUERY_LIMIT" => Status::OverQueryLimit,
    "REQUEST_DENIED" => Status::RequestDenied,
    "UNKNOWN_ERROR" => Status::UnknownError,
    "ZERO_RESULTS" => Status::ZeroResults,
};

impl std::convert::TryFrom<&str> for Status {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;
    /// Gets a `Status` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/directions/intro#StatusCodes)
    /// code.
    fn try_from(status_code: &str) -> Result<Self, Self::Error> {
        STATUSES_BY_CODE
            .get(status_code)
            .cloned()
            .ok_or_else(|| Error::InvalidStatusCode(status_code.to_string()))
    } // fn
} // impl

impl std::str::FromStr for Status {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Err = crate::directions::error::Error;
    /// Gets a `Status` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/directions/intro#StatusCodes)
    /// code.
    fn from_str(status_code: &str) -> Result<Self, Self::Err> {
        STATUSES_BY_CODE
            .get(status_code)
            .cloned()
            .ok_or_else(|| Error::InvalidStatusCode(status_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for Status {
    /// Returns a reasonable default variant for the `Status` enum type.
    fn default() -> Self {
        Self::Ok
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for Status {
    /// Formats a `Status` enum into a string that is presentable to the end
    /// user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidRequest => write!(f, "Invalid Request"),
            Self::MaxRouteLengthExceeded => write!(f, "Maximum Route Length Exceeded"),
            Self::MaxWaypointsExceeded => write!(f, "Maximum Waypoints Exceeded"),
            Self::NotFound => write!(f, "Not Found"),
            Self::Ok => write!(f, "OK"),
            Self::OverDailyLimit => write!(f, "Over Daily Limit"),
            Self::OverQueryLimit => write!(f, "Over Query Limit"),
            Self::RequestDenied => write!(f, "Request Denied"),
            Self::UnknownError => write!(f, "Unknown Error"),
            Self::ZeroResults => write!(f, "Zero Results"),
        } // match
    } // fn
} // impl
