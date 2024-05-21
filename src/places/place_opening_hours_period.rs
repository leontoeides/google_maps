//! An object describing the opening hours of a place.

use chrono::Duration;
use crate::places::PlaceOpeningHoursPeriodDetail;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// An object describing the opening hours of a place.

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PlaceOpeningHoursPeriod {
    /// Contains a pair of day and time objects describing when the place opens.
    ///
    /// See [PlaceOpeningHoursPeriodDetail](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceOpeningHoursPeriodDetail)
    /// for more information.
    pub open: PlaceOpeningHoursPeriodDetail,

    /// May contain a pair of day and time objects describing when the place
    /// closes. If a place is always open, the close section will be missing
    /// from the response. Clients can rely on always-open being represented as
    /// an open period containing day with value 0 and time with value 0000, and
    /// no close.
    ///
    /// See [PlaceOpeningHoursPeriodDetail](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceOpeningHoursPeriodDetail)
    /// for more information.
    pub close: Option<PlaceOpeningHoursPeriodDetail>,
} // struct PlaceOpeningHoursPeriod

// -----------------------------------------------------------------------------
//
/// Returns the a `chrono::Duration` that describes how long the
/// `PlaceOpeningHoursPeriod` period is.
///
/// If the `close` field is empty then this method will return a `None`.

impl PlaceOpeningHoursPeriod {
    #[must_use] pub fn duration(&self) -> Option<Duration> {
        self.close.as_ref().map(|close| {
            let days: u32 = close.day.days_since(self.open.day);
            let mut duration = Duration::days(i64::from(days));
            duration += close.time - self.open.time;
            duration
        })
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for PlaceOpeningHoursPeriod {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Places API JSON response into a usable
    /// `PlaceOpeningHoursPeriod` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    } // fn from_str
} // impl FromStr
