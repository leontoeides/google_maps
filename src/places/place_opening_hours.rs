//! An object describing the opening hours of a place.

use crate::places::{PlaceOpeningHoursPeriod, PlaceSpecialDay, SecondaryHoursType};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// -----------------------------------------------------------------------------
//
/// An object describing the opening hours of a place.

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PlaceOpeningHours {
    /// A boolean value indicating if the place is open at the current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_now: Option<bool>,

    /// An array of opening periods covering seven days, starting from Sunday,
    /// in chronological order.
    ///
    /// See [PlaceOpeningHoursPeriod](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceOpeningHoursPeriod)
    /// for more information.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub periods: Vec<PlaceOpeningHoursPeriod>,

    /// An array of up to seven entries corresponding to the next seven days.
    ///
    /// See [PlaceSpecialDay](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceSpecialDay)
    /// for more information.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub special_days: Vec<PlaceSpecialDay>,

    /// A type string used to identify the type of secondary hours (for example,
    /// `DRIVE_THROUGH`, `HAPPY_HOUR`, `DELIVERY`, `TAKEOUT`, `KITCHEN`,
    /// `BREAKFAST`, `LUNCH`, `DINNER`, `BRUNCH`, `PICKUP`, `SENIOR_HOURS`). Set
    /// for `secondary_opening_hours` only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_hours_type: Option<SecondaryHoursType>,

    /// An array of strings describing in human-readable text the hours of the
    /// place.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub weekday_text: Vec<String>,
} // struct PlaceOpeningHours

// -----------------------------------------------------------------------------

impl std::str::FromStr for PlaceOpeningHours {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Places API JSON response into a usable
    /// `PlaceOpeningHours` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    } // fn from_str
} // impl FromStr

// -----------------------------------------------------------------------------
//
/// A helper function that returns the dates of upcoming special days for a
/// place. This is meant to be used with the `Place.current_opening_hours`
/// field. Using this with the `Place.current_opening_hours` will likely just
/// return an empty `HashSet`.

impl PlaceOpeningHours {
    #[must_use]
    pub fn special_days(&self) -> HashSet<NaiveDate> {
        self.special_days
            .iter()
            .filter_map(|place_special_day| place_special_day.date)
            .collect::<HashSet<NaiveDate>>()
    } // fn
} // impl
