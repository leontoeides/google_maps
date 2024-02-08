//! The `business_status` field within the _Places API_ _Place_ response
//! object indicates the operational status of the place, if it is a business.

use crate::error::Error as GoogleMapsError;
use crate::places::error::Error as PlacesError;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------
//
/// Indicates the operational status of the place, if it is a business. If no
/// data exists, `business_status` is not returned. The allowed values include:
/// `OPERATIONAL`, `CLOSED_TEMPORARILY`, and `CLOSED_PERMANENTLY`.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BusinessStatus {
    #[default]
    Operational,
    ClosedTemporarily,
    ClosedPermanently,
} // struct

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for BusinessStatus {
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

impl Serialize for BusinessStatus {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&BusinessStatus> for &str {
    /// Converts a `BusinessStatus` enum to a `String` that contains a
    /// [business status](https://developers.google.com/maps/documentation/places/web-service/search-text#Place-business_status)
    /// code.
    fn from(status: &BusinessStatus) -> Self {
        match status {
            BusinessStatus::Operational => "OPERATIONAL",
            BusinessStatus::ClosedTemporarily => "CLOSED_TEMPORARILY",
            BusinessStatus::ClosedPermanently => "CLOSED_PERMANENTLY",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for BusinessStatus {
    /// Converts a `BusinessStatus` enum to a `String` that contains a
    /// [business status](https://developers.google.com/maps/documentation/places/web-service/search-text#Place-business_status)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&BusinessStatus> for String {
    /// Converts a `BusinessStatus` enum to a `String` that contains a
    /// [business status](https://developers.google.com/maps/documentation/places/web-service/search-text#Place-business_status)
    /// code.
    fn from(business_status: &BusinessStatus) -> Self {
        std::convert::Into::<&str>::into(business_status).to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

static STATUSES_BY_CODE: phf::Map<&'static str, BusinessStatus> = phf_map! {
    "OPERATIONAL" => BusinessStatus::Operational,
    "CLOSED_TEMPORARILY" => BusinessStatus::ClosedTemporarily,
    "CLOSED_PERMANENTLY" => BusinessStatus::ClosedPermanently,
};

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for BusinessStatus {
    // Error definitions are contained in the
    // `google_maps\src\places\place_autocomplete\error.rs` module.
    type Error = GoogleMapsError;
    /// Gets a `BusinessStatus` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlacesAutocompleteBusinessStatus)
    /// code.
    fn try_from(status_code: &str) -> Result<Self, Self::Error> {
        Ok(STATUSES_BY_CODE
            .get(status_code)
            .cloned()
            .ok_or_else(|| PlacesError::InvalidBusinessStatusCode(status_code.to_string()))?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for BusinessStatus {
    // Error definitions are contained in the
    // `google_maps\src\places\place_autocomplete\error.rs` module.
    type Err = GoogleMapsError;
    /// Gets a `BusinessStatus` enum from a `String` that contains a valid
    /// [status](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlacesAutocompleteBusinessStatus)
    /// code.
    fn from_str(status_code: &str) -> Result<Self, Self::Err> {
        Ok(STATUSES_BY_CODE
            .get(status_code)
            .cloned()
            .ok_or_else(|| PlacesError::InvalidBusinessStatusCode(status_code.to_string()))?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl BusinessStatus {
    /// Formats a `BusinessStatus` enum into a string that is presentable to the
    /// end user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::Operational => "Operational",
            Self::ClosedTemporarily => "Closed Temporarily",
            Self::ClosedPermanently => "Closed Permanently",
        } // match
    } // fn
} // impl
