//! Contains the `DurationUnit` enum and its associated traits.
//! `DurationTimeUnit` specifies a unit of time that is used to express a
//! duration, which is a measure of the time difference between two events.

use serde::{Serialize, Deserialize};

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

impl DurationUnit {
    /// Converts a `DurationUnit` enum to a `String` that contains an
    /// abbreviation for unit of time name.
    fn to_abbr(duration_unit: DurationUnit) -> String {
        match duration_unit {
            DurationUnit::Centuries => String::from("c."),
            DurationUnit::Days => String::from("d."),
            DurationUnit::Decades => String::from("dec."),
            DurationUnit::Hours => String::from("hr."),
            DurationUnit::Microseconds => String::from("µsec."),
            DurationUnit::Millenia => String::from("ml."),
            DurationUnit::Milliseconds => String::from("msec."),
            DurationUnit::Minutes => String::from("min."),
            DurationUnit::Months => String::from("mo."),
            DurationUnit::Nanoseconds => String::from("nsec."),
            DurationUnit::Seconds => String::from("sec."),
            DurationUnit::Weeks => String::from("w."),
            DurationUnit::WorkWeeks => String::from("ww."),
            DurationUnit::Years => String::from("yr."),
        } // match
    } // fn

    /// Converts a `DurationUnit` enum to a `String` that contains the formal
    /// symbol for the unit of time. If there is no authoritatively defined
    /// symbol for the unit of time, `None` returned.
    fn to_symbol(duration_unit: DurationUnit) -> Option<String> {
        match duration_unit {
            DurationUnit::Centuries => None,
            DurationUnit::Days => Some(String::from("d")),
            DurationUnit::Decades => None,
            DurationUnit::Hours => Some(String::from("h")),
            DurationUnit::Microseconds => Some(String::from("µs")),
            DurationUnit::Millenia => None,
            DurationUnit::Milliseconds => Some(String::from("ms")),
            DurationUnit::Minutes => Some(String::from("min")),
            DurationUnit::Months => None,
            DurationUnit::Nanoseconds => Some(String::from("ns")),
            DurationUnit::Seconds => Some(String::from("s")),
            DurationUnit::Weeks => None,
            DurationUnit::WorkWeeks => None,
            DurationUnit::Years => Some(String::from("a")),
        } // match
    } // fn
} // impl

impl std::convert::From<&DurationUnit> for String {
    /// Converts a `DurationUnit` enum to a `String` that contains a unit of
    /// time name.
    fn from(duration_unit: &DurationUnit) -> String {
        match duration_unit {
            DurationUnit::Centuries => String::from("Centuries"),
            DurationUnit::Days => String::from("Days"),
            DurationUnit::Decades => String::from("Decades"),
            DurationUnit::Hours => String::from("Hours"),
            DurationUnit::Microseconds => String::from("Microseconds"),
            DurationUnit::Millenia => String::from("Millenia"),
            DurationUnit::Milliseconds => String::from("Milliseconds"),
            DurationUnit::Minutes => String::from("Minutes"),
            DurationUnit::Months => String::from("Months"),
            DurationUnit::Nanoseconds => String::from("Nanoseconds"),
            DurationUnit::Seconds => String::from("Seconds"),
            DurationUnit::Weeks => String::from("Weeks"),
            DurationUnit::WorkWeeks => String::from("Work Weeks"),
            DurationUnit::Years => String::from("Years"),
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