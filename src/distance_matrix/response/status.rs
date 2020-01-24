//! Contains the `Status` enum and its associated traits. `Status` may contain
//! debugging information to help you track down why the service request failed.

use crate::distance_matrix::error::Error;
use serde::{Serialize, Deserialize};

/// The `status` fields within the response object contain the status of the
/// request, and may contain useful debugging information. The Distance Matrix
/// API returns a top-level status field, with information about the request in
/// general, as well as a status field for each element field, with information
/// about that particular origin-destination pairing.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
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

    /// Indicates that the service denied use of the Distance Matrix service by
    /// your application.
    #[serde(alias = "REQUEST_DENIED")]
    RequestDenied,

    /// Indicates a Distance Matrix request could not be processed due to a
    /// server error. The request may succeed if you try again.
    #[serde(alias = "UNKNOWN_ERROR")]
    UnknownError,

} // enum

impl std::convert::From<&Status> for String {
    /// Converts a `Status` enum to a `String` that contains a
    /// [status](https://developers.google.com/maps/documentation/distance-matrix/intro#top-level-status-codes)
    /// code.
    fn from(status: &Status) -> String {
        match status {
            Status::InvalidRequest => String::from("INVALID_REQUEST"),
            Status::MaxElementsExceeded => String::from("MAX_ELEMENTS_EXCEEDED"),
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
    // `google_maps\src\distance_matrix\error.rs` module.
    type Error = crate::distance_matrix::error::Error;

    /// Gets a `Status` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/distance-matrix/intro#top-level-status-codes)
    /// code.
    fn try_from(status: String) -> Result<Status, Error> {
        match status.as_ref() {
            "INVALID_REQUEST" => Ok(Status::InvalidRequest),
            "MAX_ELEMENTS_EXCEEDED" => Ok(Status::MaxElementsExceeded),
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
            Status::MaxElementsExceeded => write!(f, "Maximum Elements Exceeded"),
            Status::Ok => write!(f, "OK"),
            Status::OverDailyLimit => write!(f, "Over Daily Limit"),
            Status::OverQueryLimit => write!(f, "Over Query Limit"),
            Status::RequestDenied => write!(f, "Request Denied"),
            Status::UnknownError => write!(f, "Unknown Error"),
        } // match
    } // fn
} // impl