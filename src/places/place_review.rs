//! A review of the place submitted by a user.

use serde::{Deserialize, Serialize};

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
    pub time: i64,

    /// The URL to the user's Google Maps Local Guides profile, if available.
    pub author_url: Option<String>,

    /// An IETF language code indicating the language of the returned review.
    /// This field contains the main language tag only, and not the secondary
    /// tag indicating country or region. For example, all the English reviews
    /// are tagged as 'en', and not 'en-AU' or 'en-UK' and so on. This field is
    /// empty if there is only a rating with no review text.
    pub language: Option<String>,

    /// An IETF language code indicating the original language of the review. If
    /// the review has been translated, then original_language != language. This
    /// field contains the main language tag only, and not the secondary tag
    /// indicating country or region. For example, all the English reviews are
    /// tagged as 'en', and not 'en-AU' or 'en-UK' and so on. This field is
    /// empty if there is only a rating with no review text.
    pub original_language: Option<String>,

    /// The URL to the user's profile photo, if available.
    pub profile_photo_url: Option<String>,

    /// The user's review. When reviewing a location with Google Places, text
    /// reviews are considered optional. Therefore, this field may be empty.
    /// Note that this field may include simple HTML markup. For example, the
    /// entity reference `&amp;` may represent an ampersand character.
    pub text: Option<String>,

    /// A boolean value indicating if the review was translated from the
    /// original language it was written in. If a review has been translated,
    /// corresponding to a value of true, Google recommends that you indicate
    /// this to your users. For example, you can add the following string,
    /// “Translated by Google”, to the review.
    pub translated: Option<bool>,

} // struct PlaceReview

// -----------------------------------------------------------------------------

impl std::str::FromStr for PlaceReview {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Places API JSON response into a usable
    /// `PlaceReview` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    } // fn from_str
}  // impl FromStr