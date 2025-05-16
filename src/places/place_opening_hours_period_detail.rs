//! An object describing the opening hours of a place.

use chrono::{NaiveDate, NaiveTime, Weekday};
use serde::de::{Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

// -----------------------------------------------------------------------------
//
/// An object describing the opening hours of a place.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PlaceOpeningHoursPeriodDetail {
    /// A number from 0–6, corresponding to the days of the week, starting on
    /// Sunday. For example, 2 means Tuesday.
    #[serde(deserialize_with = "integer_as_weekday")]
    pub day: Weekday,

    /// May contain a time of day in 24-hour hhmm format. Values are in the
    /// range 0000–2359. The time will be reported in the place’s time zone.
    #[serde(deserialize_with = "str_as_naive_time")]
    pub time: NaiveTime,

    /// A date expressed in RFC3339 format in the local timezone for the place,
    /// for example 2010-12-31.
    #[serde(default)]
    pub date: Option<NaiveDate>,

    /// True if a given period was truncated due to a seven-day cutoff, where
    /// the period starts before midnight on the date of the request and/or ends
    /// at or after midnight on the last day. This property indicates that the
    /// period for open or close can extend past this seven-day cutoff.
    #[serde(default)]
    pub truncated: Option<bool>,
} // struct PlaceOpeningHoursPeriodDetail

// -----------------------------------------------------------------------------

fn integer_as_weekday<'de, D>(deserializer: D) -> Result<Weekday, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_u64(WeekdayVisitor)
} // fn integer_as_weekday

struct WeekdayVisitor;

impl Visitor<'_> for WeekdayVisitor {
    type Value = Weekday;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("integer representation of weekday") // write_str
    } // fn expecting

    fn visit_u64<E>(self, value: u64) -> Result<Weekday, E>
    where
        E: serde::de::Error,
    {
        match value {
            0 => Ok(Weekday::Sun),
            1 => Ok(Weekday::Mon),
            2 => Ok(Weekday::Tue),
            3 => Ok(Weekday::Wed),
            4 => Ok(Weekday::Thu),
            5 => Ok(Weekday::Fri),
            6 => Ok(Weekday::Sat),
            _ => Err(
                E::invalid_value(
                    Unexpected::Unsigned(value),
                    &"weekday between 0 and 6, where 0 is sunday",
                ), // invalid_value
            ), // _
        } // match
    } // fn visit_u64
} // impl Visitor

// -----------------------------------------------------------------------------

fn str_as_naive_time<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(NaiveTimeVisitor)
} // fn integer_as_weekday

struct NaiveTimeVisitor;

impl Visitor<'_> for NaiveTimeVisitor {
    type Value = NaiveTime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string representation of time of day in 24-hour hhmm format")
        // write_str
    } // fn expecting

    fn visit_str<E>(self, value: &str) -> Result<NaiveTime, E>
    where
        E: serde::de::Error,
    {
        NaiveTime::parse_from_str(value, "%H%M").map_err(|_err| {
            E::invalid_value(
                Unexpected::Str(value),
                &"time of day in 24-hour hhmm format. values are in the range 0000–2359",
            ) // invalid_value
        }) // map_err
    } // fn visit_str
} // impl Visitor

// -----------------------------------------------------------------------------

impl std::str::FromStr for PlaceOpeningHoursPeriodDetail {
    type Err = serde_json::Error;
    /// Parse a Google Maps Places API JSON response into a usable
    /// `PlaceOpeningHoursPeriodDetail` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        let bytes = s.to_string().into_bytes();
        serde_json::from_slice(&bytes)
    } // fn from_str
} // impl FromStr
