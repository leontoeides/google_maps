use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

// -------------------------------------------------------------------------------------------------
//
/// Defines the spatial relationship between a target location and a landmark.
///
/// Spatial relationships provide human-understandable descriptions of how places relate to nearby
/// landmarks, helping users navigate and understand locations in context.
///
/// These relationships describe position, proximity, and directional associations that make
/// geographic descriptions more intuitive and useful.
#[derive(
    // std
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
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
pub enum SpatialRelationship {
    /// Default relationship when nothing more specific applies.
    ///
    /// Used as a general proximity indicator when the target is close to a landmark but doesn't fit
    /// into any of the more specific spatial categories. This is the fallback for nearby but not
    /// precisely positioned relationships.
    #[default]
    Near = 0,

    /// The target is within the landmark's spatial boundaries.
    ///
    /// Used when the target location is completely contained inside a landmark that has defined
    /// geometric boundaries, such as being inside a shopping mall, park, campus, or building
    /// complex.
    Within = 1,

    /// The target is directly adjacent to the landmark.
    ///
    /// Used when the target location is immediately next to or touching the landmark, typically
    /// sharing a boundary or being in the next building, lot, or structure without any separation.
    Beside = 2,

    /// The target is directly opposite the landmark across a road.
    ///
    /// Used when the target location is on the opposite side of a street, road, or similar linear
    /// feature from the landmark. This relationship implies a clear view and direct line of sight
    /// across the dividing feature.
    AcrossTheRoad = 3,

    /// The target is on the same route as the landmark but at a distance.
    ///
    /// Used when the target location is further along or before the landmark on the same street,
    /// highway, or path, but not immediately beside it. Indicates they share the same route or
    /// linear reference.
    DownTheRoad = 4,

    /// The target is a single turn away from the landmark.
    ///
    /// Used when the target location requires one directional change from the landmark's position,
    /// typically around a corner or intersection. The target is nearby but not on the same direct
    /// route.
    AroundTheCorner = 5,

