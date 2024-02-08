//! Contains the `ElementStatus` enum and its associated traits. It indicates
//! the status resulting from operations on the specific individual elements.

use crate::distance_matrix::error::Error;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------

/// The `status` fields within the response object contain the status of the
/// request, and may contain useful debugging information. The Distance Matrix
/// API returns a top-level status field, with information about the request in
/// general, as well as a status field for each element field, with information
/// about that particular origin-destination pairing.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ElementStatus {
    /// Indicates the requested route is too long and cannot be processed.
    MaxRouteLengthExceeded = 0,
    /// Indicates that the origin and/or destination of this pairing could not
    /// be geocoded.
    NotFound = 1,
    /// Indicates the response contains a valid result.
    #[default]
    Ok = 2,
    /// Indicates no route could be found between the origin and destination.
    ZeroResults = 3,
} // struct

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for ElementStatus {
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

impl Serialize for ElementStatus {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&ElementStatus> for &str {
    /// Converts a `ElementStatus` enum to a `String` that contains a [element
    /// status](https://developers.google.com/maps/documentation/distance-matrix/intro#element-level-status-codes)
    /// code.
    fn from(element_status: &ElementStatus) -> Self {
        match element_status {
            ElementStatus::MaxRouteLengthExceeded => "MAX_ROUTE_LENGTH_EXCEEDED",
            ElementStatus::NotFound => "NOT_FOUND",
            ElementStatus::Ok => "OK",
            ElementStatus::ZeroResults => "ZERO_RESULTS",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for ElementStatus {
    /// Converts a `ElementStatus` enum to a `String` that contains a [element
    /// status](https://developers.google.com/maps/documentation/distance-matrix/intro#element-level-status-codes)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&ElementStatus> for String {
    /// Converts a `ElementStatus` enum to a `String` that contains a [element
    /// status](https://developers.google.com/maps/documentation/distance-matrix/intro#element-level-status-codes)
    /// code.
    fn from(element_status: &ElementStatus) -> Self {
        std::convert::Into::<&str>::into(element_status).to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

static ELEMENT_STATUSES_BY_CODE: phf::Map<&'static str, ElementStatus> = phf_map! {
    "MAX_ROUTE_LENGTH_EXCEEDED" => ElementStatus::MaxRouteLengthExceeded,
    "NOT_FOUND" => ElementStatus::NotFound,
    "OK" => ElementStatus::Ok,
    "ZERO_RESULTS" => ElementStatus::ZeroResults,
};

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for ElementStatus {
    // Error definitions are contained in the
    // `google_maps\src\distance_matrix\error.rs` module.
    type Error = crate::distance_matrix::error::Error;
    /// Gets a `ElementStatus` enum from a `String` that contains a valid
    /// [element
    /// status](https://developers.google.com/maps/documentation/distance-matrix/intro#element-level-status-codes)
    /// code.
    fn try_from(element_status_code: &str) -> Result<Self, Self::Error> {
        ELEMENT_STATUSES_BY_CODE
            .get(element_status_code)
            .cloned()
            .ok_or_else(|| Error::InvalidElementStatusCode(element_status_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for ElementStatus {
    // Error definitions are contained in the
    // `google_maps\src\distance_matrix\error.rs` module.
    type Err = crate::distance_matrix::error::Error;
    /// Gets a `ElementStatus` enum from a `String` that contains a valid
    /// [element
    /// status](https://developers.google.com/maps/documentation/distance-matrix/intro#element-level-status-codes)
    /// code.
    fn from_str(element_status_code: &str) -> Result<Self, Self::Err> {
        ELEMENT_STATUSES_BY_CODE
            .get(element_status_code)
            .cloned()
            .ok_or_else(|| Error::InvalidElementStatusCode(element_status_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl ElementStatus {
    /// Formats a `ElementStatus` enum into a string that is presentable to the
    /// end user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::MaxRouteLengthExceeded => "Maximum Route Length Exceeded",
            Self::NotFound => "Not Found",
            Self::Ok => "OK",
            Self::ZeroResults => "Zero Results",
        } // match
    } // fn
} // impl
