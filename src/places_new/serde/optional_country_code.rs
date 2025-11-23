//! Serialization helpers for `Option<CountryCode>`

use rust_iso3166::CountryCode;
use serde::Serializer;

/// Serializes an `Option<CountryCode>` as its 2-letter alpha-2 string representation.
///
/// Used with `#[serde(serialize_with = "serialize_optional_country_code")]`
#[allow(clippy::ref_option)]
pub fn serialize_optional_country_code<S>(
    country: &Option<CountryCode>,
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    match country {
        Some(country) => serializer.serialize_some(country.alpha2),
        None => serializer.serialize_none(),
    }
}