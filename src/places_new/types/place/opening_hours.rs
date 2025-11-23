use crate::places_new::{Period, SecondaryHoursType, SpecialDay};
use jiff::Timestamp;

// -------------------------------------------------------------------------------------------------
//
/// Information about business hour of the place.
/// 
/// Opening hours provide comprehensive information about when a place is open, including
/// regular weekly schedules, special day exceptions, and real-time open/closed status.
/// This information helps users understand current availability and plan future visits.
#[derive(
    //std
    Clone,
    Debug,
    Default,
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
#[serde(rename_all = "camelCase")]
pub struct OpeningHours {
    /// The periods that this place is open during the week.
    /// 
    /// The periods are in chronological order, in the place-local timezone. An empty
    /// (but not absent) value indicates a place that is never open, e.g. because it is
    /// closed temporarily for renovations. The starting day of the week is determined by
    /// the locale (language and region).
    /// 
    /// NOTE: The ordering of the periods may not be strictly chronological in some cases
    /// due to locale-specific week start days and Google's data processing.
    #[serde(default)]
    #[getset(get = "pub")]
    pub periods: Vec<Period>,

    /// Localized strings describing the opening hours of this place, one string for each day of the week.
    /// 
    /// NOTE: The order of the days and the start of the week is determined by the locale
    /// (language and region). The ordering may not match the order of periods.
    /// Will be empty if the hours are unknown or could not be converted to localized text.
    /// Example: "Sun: 18:00–06:00"
    #[serde(default)]
    #[getset(get = "pub")]
    pub weekday_descriptions: Vec<String>,

    /// A type string used to identify the type of secondary hours.
    /// 
    /// Secondary hours represent different types of operating hours beyond the main
    /// business hours (e.g., delivery hours, drive-through hours, happy hour).
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub secondary_hours_type: Option<SecondaryHoursType>,

    /// Structured information for special days that fall within the period that the returned opening hours cover.
    /// 
    /// Special days are days that could impact the business hours of a place, e.g. Christmas day.
    /// Set for currentOpeningHours and currentSecondaryOpeningHours if there are exceptional hours.
    #[serde(default)]
    #[getset(get = "pub")]
    pub special_days: Vec<SpecialDay>,

    /// The next time the current opening hours period starts up to 7 days in the future.
    /// 
    /// This field is only populated if the opening hours period is not active at the time
    /// of serving the request. Uses RFC 3339 format.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub next_open_time: Option<Timestamp>,

    /// The next time the current opening hours period ends up to 7 days in the future.
    /// 
    /// This field is only populated if the opening hours period is active at the time
    /// of serving the request. Uses RFC 3339 format.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub next_close_time: Option<Timestamp>,

    /// Whether the opening hours period is currently active.
    /// 
    /// For regular opening hours and current opening hours, this field means whether the
    /// place is open. For secondary opening hours and current secondary opening hours,
    /// this field means whether the secondary hours of this place is active.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub open_now: Option<bool>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl OpeningHours {
    /// Returns whether this place has any defined opening periods.
    #[must_use]
    pub fn has_periods(&self) -> bool {
        !self.periods.is_empty()
    }

    /// Returns whether this place has weekday descriptions.
    #[must_use]
    pub fn has_weekday_descriptions(&self) -> bool {
        !self.weekday_descriptions.is_empty()
    }

    /// Returns whether this place has special days defined.
    #[must_use]
    pub fn has_special_days(&self) -> bool {
        !self.special_days.is_empty()
    }

    /// Returns whether this place is currently open according to the Google Maps API.
    /// 
    /// This returns the value from Google's API, which may be `None` if the open status is unknown
    /// or not provided.
    #[must_use]
    pub const fn is_open_now(&self) -> Option<bool> {
        self.open_now
    }

    /// Returns whether this place has secondary hours (non-primary operating hours).
    #[must_use]
    pub const fn has_secondary_hours(&self) -> bool {
        self.secondary_hours_type.is_some()
    }

    /// Gets the secondary hours type if available.
    #[must_use]
    pub const fn get_secondary_hours_type(&self) -> Option<&SecondaryHoursType> {
        self.secondary_hours_type.as_ref()
    }

    /// Returns whether the place appears to be closed permanently or temporarily.
    /// 
    /// A place is considered closed if it has no periods defined, which typically indicates
    /// temporary closure for renovations or permanent closure.
    #[must_use]
    pub fn appears_closed(&self) -> bool {
        self.periods.is_empty()
    }

    /// Returns whether this place has 24-hour periods.
    /// 
    /// Checks if any of the periods represent 24-hour operation.
    pub fn has_24_hour_periods(&self) -> bool {
        self.periods.iter().any(Period::is_24_hour)
    }

    /// Gets all periods that represent 24-hour operation.
    #[must_use]
    pub fn get_24_hour_periods(&self) -> Vec<&Period> {
        self.periods.iter().filter(|period| period.is_24_hour()).collect()
    }

    /// Returns whether this place has next open/close time information.
    #[must_use]
    pub const fn has_next_times(&self) -> bool {
        self.next_open_time.is_some() || self.next_close_time.is_some()
    }

    /// Gets the next opening time if available.
    #[must_use]
    pub const fn get_next_open_time(&self) -> Option<Timestamp> {
        self.next_open_time
    }

    /// Gets the next closing time if available.  
    #[must_use]
    pub const fn get_next_close_time(&self) -> Option<Timestamp> {
        self.next_close_time
    }
}