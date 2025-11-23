use crate::places_new::Error;
use jiff::civil::{DateTime, Time};

// -------------------------------------------------------------------------------------------------
//
/// Status changing points.
/// 
/// Points represent specific moments in time when the status of a place changes, such as opening
/// hours transitions or special day schedules. They include both date and time information to
/// define when these changes occur.
#[derive(
    //std
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    // getset
    getset::CopyGetters,
    getset::Getters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
pub struct Point {
    /// Date in the local timezone for the place.
    /// 
    /// This represents the calendar date when this status point occurs, expressed in the local
    /// timezone of the place rather than UTC.
    #[serde(
        default,
        deserialize_with = "crate::places_new::serde::deserialize_optional_jiff_date",
        serialize_with = "crate::places_new::serde::serialize_optional_jiff_date"
    )]
    #[getset(get_copy = "pub")]
    pub date: Option<jiff::civil::Date>,

    /// Whether or not this endpoint was truncated.
    /// 
    /// Truncation occurs when the real hours are outside the times we are willing to return hours
    /// between, so we truncate the hours back to these boundaries. This ensures that at most 24 * 7
    /// hours from midnight of the day of the request are returned.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub truncated: Option<bool>,

    /// A day of the week, as an integer in the range 0-6.
    /// 
    /// 0 is Sunday, 1 is Monday, etc.
    #[serde(
        serialize_with = "crate::places_new::serde::serialize_weekday",
        deserialize_with = "crate::places_new::serde::deserialize_weekday"
    )]
    #[getset(get_copy = "pub")]
    pub day: jiff::civil::Weekday,

    /// The hour in 24 hour format.
    /// 
    /// Ranges from 0 to 23, where 0 represents midnight and 23 represents 11 PM.
    #[getset(get_copy = "pub")]
    pub hour: i32,

    /// The minute.
    /// 
    /// Ranges from 0 to 59, representing the minute within the hour.
    #[getset(get_copy = "pub")]
    pub minute: i32,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Point {
    /// Returns whether this point has a date.
    #[must_use]
    pub const fn has_date(&self) -> bool {
        self.date.is_some()
    }

    /// Returns whether this point was truncated.
    #[must_use]
    pub fn is_truncated(&self) -> bool {
        self.truncated.unwrap_or(false)
    }

    /// Attempts to create a civil Time from the hour and minute information.
    ///
    /// Returns a `jiff::civil::Time` if both hour and minute are available and valid. Time is
    /// created with zero seconds and nanoseconds.
    ///
    /// # Errors
    /// Returns an error if hour is not 0-23 or minute is not 0-59.
    pub fn try_to_civil_time(&self) -> Result<Time, Error> {
        Ok(Time::new(i8::try_from(self.hour)?, i8::try_from(self.minute)?, 0, 0)?)
    }

    /// Attempts to create a civil `DateTime` from the available date and time information.
    ///
    /// Returns a `jiff::civil::DateTime` if date is available and hour/minute can form a valid
    /// time. If hour/minute are not available, defaults to 00:00:00.
    ///
    /// # Errors
    /// Returns an error if the date and time combination is invalid.
    pub fn try_to_civil_datetime(&self) -> Result<Option<DateTime>, Error> {
        if let Some(date) = self.date {
            let date_time = date.at(i8::try_from(self.hour)?, i8::try_from(self.minute)?, 0, 0);
            Ok(Some(date_time))
        } else {
            Ok(None)
        }
    }

    /// Gets a civil `Time` if hour and minute are available, with error handling.
    ///
    /// This is a convenience method that returns `None` if the time cannot be created rather than
    /// returning an error. Useful when you want to gracefully handle invalid time data.
    #[must_use]
    pub fn civil_time(&self) -> Option<Time> {
        self.try_to_civil_time().ok()
    }

    /// Gets a civil `DateTime` if date is available, with error handling.
    ///
    /// This is a convenience method that returns `None` if the `DateTime` cannot be created rather
    /// than returning an error. Useful when you want to gracefully handle invalid date/time data.
    #[must_use]
    pub fn civil_datetime(&self) -> Option<DateTime> {
        self.try_to_civil_datetime().ok().flatten()
    }
}