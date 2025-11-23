//! Serialization helpers for `CountryCode`

use rust_iso3166::CountryCode;
use serde::{de::Error, Deserialize, Deserializer, Serializer};

/// Serializes a `CountryCode` as its 2-letter alpha-2 string representation.
///
/// Used with `#[serde(serialize_with = "serialize_country_code")]`
pub fn serialize_country_code<S>(country: &CountryCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    serializer.serialize_str(country.alpha2)
}

/// Deserializes a 2-letter alpha-2 string representation into a `CountryCode`.
///
/// Used with `#[serde(deserialize_with = "deserialize_country_code")]`
pub fn deserialize_country_code<'de, D>(deserializer: D) -> Result<CountryCode, D::Error>
where
    D: Deserializer<'de>
{
    let country_str = String::deserialize(deserializer)?;

    rust_iso3166::from_alpha2(&country_str)
        .ok_or_else(|| D::Error::custom(format!("Unrecognized country code: {country_str:?}")))
}