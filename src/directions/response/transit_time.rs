use crate::{
    directions::response::transit_time_zone::TransitTimeZone,
    serde::unix_to_primitivedatetime::unix_to_primitivedatetime,
}; // use
use serde::{Serialize, Deserialize};
use time::PrimitiveDateTime;

/// A representation of time as a Date object, a localized string, and a time
/// zone.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TransitTime {

    /// A string representing the time's value. The time is displayed in the
    /// time zone of the transit stop.
    pub text: String,

    /// The time zone in which this stop lies. The value is the name of the time
    /// zone as defined in the [IANA Time Zone
    /// Database](http://www.iana.org/time-zones), e.g. "America/New_York".
    pub time_zone: TransitTimeZone,

    /// The time of this departure or arrival.
    #[serde(deserialize_with = "unix_to_primitivedatetime")]
    pub value: PrimitiveDateTime,

} // struct