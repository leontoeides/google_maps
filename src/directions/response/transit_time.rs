//! A representation of time as a Date object, a localized string, and a time
//! zone.

use chrono::{naive::serde::ts_seconds, NaiveDateTime};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

/// A representation of time as a Date object, a localized string, and a time
/// zone.

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TransitTime {
    /// A string representing the time's value. The time is displayed in the
    /// time zone of the transit stop.
    pub text: String,
    /// The time zone in which this stop lies. The value is the name of the time
    /// zone as defined in the [IANA Time Zone
    /// Database](http://www.iana.org/time-zones), e.g. "`America/New_York`".
    pub time_zone: Tz,
    /// The time of this departure or arrival.
    #[serde(with = "ts_seconds")]
    pub value: NaiveDateTime,
} // struct
