use crate::places_new::Date;

// -------------------------------------------------------------------------------------------------
//
/// Structured information for special days that fall within the period that the returned opening
/// hours cover.
/// 
/// Special days are days that could impact the business hours of a place, such as Christmas day,
/// New Year's Eve, or other holidays when a business might have different operating hours than
/// usual. These days help provide more accurate information about when a place will actually be
/// open or closed.
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
pub struct SpecialDay {
    /// The date of this special day.
    /// 
    /// This represents the specific calendar date when the special operating conditions apply. The
    /// date is in the local time zone of the place.
    #[getset(get_copy = "pub")]
    pub date: Date,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl SpecialDay {
    /// Gets the year of this special day.
    #[must_use]
    pub const fn year(&self) -> Option<i16> {
        self.date.year
    }
    
    /// Gets the month of this special day (1-12).
    #[must_use]
    pub const fn month(&self) -> Option<i8> {
        self.date.month
    }
    
    /// Gets the day of the month for this special day (1-31).
    #[must_use]
    pub const fn day(&self) -> Option<i8> {
        self.date.day
    }
    
    /// Returns whether this special day is today.
    /// 
    /// Compares against the system's current date in the system timezone. Returns `None` if the
    /// special day date is incomplete (missing year, month, or day components).
    ///
    /// Note: This may not be accurate for places in different timezones.
    #[must_use]
    pub fn is_today(&self) -> Option<bool> {
        let today = jiff::Zoned::now().date();
        self.date.to_jiff_date().map(|date| date == today)
    }

    /// Returns whether this special day is in the future.
    ///
    /// Compares against the system's current date in the system timezone. Returns `None` if the
    /// special day date is incomplete (missing year, month, or day components).
    ///
    /// Note: This may not be accurate for places in different timezones.
    #[must_use]
    pub fn is_future(&self) -> Option<bool> {
        let today = jiff::Zoned::now().date();
        self.date.to_jiff_date().map(|date| date > today)
    }

    /// Returns whether this special day is in the past.
    ///
    /// Compares against the system's current date in the system timezone. Returns `None` if the
    /// special day date is incomplete (missing year, month, or day components).
    ///
    /// Note: This may not be accurate for places in different timezones.
    #[must_use]
    pub fn is_past(&self) -> Option<bool> {
        let today = jiff::Zoned::now().date();
        self.date.to_jiff_date().map(|date| date < today)
    }

    /// Returns the number of days until this special day.
    ///
    /// Returns a positive number for future dates, negative for past dates, and 0 for today.
    /// Returns `None` if the special day date is incomplete (missing year, month, or day
    /// components).
    ///
    /// Note: This calculation uses the system timezone and may not be accurate for places in
    /// different timezones.
    #[must_use]
    pub fn days_from_now(&self) -> Option<i32> {
        let today = jiff::Zoned::now().date();
        self.date.to_jiff_date().and_then(|date| date.since(today).ok().map(|span| span.get_days()))
    }
}

// -------------------------------------------------------------------------------------------------
//
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_date_comparisons() {
        let today = jiff::Zoned::now().date();
        let tomorrow = today.tomorrow().unwrap();
        let yesterday = today.yesterday().unwrap();

        // Today
        let today_special = SpecialDay {
            date: Date::from_jiff_date(&today),
        };
        assert_eq!(today_special.is_today(), Some(true));
        assert_eq!(today_special.is_future(), Some(false));
        assert_eq!(today_special.is_past(), Some(false));
        assert_eq!(today_special.days_from_now(), Some(0));

        // Tomorrow
        let tomorrow_special = SpecialDay {
            date: Date::from_jiff_date(&tomorrow),
        };
        assert_eq!(tomorrow_special.is_today(), Some(false));
        assert_eq!(tomorrow_special.is_future(), Some(true));
        assert_eq!(tomorrow_special.is_past(), Some(false));
        assert_eq!(tomorrow_special.days_from_now(), Some(1));

        // Yesterday
        let yesterday_special = SpecialDay {
            date: Date::from_jiff_date(&yesterday),
        };
        assert_eq!(yesterday_special.is_today(), Some(false));
        assert_eq!(yesterday_special.is_future(), Some(false));
        assert_eq!(yesterday_special.is_past(), Some(true));
        assert_eq!(yesterday_special.days_from_now(), Some(-1));
    }

    #[test]
    fn test_incomplete_date_comparisons() {
        // Incomplete date (missing day)
        let incomplete_special = SpecialDay {
            date: Date {
                year: Some(2024),
                month: Some(12),
                day: None,
            },
        };

        assert_eq!(incomplete_special.is_today(), None);
        assert_eq!(incomplete_special.is_future(), None);
        assert_eq!(incomplete_special.is_past(), None);
        assert_eq!(incomplete_special.days_from_now(), None);
    }

    #[test]
    fn test_accessor_methods() {
        let special_day = SpecialDay {
            date: Date {
                year: Some(2024),
                month: Some(12),
                day: Some(25),
            },
        };

        assert_eq!(special_day.year(), Some(2024));
        assert_eq!(special_day.month(), Some(12));
        assert_eq!(special_day.day(), Some(25));
    }
}