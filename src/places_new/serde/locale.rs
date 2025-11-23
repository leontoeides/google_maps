//! Serialization helpers for `Locale`

use icu_locale::Locale;
use serde::{de::Error, Deserialize, Deserializer, Serializer};
use std::str::FromStr;

/// Serializes a `Locale` as its BCP-47 string representation.
///
/// Used with `#[serde(serialize_with = "serialize_locale")]`
pub fn serialize_locale<S>(locale: &Locale, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&locale.to_string())
}

/// Deserializes a valid BCP-47 locale identifier string into a `Locale`.
///
/// Used with `#[serde(deserialize_with = "deserialize_locale")]`
pub fn deserialize_locale<'de, D>(deserializer: D) -> Result<Locale, D::Error>
where
    D: Deserializer<'de>,
{
    let locale_str = String::deserialize(deserializer)?;
    Locale::from_str(&locale_str)
        .map_err(|error| D::Error::custom(format!(
            "Unrecognized locale {locale_str:?}: {error}"
        )))
}