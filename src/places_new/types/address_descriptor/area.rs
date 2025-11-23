use crate::places_new::{LocalizedText, Containment};

// -------------------------------------------------------------------------------------------------
//
/// Area information describing precise geographic regions useful for location description.
///
/// Areas provide context about the geographic region containing or adjacent to a target location.
/// This includes precise sublocalities, neighborhoods, and large compounds that help describe where
/// a place is situated within the broader geographic context.
///
/// Areas are ranked by recognizability and precision, with the most relevant areas listed first.
/// They define spatial relationships to help users understand location context and proximity.
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
pub struct Area {
    /// The area's resource name in `places/{place_id}` format.
    ///
    /// Provides a stable identifier for the area that can be used in subsequent API calls to
    /// retrieve additional details about the area itself.
    #[serde(default)]
    #[getset(get = "pub")]
    pub name: String,

    /// The area's place ID.
    ///
    /// A unique identifier that can be used with other Google Places API endpoints to fetch
    /// detailed information about this area, including its boundaries and characteristics.
    #[getset(get = "pub")]
    pub place_id: String,

    /// The area's localized display name.
    ///
    /// Human-readable name of the area in the requested language, suitable for display in user
    /// interfaces and location descriptions. This is what users will see when viewing area
    /// information.
    #[getset(get = "pub")]
    pub display_name: LocalizedText,

    /// The spatial relationship between the target location and this area.
    ///
    /// Defines how the target location relates to the area boundaries, whether it's within the
    /// area, on the outskirts, or nearby. Used to provide precise location context.
    #[serde(default)]
    #[getset(get = "pub")]
    pub containment: Containment,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Area {
    /// Creates a new `Area` with the specified properties.
    ///
    /// Used to construct area information with all required spatial and identification details.
    /// The containment relationship helps establish how the target location relates to this area.
    #[must_use]
    pub const fn new(
        name: String,
        place_id: String,
        display_name: LocalizedText,
        containment: Containment,
    ) -> Self {
        Self {
            name,
            place_id,
            display_name,
            containment,
        }
    }

    /// Gets the display name text, if available.
    ///
    /// Returns the human-readable area name for display purposes. Used when showing
    /// location context to users in applications and maps.
    #[must_use]
    pub fn display_text(&self) -> &str {
        self.display_name.text()
    }

    /// Gets the language code of the display name.
    ///
    /// Returns the BCP-47 language code indicating what language the display name is in. Useful for
    /// internationalization and proper text rendering.
    #[must_use]
    pub fn display_language(&self) -> &icu_locale::Locale {
        self.display_name.language()
    }

    /// Returns whether the target location is within this area.
    ///
    /// Used to identify areas that directly contain the target location, indicating the most
    /// precise geographic context available.
    #[must_use]
    pub const fn contains_target(&self) -> bool {
        matches!(
            self.containment,
            Containment::Within | Containment::Outskirts
        )
    }

    /// Returns whether the target location is in the center of this area.
    ///
    /// Used to identify areas where the target location is centrally positioned, providing the
    /// strongest geographic association.
    #[must_use]
    pub fn target_is_central(&self) -> bool {
        self.containment == Containment::Within
    }

    /// Returns whether the target location is on the outskirts of this area.
    ///
    /// Used to identify areas where the target location is at the edge, indicating a weaker but
    /// still direct geographic relationship.
    #[must_use]
    pub fn target_is_on_outskirts(&self) -> bool {
        self.containment == Containment::Outskirts
    }

    /// Returns whether the target location is near but outside this area.
    ///
    /// Used to identify adjacent areas that provide context without direct containment, useful for
    /// understanding the broader geographic neighborhood.
    #[must_use]
    pub fn target_is_nearby(&self) -> bool {
        self.containment == Containment::Near
    }

    /// Gets a formatted description of the area relationship.
    ///
    /// Returns a human-readable description of how the target location relates to this area.
    /// Used for displaying location context in user interfaces.
    #[must_use]
    pub fn relationship_description(&self) -> String {
        match self.containment {
            Containment::Within => format!("within {}", self.display_text()),
            Containment::Outskirts => format!("on the outskirts of {}", self.display_text()),
            Containment::Near => format!("near {}", self.display_text()),
            Containment::Unspecified => format!("related to {}", self.display_text()),
        }
    }

    /// Gets a compact summary suitable for brief displays.
    ///
    /// Provides a concise area description suitable for mobile interfaces, map overlays,
    /// or space-constrained displays.
    #[must_use]
    pub fn compact_summary(&self) -> String {
        match self.containment {
            Containment::Within => format!("In {}", self.display_text()),
            Containment::Outskirts => format!("Edge of {}", self.display_text()),
            Containment::Near => format!("Near {}", self.display_text()),
            Containment::Unspecified => self.display_text().to_string(),
        }
    }

    /// Returns the resource name without the places/ prefix.
    ///
    /// Extracts just the place ID portion from the resource name for use in APIs that
    /// expect only the identifier without the full resource path.
    #[must_use]
    pub fn extract_place_id_from_name(&self) -> Option<&str> {
        self.name.strip_prefix("places/")
    }

    /// Returns whether the resource name is properly formatted.
    ///
    /// Used to validate that the area has a correctly formatted resource name following
    /// the expected `places/{place_id}` pattern.
    #[must_use]
    pub fn has_valid_resource_name(&self) -> bool {
        self.name.starts_with("places/") && self.name.len() > "places/".len()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.relationship_description())
    }
}

// -------------------------------------------------------------------------------------------------
//
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "name": "places/test123",
            "placeId": "test123",
            "displayName": {
                "text": "Downtown",
                "languageCode": "en-US"
            },
            "containment": "WITHIN"
        }"#;

        let area: Area = serde_json::from_str(json).unwrap();
        assert_eq!(area.name(), "places/test123");
        assert_eq!(area.place_id(), "test123");
        assert_eq!(area.display_name().text(), "Downtown");
        assert_eq!(area.containment(), &Containment::Within);
    }
}