    /// The target is behind the landmark relative to street-facing orientation.
    ///
    /// Used when the target location is positioned away from the landmark's main street entrance or
    /// public-facing side, typically in back areas, rear parking, or service areas that are less
    /// visible from main roads.
    Behind = 6,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl SpatialRelationship {
    /// Returns whether this relationship indicates close proximity.
    ///
    /// Used to identify relationships where the target and landmark are immediately adjacent or
    /// very close, useful for pedestrian navigation and detailed location guidance.
    #[must_use]
    pub const fn is_close_proximity(self) -> bool {
        matches!(self, Self::Within | Self::Beside | Self::AcrossTheRoad | Self::Behind)
    }

    /// Returns whether this relationship indicates the target is contained within the landmark.
    ///
    /// Used to identify when the target location is inside a larger structure or area, which
    /// affects navigation instructions and accessibility considerations.
    #[must_use]
    pub const fn is_contained(self) -> bool {
        matches!(self, Self::Within)
    }

    /// Returns whether this relationship indicates the target requires navigation changes.
    ///
    /// Used to identify relationships where reaching the target from the landmark requires turns,
    /// route changes, or more complex navigation instructions.
    #[must_use]
    pub const fn requires_navigation(self) -> bool {
        matches!(self, Self::DownTheRoad | Self::AroundTheCorner)
    }

    /// Returns whether this relationship indicates visibility between target and landmark.
    ///
    /// Used to determine if the landmark can be seen from the target location, which is useful for
    /// visual navigation cues and landmark-based directions.
    #[must_use]
    pub const fn has_line_of_sight(self) -> bool {
        matches!(
            self,
            Self::Near | Self::Beside | Self::AcrossTheRoad | Self::DownTheRoad
        )
    }

    /// Returns whether this relationship suggests pedestrian accessibility.
    ///
    /// Used to identify relationships where walking between the target and landmark is
    /// straightforward, without major obstacles or complex routing.
    #[must_use]
    pub const fn is_pedestrian_friendly(self) -> bool {
        matches!(
            self,
            Self::Near | Self::Within | Self::Beside | Self::AroundTheCorner
        )
    }

    /// Returns a human-readable description of the spatial relationship.
    ///
    /// Provides clear, conversational descriptions suitable for giving directions or explaining
    /// location relationships in user interfaces and navigation apps.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Near => "near",
            Self::Within => "inside",
            Self::Beside => "next to",
            Self::AcrossTheRoad => "across the street from",
            Self::DownTheRoad => "down the road from",
            Self::AroundTheCorner => "around the corner from",
            Self::Behind => "behind",
        }
    }

    /// Returns a preposition suitable for use in navigation instructions.
    ///
    /// Provides grammatically correct prepositions for constructing natural language directions
    /// like "The cafe is **preposition** the library."
    #[must_use]
    pub const fn preposition(self) -> &'static str {
        match self {
            Self::Near => "near",
            Self::Within => "within",
            Self::Beside => "beside",
            Self::AcrossTheRoad => "across from",
            Self::DownTheRoad => "along the road from",
            Self::AroundTheCorner => "around the corner from",
            Self::Behind => "behind",
        }
    }

    /// Returns the relative navigation complexity for this relationship.
    ///
    /// Provides a simple scale (1-3) indicating how complex navigation between the target and
    /// landmark would be, useful for routing and instruction generation.
    #[must_use]
    pub const fn navigation_complexity(self) -> u8 {
        match self {
            Self::Within | Self::Beside => 1,                       // Very simple
            Self::Near | Self::AcrossTheRoad | Self::Behind => 2,   // Moderate
            Self::DownTheRoad | Self::AroundTheCorner => 3,         // More complex
        }
    }

    /// Returns an emoji representation for visual interfaces.
    ///
    /// Provides intuitive visual indicators that can enhance user interfaces, maps, or mobile
    /// applications with quick visual relationship cues.
    #[must_use]
    pub const fn emoji(self) -> &'static str {
        match self {
            Self::Near => "📍",
            Self::Within => "🏢",
            Self::Beside => "↔️",
            Self::AcrossTheRoad => "🛣️",
            Self::DownTheRoad => "➡️",
            Self::AroundTheCorner => "↪️",
            Self::Behind => "⬅️",
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialization() {
        let relation = SpatialRelationship::AcrossTheRoad;
        let json = serde_json::to_string(&relation).unwrap();
        assert_eq!(json, r#""ACROSS_THE_ROAD""#);

        let relation = SpatialRelationship::AroundTheCorner;
        let json = serde_json::to_string(&relation).unwrap();
        assert_eq!(json, r#""AROUND_THE_CORNER""#);
    }

    #[test]
    fn test_deserialization() {
        let json = r#""WITHIN""#;
        let relation: SpatialRelationship = serde_json::from_str(json).unwrap();
        assert_eq!(relation, SpatialRelationship::Within);

        let json = r#""NEAR""#;
        let relation: SpatialRelationship = serde_json::from_str(json).unwrap();
        assert_eq!(relation, SpatialRelationship::Near);
    }

    #[test]
    fn test_default() {
        let default_relation = SpatialRelationship::default();
        assert_eq!(default_relation, SpatialRelationship::Near);
    }

    #[test]
    fn test_close_proximity() {
        assert!(SpatialRelationship::Within.is_close_proximity());
        assert!(SpatialRelationship::Beside.is_close_proximity());
        assert!(SpatialRelationship::AcrossTheRoad.is_close_proximity());
        assert!(SpatialRelationship::Behind.is_close_proximity());
        assert!(!SpatialRelationship::DownTheRoad.is_close_proximity());
        assert!(!SpatialRelationship::AroundTheCorner.is_close_proximity());
    }

    #[test]
    fn test_contained() {
        assert!(SpatialRelationship::Within.is_contained());
        assert!(!SpatialRelationship::Beside.is_contained());
        assert!(!SpatialRelationship::Near.is_contained());
    }

    #[test]
    fn test_requires_navigation() {
        assert!(SpatialRelationship::DownTheRoad.requires_navigation());
        assert!(SpatialRelationship::AroundTheCorner.requires_navigation());
        assert!(!SpatialRelationship::Beside.requires_navigation());
        assert!(!SpatialRelationship::Within.requires_navigation());
    }

    #[test]
    fn test_line_of_sight() {
        assert!(SpatialRelationship::Near.has_line_of_sight());
        assert!(SpatialRelationship::Beside.has_line_of_sight());
        assert!(SpatialRelationship::AcrossTheRoad.has_line_of_sight());
        assert!(SpatialRelationship::DownTheRoad.has_line_of_sight());
        assert!(!SpatialRelationship::Behind.has_line_of_sight());
        assert!(!SpatialRelationship::AroundTheCorner.has_line_of_sight());
    }

    #[test]
    fn test_pedestrian_friendly() {
        assert!(SpatialRelationship::Near.is_pedestrian_friendly());
        assert!(SpatialRelationship::Within.is_pedestrian_friendly());
        assert!(SpatialRelationship::Beside.is_pedestrian_friendly());
        assert!(SpatialRelationship::AroundTheCorner.is_pedestrian_friendly());
        assert!(!SpatialRelationship::AcrossTheRoad.is_pedestrian_friendly()); // Crossing street
        assert!(!SpatialRelationship::DownTheRoad.is_pedestrian_friendly()); // Longer distance
    }

    #[test]
    fn test_descriptions() {
        assert_eq!(SpatialRelationship::Near.description(), "near");
        assert_eq!(SpatialRelationship::Within.description(), "inside");
        assert_eq!(SpatialRelationship::AcrossTheRoad.description(), "across the street from");
        assert_eq!(SpatialRelationship::AroundTheCorner.description(), "around the corner from");
    }

    #[test]
    fn test_prepositions() {
        assert_eq!(SpatialRelationship::Beside.preposition(), "beside");
        assert_eq!(SpatialRelationship::Within.preposition(), "within");
        assert_eq!(SpatialRelationship::Behind.preposition(), "behind");
    }

    #[test]
    fn test_navigation_complexity() {
        assert_eq!(SpatialRelationship::Within.navigation_complexity(), 1);
        assert_eq!(SpatialRelationship::Beside.navigation_complexity(), 1);
        assert_eq!(SpatialRelationship::Near.navigation_complexity(), 2);
        assert_eq!(SpatialRelationship::AcrossTheRoad.navigation_complexity(), 2);
        assert_eq!(SpatialRelationship::DownTheRoad.navigation_complexity(), 3);
        assert_eq!(SpatialRelationship::AroundTheCorner.navigation_complexity(), 3);
    }

    #[test]
    fn test_emojis() {
        assert_eq!(SpatialRelationship::Near.emoji(), "📍");
        assert_eq!(SpatialRelationship::Within.emoji(), "🏢");
        assert_eq!(SpatialRelationship::AcrossTheRoad.emoji(), "🛣️");
        assert_eq!(SpatialRelationship::AroundTheCorner.emoji(), "↪️");
    }
}