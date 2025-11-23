use crate::places_new::LocalizedText;
use url::Url;

// -------------------------------------------------------------------------------------------------
//
/// AI-generated summary of a place using user reviews.
///
/// Review summaries provide synthesized insights from user reviews, highlighting common themes,
/// sentiments, and notable aspects mentioned by customers. This helps users quickly understand what
/// reviewers typically say about a place without reading through individual reviews.
///
/// The content is generated using Gemini AI capabilities to analyze patterns and extract meaningful
/// insights from the collection of user reviews.
#[derive(
    //std
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    // getset
    getset::Getters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSummary {
    /// The summary of user reviews.
    ///
    /// Contains AI-generated text that synthesizes common themes, opinions, and experiences from
    /// user reviews. Highlights frequently mentioned aspects like service quality, atmosphere,
    /// value, and other relevant characteristics.
    #[getset(get = "pub")]
    pub text: LocalizedText,

    /// A link where users can flag a problem with the summary.
    ///
    /// Provides a URL where users can report issues with the AI-generated review summary, such as
    /// inaccurate representations of reviews or inappropriate content, enabling quality control and
    /// improvement of the summarization system.
    #[getset(get = "pub")]
    pub flag_content_uri: Url,

    /// The AI disclosure message indicating the content source.
    ///
    /// Contains localized text such as "Summarized with Gemini" to inform users that the summary
    /// was generated using AI technology. Appears in the language specified in the request when
    /// available.
    #[getset(get = "pub")]
    pub disclosure_text: LocalizedText,

    /// A link to show reviews of this place on Google Maps.
    ///
    /// Provides direct access to the full collection of user reviews on Google Maps, allowing users
    /// to read individual reviews for more detailed opinions and experiences.
    #[getset(get = "pub")]
    pub reviews_uri: Url,
}