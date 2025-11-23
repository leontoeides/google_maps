#![allow(clippy::ref_option, clippy::trivially_copy_pass_by_ref, reason = "for serde")]

//! Serde helpers for day (i8 with zero sentinel)

use serde::{Deserialize, Deserializer, Serializer};
use serde::de::Error;

/// Serializes an `Option<i8>` day where `None` becomes `0` and `Some(day)` becomes the day value.
///
/// Used with `#[serde(serialize_with = "serialize_optional_day")]`
pub fn serialize_optional_day<S>(day: &Option<i8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match day {
        Some(day) => serializer.serialize_i8(*day),
        None => serializer.serialize_i8(0),
    }
}

/// Deserializes an `i8` day where `0` becomes `None` and `1`-`31` become `Some(day)`.
///
/// Used with `#[serde(deserialize_with = "deserialize_optional_day")]`
pub fn deserialize_optional_day<'de, D>(deserializer: D) -> Result<Option<i8>, D::Error>
where
    D: Deserializer<'de>,
{
    let day_i8 = i8::deserialize(deserializer)?;

    match day_i8 {
        0 => Ok(None),
        1..=31 => Ok(Some(day_i8)),
        _ => Err(D::Error::custom(format!(
            "invalid day value {day_i8:?}: must be 0 (unspecified) or 1-31"
        ))),
    }
}