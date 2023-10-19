//! The `"status"` field within the Places API _Place Search_ response
//! objects contain the status of the request, and may contain debugging
//! information to help your request is not working.

use crate::places::error::Error;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize};

// -----------------------------------------------------------------------------

/// Indicates the status of the response.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Status {
    /// Indicates that the request was successful.
    #[serde(alias = "OK")]
    Ok,
    /// Indicates that the search was successful but returned no results. This
    /// may occur if the search was passed a `latlng` in a remote location.
    #[serde(alias = "ZERO_RESULTS")]
    ZeroResults,
    /// Indicates the API request was malformed, generally due to missing
    /// required query parameter (`location` or `radius`).
    #[serde(alias = "INVALID_REQUEST")]
    InvalidRequest,
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
    /// * The `key` parameter is invalid.
    #[serde(alias = "REQUEST_DENIED")]
    RequestDenied,
    /// Indicates an unknown error.
    #[serde(alias = "UNKNOWN_ERROR")]
    UnknownError,
    /// Indicates that that the referenced location, `place_id`, was not found
    /// in the Places database.
    #[serde(alias = "NOT_FOUND")]
    NotFound,
} // struct

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for Status {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match Status::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string())),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Status> for String {
    /// Converts a `Status` enum to a `String` that contains a
    /// [status](https://developers.google.com/maps/documentation/places/web-service/search-text#PlacesSearchStatus)
    /// code.
    fn from(status: &Status) -> String {
        match status {
            Status::InvalidRequest => String::from("INVALID_REQUEST"),
            Status::Ok => String::from("OK"),
            Status::OverQueryLimit => String::from("OVER_QUERY_LIMIT"),
            Status::RequestDenied => String::from("REQUEST_DENIED"),
            Status::UnknownError => String::from("UNKNOWN_ERROR"),
            Status::ZeroResults => String::from("ZERO_RESULTS"),
            Status::NotFound => String::from("NOT_FOUND"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static STATUSES_BY_CODE: phf::Map<&'static str, Status> = phf_map! {
    "INVALID_REQUEST" => Status::InvalidRequest,
    "OK" => Status::Ok,
    "OVER_QUERY_LIMIT" => Status::OverQueryLimit,
    "REQUEST_DENIED" => Status::RequestDenied,
    "UNKNOWN_ERROR" => Status::UnknownError,
    "ZERO_RESULTS" => Status::ZeroResults,
    "NOT_FOUND" => Status::NotFound,
};

impl std::convert::TryFrom<&str> for Status {
    // Error definitions are contained in the
    // `google_maps\src\places\error.rs` module.
    type Error = crate::places::error::Error;
    /// Gets a `Status` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/places/web-service/search-text#PlacesSearchStatus)
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
    // `google_maps\src\places\error.rs` module.
    type Err = crate::places::error::Error;
    /// Gets a `Status` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/places/web-service/search-text#PlacesSearchStatus)
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
            Status::NotFound => write!(f, "Not Found"),
        } // match
    } // fn
} // impl
