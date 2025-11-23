#![allow(clippy::ref_option, clippy::trivially_copy_pass_by_ref, reason = "for serde")]

//! Serde helpers for year (i16 with zero sentinel)

use serde::{Deserialize, Deserializer, Serializer};
use serde::de::Error;

/// Serializes an `Option<i16>` year where `None` becomes `0` and `Some(year)` becomes the year
/// value.
///
/// Used with `#[serde(serialize_with = "serialize_optional_year")]`
pub fn serialize_optional_year<S>(year: &Option<i16>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match year {
        Some(year) => serializer.serialize_i16(*year),
        None => serializer.serialize_i16(0),
    }
}

/// Deserializes an `i16` year where `0` becomes `None` and positive values become `Some(year)`.
///
/// Used with `#[serde(deserialize_with = "deserialize_optional_year")]`
pub fn deserialize_optional_year<'de, D>(deserializer: D) -> Result<Option<i16>, D::Error>
where
    D: Deserializer<'de>,
{
    let year_i16 = i16::deserialize(deserializer)?;

    match year_i16 {
        0 => Ok(None),
        1..=9_999 => Ok(Some(year_i16)),
        _ => Err(D::Error::custom(format!(
            "invalid year value {year_i16:?}: must be 0 (unspecified) or 1-9999"
        ))),
    }
}