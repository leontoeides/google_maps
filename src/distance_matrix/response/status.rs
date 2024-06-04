//! Contains the `Status` enum and its associated traits. It may contain
//! debugging information to help you track down why the service request failed.

use crate::distance_matrix::error::Error;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize};

// -----------------------------------------------------------------------------

/// The `status` fields within the response object contain the status of the
/// request, and may contain useful debugging information. The Distance Matrix
/// API returns a top-level status field, with information about the request in
/// general, as well as a status field for each element field, with information
/// about that particular origin-destination pairing.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Status {
    /// Indicates that the provided request was invalid. Common causes of this
    /// status include an invalid parameter or parameter value.
    #[serde(alias = "INVALID_REQUEST")]
    InvalidRequest,
    /// Indicates that the product of origins and destinations exceeds the
    /// per-query
    /// [limit](https://developers.google.com/maps/documentation/distance-matrix/usage-and-billing).
    #[serde(alias = "MAX_ELEMENTS_EXCEEDED")]
    MaxElementsExceeded,
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
    /// Indicates that the service denied use of the Distance Matrix service by
    /// your application.
    #[serde(alias = "REQUEST_DENIED")]
    RequestDenied,
    /// Indicates a Distance Matrix request could not be processed due to a
    /// server error. The request may succeed if you try again.
    #[serde(alias = "UNKNOWN_ERROR")]
    UnknownError,
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
    /// [status](https://developers.google.com/maps/documentation/distance-matrix/intro#top-level-status-codes)
    /// code.
    fn from(status: &Status) -> Self {
        match status {
            Status::InvalidRequest => Self::from("INVALID_REQUEST"),
            Status::MaxElementsExceeded => Self::from("MAX_ELEMENTS_EXCEEDED"),
            Status::Ok => Self::from("OK"),
            Status::OverDailyLimit => Self::from("OVER_DAILY_LIMIT"),
            Status::OverQueryLimit => Self::from("OVER_QUERY_LIMIT"),
            Status::RequestDenied => Self::from("REQUEST_DENIED"),
            Status::UnknownError => Self::from("UNKNOWN_ERROR"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static STATUSES_BY_CODE: phf::Map<&'static str, Status> = phf_map! {
    "INVALID_REQUEST" => Status::InvalidRequest,
    "MAX_ELEMENTS_EXCEEDED" => Status::MaxElementsExceeded,
    "OK" => Status::Ok,
    "OVER_DAILY_LIMIT" => Status::OverDailyLimit,
    "OVER_QUERY_LIMIT" => Status::OverQueryLimit,
    "REQUEST_DENIED" => Status::RequestDenied,
    "UNKNOWN_ERROR" => Status::UnknownError,
};

impl std::convert::TryFrom<&str> for Status {
    // Error definitions are contained in the
    // `google_maps\src\distance_matrix\error.rs` module.
    type Error = crate::distance_matrix::error::Error;
    /// Gets a `Status` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/distance-matrix/intro#top-level-status-codes)
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
    // `google_maps\src\distance_matrix\error.rs` module.
    type Err = crate::distance_matrix::error::Error;
    /// Gets a `Status` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/distance-matrix/intro#top-level-status-codes)
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
            Self::MaxElementsExceeded => write!(f, "Maximum Elements Exceeded"),
            Self::Ok => write!(f, "OK"),
            Self::OverDailyLimit => write!(f, "Over Daily Limit"),
            Self::OverQueryLimit => write!(f, "Over Query Limit"),
            Self::RequestDenied => write!(f, "Request Denied"),
            Self::UnknownError => write!(f, "Unknown Error"),
        } // match
    } // fn
} // impl
