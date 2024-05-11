//! The `"type"` field within the _Places API_ _`PlaceOpeningHours`_ response
//! object describing the opening hours of a place.

use crate::error::Error as GoogleMapsError;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------
//
/// A type string used to identify the type of secondary hours (for example,
/// `DRIVE_THROUGH`, `HAPPY_HOUR`, `DELIVERY`, `TAKEOUT`, `KITCHEN`,
/// `BREAKFAST`, `LUNCH`, `DINNER`, `BRUNCH`, `PICKUP`, `SENIOR_HOURS`). Set for
/// `secondary_opening_hours` only.

#[derive(Clone, Debug, Eq, Default, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
#[non_exhaustive]
pub enum SecondaryHoursType {
    #[default]
    DriveThrough = 0,
    HappyHour = 1,
    Delivery = 2,
    Takeout = 3,
    Kitchen = 4,
    Breakfast = 5,
    Lunch = 6,
    Dinner = 7,
    Brunch = 8,
    Pickup = 9,
    SeniorHours = 10,
    /// If the hours type is not recognized by
    /// [serde](https://crates.io/crates/serde) when reading data from
    /// Google it will be assigned to this `Other` variant.
    ///
    /// As new types are added to Google Maps, they must also be added to this
    /// crate. However, in the meantime, the `Other` catch-all variant allows
    /// `serde` to read data from Google without producing an error until the
    /// new variant added to this `enum`.
    Other(String) = 11,
} // struct

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for SecondaryHoursType {
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

impl Serialize for SecondaryHoursType {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl<'a> std::convert::From<&'a SecondaryHoursType> for &'a str {
    /// Converts a `SecondaryHoursType` enum to a `String` that contains a
    /// [secondary hours type](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceOpeningHours-type)
    /// code.
    fn from(hours_type: &'a SecondaryHoursType) -> Self {
        match hours_type {
            SecondaryHoursType::DriveThrough => "DRIVE_THROUGH",
            SecondaryHoursType::HappyHour => "HAPPY_HOUR",
            SecondaryHoursType::Delivery => "DELIVERY",
            SecondaryHoursType::Takeout => "TAKEOUT",
            SecondaryHoursType::Kitchen => "KITCHEN",
            SecondaryHoursType::Breakfast => "BREAKFAST",
            SecondaryHoursType::Lunch => "LUNCH",
            SecondaryHoursType::Dinner => "DINNER",
            SecondaryHoursType::Brunch => "BRUNCH",
            SecondaryHoursType::Pickup => "PICKUP",
            SecondaryHoursType::SeniorHours => "SENIOR_HOURS",
            SecondaryHoursType::Other(string) => string,
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for SecondaryHoursType {
    /// Converts a `SecondaryHoursType` enum to a `String` that contains a
    /// [secondary hours type](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceOpeningHours-type)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&SecondaryHoursType> for String {
    /// Converts a `SecondaryHoursType` enum to a `String` that contains a
    /// [secondary hours type](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceOpeningHours-type)
    /// code.
    fn from(secondary_hours_type: &SecondaryHoursType) -> Self {
        std::convert::Into::<&str>::into(secondary_hours_type).to_string()
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

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for SecondaryHoursType {
    // Error definitions are contained in the
    // `google_maps\src\places\error.rs` module.
    type Error = GoogleMapsError;
    /// Gets a `SecondaryHoursType` enum from a `String` that contains a valid
    /// [secondary hours type](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceOpeningHours-type)
    /// code.
    fn try_from(hours_type: &str) -> Result<Self, Self::Error> {
        Ok(STATUSES_BY_CODE
            .get(hours_type)
            .cloned()
            .unwrap_or_else(|| Self::Other(hours_type.to_string())))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for SecondaryHoursType {
    // Error definitions are contained in the
    // `google_maps\src\places\error.rs` module.
    type Err = GoogleMapsError;
    /// Gets a `SecondaryHoursType` enum from a `String` that contains a valid
    /// [secondary hours type](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceOpeningHours-type)
    /// code.
    fn from_str(hours_type: &str) -> Result<Self, Self::Err> {
        Ok(STATUSES_BY_CODE
            .get(hours_type)
            .cloned()
            .unwrap_or_else(|| Self::Other(hours_type.to_string())))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl SecondaryHoursType {
    /// Formats a `SecondaryHoursType` enum into a string that is presentable to the
    /// end user.
    #[must_use]
    pub fn display(&self) -> &str {
        match self {
            Self::DriveThrough => "Drive Through",
            Self::HappyHour => "Happy Hour",
            Self::Delivery => "Delivery",
            Self::Takeout => "Takeout",
            Self::Kitchen => "Kitchen",
            Self::Breakfast => "Breakfast",
            Self::Lunch => "Lunch",
            Self::Dinner => "Dinner",
            Self::Brunch => "Brunch",
            Self::Pickup => "Pickup",
            Self::SeniorHours => "Senior Hours",
            Self::Other(string) => string,
        } // match
    } // fn
} // impl
