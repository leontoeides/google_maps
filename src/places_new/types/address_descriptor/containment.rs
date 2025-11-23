use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

// -------------------------------------------------------------------------------------------------
//
/// Defines the spatial relationship between a target location and an area.
///
/// Containment describes how a target location relates to larger geographic areas such as
/// neighborhoods, districts, or regions. This helps provide context about whether a place is
/// centrally located, on the periphery, or nearby but outside an area, which is useful for location
/// descriptions and geographic understanding.
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
pub enum Containment {
    /// The containment relationship is unspecified or unknown.
    ///
    /// Used as a fallback when the spatial relationship between the target location and area cannot
    /// be determined from available data. Applications should handle this as missing containment
    /// information.
    #[default]
    #[serde(rename = "CONTAINMENT_UNSPECIFIED")]
    Unspecified = 0,

    /// The target location is within the area region, close to the center.
    ///
    /// Used when the target location is well inside the boundaries of an area and positioned toward
    /// the central or core part of that region. This indicates strong association with the area's
    /// main characteristics.
    Within = 1,

    /// The target location is within the area region, close to the edge.
    ///
    /// Used when the target location is inside the area boundaries but positioned near the
    /// periphery or border. This indicates partial association with the area while being close to
    /// adjacent regions or areas.
    Outskirts = 2,

