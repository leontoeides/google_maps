//! Contains the `ElementStatus` enum and its associated traits. It indicates
//! the status resulting from operations on the specific individual elements.

use crate::distance_matrix::error::Error;
use serde::{Serialize, Deserialize};

/// The `status` fields within the response object contain the status of the
/// request, and may contain useful debugging information. The Distance Matrix
/// API returns a top-level status field, with information about the request in
/// general, as well as a status field for each element field, with information
/// about that particular origin-destination pairing.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
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

impl std::convert::TryFrom<&str> for ElementStatus {

    // Error definitions are contained in the
    // `google_maps\src\distance_matrix\error.rs` module.
    type Error = crate::distance_matrix::error::Error;

    /// Gets a `ElementStatus` enum from a `String` that contains a valid
    /// [element
    /// status](https://developers.google.com/maps/documentation/distance-matrix/intro#element-level-status-codes)
    /// code.
    fn try_from(element_status: &str) -> Result<ElementStatus, Error> {
        match element_status {
            "MAX_ROUTE_LENGTH_EXCEEDED" => Ok(ElementStatus::MaxRouteLengthExceeded),
            "NOT_FOUND" => Ok(ElementStatus::NotFound),
            "OK" => Ok(ElementStatus::Ok),
            "ZERO_RESULTS" => Ok(ElementStatus::ZeroResults),
            _ => Err(Error::InvalidElementStatusCode(element_status.to_string())),
        } // match
    } // fn

} // impl

impl std::default::Default for ElementStatus {
    /// Returns a reasonable default variant for the `ElementStatus` enum type.
    fn default() -> Self {
        ElementStatus::Ok
    } // fn
} // impl

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