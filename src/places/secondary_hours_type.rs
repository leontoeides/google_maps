//! The `"type"` field within the _Places API_ _PlaceOpeningHours_ response
//! object describing the opening hours of a place.

use crate::places::error::Error;
use phf::phf_map;
use serde::{Deserialize, Serialize, Deserializer};

// -----------------------------------------------------------------------------
//
/// A type string used to identify the type of secondary hours (for example,
/// `DRIVE_THROUGH`, `HAPPY_HOUR`, `DELIVERY`, `TAKEOUT`, `KITCHEN`,
/// `BREAKFAST`, `LUNCH`, `DINNER`, `BRUNCH`, `PICKUP`, `SENIOR_HOURS`). Set for
/// `secondary_opening_hours` only.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SecondaryHoursType {
    #[serde(alias = "DRIVE_THROUGH")]
    DriveThrough,
    #[serde(alias = "HAPPY_HOUR")]
    HappyHour,
    #[serde(alias = "DELIVERY")]
    Delivery,
    #[serde(alias = "TAKEOUT")]
    Takeout,
    #[serde(alias = "KITCHEN")]
    Kitchen,
    #[serde(alias = "BREAKFAST")]
    Breakfast,
    #[serde(alias = "LUNCH")]
    Lunch,
    #[serde(alias = "DINNER")]
    Dinner,
    #[serde(alias = "BRUNCH")]
    Brunch,
    #[serde(alias = "PICKUP")]
    Pickup,
    #[serde(alias = "SENIOR_HOURS")]
    SeniorHours,
} // struct

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for SecondaryHoursType {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match SecondaryHoursType::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string()))
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&SecondaryHoursType> for String {
    /// Converts a `SecondaryHoursType` enum to a `String` that contains a
    /// [secondary hours type](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceOpeningHours-type)
    /// code.
    fn from(hours_type: &SecondaryHoursType) -> String {
        match hours_type {
            SecondaryHoursType::DriveThrough => String::from("DRIVE_THROUGH"),
            SecondaryHoursType::HappyHour => String::from("HAPPY_HOUR"),
            SecondaryHoursType::Delivery => String::from("DELIVERY"),
            SecondaryHoursType::Takeout => String::from("TAKEOUT"),
            SecondaryHoursType::Kitchen => String::from("KITCHEN"),
            SecondaryHoursType::Breakfast => String::from("BREAKFAST"),
            SecondaryHoursType::Lunch => String::from("LUNCH"),
            SecondaryHoursType::Dinner => String::from("DINNER"),
            SecondaryHoursType::Brunch => String::from("BRUNCH"),
            SecondaryHoursType::Pickup => String::from("PICKUP"),
            SecondaryHoursType::SeniorHours => String::from("SENIOR_HOURS"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static STATUSES_BY_CODE: phf::Map<&'static str, SecondaryHoursType> = phf_map! {
    "DRIVE_THROUGH" => SecondaryHoursType::DriveThrough,
    "HAPPY_HOUR" => SecondaryHoursType::HappyHour,
    "DELIVERY" => SecondaryHoursType::Delivery,
    "TAKEOUT" => SecondaryHoursType::Takeout,
    "KITCHEN" => SecondaryHoursType::Kitchen,
    "BREAKFAST" => SecondaryHoursType::Breakfast,
    "LUNCH" => SecondaryHoursType::Lunch,
    "DINNER" => SecondaryHoursType::Dinner,
    "BRUNCH" => SecondaryHoursType::Brunch,
    "PICKUP" => SecondaryHoursType::Pickup,
    "SENIOR_HOURS" => SecondaryHoursType::SeniorHours,
};

impl std::convert::TryFrom<&str> for SecondaryHoursType {
    // Error definitions are contained in the
    // `google_maps\src\places\error.rs` module.
    type Error = crate::places::error::Error;
    /// Gets a `SecondaryHoursType` enum from a `String` that contains a valid
    /// [secondary hours type](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceOpeningHours-type)
    /// code.
    fn try_from(hours_type: &str) -> Result<Self, Self::Error> {
        STATUSES_BY_CODE
            .get(hours_type)
            .cloned()
            .ok_or_else(|| Error::InvalidSecondaryHoursType(hours_type.to_string()))
    } // fn
} // impl

impl std::str::FromStr for SecondaryHoursType {
    // Error definitions are contained in the
    // `google_maps\src\places\error.rs` module.
    type Err = crate::places::error::Error;
    /// Gets a `SecondaryHoursType` enum from a `String` that contains a valid
    /// [secondary hours type](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceOpeningHours-type)
    /// code.
    fn from_str(hours_type: &str) -> Result<Self, Self::Err> {
        STATUSES_BY_CODE
            .get(hours_type)
            .cloned()
            .ok_or_else(|| Error::InvalidSecondaryHoursType(hours_type.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for SecondaryHoursType {
    /// Returns a reasonable default variant for the `SecondaryHoursType` enum
    /// type.
    fn default() -> Self {
        SecondaryHoursType::DriveThrough
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for SecondaryHoursType {
    /// Formats a `SecondaryHoursType` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SecondaryHoursType::DriveThrough => write!(f, "Drive Through"),
            SecondaryHoursType::HappyHour => write!(f, "Happy Hour"),
            SecondaryHoursType::Delivery => write!(f, "Delivery"),
            SecondaryHoursType::Takeout => write!(f, "Takeout"),
            SecondaryHoursType::Kitchen => write!(f, "Kitchen"),
            SecondaryHoursType::Breakfast => write!(f, "Breakfast"),
            SecondaryHoursType::Lunch => write!(f, "Lunch"),
            SecondaryHoursType::Dinner => write!(f, "Dinner"),
            SecondaryHoursType::Brunch => write!(f, "Brunch"),
            SecondaryHoursType::Pickup => write!(f, "Pickup"),
            SecondaryHoursType::SeniorHours => write!(f, "Senior Hours"),
        } // match
    } // fn
} // impl