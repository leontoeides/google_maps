use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

// -------------------------------------------------------------------------------------------------
//
/// Ranking preference for text search results.
///
/// Controls how the API orders places in the response. The best ranking method depends on what kind
/// of query you're making.
///
/// # Query Types and Ranking
///
/// **Type-based queries** (includes a Place Type like "restaurant" or "gas station"):
/// - Examples: "restaurants in NYC", "coffee shops near me", "gas stations in Seattle"
/// - Default ranking: `Relevance` (most relevant places first)
/// - You can explicitly set either `Relevance` or `Distance`
///
/// **Location/name queries** (specific place or just a location):
/// - Examples: "Eiffel Tower", "Mountain View, CA", "123 Main Street"
/// - Recommended: Leave `RankPreference` unset to let Google optimize automatically
/// - Setting a rank preference for these queries may not produce useful results
///
/// # When to Use Each
///
/// Use `Relevance` when you want places that best match what the user is looking for, considering
/// factors like ratings, popularity, and how well they match the query.
///
/// Use `Distance` when proximity is the primary concern, like "find the nearest gas station" or
/// "closest coffee shop".
#[derive(
    // std
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    // serde
    Serialize,
    Deserialize,
    // strum
    AsRefStr,
    Display,
    EnumIter,
    EnumString,
    FromRepr,
    IntoStaticStr
)]
#[non_exhaustive]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum RankPreference {
    /// Rank by search relevance to the query.
    ///
    /// Default for type-based queries like "restaurants in NYC". Places are ordered by how well
    /// they match the search intent, considering factors like ratings, popularity, and relevance.
    #[default]
    Relevance = 0,

    /// Rank by distance from the location bias or restriction.
    ///
    /// Places closer to the specified location appear first, regardless of how well they match
    /// other relevance factors. Most useful for proximity-focused queries like "nearest coffee
    /// shop".
    ///
    /// > ⚠️ Requires a location bias or restriction to be set in the request, otherwise distance
    /// > cannot be calculated.
    Distance = 1,
}