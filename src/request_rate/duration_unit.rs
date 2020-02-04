//! Contains the `DurationUnit` enum and its associated traits.
//! `DurationUnit` specifies a unit of time that is used to express a duration,
//! which is a scale of measure of time between two events.

use serde::{Serialize, Deserialize};

/// A "Duration unit" is a "duration unit of time." By adjusting the scale of
/// time when presenting information to the user, the presentation can be made
/// easier to understand.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum DurationUnit {
    /// A century is a period of 100 years.
    Centuries,
    /// A day is the approximate time it takes for the Earth to complete one
    /// rotation. It is defined as exactly 86,400 seconds.
    Days,
    /// A decade is a period of 10 years.
    Decades,
    /// An hour is a unit of time equal to 60 minutes, or 3,600 seconds.
    Hours,
    /// A microsecond is one millionth of a second.
    Microseconds,
    /// A millenium is a period of 1,000 years.
    Millenia,
    /// A millisecond is one thousandth of a second.
    Milliseconds,
    /// A minute is a unit of time equal to 60 seconds.
    Minutes,
    /// A month is 1/12th of a year. In the Gregorian calendar, an average month
    /// has exactly 30.436875 days. It was originally based on the time it takes
    /// for the moon to rotate the Earth.
    Months,
    /// A nanosecond is one billionth of a second.
    Nanoseconds,
    /// A second is the base unit of time. Originally, it was based on the
    /// length of the day, but it has since been standardized based on the
    /// radiation wavelength of caesium-133.
    Seconds,
    /// A week is a period of 7 days.
    Weeks,
    /// A typical work week is considered to be 40 hours: 8 hours a day for 5
    /// days.
    WorkWeeks,
    /// In the Gregorian calendar, a year has on average 365.2425 days. It is
    /// based on the amount of time it takes for the Earth to rotate the sun.
    Years,
} // enum

impl std::convert::From<&DurationUnit> for String {
    /// Converts a `DurationUnit` enum to a `String` that contains a unit of
    /// time name.
    fn from(duration_unit: &DurationUnit) -> String {
        match duration_unit {
            DurationUnit::Centuries => String::from("centuries"),
            DurationUnit::Days => String::from("days"),
            DurationUnit::Decades => String::from("decades"),
            DurationUnit::Hours => String::from("hours"),
            DurationUnit::Microseconds => String::from("microseconds"),
            DurationUnit::Millenia => String::from("millenia"),
            DurationUnit::Milliseconds => String::from("milliseconds"),
            DurationUnit::Minutes => String::from("minutes"),
            DurationUnit::Months => String::from("months"),
            DurationUnit::Nanoseconds => String::from("nanoseconds"),
            DurationUnit::Seconds => String::from("seconds"),
            DurationUnit::Weeks => String::from("weeks"),
            DurationUnit::WorkWeeks => String::from("work weeks"),
            DurationUnit::Years => String::from("years"),
        } // match
    } // fn
} // impl

impl std::default::Default for DurationUnit {
    /// Returns a reasonable default variant for the `DurationUnit` enum type.
    /// The second is an SI unit of time.
    fn default() -> Self {
        DurationUnit::Seconds
    } // fn
} // impl

impl std::fmt::Display for DurationUnit {
    /// Formats an `DurationUnit` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    } // fn
} // impl