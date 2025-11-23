//! Serialization helpers for `Vec<CountryCode>`

use rust_iso3166::CountryCode;
use serde::Serializer;

/// Serializes a `Vec<CountryCode>` as a vector of 2-letter alpha-2 strings.
///
/// Used with `#[serde(serialize_with = "serialize_country_codes")]`
#[allow(clippy::ptr_arg, reason = "for serde, not for end-user usage")]
pub fn serialize_vec_country_code<S>(
    countries: &Vec<CountryCode>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let alpha2_codes: Vec<&str> = countries.iter().map(|c| c.alpha2).collect();
    serializer.collect_seq(alpha2_codes)
}