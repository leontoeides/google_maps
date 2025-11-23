#![allow(clippy::ref_option, clippy::trivially_copy_pass_by_ref, reason = "for serde")]

//! Serde helpers for month (i8 with zero sentinel)

use serde::{Deserialize, Deserializer, Serializer};
use serde::de::Error;

/// Serializes an `Option<i8>` month where `None` becomes `0` and `Some(month)` becomes the month
/// value.
///
/// Used with `#[serde(serialize_with = "serialize_optional_month")]`
pub fn serialize_optional_month<S>(month: &Option<i8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match month {
        Some(month) => serializer.serialize_i8(*month),
        None => serializer.serialize_i8(0),
    }
}

/// Deserializes an `i8` month where `0` becomes None and `1`-`12` become `Some(month)`.
///
/// Used with `#[serde(deserialize_with = "deserialize_optional_month")]`
pub fn deserialize_optional_month<'de, D>(deserializer: D) -> Result<Option<i8>, D::Error>
where
    D: Deserializer<'de>,
{
    let month_i8 = i8::deserialize(deserializer)?;

    match month_i8 {
        0 => Ok(None),
        1..=12 => Ok(Some(month_i8)),
        _ => Err(D::Error::custom(format!(
            "invalid month value {month_i8:?}: must be 0 (unspecified) or 1-12"
        ))),
    }
}