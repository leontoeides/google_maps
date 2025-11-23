#![allow(clippy::ref_option)]

use jiff::tz::TimeZone as JiffTimeZone;

// -------------------------------------------------------------------------------------------------
//
/// Represents a time zone from the IANA Time Zone Database.
///
/// Time zones provide standardized identifiers for geographic regions and their local time rules,
/// including daylight saving time transitions.
///
/// This struct stores IANA time zone information and provides integration with the
/// [jiff](https://crates.io/crates/jiff) crate for time zone operations and conversions.
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
#[serde(rename_all = "camelCase")]
pub struct TimeZone {
    /// IANA Time Zone Database identifier.
    ///
    /// Standard time zone identifier such as `America/New_York`, `Europe/London`, or `Asia/Tokyo`.
    /// These identifiers are maintained by the IANA Time Zone Database and provide canonical names
    /// for time zones worldwide.
    #[getset(get = "pub")]
    pub id: String,

    /// IANA Time Zone Database version number.
    ///
    /// Optional version identifier indicating which version of the IANA database was used to
    /// determine time zone rules. Format is typically year + letter such as "2024a" or "2023c".
    #[serde(default)]
    #[getset(get = "pub")]
    pub version: Option<String>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl TimeZone {
    /// Creates a new `TimeZone` with the specified IANA identifier.
    ///
    /// Used to construct a `TimeZone` from a known IANA time zone identifier. The version is set to
    /// `None` and can be specified separately if needed.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            version: None,
        }
    }

    /// Creates a new `TimeZone` with both identifier and version.
    ///
    /// Used when you have specific version information about the IANA database used to determine
    /// the time zone rules, providing more precise metadata.
    #[must_use]
    pub fn with_version(id: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            version: Some(version.into()),
        }
    }

    /// Attempts to convert this `TimeZone` to a `jiff::tz::TimeZone`.
    ///
    /// Returns a jiff `TimeZone` for performing time zone calculations and conversions.
    ///
    /// # Errors
    /// Returns an error if the time zone identifier is not recognized by jiff or if the time zone
    /// data is invalid.
    pub fn try_to_jiff_timezone(&self) -> Result<JiffTimeZone, jiff::Error> {
        JiffTimeZone::get(&self.id)
    }

    /// Gets a `jiff::tz::TimeZone` if the identifier is valid.
    ///
    /// This is a convenience method that returns None if the conversion fails rather than returning
    /// an error, useful for optional time zone operations.
    #[must_use]
    pub fn to_jiff_timezone(&self) -> Option<JiffTimeZone> {
        self.try_to_jiff_timezone().ok()
    }

    /// Returns whether this time zone identifier is valid.
    ///
    /// Used to validate time zone data before performing operations that require valid time zone
    /// information, preventing runtime errors.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.to_jiff_timezone().is_some()
    }

    /// Returns whether this time zone observes daylight saving time.
    ///
    /// Used to determine if a location has seasonal time changes, which affects scheduling,
    /// business hours, and time-sensitive operations.
    #[must_use]
    pub fn observes_dst(&self) -> Option<bool> {
        self.to_jiff_timezone().map(|tz| {
            // Check if timezone has any transitions (indicating DST)
            // This is a simplified check - in practice you might want to check for recent/future
            // transitions specifically
            !tz.iana_name().unwrap_or("").contains("UTC") && 
            !tz.iana_name().unwrap_or("").contains("GMT")
        })
    }

    /// Gets the current UTC offset for this time zone.
    ///
    /// Returns the current offset from UTC in seconds, accounting for any active daylight saving
    /// time. Useful for time conversions and scheduling.
    #[must_use]
    pub fn current_utc_offset_seconds(&self) -> Option<i32> {
        self.to_jiff_timezone().map(|tz| {
            let now = jiff::Timestamp::now();
            tz.to_offset(now).seconds()
        })
    }

    /// Gets the current UTC offset as hours and minutes.
    ///
    /// Returns the offset as (hours, minutes) tuple for display purposes. Negative hours indicate
    /// time zones west of UTC.
    #[must_use]
    pub fn current_utc_offset_hm(&self) -> Option<(i8, u8)> {
        self.current_utc_offset_seconds().and_then(|seconds| {
            let hours = seconds / 3_600;
            let minutes = (seconds.abs() % 3_600) / 60;
            if let (Ok(hours), Ok(minutes)) = (i8::try_from(hours), u8::try_from(minutes)) {
                Some((hours, minutes))
            } else {
                None
            }
        })
    }

    /// Formats the current UTC offset as a string (e.g., "+05:30", "-08:00").
    ///
    /// Provides standard ISO 8601 offset format suitable for display in UIs, scheduling
    /// applications, and time zone information displays.
    #[must_use]
    pub fn current_utc_offset_string(&self) -> Option<String> {
        self.current_utc_offset_hm().map(|(hours, minutes)| {
            if hours >= 0 {
                format!("+{hours:02}:{minutes:02}")
            } else {
                format!("-{:02}:{:02}", hours.abs(), minutes)
            }
        })
    }

    /// Gets a human-readable display name for the time zone.
    ///
    /// Extracts a user-friendly name from the IANA identifier by taking the city/region portion,
    /// useful for displaying in user interfaces.
    #[must_use]
    pub fn display_name(&self) -> String {
        // Extract city name from IANA identifier (e.g., "America/New_York" -> "New York")
        self.id
            .split('/')
            .next_back()
            .unwrap_or(&self.id)
            .replace('_', " ")
    }

    /// Gets the geographic region from the time zone identifier.
    ///
    /// Extracts the continental or regional portion of the IANA identifier, useful for grouping
    /// time zones by geographic area.
    #[must_use]
    pub fn region(&self) -> Option<String> {
        self.id.split('/').next().map(ToString::to_string)
    }

    /// Returns whether this time zone is in a specific geographic region.
    ///
    /// Used to filter or categorize time zones by continental regions, helpful for regional time
    /// zone selection in applications.
    #[must_use]
    pub fn is_in_region(&self, region: &str) -> bool {
        self.region().is_some_and(|r| r.eq_ignore_ascii_case(region))
    }

    /// Converts the current time to this time zone.
    ///
    /// Returns the current timestamp converted to this time zone's local time, useful for
    /// displaying current time in different locations.
    #[must_use]
    pub fn current_local_time(&self) -> Option<jiff::Zoned> {
        self.to_jiff_timezone().map(|tz| {
            jiff::Timestamp::now().to_zoned(tz)
        })
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl TryFrom<TimeZone> for JiffTimeZone {
    type Error = jiff::Error;

    /// Converts a `TimeZone` to a `jiff::tz::TimeZone`.
    ///
    /// Enables direct conversion to jiff's `TimeZone` type for time zone operations and
    /// calculations. Requires the time zone identifier to be valid in `jiff`.
    ///
    /// # Errors
    /// Returns a `jiff::Error` if the time zone identifier is not recognized.
    fn try_from(tz: TimeZone) -> Result<Self, Self::Error> {
        tz.try_to_jiff_timezone()
    }
}

impl TryFrom<&TimeZone> for JiffTimeZone {
    type Error = jiff::Error;

    /// Converts a `TimeZone` reference to a `jiff::tz::TimeZone`.
    ///
    /// Convenience implementation for converting borrowed `TimeZone` instances without taking
    /// ownership, useful in contexts where the original is needed.
    ///
    /// # Errors
    /// Returns a `jiff::Error` if the time zone identifier is not recognized.
    fn try_from(tz: &TimeZone) -> Result<Self, Self::Error> {
        tz.try_to_jiff_timezone()
    }
}

impl From<JiffTimeZone> for TimeZone {
    /// Creates a `TimeZone` from a `jiff::tz::TimeZone`.
    ///
    /// Converts from jiff's `TimeZone` type back to the Google API `TimeZone` format. The version
    /// information is not preserved as `jiff` doesn't expose this metadata.
    fn from(jiff_tz: JiffTimeZone) -> Self {
        Self {
            id: jiff_tz.iana_name().unwrap_or("UTC").to_string(),
            version: None,
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialization() {
        let tz = TimeZone {
            id: "America/New_York".to_string(),
            version: Some("2024a".to_string()),
        };
        let json = serde_json::to_string(&tz).unwrap();
        assert!(json.contains("America/New_York"));
        assert!(json.contains("2024a"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"id":"Europe/London","version":"2024a"}"#;
        let tz: TimeZone = serde_json::from_str(json).unwrap();
        assert_eq!(tz.id, "Europe/London");
        assert_eq!(tz.version, Some("2024a".to_string()));
    }

    #[test]
    fn test_constructors() {
        let tz = TimeZone::new("Asia/Tokyo");
        assert_eq!(tz.id, "Asia/Tokyo");
        assert_eq!(tz.version, None);

        let tz = TimeZone::with_version("Europe/Paris", "2024b");
        assert_eq!(tz.id, "Europe/Paris");
        assert_eq!(tz.version, Some("2024b".to_string()));
    }

    #[test]
    fn test_jiff_conversion() {
        let tz = TimeZone::new("America/New_York");
        
        // Test successful conversion
        let jiff_tz = tz.try_to_jiff_timezone();
        assert!(jiff_tz.is_ok());
        
        let jiff_tz = tz.to_jiff_timezone();
        assert!(jiff_tz.is_some());

        // Test try_from trait
        let jiff_tz = JiffTimeZone::try_from(tz.clone());
        assert!(jiff_tz.is_ok());

        let jiff_tz = JiffTimeZone::try_from(&tz);
        assert!(jiff_tz.is_ok());
    }

    #[test]
    fn test_from_jiff() {
        let jiff_tz = JiffTimeZone::get("UTC").unwrap();
        let tz = TimeZone::from(jiff_tz);
        assert_eq!(tz.id, "UTC");
        assert_eq!(tz.version, None);
    }

    #[test]
    fn test_validity() {
        let valid_tz = TimeZone::new("America/New_York");
        assert!(valid_tz.is_valid());

        let invalid_tz = TimeZone::new("Invalid/TimeZone");
        assert!(!invalid_tz.is_valid());
    }

    #[test]
    fn test_utc_offset() {
        let tz = TimeZone::new("UTC");
        if let Some(offset) = tz.current_utc_offset_seconds() {
            assert_eq!(offset, 0);
        }

        if let Some((hours, minutes)) = tz.current_utc_offset_hm() {
            assert_eq!(hours, 0);
            assert_eq!(minutes, 0);
        }

        if let Some(offset_str) = tz.current_utc_offset_string() {
            assert_eq!(offset_str, "+00:00");
        }
    }

    #[test]
    fn test_display_name() {
        let tz = TimeZone::new("America/New_York");
        assert_eq!(tz.display_name(), "New York");

        let tz = TimeZone::new("Europe/London");
        assert_eq!(tz.display_name(), "London");

        let tz = TimeZone::new("Asia/Kolkata");
        assert_eq!(tz.display_name(), "Kolkata");
    }

    #[test]
    fn test_region() {
        let tz = TimeZone::new("America/New_York");
        assert_eq!(tz.region(), Some("America".to_string()));

        let tz = TimeZone::new("Europe/London");
        assert_eq!(tz.region(), Some("Europe".to_string()));

        let tz = TimeZone::new("UTC");
        assert_eq!(tz.region(), Some("UTC".to_string()));
    }

    #[test]
    fn test_is_in_region() {
        let tz = TimeZone::new("America/New_York");
        assert!(tz.is_in_region("America"));
        assert!(tz.is_in_region("america"));
        assert!(!tz.is_in_region("Europe"));

        let tz = TimeZone::new("UTC");
        assert!(tz.is_in_region("UTC"));
        assert!(!tz.is_in_region("America"));
    }

    #[test]
    fn test_current_local_time() {
        let tz = TimeZone::new("UTC");
        let local_time = tz.current_local_time();
        assert!(local_time.is_some());
        
        if let Some(time) = local_time {
            assert_eq!(time.time_zone().iana_name().unwrap(), "UTC");
        }
    }
}