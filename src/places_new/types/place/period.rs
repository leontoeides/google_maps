use crate::places_new::Error;
use jiff::civil::DateTime;
use std::ops::Range;
use crate::places_new::Point;

// -------------------------------------------------------------------------------------------------
//
/// A period the place remains in `openNow` status.
/// 
/// Periods define time ranges when a place maintains a particular status, such as being open or
/// closed. Each period has an opening point and optional closing point.
#[derive(
    //std
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    // getset
    getset::Getters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
pub struct Period {
    /// The time that the place starts to be open.
    /// 
    /// This point defines when the place transitions to an "open" status. It includes day of week
    /// and time information to specify exactly when the opening occurs.
    #[getset(get = "pub")]
    pub open: Point,

    /// The time that the place starts to be closed.
    /// 
    /// This point defines when the place transitions to a "closed" status. For places that are open
    /// 24 hours, this field may be absent. The absence of a close time typically indicates the
    /// place remains open until the next period begins.
    #[serde(default)]
    #[getset(get = "pub")]
    pub close: Option<Point>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Period {
    /// Returns whether this period has a closing time.
    #[must_use]
    pub const fn has_close(&self) -> bool {
        self.close.is_some()
    }

    /// Returns whether this period represents a 24-hour operation.
    /// 
    /// A period is considered 24-hour if the opening time is at midnight (00:00) it has no closing
    /// time.
    #[must_use]
    pub const fn is_24_hour(&self) -> bool {
        match (self.open.hour, self.open.minute) {
            (0, 0) => match &self.close {
                Some(close) => match (close.hour, close.minute) {
                    (0, 0) | (23, 59) => true,  // 00:00 to 23:59, 00:00 to 00:00 (next day)
                    _ => false,                 // Opens at midnight but not a 24-hour closing time
                },
                None => true,                   // 00:00 with no closing time = 24-hour
            },
            _ => false,                         // Doesn't open at midnight = not 24-hour
        }
    }

    /// Attempts to create a `Range<DateTime>` for this period.
    ///
    /// Returns a `Range<DateTime>` if the opening time can be converted to a valid `DateTime`.
    ///
    /// - For 24-hour periods (no closing time + opens at midnight), returns a 24-hour range from
    ///   the opening datetime to the same time the next day.
    ///
    /// - For regular periods, returns a range from opening to closing time.
    ///
    /// # Errors
    /// Returns an error if either opening or closing time cannot be converted to valid a
    /// `DateTime`.
    pub fn try_to_range(&self) -> Result<Option<Range<DateTime>>, Error> {
        if let Some(open_datetime) = self.open.try_to_civil_datetime()? {
            if let Some(ref close_point) = self.close {
                // Regular period with closing time
                let Some(close_datetime) = close_point.try_to_civil_datetime()? else { return Ok(None) };

                Ok(Some(open_datetime..close_datetime))
            } else if self.is_24_hour() {
                // 24-hour period: create range from opening datetime to same time next day
                let close_datetime = open_datetime.checked_add(jiff::Span::new().days(1))?;
                Ok(Some(open_datetime..close_datetime))
            } else {
                // No closing time but not a proper 24-hour period
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Gets a `Range<DateTime>` for this period.
    ///
    /// This is a convenience method that returns `None` if the range cannot be created rather than
    /// returning an error.
    #[must_use]
    pub fn range(&self) -> Option<Range<DateTime>> {
        self.try_to_range().ok().flatten()
    }

    /// Returns whether this period can be represented as a datetime range.
    ///
    /// Returns `true` if the opening time has valid date-time information that can be converted to
    /// `DateTime`, and either:
    ///
    /// - There's a valid closing time, or
    ///
    /// - It's a 24-hour period (no closing time + opens at midnight)
    #[must_use]
    pub fn has_range(&self) -> bool {
        self.range().is_some()
    }
}