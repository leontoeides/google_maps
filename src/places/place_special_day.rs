//! An object describing the opening hours of a place on special days or
//! holidays.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// An object describing the opening hours of a place on special days or
/// holidays.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PlaceSpecialDay {
    /// A date expressed in RFC3339 format in the local timezone for the place,
    /// for example 2010-12-31.
    pub date: Option<NaiveDate>,

    /// True if there are exceptional hours for this day. If `true`, this means
    /// that there is at least one exception for this day. Exceptions cause
    /// different values to occur in the subfields of `current_opening_hours`
    /// and `secondary_opening_hours` such as `periods`, `weekday_text`,
    /// `open_now`. The exceptions apply to the hours, and the hours are used to
    /// generate the other fields.
    pub exceptional_hours: Option<bool>,
} // struct PlaceSpecialDay

// -----------------------------------------------------------------------------

impl std::str::FromStr for PlaceSpecialDay {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Places API JSON response into a usable
    /// `PlaceSpecialDay` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    } // fn from_str
} // impl FromStr
