use serde::{Serialize, Deserialize};
use time::PrimitiveDateTime;

/// A representation of time as a Date object, a localized string, and a time
/// zone.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Time {

    /// A string representing the time's value. The time is displayed in the
    /// time zone of the transit stop.
    text: String,

    /// The time zone in which this stop lies. The value is the name of the time
    /// zone as defined in the [IANA Time Zone
    /// Database](http://www.iana.org/time-zones), e.g. "America/New_York".
    time_zone: String,

    /// The time of this departure or arrival.
    value: u32,

} // struct