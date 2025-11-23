// -------------------------------------------------------------------------------------------------
//
/// Information about the accessibility options a place offers.
///
/// Accessibility options provide information about accommodations available for people with
/// disabilities, helping ensure access to businesses and services.
///
/// This information enables individuals with mobility needs to plan their visits and make informed
/// decisions about which venues can accommodate their requirements.
#[derive(
    //std
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
    // getset
    getset::CopyGetters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[allow(clippy::struct_field_names)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityOptions {
    /// Place offers wheelchair accessible parking.
    ///
    /// Indicates whether the business provides designated parking spaces that are wider and closer
    /// to entrances, complying with accessibility standards for individuals using wheelchairs or
    /// mobility devices.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub wheelchair_accessible_parking: Option<bool>,

    /// Place has wheelchair accessible entrance.
    ///
    /// Indicates whether the business has at least one entrance that can be used by individuals in
    /// wheelchairs, featuring appropriate door widths, ramps, and level access without barriers or
    /// steps.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub wheelchair_accessible_entrance: Option<bool>,

    /// Place has wheelchair accessible restroom.
    ///
    /// Indicates whether the business provides restroom facilities that meet accessibility
    /// standards, including appropriate door widths, grab bars, and sufficient space for wheelchair
    /// maneuvering.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub wheelchair_accessible_restroom: Option<bool>,

    /// Place has wheelchair accessible seating.
    ///
    /// Indicates whether the business provides seating areas that can accommodate wheelchairs,
    /// including appropriate table heights, accessible pathways, and designated wheelchair seating
    /// spaces.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub wheelchair_accessible_seating: Option<bool>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl AccessibilityOptions {
    /// Creates a new `AccessibilityOptions` with all options unspecified.
    ///
    /// Used to create an accessibility options structure where accessibility information will be
    /// added individually based on available venue data.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            wheelchair_accessible_parking: None,
            wheelchair_accessible_entrance: None,
            wheelchair_accessible_restroom: None,
            wheelchair_accessible_seating: None,
        }
    }

    /// Creates `AccessibilityOptions` indicating full wheelchair accessibility.
    ///
    /// Used for venues that provide comprehensive accessibility accommodations across all major
    /// areas including parking, entrance, restrooms, and seating.
    #[must_use]
    pub const fn fully_accessible() -> Self {
        Self {
            wheelchair_accessible_parking: Some(true),
            wheelchair_accessible_entrance: Some(true),
            wheelchair_accessible_restroom: Some(true),
            wheelchair_accessible_seating: Some(true),
        }
    }

    /// Creates `AccessibilityOptions` for basic accessibility compliance.
    ///
    /// Used for venues that meet minimum accessibility requirements with accessible entrance and
    /// restroom, but may have limited parking or seating options.
    #[must_use]
    pub const fn basic_accessibility() -> Self {
        Self {
            wheelchair_accessible_parking: None,
            wheelchair_accessible_entrance: Some(true),
            wheelchair_accessible_restroom: Some(true),
            wheelchair_accessible_seating: Some(false),
        }
    }

    /// Creates `AccessibilityOptions` indicating limited or no accessibility.
    ///
    /// Used for older venues or locations that have not yet been upgraded to meet modern
    /// accessibility standards, helping users understand limitations.
    #[must_use]
    pub const fn limited_accessibility() -> Self {
        Self {
            wheelchair_accessible_parking: Some(false),
            wheelchair_accessible_entrance: Some(false),
            wheelchair_accessible_restroom: Some(false),
            wheelchair_accessible_seating: Some(false),
        }
    }

    /// Returns whether any accessibility information is available.
    ///
    /// Used to determine if the business has provided any accessibility information, helping decide
    /// whether to display accessibility details to users.
    #[must_use]
    pub const fn has_accessibility_info(self) -> bool {
        self.wheelchair_accessible_parking.is_some()
            || self.wheelchair_accessible_entrance.is_some()
            || self.wheelchair_accessible_restroom.is_some()
            || self.wheelchair_accessible_seating.is_some()
    }

    /// Returns whether the venue is wheelchair accessible for entry and basic use.
    ///
    /// Used to determine if the essential accessibility features (entrance and restroom) are
    /// available, which are the minimum requirements for wheelchair accessibility.
    #[must_use]
    pub const fn is_wheelchair_accessible(self) -> Option<bool> {
        match (self.wheelchair_accessible_entrance, self.wheelchair_accessible_restroom) {
            (Some(true), Some(true)) => Some(true),
            (Some(false), _) | (_, Some(false)) => Some(false),
            _ => None, // Insufficient information
        }
    }

    /// Returns whether the venue provides comprehensive accessibility.
    ///
    /// Used to identify venues that offer full accessibility accommodations across all areas,
    /// providing confidence for users with mobility needs.
    #[must_use]
    pub fn is_fully_accessible(self) -> Option<bool> {
        let all_features = [
            self.wheelchair_accessible_parking,
            self.wheelchair_accessible_entrance,
            self.wheelchair_accessible_restroom,
            self.wheelchair_accessible_seating,
        ];

        if all_features.iter().all(|&feature| feature == Some(true)) {
            Some(true)
        } else if all_features.contains(&Some(false)) {
            Some(false)
        } else {
            None // Insufficient information
        }
    }

    /// Returns whether the venue has accessible arrival and departure.
    ///
    /// Used to determine if visitors can safely and independently arrive at and leave the venue,
    /// considering parking and entrance accessibility.
    #[must_use]
    pub const fn has_accessible_arrival(self) -> Option<bool> {
        match (self.wheelchair_accessible_parking, self.wheelchair_accessible_entrance) {
            (Some(true) | _, Some(true)) => Some(true),
            (_, Some(false)) => Some(false), // Cannot enter if entrance not accessible
            _ => None,
        }
    }

    /// Returns whether the venue provides accessible dining or seating experience.
    ///
    /// Used for restaurants, cafes, theaters, and other venues where seating is a primary part of
    /// the experience, ensuring comfort during visits.
    #[must_use]
    pub const fn has_accessible_dining(self) -> Option<bool> {
        match (self.wheelchair_accessible_entrance, self.wheelchair_accessible_seating) {
            (Some(true), Some(true)) => Some(true),
            (Some(false), _) | (_, Some(false)) => Some(false),
            _ => None,
        }
    }

    /// Gets a list of available accessibility features for display.
    ///
    /// Returns a vector of user-friendly accessibility feature names that can be displayed in user
    /// interfaces, filtered to only confirmed accommodations.
    #[must_use]
    pub fn available_features(self) -> Vec<&'static str> {
        let mut features = Vec::new();

        if self.wheelchair_accessible_parking == Some(true) {
            features.push("Accessible Parking");
        }
        if self.wheelchair_accessible_entrance == Some(true) {
            features.push("Accessible Entrance");
        }
        if self.wheelchair_accessible_restroom == Some(true) {
            features.push("Accessible Restroom");
        }
        if self.wheelchair_accessible_seating == Some(true) {
            features.push("Accessible Seating");
        }

        features
    }

    /// Gets a list of accessibility limitations for transparency.
    ///
    /// Returns features that are explicitly marked as not available, helping users understand
    /// specific limitations and plan accordingly.
    #[must_use]
    pub fn known_limitations(self) -> Vec<&'static str> {
        let mut limitations = Vec::new();

        if self.wheelchair_accessible_parking == Some(false) {
            limitations.push("No Accessible Parking");
        }
        if self.wheelchair_accessible_entrance == Some(false) {
            limitations.push("No Accessible Entrance");
        }
        if self.wheelchair_accessible_restroom == Some(false) {
            limitations.push("No Accessible Restroom");
        }
        if self.wheelchair_accessible_seating == Some(false) {
            limitations.push("No Accessible Seating");
        }

        limitations
    }

    /// Gets a user-friendly description of accessibility features.
    ///
    /// Provides a human-readable summary of accessibility accommodations suitable for displaying in
    /// search results, business listings, or detailed information views.
    #[must_use]
    pub fn description(self) -> String {
        let features = self.available_features();

        match features.len() {
            1 => format!("{} available", features[0]),
            2 => format!("{} and {} available", features[0], features[1]),
            _ => features.last().map_or_else(
                || if self.has_accessibility_info() {
                    "Limited accessibility information available".to_string()
                } else {
                    "Accessibility information not available".to_string()
                },
                |last| {
                    let others = features[..features.len()-1].join(", ");
                    format!("{others}, and {last} available")
                })
        }
    }

    /// Gets a short accessibility summary for compact displays.
    ///
    /// Provides brief accessibility information suitable for mobile interfaces, map overlays, or
    /// space-constrained displays with essential information.
    #[must_use]
    pub fn short_description(self) -> String {
        match self.is_wheelchair_accessible() {
            Some(true) => "Wheelchair Accessible".to_string(),
            Some(false) => "Limited Accessibility".to_string(),
            None => {
                let feature_count = self.available_features().len();
                if feature_count > 0 {
                    format!("{feature_count} Accessible Features")
                } else {
                    "Accessibility Unknown".to_string()
                }
            }
        }
    }

    /// Returns accessibility icons for visual display.
    ///
    /// Provides commonly recognized accessibility symbols for visual interfaces, maps, and quick
    /// identification of accessibility accommodations.
    #[must_use]
    pub fn accessibility_icons(self) -> Vec<&'static str> {
        let mut icons = Vec::new();

        match self.is_wheelchair_accessible() {
            Some(true) => icons.push("♿"), // Universal accessibility symbol
            Some(false) => icons.push("⚠️"), // Warning for limited accessibility
            None => {
                if !self.available_features().is_empty() {
                    icons.push("♿"); // Some accessibility features available
                }
            }
        }

        if self.wheelchair_accessible_parking == Some(true) {
            icons.push("🅿️"); // Accessible parking indicator
        }

        // Remove duplicates while preserving order
        icons.dedup();
        icons
    }

    /// Returns an accessibility compliance score (0-4 scale).
    ///
    /// Provides a numerical assessment of accessibility compliance, with higher scores indicating
    /// more comprehensive accessibility accommodations.
    #[must_use]
    pub fn compliance_score(self) -> Option<u8> {
        if !self.has_accessibility_info() {
            return None;
        }

        let mut score = 0u8;

        if self.wheelchair_accessible_parking == Some(true) {
            score += 1;
        }
        if self.wheelchair_accessible_entrance == Some(true) {
            score += 1;
        }
        if self.wheelchair_accessible_restroom == Some(true) {
            score += 1;
        }
        if self.wheelchair_accessible_seating == Some(true) {
            score += 1;
        }

        Some(score)
    }

    /// Returns whether this venue is suitable for accessibility-focused events.
    ///
    /// Used to identify venues appropriate for hosting events where accessibility is a primary
    /// consideration, requiring comprehensive accommodations.
    #[must_use]
    pub const fn is_event_accessible(self) -> Option<bool> {
        // Events typically require entrance, restroom, and seating accessibility
        match (
            self.wheelchair_accessible_entrance,
            self.wheelchair_accessible_restroom,
            self.wheelchair_accessible_seating,
        ) {
            (Some(true), Some(true), Some(true)) => Some(true),
            (Some(false), _, _) | (_, Some(false), _) | (_, _, Some(false)) => Some(false),
            _ => None,
        }
    }

    /// Returns whether accessibility features meet modern standards.
    ///
    /// Used to identify venues that likely comply with current accessibility regulations and
    /// provide contemporary accessibility accommodations.
    #[must_use]
    pub fn meets_modern_standards(self) -> Option<bool> {
        // Modern standards typically require at least entrance and restroom accessibility
        match self.compliance_score() {
            Some(score) if score >= 2 => Some(true),
            Some(score) if score < 2 => Some(false),
            _ => None,
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
        let accessibility = AccessibilityOptions {
            wheelchair_accessible_parking: Some(true),
            wheelchair_accessible_entrance: Some(true),
            wheelchair_accessible_restroom: Some(false),
            wheelchair_accessible_seating: None,
        };

        let json = serde_json::to_string(&accessibility).unwrap();
        assert!(json.contains("wheelchairAccessibleParking"));
        assert!(json.contains("wheelchairAccessibleEntrance"));
        assert!(json.contains("wheelchairAccessibleRestroom"));
        assert!(json.contains("wheelchairAccessibleSeating"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "wheelchairAccessibleParking": true,
            "wheelchairAccessibleEntrance": false,
            "wheelchairAccessibleRestroom": true,
            "wheelchairAccessibleSeating": null
        }"#;

        let accessibility: AccessibilityOptions = serde_json::from_str(json).unwrap();
        assert_eq!(accessibility.wheelchair_accessible_parking, Some(true));
        assert_eq!(accessibility.wheelchair_accessible_entrance, Some(false));
        assert_eq!(accessibility.wheelchair_accessible_restroom, Some(true));
        assert_eq!(accessibility.wheelchair_accessible_seating, None);
    }

    #[test]
    fn test_constructors() {
        let new = AccessibilityOptions::new();
        assert!(!new.has_accessibility_info());

        let fully = AccessibilityOptions::fully_accessible();
        assert_eq!(fully.is_fully_accessible(), Some(true));
        assert_eq!(fully.compliance_score(), Some(4));

        let basic = AccessibilityOptions::basic_accessibility();
        assert_eq!(basic.is_wheelchair_accessible(), Some(true));
        assert_eq!(basic.wheelchair_accessible_seating, Some(false));

        let limited = AccessibilityOptions::limited_accessibility();
        assert_eq!(limited.is_wheelchair_accessible(), Some(false));
        assert_eq!(limited.compliance_score(), Some(0));
    }

    #[test]
    fn test_has_accessibility_info() {
        let empty = AccessibilityOptions::new();
        assert!(!empty.has_accessibility_info());

        let partial = AccessibilityOptions {
            wheelchair_accessible_entrance: Some(true),
            ..AccessibilityOptions::new()
        };
        assert!(partial.has_accessibility_info());
    }

    #[test]
    fn test_is_wheelchair_accessible() {
        let accessible = AccessibilityOptions {
            wheelchair_accessible_entrance: Some(true),
            wheelchair_accessible_restroom: Some(true),
            ..AccessibilityOptions::new()
        };
        assert_eq!(accessible.is_wheelchair_accessible(), Some(true));

        let not_accessible = AccessibilityOptions {
            wheelchair_accessible_entrance: Some(false),
            wheelchair_accessible_restroom: Some(true),
            ..AccessibilityOptions::new()
        };
        assert_eq!(not_accessible.is_wheelchair_accessible(), Some(false));

        let unknown = AccessibilityOptions {
            wheelchair_accessible_entrance: None,
            wheelchair_accessible_restroom: Some(true),
            ..AccessibilityOptions::new()
        };
        assert_eq!(unknown.is_wheelchair_accessible(), None);
    }

    #[test]
    fn test_is_fully_accessible() {
        let fully = AccessibilityOptions::fully_accessible();
        assert_eq!(fully.is_fully_accessible(), Some(true));

        let partial = AccessibilityOptions {
            wheelchair_accessible_entrance: Some(true),
            wheelchair_accessible_restroom: Some(false),
            wheelchair_accessible_parking: Some(true),
            wheelchair_accessible_seating: Some(true),
        };
        assert_eq!(partial.is_fully_accessible(), Some(false));

        let unknown = AccessibilityOptions::new();
        assert_eq!(unknown.is_fully_accessible(), None);
    }

    #[test]
    fn test_has_accessible_arrival() {
        let with_parking = AccessibilityOptions {
            wheelchair_accessible_parking: Some(true),
            wheelchair_accessible_entrance: Some(true),
            ..AccessibilityOptions::new()
        };
        assert_eq!(with_parking.has_accessible_arrival(), Some(true));

        let entrance_only = AccessibilityOptions {
            wheelchair_accessible_parking: Some(false),
            wheelchair_accessible_entrance: Some(true),
            ..AccessibilityOptions::new()
        };
        assert_eq!(entrance_only.has_accessible_arrival(), Some(true));

        let no_entrance = AccessibilityOptions {
            wheelchair_accessible_parking: Some(true),
            wheelchair_accessible_entrance: Some(false),
            ..AccessibilityOptions::new()
        };
        assert_eq!(no_entrance.has_accessible_arrival(), Some(false));
    }

    #[test]
    fn test_has_accessible_dining() {
        let dining_accessible = AccessibilityOptions {
            wheelchair_accessible_entrance: Some(true),
            wheelchair_accessible_seating: Some(true),
            ..AccessibilityOptions::new()
        };
        assert_eq!(dining_accessible.has_accessible_dining(), Some(true));

        let no_seating = AccessibilityOptions {
            wheelchair_accessible_entrance: Some(true),
            wheelchair_accessible_seating: Some(false),
            ..AccessibilityOptions::new()
        };
        assert_eq!(no_seating.has_accessible_dining(), Some(false));
    }

    #[test]
    fn test_available_features() {
        let multiple = AccessibilityOptions {
            wheelchair_accessible_parking: Some(true),
            wheelchair_accessible_entrance: Some(true),
            wheelchair_accessible_restroom: Some(false),
            wheelchair_accessible_seating: Some(true),
        };
        let features = multiple.available_features();
        assert_eq!(features.len(), 3);
        assert!(features.contains(&"Accessible Parking"));
        assert!(features.contains(&"Accessible Entrance"));
        assert!(features.contains(&"Accessible Seating"));
        assert!(!features.contains(&"Accessible Restroom"));

        let none = AccessibilityOptions::limited_accessibility();
        assert!(none.available_features().is_empty());
    }

    #[test]
    fn test_known_limitations() {
        let limited = AccessibilityOptions {
            wheelchair_accessible_parking: Some(false),
            wheelchair_accessible_entrance: Some(true),
            wheelchair_accessible_restroom: Some(false),
            wheelchair_accessible_seating: None,
        };
        let limitations = limited.known_limitations();
        assert_eq!(limitations.len(), 2);
        assert!(limitations.contains(&"No Accessible Parking"));
        assert!(limitations.contains(&"No Accessible Restroom"));
    }

    #[test]
    fn test_description() {
        let single = AccessibilityOptions {
            wheelchair_accessible_entrance: Some(true),
            ..AccessibilityOptions::new()
        };
        assert_eq!(single.description(), "Accessible Entrance available");

        let multiple = AccessibilityOptions {
            wheelchair_accessible_entrance: Some(true),
            wheelchair_accessible_restroom: Some(true),
            wheelchair_accessible_seating: Some(true),
            ..AccessibilityOptions::new()
        };
        assert_eq!(multiple.description(), "Accessible Entrance, Accessible Restroom, and Accessible Seating available");

        let none = AccessibilityOptions::new();
        assert_eq!(none.description(), "Accessibility information not available");
    }

    #[test]
    fn test_short_description() {
        let accessible = AccessibilityOptions::basic_accessibility();
        assert_eq!(accessible.short_description(), "Wheelchair Accessible");

        let limited = AccessibilityOptions::limited_accessibility();
        assert_eq!(limited.short_description(), "Limited Accessibility");

        let partial = AccessibilityOptions {
            wheelchair_accessible_parking: Some(true),
            ..AccessibilityOptions::new()
        };
        assert_eq!(partial.short_description(), "1 Accessible Features");

        let unknown = AccessibilityOptions::new();
        assert_eq!(unknown.short_description(), "Accessibility Unknown");
    }

    #[test]
    fn test_accessibility_icons() {
        let accessible = AccessibilityOptions::fully_accessible();
        let icons = accessible.accessibility_icons();
        assert!(icons.contains(&"♿"));
        assert!(icons.contains(&"🅿️"));

        let limited = AccessibilityOptions::limited_accessibility();
        let icons = limited.accessibility_icons();
        assert!(icons.contains(&"⚠️"));

        let unknown = AccessibilityOptions::new();
        let icons = unknown.accessibility_icons();
        assert!(icons.is_empty());
    }

    #[test]
    fn test_compliance_score() {
        let fully = AccessibilityOptions::fully_accessible();
        assert_eq!(fully.compliance_score(), Some(4));

        let basic = AccessibilityOptions::basic_accessibility();
        assert_eq!(basic.compliance_score(), Some(2)); // entrance + restroom

        let limited = AccessibilityOptions::limited_accessibility();
        assert_eq!(limited.compliance_score(), Some(0));

        let unknown = AccessibilityOptions::new();
        assert_eq!(unknown.compliance_score(), None);
    }

    #[test]
    fn test_is_event_accessible() {
        let event_ready = AccessibilityOptions {
            wheelchair_accessible_entrance: Some(true),
            wheelchair_accessible_restroom: Some(true),
            wheelchair_accessible_seating: Some(true),
            ..AccessibilityOptions::new()
        };
        assert_eq!(event_ready.is_event_accessible(), Some(true));

        let no_seating = AccessibilityOptions {
            wheelchair_accessible_entrance: Some(true),
            wheelchair_accessible_restroom: Some(true),
            wheelchair_accessible_seating: Some(false),
            ..AccessibilityOptions::new()
        };
        assert_eq!(no_seating.is_event_accessible(), Some(false));
    }

    #[test]
    fn test_meets_modern_standards() {
        let modern = AccessibilityOptions {
            wheelchair_accessible_entrance: Some(true),
            wheelchair_accessible_restroom: Some(true),
            ..AccessibilityOptions::new()
        };
        assert_eq!(modern.meets_modern_standards(), Some(true));

        let outdated = AccessibilityOptions {
            wheelchair_accessible_entrance: Some(true),
            wheelchair_accessible_restroom: Some(false),
            wheelchair_accessible_parking: Some(false),
            wheelchair_accessible_seating: Some(false),
        };
        assert_eq!(outdated.meets_modern_standards(), Some(false));
    }

    #[test]
    fn test_default() {
        let default_accessibility = AccessibilityOptions::default();
        assert!(!default_accessibility.has_accessibility_info());
        assert_eq!(default_accessibility.compliance_score(), None);
    }
}