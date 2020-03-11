//! The `"status"` field within the Time Zone API response object contains the
//! status of the request, and may contain debugging information to help you
//! track down why geocoding is not working.

use crate::geocoding::error::Error;
use serde::{Serialize, Deserialize};

/// Indicates the status of the response.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
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

    /// Indicates that the API did not complete the request. Confirm that the
    /// request was sent over HTTPS instead of HTTP.
    #[serde(alias = "REQUEST_DENIED")]
    RequestDenied,

    /// Indicates an unknown error.
    #[serde(alias = "UNKNOWN_ERROR")]
    UnknownError,

    /// Indicates that no time zone data could be found for the specified
    /// position or time. Confirm that the request is for a location on land,
    /// and not over water.
    #[serde(alias = "ZERO_RESULTS")]
    ZeroResults,

} // struct

impl std::convert::From<&Status> for String {
    /// Converts a `Status` enum to a `String` that contains a
    /// [status](https://developers.google.com/maps/documentation/timezone/intro#Responses)
    /// code.
    fn from(status: &Status) -> String {
        match status {
            Status::InvalidRequest => String::from("INVALID_REQUEST"),
            Status::Ok => String::from("OK"),
            Status::OverDailyLimit => String::from("OVER_DAILY_LIMIT"),
            Status::OverQueryLimit => String::from("OVER_QUERY_LIMIT"),
            Status::RequestDenied => String::from("REQUEST_DENIED"),
            Status::UnknownError => String::from("UNKNOWN_ERROR"),
            Status::ZeroResults => String::from("ZERO_RESULTS"),
        } // match
    } // fn
} // impl

impl std::convert::TryFrom<&str> for Status {

    // Error definitions are contained in the
    // `google_maps\src\geocoding\error.rs` module.
    type Error = crate::geocoding::error::Error;

    /// Gets a `Status` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/timezone/intro#Responses)
    /// code.
    fn try_from(status: &str) -> Result<Status, Error> {
        match status {
            "INVALID_REQUEST" => Ok(Status::InvalidRequest),
            "OK" => Ok(Status::Ok),
            "OVER_DAILY_LIMIT" => Ok(Status::OverDailyLimit),
            "OVER_QUERY_LIMIT" => Ok(Status::OverQueryLimit),
            "REQUEST_DENIED" => Ok(Status::RequestDenied),
            "UNKNOWN_ERROR" => Ok(Status::UnknownError),
            "ZERO_RESULTS" => Ok(Status::ZeroResults),
            _ => Err(Error::InvalidStatusCode(status.to_string())),
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
            Status::Ok => write!(f, "OK"),
            Status::OverDailyLimit => write!(f, "Over Daily Limit"),
            Status::OverQueryLimit => write!(f, "Over Query Limit"),
            Status::RequestDenied => write!(f, "Request Denied"),
            Status::UnknownError => write!(f, "Unknown Error"),
            Status::ZeroResults => write!(f, "Zero Results"),
        } // match
    } // fn
} // impl