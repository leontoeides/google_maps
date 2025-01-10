//! Contains Serde serializer/deserializer for converting a Unix Timestamp in
//! `String` format into a `time::PrimitiveDateTime` struct.

use serde::{Deserialize, Deserializer};
use time::PrimitiveDateTime;

/// This trait converts a Unix Timestamp in `String` format into a
/// `time::PrimitiveDateTime` struct. The Unix Timestamp is the number of
/// seconds that have elapsed since the Unix epoch, that is the time
/// `00:00:00 UTC on 1 January 1970`, minus leap seconds. The Google Maps
/// Platform returns some fields in the Unix Timestamp format and it's handier
/// to be able to use them as a PrimitiveDateTime.
pub fn unix_to_primitivedatetime<'de, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the field (from a `String`) into an `i64`. This is what the
    // `time::PrimitiveDateTime::from_unix_timestamp()` method expects:
    let unix_timestamp: i64 = Deserialize::deserialize(deserializer)?;
    // This handy-dandy method converts from the Unix Timestamp in `i64` format
    // into a `PrimitiveDateTime` struct:
    Ok(PrimitiveDateTime::from_unix_timestamp(unix_timestamp))
} // fn