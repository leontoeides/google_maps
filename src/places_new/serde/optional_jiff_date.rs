#![allow(clippy::ref_option, clippy::trivially_copy_pass_by_ref, reason = "for serde")]

//! Serde helpers for `jiff::civil::Date`

use crate::places_new::Date;
use serde::{Deserialize, Serialize, Serializer};

/// Deserializes an optional `jiff::civil::Date` from Google's structured date format.
///
/// Google represents dates as objects with `year`, `month`, and `day` fields, using sentinel
/// values (0) for missing components. This function deserializes through the `Date` type and
/// converts complete dates to `jiff::civil::Date`.
pub fn deserialize_optional_jiff_date<'de, D>(
    deserializer: D,
) -> Result<Option<jiff::civil::Date>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date = Option::<Date>::deserialize(deserializer)?;
    let result = date
        .and_then(|d| d.try_to_jiff_date().transpose())
        .transpose()
        .map_err(serde::de::Error::custom)?;
    
    Ok(result)
}

/// Serializes an optional `jiff::civil::Date` into Google's structured date format.
///
/// Converts a `jiff::civil::Date` into Google's object representation with separate `year`,
/// `month`, and `day` fields. If the date is `None`, serializes as `None`.
pub fn serialize_optional_jiff_date<S>(
    date: &Option<jiff::civil::Date>,
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    match date {
        Some(jiff_date) => {
            let google_date = Date::from_jiff_date(jiff_date);
            google_date.serialize(serializer)
        }
        None => serializer.serialize_none(),
    }
}
