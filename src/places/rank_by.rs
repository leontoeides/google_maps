//! The `"type"` field within the _Places API_ _`PlaceOpeningHours`_ response
//! object describing the opening hours of a place.

use crate::error::Error as GoogleMapsError;
use crate::places::error::Error as PlacesError;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------
//
/// Specifies the order in which results are listed.

#[derive(Clone, Debug, Eq, Default, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum RankBy {
    /// This option sorts results based on their importance. Ranking will favor
    /// prominent places within the set radius over nearby places that match but
    /// that are less prominent. Prominence can be affected by a place's ranking
    /// in Google's index, global popularity, and other factors. When prominence
    /// is specified, the `radius` parameter is required.
    #[default]
    Prominence = 0,

    /// This option biases search results in ascending order by their distance
    /// from the specified location. When `distance` is specified, one or more
    /// of `keyword`, `name`, or `type` is required and radius is disallowed.
    Distance = 1,
} // struct

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for RankBy {
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

impl Serialize for RankBy {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&RankBy> for &str {
    /// Converts a `RankBy` enum to a `String` that contains a
    /// [rankby type](https://developers.google.com/maps/documentation/places/web-service/search-nearby#rankby)
    /// code.
    fn from(hours_type: &RankBy) -> Self {
        match hours_type {
            RankBy::Prominence => "prominence",
            RankBy::Distance => "distance",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for RankBy {
    /// Converts a `RankBy` enum to a `String` that contains a
    /// [rankby type](https://developers.google.com/maps/documentation/places/web-service/search-nearby#rankby)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&RankBy> for String {
    /// Converts a `RankBy` enum to a `String` that contains a
    /// [rankby type](https://developers.google.com/maps/documentation/places/web-service/search-nearby#rankby)
    /// code.
    fn from(secondary_hours_type: &RankBy) -> Self {
        std::convert::Into::<&str>::into(secondary_hours_type).to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

static STATUSES_BY_CODE: phf::Map<&'static str, RankBy> = phf_map! {
    "prominence" => RankBy::Prominence,
    "distance" => RankBy::Distance,
};

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for RankBy {
    // Error definitions are contained in the
    // `google_maps\src\places\error.rs` module.
    type Error = GoogleMapsError;
    /// Gets a `RankBy` enum from a `String` that contains a valid
    /// [rankby type](https://developers.google.com/maps/documentation/places/web-service/search-nearby#rankby)
    /// code.
    fn try_from(hours_type: &str) -> Result<Self, Self::Error> {
        Ok(STATUSES_BY_CODE
            .get(hours_type)
            .cloned()
            .ok_or_else(|| PlacesError::InvalidRankByCode(hours_type.to_string()))?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for RankBy {
    // Error definitions are contained in the
    // `google_maps\src\places\error.rs` module.
    type Err = GoogleMapsError;
    /// Gets a `RankBy` enum from a `String` that contains a valid
    /// [rankby type](https://developers.google.com/maps/documentation/places/web-service/search-nearby#rankby)
    /// code.
    fn from_str(hours_type: &str) -> Result<Self, Self::Err> {
        Ok(STATUSES_BY_CODE
            .get(hours_type)
            .cloned()
            .ok_or_else(|| PlacesError::InvalidRankByCode(hours_type.to_string()))?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl RankBy {
    /// Formats a `RankBy` enum into a string that is presentable to the
    /// end user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::Prominence => "Prominence",
            Self::Distance => "Distance",
        } // match
    } // fn
} // impl