    /// The target location is outside the area region, but close by.
    ///
    /// Used when the target location is not within the area boundaries but is in close proximity to
    /// it. This indicates the location may be associated with or easily accessible from the area.
    Near = 3,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Containment {
    /// Returns whether the target location is inside the area boundaries.
    ///
    /// Used to determine if a location is officially within an area's limits, which affects
    /// area-based filtering, local regulations, and service areas.
    #[must_use]
    pub const fn is_inside_boundaries(self) -> Option<bool> {
        match self {
            Self::Within | Self::Outskirts => Some(true),
            Self::Near => Some(false),
            Self::Unspecified => None,
        }
    }

    /// Returns whether the target location is centrally positioned within the area.
    ///
    /// Used to identify locations that are representative of an area's core characteristics, useful
    /// for recommendations and area-specific searches.
    #[must_use]
    pub const fn is_central(self) -> bool {
        matches!(self, Self::Within)
    }

    /// Returns whether the target location is on the periphery of the area.
    ///
    /// Used to identify locations that may have characteristics of multiple areas or serve as
    /// transition zones between different regions.
    #[must_use]
    pub const fn is_peripheral(self) -> bool {
        matches!(self, Self::Outskirts)
    }

    /// Returns whether the target location is accessible from the area.
    ///
    /// Used to determine if a location can reasonably be considered part of an area's sphere of
    /// influence for services, transit, or recommendations.
    #[must_use]
    pub const fn is_accessible_from_area(self) -> Option<bool> {
        match self {
            Self::Within | Self::Outskirts | Self::Near => Some(true),
            Self::Unspecified => None,
        }
    }

    /// Returns the strength of association with the area (0-3 scale).
    ///
    /// Provides a numerical indication of how strongly associated the target location is with the
    /// area, useful for sorting and relevance scoring.
    #[must_use]
    pub const fn association_strength(self) -> Option<u8> {
        match self {
            Self::Within => Some(3),        // Strongest association
            Self::Outskirts => Some(2),     // Moderate association
            Self::Near => Some(1),          // Weak association
            Self::Unspecified => None,      // Unknown
        }
    }

    /// Returns whether this containment level is suitable for area-based recommendations.
    ///
    /// Used to determine if a location should be included when making recommendations for places
    /// "in" a particular area or neighborhood.
    #[must_use]
    pub const fn suitable_for_area_recommendations(self) -> bool {
        matches!(self, Self::Within | Self::Outskirts)
    }

    /// Returns a human-readable description of the containment relationship.
    ///
    /// Provides clear descriptions suitable for displaying location context in user interfaces,
    /// search results, and geographic descriptions.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Unspecified => "Unknown location relationship",
            Self::Within => "In the central area",
            Self::Outskirts => "On the outskirts",
            Self::Near => "Near the area",
        }
    }

    /// Returns a short description for compact displays.
    ///
    /// Provides brief location indicators suitable for mobile interfaces, map overlays, or anywhere
    /// space is limited.
    #[must_use]
    pub const fn short_description(self) -> &'static str {
        match self {
            Self::Unspecified => "Unknown",
            Self::Within => "Central",
            Self::Outskirts => "Edge",
            Self::Near => "Nearby",
        }
    }

    /// Returns a preposition for constructing location descriptions.
    ///
    /// Provides grammatically correct prepositions for natural language descriptions like "The
    /// restaurant is **preposition** downtown."
    #[must_use]
    pub const fn preposition(self) -> &'static str {
        match self {
            Self::Unspecified => "around",
            Self::Within => "in",
            Self::Outskirts => "on the edge of",
            Self::Near => "near",
        }
    }

    /// Returns whether this containment indicates official area membership.
    ///
    /// Used for administrative purposes, local regulations, or services that have strict boundary
    /// requirements.
    #[must_use]
    pub const fn is_official_member(self) -> Option<bool> {
        match self {
            Self::Within | Self::Outskirts => Some(true),
            Self::Near => Some(false),
            Self::Unspecified => None,
        }
    }

    /// Returns an emoji representation for visual interfaces.
    ///
    /// Provides intuitive visual indicators for containment relationships in maps, mobile apps, or
    /// other visual interfaces.
    #[must_use]
    pub const fn emoji(self) -> &'static str {
        match self {
            Self::Unspecified => "❓",
            Self::Within => "🎯",
            Self::Outskirts => "🔘",
            Self::Near => "📍",
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
        let containment = Containment::Within;
        let json = serde_json::to_string(&containment).unwrap();
        assert_eq!(json, r#""WITHIN""#);

        let containment = Containment::Outskirts;
        let json = serde_json::to_string(&containment).unwrap();
        assert_eq!(json, r#""OUTSKIRTS""#);
    }

    #[test]
    fn test_deserialization() {
        let json = r#""NEAR""#;
        let containment: Containment = serde_json::from_str(json).unwrap();
        assert_eq!(containment, Containment::Near);

        let json = r#""CONTAINMENT_UNSPECIFIED""#;
        let containment: Containment = serde_json::from_str(json).unwrap();
        assert_eq!(containment, Containment::Unspecified);
    }

    #[test]
    fn test_default() {
        let default_containment = Containment::default();
        assert_eq!(default_containment, Containment::Unspecified);
    }

    #[test]
    fn test_inside_boundaries() {
        assert_eq!(Containment::Within.is_inside_boundaries(), Some(true));
        assert_eq!(Containment::Outskirts.is_inside_boundaries(), Some(true));
        assert_eq!(Containment::Near.is_inside_boundaries(), Some(false));
        assert_eq!(Containment::Unspecified.is_inside_boundaries(), None);
    }

    #[test]
    fn test_central_and_peripheral() {
        assert!(Containment::Within.is_central());
        assert!(!Containment::Outskirts.is_central());
        assert!(!Containment::Near.is_central());

        assert!(!Containment::Within.is_peripheral());
        assert!(Containment::Outskirts.is_peripheral());
        assert!(!Containment::Near.is_peripheral());
    }

    #[test]
    fn test_accessible_from_area() {
        assert_eq!(Containment::Within.is_accessible_from_area(), Some(true));
        assert_eq!(Containment::Outskirts.is_accessible_from_area(), Some(true));
        assert_eq!(Containment::Near.is_accessible_from_area(), Some(true));
        assert_eq!(Containment::Unspecified.is_accessible_from_area(), None);
    }

    #[test]
    fn test_association_strength() {
        assert_eq!(Containment::Within.association_strength(), Some(3));
        assert_eq!(Containment::Outskirts.association_strength(), Some(2));
        assert_eq!(Containment::Near.association_strength(), Some(1));
        assert_eq!(Containment::Unspecified.association_strength(), None);
    }

    #[test]
    fn test_suitable_for_recommendations() {
        assert!(Containment::Within.suitable_for_area_recommendations());
        assert!(Containment::Outskirts.suitable_for_area_recommendations());
        assert!(!Containment::Near.suitable_for_area_recommendations());
        assert!(!Containment::Unspecified.suitable_for_area_recommendations());
    }

    #[test]
    fn test_official_membership() {
        assert_eq!(Containment::Within.is_official_member(), Some(true));
        assert_eq!(Containment::Outskirts.is_official_member(), Some(true));
        assert_eq!(Containment::Near.is_official_member(), Some(false));
        assert_eq!(Containment::Unspecified.is_official_member(), None);
    }

    #[test]
    fn test_descriptions() {
        assert_eq!(Containment::Within.description(), "In the central area");
        assert_eq!(Containment::Outskirts.description(), "On the outskirts");
        assert_eq!(Containment::Near.description(), "Near the area");
        assert_eq!(Containment::Unspecified.description(), "Unknown location relationship");
    }

    #[test]
    fn test_short_descriptions() {
        assert_eq!(Containment::Within.short_description(), "Central");
        assert_eq!(Containment::Outskirts.short_description(), "Edge");
        assert_eq!(Containment::Near.short_description(), "Nearby");
        assert_eq!(Containment::Unspecified.short_description(), "Unknown");
    }

    #[test]
    fn test_prepositions() {
        assert_eq!(Containment::Within.preposition(), "in");
        assert_eq!(Containment::Outskirts.preposition(), "on the edge of");
        assert_eq!(Containment::Near.preposition(), "near");
        assert_eq!(Containment::Unspecified.preposition(), "around");
    }

    #[test]
    fn test_emojis() {
        assert_eq!(Containment::Within.emoji(), "🎯");
        assert_eq!(Containment::Outskirts.emoji(), "🔘");
        assert_eq!(Containment::Near.emoji(), "📍");
        assert_eq!(Containment::Unspecified.emoji(), "❓");
    }
}