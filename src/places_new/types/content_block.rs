use crate::places_new::LocalizedText;

// -------------------------------------------------------------------------------------------------
//
/// A block of content that can be served individually.
///
/// Content blocks provide structured information about places through AI-generated or curated
/// content. These blocks can contain various types of information and may reference other places to
/// provide comprehensive context and related recommendations.
///
/// Content blocks are designed to be modular and can be displayed independently or combined with
/// other blocks to create rich informational experiences.
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
pub struct ContentBlock {
    /// Content related to the topic.
    ///
    /// Contains the main textual information for this content block, including AI-generated
    /// summaries, descriptions, or other relevant content about the place or topic.
    #[getset(get = "pub")]
    pub content: LocalizedText,

    /// The list of resource names of referenced places.
    ///
    /// Contains resource names in `places/{place_id}` format for places that are mentioned or
    /// referenced within this content block. These can be used to make additional API calls to
    /// retrieve detailed information about related places.
    #[getset(get = "pub")]
    pub referenced_places: Vec<String>,
}