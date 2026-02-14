//! Contains the `DurationUnit` enum and its associated traits.
//! `DurationUnit` specifies a unit of time that is used to express a duration,
//! which is a scale of measure of time between two events.

use serde::{Deserialize, Serialize};

/// A "Duration unit" is a "duration unit of time." By adjusting the scale of
/// time when presenting information to the user, the presentation can be made
/// easier to understand.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum DurationUnit {
    /// A day is the approximate time it takes for the Earth to complete one
    /// rotation. It is defined as exactly 86,400 seconds.
    Days = 0,
    /// An hour is a unit of time equal to 60 minutes, or 3,600 seconds.
    Hours = 1,
    /// A millisecond is one thousandth of a second.
    Milliseconds = 2,
    /// A minute is a unit of time equal to 60 seconds.
    Minutes = 3,
    /// A month is 1/12th of a year. In the Gregorian calendar, an average month
    /// has exactly 30.436875 days. It was originally based on the time it takes
    /// for the moon to rotate the Earth.
    Months = 4,
    /// A second is the base unit of time. Originally, it was based on the
    /// length of the day, but it has since been standardized based on the
    /// radiation wavelength of caesium-133.
    Seconds = 5,
    /// A week is a period of 7 days.
    Weeks = 6,
    /// In the Gregorian calendar, a year has on average 365.2425 days. It is
    /// based on the amount of time it takes for the Earth to rotate the sun.
    Years = 7,
} // enum

impl std::convert::From<&DurationUnit> for String {
    /// Converts a `DurationUnit` enum to a `String` that contains a unit of
    /// time name.
    fn from(duration_unit: &DurationUnit) -> Self {
        match duration_unit {
            DurationUnit::Days => Self::from("days"),
            DurationUnit::Hours => Self::from("hours"),
            DurationUnit::Milliseconds => Self::from("milliseconds"),
            DurationUnit::Minutes => Self::from("minutes"),
            DurationUnit::Months => Self::from("months"),
            DurationUnit::Seconds => Self::from("seconds"),
            DurationUnit::Weeks => Self::from("weeks"),
            DurationUnit::Years => Self::from("years"),
        } // match
    } // fn
} // impl

impl std::default::Default for DurationUnit {
    /// Returns a reasonable default variant for the `DurationUnit` enum type.
    /// The second is an SI unit of time.
    fn default() -> Self {
        Self::Seconds
    } // fn
} // impl

impl std::fmt::Display for DurationUnit {
    /// Formats an `DurationUnit` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    } // fn
} // impl
