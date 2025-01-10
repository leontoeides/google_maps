//! Contains Serde serializer/deserializer for converting a quantity of seconds
//! in `String` format into a `time::Duration` struct.

use chrono::Duration;
use serde::{Deserialize, Deserializer};

/// This trait converts a quantity of seconds in `String` format into a
/// `time::Duration` struct. The Google Maps Platform returns duration fields in
/// seconds and it's handier to be able to use them as a `time::Duration` structs.
pub fn seconds_to_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the field (from a `String`) into an `i64`. This is what the
    // `time::Duration::seconds()` method expects:
    let seconds: i64 = Deserialize::deserialize(deserializer)?;
    // This handy-dandy method converts from the seconds count in `i64` format
    // into a `Duration` struct:
    Duration::try_seconds(seconds).ok_or_else(|| {
        serde::de::Error::custom(format!(
            "could not deserialize a duration from seconds: {seconds}"
        ))
    })
} // fn
