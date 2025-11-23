//! Serialization helpers for `Option<Locale>`

use icu_locale::Locale;
use serde::{de::Error, Deserialize, Deserializer, Serializer};
use std::str::FromStr;

/// Serializes an `Option<Locale>` as its BCP-47 string representation.
///
/// Used with `#[serde(serialize_with = "serialize_optional_locale")]`
#[allow(clippy::ref_option)]
pub fn serialize_optional_locale<S>(
    locale: &Option<Locale>,
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    match locale {
        Some(locale) => serializer.serialize_some(&locale.to_string()),
        None => serializer.serialize_none(),
    }
}

/// Deserializes an optional BCP-47 string into an `Option<Locale>`.
///
/// Used with `#[serde(deserialize_with = "deserialize_optional_locale")]`
pub fn deserialize_optional_locale<'de, D>(
    deserializer: D
) -> Result<Option<Locale>, D::Error>
where
    D: Deserializer<'de>
{
    let opt_string: Option<String> = Option::deserialize(deserializer)?;
    match opt_string {
        Some(locale_str) => {
            let locale = Locale::from_str(&locale_str)
                .map_err(|error| D::Error::custom(format!(
                    "Unrecognized locale {locale_str:?}: {error}"
                )))?;
            Ok(Some(locale))
        }
        None => Ok(None),
    }
}