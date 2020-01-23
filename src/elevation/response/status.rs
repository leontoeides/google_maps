//! Contains the `Status` enum and its associated traits. `Status` may contain
//! debugging information to help you track down why the service request failed.

use crate::elevation::error::Error;
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

    /// Indicates that the API did not complete the request.
    #[serde(alias = "REQUEST_DENIED")]
    RequestDenied,

    /// Indicates an unknown error.
    #[serde(alias = "UNKNOWN_ERROR")]
    UnknownError,

} // enum

impl std::convert::From<&Status> for String {
    /// Converts a `Status` enum to a `String` that contains a
    /// [status](https://developers.google.com/maps/documentation/elevation/intro#ElevationResponses)
    /// code.
    fn from(status: &Status) -> String {
        match status {
            Status::InvalidRequest => String::from("INVALID_REQUEST"),
            Status::Ok => String::from("OK"),
            Status::OverDailyLimit => String::from("OVER_DAILY_LIMIT"),
            Status::OverQueryLimit => String::from("OVER_QUERY_LIMIT"),
            Status::RequestDenied => String::from("REQUEST_DENIED"),
            Status::UnknownError => String::from("UNKNOWN_ERROR"),
        } // match
    } // fn
} // impl

impl std::convert::TryFrom<String> for Status {

    // Error definitions are contained in the
    // `google_maps\src\elevation\error.rs` module.
    type Error = crate::elevation::error::Error;

    /// Gets a `Status` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/elevation/intro#ElevationResponses)
    /// code.
    fn try_from(status: String) -> Result<Status, Error> {
        match status.as_ref() {
            "INVALID_REQUEST" => Ok(Status::InvalidRequest),
            "OK" => Ok(Status::Ok),
            "OVER_DAILY_LIMIT" => Ok(Status::OverDailyLimit),
            "OVER_QUERY_LIMIT" => Ok(Status::OverQueryLimit),
            "REQUEST_DENIED" => Ok(Status::RequestDenied),
            "UNKNOWN_ERROR" => Ok(Status::UnknownError),
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
            Status::Ok => write!(f, "OK"),
            Status::OverDailyLimit => write!(f, "Over Daily Limit"),
            Status::OverQueryLimit => write!(f, "Over Query Limit"),
            Status::RequestDenied => write!(f, "Request Denied"),
            Status::UnknownError => write!(f, "Unknown Error"),
        } // match
    } // fn
} // impl