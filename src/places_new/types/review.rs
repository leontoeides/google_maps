use crate::places_new::LocalizedText;
use url::Url;

// -------------------------------------------------------------------------------------------------
//
/// Information about a review of a place.
/// 
/// Reviews provide user feedback and ratings for places, including the review text, rating score,
/// author information, and metadata about when the review was published. This information helps
/// users make informed decisions about places.
#[derive(
    //std
    Clone,
    Debug,
    Default,
    PartialEq,
    // getset
    getset::CopyGetters,
    getset::Getters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    /// A reference representing this place review which may be used to look up this place review
    /// again.
    /// 
    /// This is also called the API "resource" name in the format
    /// `places/{place_id}/reviews/{review_id}`. This name can be used in subsequent API calls to
    /// reference this specific review.
    #[serde(default)]
    #[getset(get = "pub")]
    pub name: Option<String>,

    /// A string of formatted recent time, expressing the review time relative to the current time.
    /// 
    /// This is presented in a form appropriate for the language and country, such as "2 weeks ago"
    /// or "3 months ago". This provides a user-friendly way to understand when the review was
    /// written.
    #[serde(default)]
    #[getset(get = "pub")]
    pub relative_publish_time_description: Option<String>,

    /// The localized text of the review.
    /// 
    /// This contains the main review content in the language it was written in, along with language
    /// code information for proper display and processing.
    #[serde(default)]
    #[getset(get = "pub")]
    pub text: Option<LocalizedText>,

    /// The review text in its original language.
    /// 
    /// This field preserves the review text as originally written by the author,/ which may differ
    /// from the `text` field if translation or localization has occurred.
    #[serde(default)]
    #[getset(get = "pub")]
    pub original_text: Option<LocalizedText>,

    /// A number between 1.0 and 5.0, also called the number of stars.
    /// 
    /// This represents the reviewer's rating of the place on a standard 5-star scale, where 1.0 is
    /// the lowest rating and 5.0 is the highest rating.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub rating: Option<f64>,

    /// This review's author.
    /// 
    /// Contains attribution information about who wrote the review, including their display name
    /// and profile information for proper crediting.
    #[serde(default)]
    #[getset(get = "pub")]
    pub author_attribution: Option<crate::places_new::AuthorAttribution>,

    /// Timestamp for the review.
    /// 
    /// Uses RFC 3339 format, where generated output will always be Z-normalized and uses 0, 3, 6 or
    /// 9 fractional digits. Offsets other than "Z" are also accepted. Examples:
    /// "2014-10-02T15:01:23Z", "2014-10-02T15:01:23.045123456Z".
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub publish_time: Option<jiff::Timestamp>,

    /// A link where users can flag a problem with the review.
    /// 
    /// This URI allows users to report inappropriate content or other issues with the review to the
    /// platform for moderation.
    #[serde(default)]
    #[getset(get = "pub")]
    pub flag_content_uri: Option<Url>,

    /// A link to show the review on Google Maps.
    /// 
    /// This URI provides a direct link to view this review within the Google Maps interface,
    /// allowing users to see the review in its full context.
    #[serde(default)]
    #[getset(get = "pub")]
    pub google_maps_uri: Option<Url>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Review {
    /// Returns whether this review has review text.
    #[must_use]
    pub const fn has_text(&self) -> bool {
        self.text.is_some()
    }

    /// Returns whether this review has original text (potentially in a different language).
    #[must_use]
    pub const fn has_original_text(&self) -> bool {
        self.original_text.is_some()
    }

    /// Returns whether this review has a rating.
    #[must_use]
    pub const fn has_rating(&self) -> bool {
        self.rating.is_some()
    }

    /// Returns whether this review has author attribution information.
    #[must_use]
    pub const fn has_author_attribution(&self) -> bool {
        self.author_attribution.is_some()
    }

    /// Returns whether this review has a publish time.
    #[must_use]
    pub const fn has_publish_time(&self) -> bool {
        self.publish_time.is_some()
    }

    /// Returns whether this review has been translated.
    ///
    /// Used to determine if a "See original" UI element should be shown, allowing users to view
    /// the review in its original language alongside the translation.
    #[must_use]
    pub fn is_translated(&self) -> bool {
        match (&self.text, &self.original_text) {
            (Some(text), Some(original)) =>
                text.language() != original.language(),
            _ => false,
        }
    }
}