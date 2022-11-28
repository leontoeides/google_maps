//! An object describing the opening hours of a place.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// An object describing the opening hours of a place.

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PlaceOpeningHoursPeriodDetail {

    /// A number from 0–6, corresponding to the days of the week, starting on
    /// Sunday. For example, 2 means Tuesday.
    pub day: u8,

    /// May contain a time of day in 24-hour hhmm format. Values are in the
    /// range 0000–2359. The time will be reported in the place’s time zone.
    pub time: String,

    /// A date expressed in RFC3339 format in the local timezone for the place,
    /// for example 2010-12-31.
    pub date: Option<NaiveDate>,

    /// True if a given period was truncated due to a seven-day cutoff, where
    /// the period starts before midnight on the date of the request and/or ends
    /// at or after midnight on the last day. This property indicates that the
    /// period for open or close can extend past this seven-day cutoff.
    pub truncated: Option<bool>,

} // struct PlaceOpeningHoursPeriodDetail

// -----------------------------------------------------------------------------

impl std::str::FromStr for PlaceOpeningHoursPeriodDetail {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Places API JSON response into a usable
    /// `PlaceOpeningHoursPeriodDetail` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    } // fn from_str
}  // impl FromStr