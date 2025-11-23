#![allow(clippy::ref_option, clippy::trivially_copy_pass_by_ref)]

use jiff::civil::Date as JiffDate;

// -------------------------------------------------------------------------------------------------
//
/// Represents a whole or partial calendar date, such as a birthday.
/// 
/// The time of day and time zone are either specified elsewhere or are insignificant. The date is
/// relative to the Gregorian Calendar. This can represent one of the following:
/// 
/// - A full date, with non-zero year, month, and day values.
/// - A month and day, with a zero year (for example, an anniversary).
/// - A year on its own, with a zero month and a zero day.
/// - A year and month, with a zero day (for example, a credit card expiration date).
/// 
/// Related types:
/// - google.type.TimeOfDay
/// - google.type.DateTime  
/// - [google.protobuf.Timestamp](https://protobuf.dev/reference/protobuf/google.protobuf/#timestamp)
#[derive(
    //std
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
    // getset
    getset::CopyGetters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    /// Year of the date.
    /// 
    /// Must be from `1` to `9_999`, or `0` to specify a date without a year.
    #[serde(
        default,
        serialize_with = "crate::places_new::serde::serialize_optional_year",
        deserialize_with = "crate::places_new::serde::deserialize_optional_year"
    )]
    #[getset(get_copy = "pub")]
    pub year: Option<i16>,

    /// Month of a year.
    ///
    /// Must be from `1` to `12`, or `0` to specify a year without a month and day.
    #[serde(
        default,
        serialize_with = "crate::places_new::serde::serialize_optional_month",
        deserialize_with = "crate::places_new::serde::deserialize_optional_month"
    )]
    #[getset(get_copy = "pub")]
    pub month: Option<i8>,

    /// Day of a month.
    ///
    /// Must be from `1` to `31` and valid for the year and month, or `0` to specify a year by
    /// itself or a year and month where the day isn't significant.
    #[serde(
        default,
        serialize_with = "crate::places_new::serde::serialize_optional_day",
        deserialize_with = "crate::places_new::serde::deserialize_optional_day"
    )]
    #[getset(get_copy = "pub")]
    pub day: Option<i8>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Date {
    /// Returns whether this date has a year component.
    #[must_use]
    pub const fn has_year(&self) -> bool {
        self.year.is_some()
    }

    /// Returns whether this date has a month component.
    #[must_use]
    pub const fn has_month(&self) -> bool {
        self.month.is_some()
    }

    /// Returns whether this date has a day component.
    #[must_use]
    pub const fn has_day(&self) -> bool {
        self.day.is_some()
    }

    /// Returns whether this is a complete date (has year, month, and day).
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.has_year() && self.has_month() && self.has_day()
    }

    /// Returns whether this date is completely empty (no components specified).
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        !self.has_year() && !self.has_month() && !self.has_day()
    }

    /// Attempts to convert this `Date` to a `jiff::civil::Date`.
    ///
    /// Returns a `Date` if this Date has complete year, month, and day information and represents a
    /// valid calendar date. Returns `None` for incomplete dates.
    ///
    /// # Errors
    /// Returns an error if the date components form an invalid date (e.g., February 30th).
    pub fn try_to_jiff_date(&self) -> Result<Option<JiffDate>, jiff::Error> {
        Option::<JiffDate>::try_from(self)
    }

    /// Gets a `jiff::civil::Date` if this is a complete and valid date.
    ///
    /// This is a convenience method that returns `None` if the conversion fails rather than
    /// returning an error.
    #[must_use]
    pub fn to_jiff_date(&self) -> Option<JiffDate> {
        self.try_to_jiff_date().ok().flatten()
    }

    /// Creates a Date from a `jiff::civil::Date`.
    #[must_use]
    pub fn from_jiff_date(jiff_date: &JiffDate) -> Self {
        Self::from(jiff_date)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl From<&JiffDate> for Date {
    /// Converts a `jiff::civil::Date` to Google Maps Places (new) `Date`.
    fn from(jiff_date: &JiffDate) -> Self {
        Self {
            year: Some(jiff_date.year()),
            month: Some(jiff_date.month()),
            day: Some(jiff_date.day()),
        }
    }
}

impl TryFrom<&Date> for Option<JiffDate> {
    type Error = jiff::Error;

    /// Converts a Google Maps Places (new) `Date` to an `Option<jiff::civil::Date>`.
    ///
    /// Requires the Date to have complete year, month, and day components that form a valid
    /// calendar date or this will return `None`.
    ///
    /// # Errors
    /// Returns an error if the components form an invalid date.
    fn try_from(date: &Date) -> Result<Self, Self::Error> {
        if let (Some(year), Some(month), Some(day)) = (date.year, date.month, date.day) {
            Ok(Some(JiffDate::new(year, month, day)?))
        } else {
            Ok(None)
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Serialize, Deserialize)]
    struct TestDate {
        #[serde(
            serialize_with = "crate::places_new::serde::serialize_optional_year",
            deserialize_with = "crate::places_new::serde::deserialize_optional_year"
        )]
        year: Option<i16>,

        #[serde(
            serialize_with = "crate::places_new::serde::serialize_optional_month",
            deserialize_with = "crate::places_new::serde::deserialize_optional_month"
        )]
        month: Option<i8>,

        #[serde(
            serialize_with = "crate::places_new::serde::serialize_optional_day",
            deserialize_with = "crate::places_new::serde::deserialize_optional_day"
        )]
        day: Option<i8>,
    }

    #[test]
    fn test_date_serialization() {
        // Full date
        let date = TestDate {
            year: Some(2024),
            month: Some(3),
            day: Some(15),
        };
        let json = serde_json::to_string(&date).unwrap();
        assert_eq!(json, r#"{"year":2024,"month":3,"day":15}"#);

        // Partial date (anniversary)
        let anniversary = TestDate {
            year: None,
            month: Some(3),
            day: Some(15),
        };
        let json = serde_json::to_string(&anniversary).unwrap();
        assert_eq!(json, r#"{"year":0,"month":3,"day":15}"#);

        // Year only
        let year_only = TestDate {
            year: Some(2024),
            month: None,
            day: None,
        };
        let json = serde_json::to_string(&year_only).unwrap();
        assert_eq!(json, r#"{"year":2024,"month":0,"day":0}"#);
    }

    #[test]
    fn test_date_deserialization() {
        // Full date
        let json = r#"{"year":2024,"month":3,"day":15}"#;
        let date: TestDate = serde_json::from_str(json).unwrap();
        assert_eq!(date.year, Some(2024));
        assert_eq!(date.month, Some(3));
        assert_eq!(date.day, Some(15));

        // Partial date with zeros
        let json = r#"{"year":0,"month":3,"day":15}"#;
        let date: TestDate = serde_json::from_str(json).unwrap();
        assert_eq!(date.year, None);
        assert_eq!(date.month, Some(3));
        assert_eq!(date.day, Some(15));
    }

    #[test]
    fn test_try_into_jiff_date_complete() {
        let date = Date {
            year: Some(2024),
            month: Some(3),
            day: Some(15),
        };

        let result = date.try_to_jiff_date().unwrap();
        let jiff_date = result.unwrap();
        assert_eq!(jiff_date.year(), 2024);
        assert_eq!(jiff_date.month(), 3);
        assert_eq!(jiff_date.day(), 15);
    }

    #[test]
    fn test_try_into_jiff_date_incomplete() {
        let date = Date {
            year: Some(2024),
            month: Some(3),
            day: None,
        };

        let result = date.try_to_jiff_date().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_try_into_jiff_date_invalid() {
        let date = Date {
            year: Some(2024),
            month: Some(2),
            day: Some(30), // Invalid: February 30th
        };

        let result = date.try_to_jiff_date();
        assert!(result.is_err());
    }
}