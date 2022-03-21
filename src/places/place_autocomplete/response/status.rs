//! The `"status"` field within the Places API _Place Autocomplete_ response
//! object contains the status of the request, and may contain debugging
//! information to help your request is not working.

use crate::places::place_autocomplete::error::Error;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------

/// Indicates the status of the response.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Status {
    /// Indicates the API request was malformed, generally due to the missing
    /// input parameter.
    #[serde(alias = "INVALID_REQUEST")]
    InvalidRequest,

    /// Indicates that the request was successful.
    #[serde(alias = "OK")]
    Ok,

    /// Indicates any of the following:
    /// * You have exceeded the QPS limits.
    /// * Billing has not been enabled on your account.
    /// * The monthly $200 credit, or a self-imposed usage cap, has been
    /// exceeded.
    /// * The provided method of payment is no longer valid (for example, a
    /// credit card has expired).
    ///
    /// See the [Maps FAQ](https://developers.google.com/maps/faq#over-limit-key-error)
    /// for more information about how to resolve this error.
    #[serde(alias = "OVER_QUERY_LIMIT")]
    OverQueryLimit,

    /// Indicates that your request was denied, generally because:
    /// * The request is missing an API key.
    /// * The key parameter is invalid.
    #[serde(alias = "REQUEST_DENIED")]
    RequestDenied,

    /// Indicates an unknown error.
    #[serde(alias = "UNKNOWN_ERROR")]
    UnknownError,

    /// Indicates that the search was successful but returned no results. This
    /// may occur if the search was passed a bounds in a remote location.
    #[serde(alias = "ZERO_RESULTS")]
    ZeroResults,
} // struct

// -----------------------------------------------------------------------------

impl std::convert::From<&Status> for String {
    /// Converts a `Status` enum to a `String` that contains a
    /// [status](https://developers.google.com/maps/documentation/timezone/intro#Responses)
    /// code.
    fn from(status: &Status) -> String {
        match status {
            Status::InvalidRequest => String::from("INVALID_REQUEST"),
            Status::Ok => String::from("OK"),
            Status::OverQueryLimit => String::from("OVER_QUERY_LIMIT"),
            Status::RequestDenied => String::from("REQUEST_DENIED"),
            Status::UnknownError => String::from("UNKNOWN_ERROR"),
            Status::ZeroResults => String::from("ZERO_RESULTS"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for Status {
    // Error definitions are contained in the
    // `google_maps\src\places\place_autocomplete\error.rs` module.
    type Error = crate::places::place_autocomplete::error::Error;

    /// Gets a `Status` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/timezone/intro#Responses)
    /// code.
    fn try_from(status: &str) -> Result<Status, Error> {
        match status {
            "INVALID_REQUEST" => Ok(Status::InvalidRequest),
            "OK" => Ok(Status::Ok),
            "OVER_QUERY_LIMIT" => Ok(Status::OverQueryLimit),
            "REQUEST_DENIED" => Ok(Status::RequestDenied),
            "UNKNOWN_ERROR" => Ok(Status::UnknownError),
            "ZERO_RESULTS" => Ok(Status::ZeroResults),
            _ => Err(Error::InvalidStatusCode(status.to_string())),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for Status {
    /// Returns a reasonable default variant for the `Status` enum type.
    fn default() -> Self {
        Status::Ok
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for Status {
    /// Formats a `Status` enum into a string that is presentable to the end
    /// user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::InvalidRequest => write!(f, "Invalid Request"),
            Status::Ok => write!(f, "OK"),
            Status::OverQueryLimit => write!(f, "Over Query Limit"),
            Status::RequestDenied => write!(f, "Request Denied"),
            Status::UnknownError => write!(f, "Unknown Error"),
            Status::ZeroResults => write!(f, "Zero Results"),
        } // match
    } // fn
} // impl