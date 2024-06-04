//! The `"status"` field within the Time Zone API response object contains the
//! status of the request, and may contain debugging information to help you
//! track down why time zone is not working.

use crate::time_zone::error::Error;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize};

// -----------------------------------------------------------------------------

/// Indicates the status of the response.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
    ///   credit card has expired).
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
    /// [status](https://developers.google.com/maps/documentation/timezone/intro#Responses)
    /// code.
    fn from(status: &Status) -> Self {
        match status {
            Status::InvalidRequest => Self::from("INVALID_REQUEST"),
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
    "OK" => Status::Ok,
    "OVER_DAILY_LIMIT" => Status::OverDailyLimit,
    "OVER_QUERY_LIMIT" => Status::OverQueryLimit,
    "REQUEST_DENIED" => Status::RequestDenied,
    "UNKNOWN_ERROR" => Status::UnknownError,
    "ZERO_RESULTS" => Status::ZeroResults,
};

impl std::convert::TryFrom<&str> for Status {
    // Error definitions are contained in the
    // `google_maps\src\time_zone\error.rs` module.
    type Error = crate::time_zone::error::Error;
    /// Gets a `Status` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/timezone/intro#Responses)
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
    // `google_maps\src\time_zone\error.rs` module.
    type Err = crate::time_zone::error::Error;
    /// Gets a `Status` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/timezone/intro#Responses)
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
            Self::Ok => write!(f, "OK"),
            Self::OverDailyLimit => write!(f, "Over Daily Limit"),
            Self::OverQueryLimit => write!(f, "Over Query Limit"),
            Self::RequestDenied => write!(f, "Request Denied"),
            Self::UnknownError => write!(f, "Unknown Error"),
            Self::ZeroResults => write!(f, "Zero Results"),
        } // match
    } // fn
} // impl
