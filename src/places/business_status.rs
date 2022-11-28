//! The `"business_status"` field within the _Places API_ _Place_ response
//! object indicates the operational status of the place, if it is a business.

use crate::places::error::Error;
use phf::phf_map;
use serde::{Deserialize, Serialize, Deserializer};

// -----------------------------------------------------------------------------
//
/// Indicates the operational status of the place, if it is a business. If no
/// data exists, business_status is not returned. The allowed values include:
/// `OPERATIONAL`, `CLOSED_TEMPORARILY`, and `CLOSED_PERMANENTLY`.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BusinessStatus {
    #[serde(alias = "OPERATIONAL")]
    Operational,
    #[serde(alias = "CLOSED_TEMPORARILY")]
    ClosedTemporarily,
    #[serde(alias = "CLOSED_PERMANENTLY")]
    ClosedPermanently,
} // struct

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for BusinessStatus {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match BusinessStatus::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string()))
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&BusinessStatus> for String {
    /// Converts a `BusinessStatus` enum to a `String` that contains a
    /// [business status](https://developers.google.com/maps/documentation/places/web-service/search-text#Place-business_status)
    /// code.
    fn from(status: &BusinessStatus) -> String {
        match status {
            BusinessStatus::Operational => String::from("OPERATIONAL"),
            BusinessStatus::ClosedTemporarily => String::from("CLOSED_TEMPORARILY"),
            BusinessStatus::ClosedPermanently => String::from("CLOSED_PERMANENTLY"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static STATUSES_BY_CODE: phf::Map<&'static str, BusinessStatus> = phf_map! {
    "OPERATIONAL" => BusinessStatus::Operational,
    "CLOSED_TEMPORARILY" => BusinessStatus::ClosedTemporarily,
    "CLOSED_PERMANENTLY" => BusinessStatus::ClosedPermanently,
};

impl std::convert::TryFrom<&str> for BusinessStatus {
    // Error definitions are contained in the
    // `google_maps\src\places\place_autocomplete\error.rs` module.
    type Error = crate::places::error::Error;
    /// Gets a `BusinessStatus` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlacesAutocompleteBusinessStatus)
    /// code.
    fn try_from(status_code: &str) -> Result<Self, Self::Error> {
        STATUSES_BY_CODE
            .get(status_code)
            .cloned()
            .ok_or_else(|| Error::InvalidBusinessStatusCode(status_code.to_string()))
    } // fn
} // impl

impl std::str::FromStr for BusinessStatus {
    // Error definitions are contained in the
    // `google_maps\src\places\place_autocomplete\error.rs` module.
    type Err = crate::places::error::Error;
    /// Gets a `BusinessStatus` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlacesAutocompleteBusinessStatus)
    /// code.
    fn from_str(status_code: &str) -> Result<Self, Self::Err> {
        STATUSES_BY_CODE
            .get(status_code)
            .cloned()
            .ok_or_else(|| Error::InvalidBusinessStatusCode(status_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for BusinessStatus {
    /// Returns a reasonable default variant for the `BusinessStatus` enum type.
    fn default() -> Self {
        BusinessStatus::Operational
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for BusinessStatus {
    /// Formats a `BusinessStatus` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BusinessStatus::Operational => write!(f, "Operational"),
            BusinessStatus::ClosedTemporarily => write!(f, "Closed Temporarily"),
            BusinessStatus::ClosedPermanently => write!(f, "Closed Permanently"),
        } // match
    } // fn
} // impl