//! Contains the `ElementStatus` enum and its associated traits. It indicates
//! the status resulting from operations on the specific individual elements.

use crate::distance_matrix::error::Error;
use phf::phf_map;
use serde::{Deserialize, Serialize, Deserializer};

// -----------------------------------------------------------------------------

/// The `status` fields within the response object contain the status of the
/// request, and may contain useful debugging information. The Distance Matrix
/// API returns a top-level status field, with information about the request in
/// general, as well as a status field for each element field, with information
/// about that particular origin-destination pairing.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ElementStatus {
    /// Indicates the requested route is too long and cannot be processed.
    #[serde(alias = "MAX_ROUTE_LENGTH_EXCEEDED")]
    MaxRouteLengthExceeded,
    /// Indicates that the origin and/or destination of this pairing could not
    /// be geocoded.
    #[serde(alias = "NOT_FOUND")]
    NotFound,
    /// Indicates the response contains a valid result.
    #[serde(alias = "OK")]
    Ok,
    /// Indicates no route could be found between the origin and destination.
    #[serde(alias = "ZERO_RESULTS")]
    ZeroResults,
} // struct

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for ElementStatus {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match ElementStatus::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string()))
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&ElementStatus> for String {
    /// Converts a `ElementStatus` enum to a `String` that contains a [element
    /// status](https://developers.google.com/maps/documentation/distance-matrix/intro#element-level-status-codes)
    /// code.
    fn from(element_status: &ElementStatus) -> String {
        match element_status {
            ElementStatus::MaxRouteLengthExceeded => String::from("MAX_ROUTE_LENGTH_EXCEEDED"),
            ElementStatus::NotFound => String::from("NOT_FOUND"),
            ElementStatus::Ok => String::from("OK"),
            ElementStatus::ZeroResults => String::from("ZERO_RESULTS"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static ELEMENT_STATUSES_BY_CODE: phf::Map<&'static str, ElementStatus> = phf_map! {
    "MAX_ROUTE_LENGTH_EXCEEDED" => ElementStatus::MaxRouteLengthExceeded,
    "NOT_FOUND" => ElementStatus::NotFound,
    "OK" => ElementStatus::Ok,
    "ZERO_RESULTS" => ElementStatus::ZeroResults,
};

impl std::convert::TryFrom<&str> for ElementStatus {
    // Error definitions are contained in the
    // `google_maps\src\distance_matrix\error.rs` module.
    type Error = crate::distance_matrix::error::Error;
    /// Gets a `ElementStatus` enum from a `String` that contains a valid
    /// [element
    /// status](https://developers.google.com/maps/documentation/distance-matrix/intro#element-level-status-codes)
    /// code.
    fn try_from(element_status_code: &str) -> Result<ElementStatus, Error> {
        ELEMENT_STATUSES_BY_CODE
            .get(element_status_code)
            .cloned()
            .ok_or_else(|| Error::InvalidElementStatusCode(element_status_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for ElementStatus {
    /// Returns a reasonable default variant for the `ElementStatus` enum type.
    fn default() -> Self {
        ElementStatus::Ok
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for ElementStatus {
    /// Formats a `ElementStatus` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ElementStatus::MaxRouteLengthExceeded => write!(f, "Maximum Route Length Exceeded"),
            ElementStatus::NotFound => write!(f, "Not Found"),
            ElementStatus::Ok => write!(f, "OK"),
            ElementStatus::ZeroResults => write!(f, "Zero Results"),
        } // match
    } // fn
} // impl
