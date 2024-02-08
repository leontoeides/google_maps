//! Contains the `SortOrder` enum and its associated traits. It is used to
//! specify the sort order of reviews in the place details.

use crate::error::Error as GoogleMapsError;
use crate::places::error::Error as PlacesError;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------
//
/// The sorting method to use when returning reviews. Google recommends that you
/// display how the reviews are being sorted to the end user.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SortOrder {
    /// **Default** Reviews are sorted by relevance; the service will bias the
    /// results to return reviews originally written in the preferred language.
    #[default]
    MostRelevant = 0,
    /// Reviews are sorted in chronological order; the preferred language does
    /// not affect the sort order.
    Newest = 1,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for SortOrder {
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

impl Serialize for SortOrder {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&SortOrder> for &str {
    /// Converts a `SortOrder` enum to a `String` that contains a
    /// [field](https://developers.google.com/maps/documentation/places/web-service/details#fields)
    /// code.
    fn from(field: &SortOrder) -> Self {
        match field {
            SortOrder::MostRelevant => "most_relevant",
            SortOrder::Newest => "newest",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for SortOrder {
    /// Converts a `SortOrder` enum to a `String` that contains a
    /// [field](https://developers.google.com/maps/documentation/places/web-service/details#fields)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&SortOrder> for String {
    /// Converts a `SortOrder` enum to a `String` that contains a
    /// [field](https://developers.google.com/maps/documentation/places/web-service/details#fields)
    /// code.
    fn from(sort_order: &SortOrder) -> Self {
        std::convert::Into::<&str>::into(sort_order).to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

static SORT_ORDER_TYPES_BY_CODE: phf::Map<&'static str, SortOrder> = phf_map! {
    // Basic
    "most_relevant" => SortOrder::MostRelevant,
    "newest" => SortOrder::Newest,
};

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for SortOrder {
    // Error definitions are contained in the `google_maps\src\places\error.rs` module.
    type Error = GoogleMapsError;
    /// Gets a `SortOrder` enum from a `String` that contains a supported
    /// [sort order](https://developers.google.com/maps/documentation/places/web-service/details#reviews_sort)
    /// code.
    fn try_from(sort_order_code: &str) -> Result<Self, Self::Error> {
        Ok(SORT_ORDER_TYPES_BY_CODE
            .get(sort_order_code)
            .cloned()
            .ok_or_else(|| PlacesError::InvalidSortOrderCode(sort_order_code.to_string()))?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for SortOrder {
    // Error definitions are contained in the `google_maps\src\places\error.rs` module.
    type Err = GoogleMapsError;
    /// Gets a `SortOrder` enum from a `String` that contains a supported
    /// [sort order](https://developers.google.com/maps/documentation/places/web-service/details#reviews_sort)
    /// code.
    fn from_str(sort_order_code: &str) -> Result<Self, Self::Err> {
        Ok(SORT_ORDER_TYPES_BY_CODE
            .get(sort_order_code)
            .cloned()
            .ok_or_else(|| PlacesError::InvalidSortOrderCode(sort_order_code.to_string()))?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl SortOrder {
    /// Formats a `SortOrder` enum into a string that is presentable to the end
    /// user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::MostRelevant => "Most Relevant",
            Self::Newest => "Newest",
        } // match
    } // fn
} // impl
