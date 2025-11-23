//! Serialization helpers for `Option<Weekday>`

use serde::{Deserialize, Deserializer, Serializer};
use serde::de::Error;
use jiff::civil::Weekday;

/// Serializes a `jiff::Weekday` to Google's day numbering system (0=Sunday, 6=Saturday).
/// 
/// Converts from jiff's ISO-8601 numbering (1=Monday, 7=Sunday) to Google's numbering system
/// (0=Sunday, 6=Saturday).
/// 
/// Used with `#[serde(serialize_with = "serialize_weekday")]`
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn serialize_weekday<S>(weekday: &Weekday, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let google_day = weekday_to_google_day(weekday);
    serializer.serialize_i32(google_day)
}

/// Deserializes Google's day numbering to a `jiff::Weekday`.
/// 
/// Converts from Google's numbering system (0=Sunday, 6=Saturday) to jiff's ISO-8601 `Weekday`
/// enum.
/// 
/// Used with `#[serde(deserialize_with = "deserialize_weekday")]`
pub fn deserialize_weekday<'de, D>(deserializer: D) -> Result<Weekday, D::Error>
where
    D: Deserializer<'de>
{
    let google_day = i32::deserialize(deserializer)?;
    
    google_day_to_weekday(google_day)
        .map_or_else(
            || Err(D::Error::custom(format!(
                "Unrecognized weekday {google_day:#?}: expected 0-6 (0=Sunday, 6=Saturday)",
            ))),
            Ok
        )
}

// -------------------------------------------------------------------------------------------------
//
// Private Functions

const fn google_day_to_weekday(google_day: i32) -> Option<Weekday> {
    match google_day {
        0 => Some(Weekday::Sunday),
        1 => Some(Weekday::Monday),
        2 => Some(Weekday::Tuesday),
        3 => Some(Weekday::Wednesday),
        4 => Some(Weekday::Thursday),
        5 => Some(Weekday::Friday),
        6 => Some(Weekday::Saturday),
        _ => None,
    }
}

#[allow(clippy::trivially_copy_pass_by_ref, reason = "for serde")]
const fn weekday_to_google_day(weekday: &Weekday) -> i32 {
    match weekday {
        Weekday::Sunday => 0,
        Weekday::Monday => 1,
        Weekday::Tuesday => 2,
        Weekday::Wednesday => 3,
        Weekday::Thursday => 4,
        Weekday::Friday => 5,
        Weekday::Saturday => 6,
    }
}

// -------------------------------------------------------------------------------------------------
//
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weekday_conversion() {
        // Test Google -> jiff conversion
        assert_eq!(google_day_to_weekday(0), Some(Weekday::Sunday));
        assert_eq!(google_day_to_weekday(1), Some(Weekday::Monday));
        assert_eq!(google_day_to_weekday(6), Some(Weekday::Saturday));
        assert_eq!(google_day_to_weekday(7), None); // Invalid

        // Test jiff -> Google conversion
        assert_eq!(weekday_to_google_day(&Weekday::Sunday), 0);
        assert_eq!(weekday_to_google_day(&Weekday::Monday), 1);
        assert_eq!(weekday_to_google_day(&Weekday::Saturday), 6);
    }
}