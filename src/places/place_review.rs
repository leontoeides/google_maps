//! A review of the place submitted by a user.

use crate::types::Language;
use chrono::{DateTime, LocalResult, TimeZone, Utc};
use serde::de::{Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

// -----------------------------------------------------------------------------
//
/// A review of the place submitted by a user.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PlaceReview {
    /// The name of the user who submitted the review. Anonymous reviews are
    /// attributed to "A Google user".
    pub author_name: String,

    /// The user's overall rating for this place. This is a whole number,
    /// ranging from 1 to 5.
    pub rating: u8,

    /// The time that the review was submitted in text, relative to the current
    /// time.
    pub relative_time_description: String,

    /// The time that the review was submitted, measured in the number of
    /// seconds since since midnight, January 1, 1970 UTC.
    #[serde(deserialize_with = "integer_as_date_time")]
    pub time: DateTime<Utc>,

    /// The URL to the user's Google Maps Local Guides profile, if available.
    #[serde(default)]
    pub author_url: Option<String>,

    /// An IETF language code indicating the language of the returned review.
    /// This field contains the main language tag only, and not the secondary
    /// tag indicating country or region. For example, all the English reviews
    /// are tagged as 'en', and not 'en-AU' or 'en-UK' and so on. This field is
    /// empty if there is only a rating with no review text.
    #[serde(default)]
    pub language: Option<Language>,

    /// An IETF language code indicating the original language of the review. If
    /// the review has been translated, then `original_language` != language. This
    /// field contains the main language tag only, and not the secondary tag
    /// indicating country or region. For example, all the English reviews are
    /// tagged as 'en', and not 'en-AU' or 'en-UK' and so on. This field is
    /// empty if there is only a rating with no review text.
    #[serde(default)]
    pub original_language: Option<Language>,

    /// The URL to the user's profile photo, if available.
    #[serde(default)]
    pub profile_photo_url: Option<String>,

    /// The user's review. When reviewing a location with Google Places, text
    /// reviews are considered optional. Therefore, this field may be empty.
    /// Note that this field may include simple HTML markup. For example, the
    /// entity reference `&amp;` may represent an ampersand character.
    #[serde(default)]
    pub text: Option<String>,

    /// A boolean value indicating if the review was translated from the
    /// original language it was written in. If a review has been translated,
    /// corresponding to a value of true, Google recommends that you indicate
    /// this to your users. For example, you can add the following string,
    /// “Translated by Google”, to the review.
    #[serde(default)]
    pub translated: Option<bool>,
} // struct PlaceReview

// -----------------------------------------------------------------------------

fn integer_as_date_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_u64(DateTimeUtcVisitor)
} // fn integer_as_date_time

struct DateTimeUtcVisitor;

impl Visitor<'_> for DateTimeUtcVisitor {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("integer representation of seconds since January 1, 1970 UTC")
        // write_str
    } // fn expecting

    fn visit_u64<E>(self, value: u64) -> Result<DateTime<Utc>, E>
    where
        E: serde::de::Error,
    {
        let value: i64 = match value.try_into() {
            Ok(value_as_i64) => value_as_i64,
            Err(_error) => {
                return Err(
                    E::invalid_value(
                        Unexpected::Unsigned(value),
                        &"UNIX timestamp representing seconds since January 1, 1970 UTC",
                    ), // invalid_value
                );
            } // Err
        }; // match

        match Utc.timestamp_opt(value, 0) {
            LocalResult::Single(date_time_utc) => Ok(date_time_utc),
            _ => Err(
                E::invalid_value(
                    Unexpected::Signed(value),
                    &"UNIX timestamp representing seconds since January 1, 1970 UTC",
                ), // invalid_value
            ), // Err
        } // match
    } // fn visit_i64
} // impl Visitor

// -----------------------------------------------------------------------------

impl std::str::FromStr for PlaceReview {
    type Err = serde_json::Error;
    /// Parse a Google Maps Places API JSON response into a usable
    /// `PlaceReview` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        let bytes = s.to_string().into_bytes();
        serde_json::from_slice(&bytes)
    } // fn from_str
} // impl FromStr
