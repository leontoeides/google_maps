//! Contains Serde serializer/deserializer for converting a quantity of seconds
//! in `time::Duration` struct into `String` format.

use chrono::Duration;
use serde::Serializer;

pub fn duration_to_seconds<S>(data: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let seconds = data.num_seconds();
    serializer.serialize_i64(seconds)
} // fn
