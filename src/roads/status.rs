//! The `"status"` field within the Roads API response object contains the
//! status of the request, and may contain debugging information to help you
//! track down why the Roads API is not working.

use crate::roads::error::Error;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize};

// -----------------------------------------------------------------------------

/// Indicates the status of the response.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Status {
    /// 1. Your API key is not valid or was not included in the request. Please
    ///    ensure that you've included the entire key, and that you've enabled
    ///    the API for this key.
    ///
    /// 2. Your request contained invalid arguments. The most likely causes of
    ///    this error are:
    ///
    /// * A problem with your path parameter.
    /// * Please ensure you have at least 1, and fewer than 100 points. Each
    ///   point should be a pair of numbers separated by a comma, such as:
    ///   `48.409114,-123.369158`. Points should be separated by a pipe: '|'.
    /// * Your request included an invalid `placeId`.
    /// * Your request included both `placeId`s and a `path`. Only one of these
    ///   parameters may be specified for each request.
    ///   This error will not be returned if a `placeId` is passed for a road
    ///   which no longer exists, or for a place which is not a road.
    #[serde(alias = "INVALID_ARGUMENT")]
    InvalidArgument,

    /// The request was denied for one or more of the following reasons:
    ///
    /// * The API key is missing or invalid.
    /// * Billing has not been enabled on your account.
    /// * A self-imposed usage cap has been exceeded.
    /// * The provided method of payment is no longer valid (for example, a
    ///   credit card has expired).
    ///
    /// In order to use Google Maps Platform products, billing must be enabled
    /// on your account, and all requests must include a valid API key. To fix
    /// this, take the following steps:
    ///
    /// * [Get an API key](https://developers.google.com/maps/documentation/roads/errors?hl=en#new-key)
    /// * [Enable billing](https://console.cloud.google.com/project/_/billing/enable)
    ///   on your account.
    /// * [Adjust your usage cap](https://developers.google.com/maps/documentation/roads/errors?hl=en#usage-cap)
    ///   to increase your daily limit (if applicable).
    #[serde(alias = "PERMISSION_DENIED")]
    PermissionDenied,

    /// Ensure that you are sending requests to `https://roads.googleapis.com/`
    /// and not `http://roads.googleapis.com/`.
    #[serde(alias = "NOT_FOUND")]
    NotFound,

    /// You have exceeded the request limit that you configured in the Google
    /// Cloud Platform Console. This limit is typically set as requests per day,
    /// requests per 100 seconds, and requests per 100 seconds per user. This
    /// limit should be configured to prevent a single or small group of users
    /// from exhausting your daily quota, while still allowing reasonable access
    /// to all users. See Capping API Usage to configure these limits.
    #[serde(alias = "RESOURCE_EXHAUSTED")]
    ResourceExhausted,
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
    /// Converts a `Status` enum to a `String` that contains a status
    /// code.
    fn from(status: &Status) -> Self {
        match status {
            Status::InvalidArgument => Self::from("INVALID_ARGUMENT"),
            Status::PermissionDenied => Self::from("PERMISSION_DENIED"),
            Status::NotFound => Self::from("NOT_FOUND"),
            Status::ResourceExhausted => Self::from("RESOURCE_EXHAUSTED"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static STATUSES_BY_CODE: phf::Map<&'static str, Status> = phf_map! {
    "INVALID_ARGUMENT" => Status::InvalidArgument,
    "PERMISSION_DENIED" => Status::PermissionDenied,
    "NOT_FOUND" => Status::NotFound,
    "RESOURCE_EXHAUSTED" => Status::ResourceExhausted,
};

impl std::convert::TryFrom<&str> for Status {
    // Error definitions are contained in the
    // `google_maps\src\roads\error.rs` module.
    type Error = crate::roads::error::Error;
    /// Gets a `Status` enum from a `String` that contains a valid status
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
    // `google_maps\src\roads\error.rs` module.
    type Err = crate::roads::error::Error;
    /// Gets a `Status` enum from a `String` that contains a valid status
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
        Self::InvalidArgument
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for Status {
    /// Formats a `Status` enum into a string that is presentable to the end
    /// user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidArgument => write!(f, "Invalid Argument"),
            Self::PermissionDenied => write!(f, "Permission Denied"),
            Self::NotFound => write!(f, "Not Found"),
            Self::ResourceExhausted => write!(f, "Resource Exhausted"),
        } // match
    } // fn
} // impl
