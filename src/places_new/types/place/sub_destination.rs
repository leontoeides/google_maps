// -------------------------------------------------------------------------------------------------
//
/// A specific place associated with a main destination for more targeted navigation.
///
/// Sub-destinations provide specific locations within larger, complex places like airports,
/// universities, shopping centers, or stadiums.
///
/// These enable more precise navigation by helping users find specific terminals, buildings,
/// stores, or facilities rather than just the general location. Each sub-destination has its own
/// place ID for detailed lookups.
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
pub struct SubDestination {
    /// The resource name of the sub-destination.
    ///
    /// Google Places API resource identifier that can be used in subsequent Place Details requests
    /// to fetch comprehensive information about this specific sub-destination, including detailed
    /// location data, hours, and amenities.
    #[getset(get = "pub")]
    pub name: String,

    /// The place ID of the sub-destination.
    ///
    /// Unique identifier that can be used across Google Maps Platform APIs to reference this
    /// specific location. This ID provides a stable way to identify the sub-destination for
    /// navigation, search, and integration with other location services.
    #[getset(get = "pub")]
    pub id: String,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl SubDestination {
    /// Creates a new `SubDestination` with the specified name and ID.
    ///
    /// Used to construct a sub-destination with the essential identifiers needed for API lookups
    /// and navigation. Both parameters should be non-empty for a valid sub-destination reference.
    #[must_use]
    pub fn new(name: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            id: id.into(),
        }
    }

    /// Returns whether this sub-destination has valid identifiers.
    ///
    /// Used to validate sub-destination data before using it for API calls or navigation, ensuring
    /// both name and ID contain meaningful values.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty() && !self.id.trim().is_empty()
    }

    /// Extracts a display-friendly name from the resource name.
    ///
    /// Attempts to extract a human-readable name from the Google Places API resource identifier,
    /// useful when the resource name contains more descriptive information than the basic ID.
    #[must_use]
    pub fn display_name(&self) -> String {
        // Extract the last part of the resource name which is typically more human-readable
        // e.g., "places/ChIJ..." -> "ChIJ..." or "Terminal A" -> "Terminal A"
        self.name
            .split('/')
            .next_back()
            .map_or_else(
                || self.name.clone(),
                |last_part| if last_part.starts_with("ChIJ") || last_part.starts_with("places/") {
                    self.name.clone()
                } else {
                    last_part.to_string()
                })
    }

    /// Returns whether this appears to be a terminal or gate sub-destination.
    ///
    /// Used to identify airport terminals, bus terminals, or similar transit-related
    /// sub-destinations for specialized handling in navigation and display logic.
    #[must_use]
    pub fn is_terminal(&self) -> bool {
        let name_lower = self.name.to_lowercase();
        let display_lower = self.display_name().to_lowercase();
        
        ["terminal", "gate", "concourse", "pier"].iter().any(|&keyword| {
            name_lower.contains(keyword) || display_lower.contains(keyword)
        })
    }

    /// Returns whether this appears to be a parking-related sub-destination.
    ///
    /// Used to identify parking lots, garages, or parking structures within larger venues, helpful
    /// for navigation and parking-specific features.
    #[must_use]
    pub fn is_parking(&self) -> bool {
        let name_lower = self.name.to_lowercase();
        let display_lower = self.display_name().to_lowercase();
        
        ["parking", "garage", "lot", "deck"].iter().any(|&keyword| {
            name_lower.contains(keyword) || display_lower.contains(keyword)
        })
    }

    /// Returns whether this appears to be a building or structure sub-destination.
    ///
    /// Used to identify specific buildings within campuses, complexes, or large facilities, useful
    /// for campus navigation and building directories.
    #[must_use]
    pub fn is_building(&self) -> bool {
        let name_lower = self.name.to_lowercase();
        let display_lower = self.display_name().to_lowercase();
        
        ["building", "hall", "center", "tower", "wing", "block"].iter().any(|&keyword| {
            name_lower.contains(keyword) || display_lower.contains(keyword)
        })
    }

    /// Returns whether this appears to be a store or retail sub-destination.
    ///
    /// Used to identify specific stores within shopping centers, malls, or retail complexes,
    /// helpful for shopping navigation and store directories.
    #[must_use]
    pub fn is_store(&self) -> bool {
        let name_lower = self.name.to_lowercase();
        let display_lower = self.display_name().to_lowercase();
        
        ["store", "shop", "boutique", "outlet", "kiosk"].iter().any(|&keyword| {
            name_lower.contains(keyword) || display_lower.contains(keyword)
        })
    }

    /// Gets the sub-destination category based on its characteristics.
    ///
    /// Returns a category classification that can be used for filtering, grouping, or specialized
    /// display logic based on the type of sub-destination.
    #[must_use]
    pub fn category(&self) -> SubDestinationCategory {
        if self.is_terminal() {
            SubDestinationCategory::Terminal
        } else if self.is_parking() {
            SubDestinationCategory::Parking
        } else if self.is_building() {
            SubDestinationCategory::Building
        } else if self.is_store() {
            SubDestinationCategory::Store
        } else {
            SubDestinationCategory::Other
        }
    }

    /// Generates a Google Maps URL for this sub-destination.
    ///
    /// Creates a direct link to view this sub-destination in Google Maps, useful for "View in
    /// Google Maps" functionality and external navigation.
    #[must_use]
    pub fn google_maps_url(&self) -> String {
        format!("https://maps.google.com/maps/place/?q=place_id:{}", self.id)
    }

    /// Returns whether this sub-destination is suitable for main navigation.
    ///
    /// Used to determine if this sub-destination should be prominently featured in navigation
    /// interfaces or if it's more of a secondary reference point.
    #[must_use]
    pub fn is_primary_destination(&self) -> bool {
        // Primary destinations are typically terminals, main buildings, or major facilities rather
        // than specific stores or minor facilities
        matches!(self.category(), 
            SubDestinationCategory::Terminal | 
            SubDestinationCategory::Building |
            SubDestinationCategory::Parking
        )
    }

    /// Gets an icon representation for this sub-destination type.
    ///
    /// Provides appropriate emoji or symbol for visual representation of the sub-destination
    /// category in user interfaces and maps.
    #[must_use]
    pub fn icon(&self) -> &'static str {
        match self.category() {
            SubDestinationCategory::Terminal => "🚪",
            SubDestinationCategory::Parking => "🅿️", 
            SubDestinationCategory::Building => "🏢",
            SubDestinationCategory::Store => "🏪",
            SubDestinationCategory::Other => "📍",
        }
    }

    /// Returns a short description suitable for compact displays.
    ///
    /// Provides brief, user-friendly description that combines the display name with category
    /// information for quick identification in mobile interfaces.
    #[must_use]
    pub fn short_description(&self) -> String {
        let display = self.display_name();
        match self.category() {
            SubDestinationCategory::Terminal |
                SubDestinationCategory::Parking |
                SubDestinationCategory::Building |
                SubDestinationCategory::Store => format!("{} {}", self.icon(), display),
            SubDestinationCategory::Other => display,
        }
    }

    /// Extracts potential keywords from the sub-destination name.
    ///
    /// Returns searchable terms that can be used for filtering or searching within collections of
    /// sub-destinations, useful for type-ahead search.
    #[must_use]
    pub fn search_keywords(&self) -> Vec<String> {
        let mut keywords = Vec::new();
        
        // Add the display name words
        keywords.extend(
            self.display_name()
                .split_whitespace()
                .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
                .filter(|word| !word.is_empty() && word.len() > 1)
        );
        
        // Add category-specific keywords
        match self.category() {
            SubDestinationCategory::Terminal => {
                keywords.extend(["terminal", "gate", "departure", "arrival"].iter().map(ToString::to_string));
            }
            SubDestinationCategory::Parking => {
                keywords.extend(["parking", "garage", "cars", "vehicles"].iter().map(ToString::to_string));
            }
            SubDestinationCategory::Building => {
                keywords.extend(["building", "office", "facility"].iter().map(ToString::to_string));
            }
            SubDestinationCategory::Store => {
                keywords.extend(["store", "shop", "retail", "shopping"].iter().map(ToString::to_string));
            }
            SubDestinationCategory::Other => {}
        }
        
        // Remove duplicates while preserving order
        keywords.sort();
        keywords.dedup();
        keywords
    }
}

// -------------------------------------------------------------------------------------------------
//
/// Classification categories for sub-destinations.
///
/// Used to group and filter sub-destinations by their primary purpose, enabling specialized
/// handling and display logic for different types of locations within larger venues.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum SubDestinationCategory {
    /// Airport terminals, bus terminals, train stations, or similar transit facilities.
    Terminal,
    /// Parking lots, garages, or parking structures.
    Parking,
    /// Buildings, halls, or major structural divisions.
    Building,
    /// Retail stores, shops, or commercial establishments.
    Store,
    /// Other or unclassified sub-destinations.
    Other,
}

impl SubDestinationCategory {
    /// Returns a user-friendly name for this category.
    ///
    /// Provides descriptive text suitable for display in category filters, grouping headers, or
    /// navigation interfaces.
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Terminal => "Terminal",
            Self::Parking => "Parking",
            Self::Building => "Building", 
            Self::Store => "Store",
            Self::Other => "Other",
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
        let sub_dest = SubDestination {
            name: "places/ChIJExample123".to_string(),
            id: "ChIJExample123".to_string(),
        };

        let json = serde_json::to_string(&sub_dest).unwrap();
        assert!(json.contains("ChIJExample123"));
        assert!(json.contains("name"));
        assert!(json.contains("id"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "name": "Terminal A",
            "id": "ChIJTerminalA123"
        }"#;

        let sub_dest: SubDestination = serde_json::from_str(json).unwrap();
        assert_eq!(sub_dest.name, "Terminal A");
        assert_eq!(sub_dest.id, "ChIJTerminalA123");
    }

    #[test]
    fn test_constructor() {
        let sub_dest = SubDestination::new("Test Name", "test_id_123");
        assert_eq!(sub_dest.name, "Test Name");
        assert_eq!(sub_dest.id, "test_id_123");
    }

    #[test]
    fn test_is_valid() {
        let valid = SubDestination::new("Terminal A", "ChIJ123");
        assert!(valid.is_valid());

        let empty_name = SubDestination::new("", "ChIJ123");
        assert!(!empty_name.is_valid());

        let empty_id = SubDestination::new("Terminal A", "");
        assert!(!empty_id.is_valid());

        let whitespace = SubDestination::new("   ", "  ");
        assert!(!whitespace.is_valid());
    }

    #[test]
    fn test_display_name() {
        let resource_name = SubDestination::new("places/ChIJ123", "ChIJ123");
        assert_eq!(resource_name.display_name(), "places/ChIJ123");

        let simple_name = SubDestination::new("Terminal A", "ChIJ123");
        assert_eq!(simple_name.display_name(), "Terminal A");

        let path_name = SubDestination::new("venues/airport/Terminal-B", "ChIJ456");
        assert_eq!(path_name.display_name(), "Terminal-B");
    }

    #[test]
    fn test_category_detection() {
        let terminal = SubDestination::new("Terminal A", "id1");
        assert!(terminal.is_terminal());
        assert_eq!(terminal.category(), SubDestinationCategory::Terminal);

        let gate = SubDestination::new("Gate 15B", "id2");
        assert!(gate.is_terminal());

        let parking = SubDestination::new("Parking Garage Level 3", "id3");
        assert!(parking.is_parking());
        assert_eq!(parking.category(), SubDestinationCategory::Parking);

        let building = SubDestination::new("Science Building", "id4");
        assert!(building.is_building());
        assert_eq!(building.category(), SubDestinationCategory::Building);

        let store = SubDestination::new("Apple Store", "id5");
        assert!(store.is_store());
        assert_eq!(store.category(), SubDestinationCategory::Store);

        let other = SubDestination::new("Food Court", "id6");
        assert_eq!(other.category(), SubDestinationCategory::Other);
    }

    #[test]
    fn test_google_maps_url() {
        let sub_dest = SubDestination::new("Terminal A", "ChIJ123ABC");
        let url = sub_dest.google_maps_url();
        assert!(url.contains("https://maps.google.com/maps/place/"));
        assert!(url.contains("ChIJ123ABC"));
    }

    #[test]
    fn test_is_primary_destination() {
        let terminal = SubDestination::new("Terminal A", "id1");
        assert!(terminal.is_primary_destination());

        let building = SubDestination::new("Main Building", "id2");
        assert!(building.is_primary_destination());

        let parking = SubDestination::new("Parking Lot", "id3");
        assert!(parking.is_primary_destination());

        let store = SubDestination::new("Coffee Shop", "id4");
        assert!(!store.is_primary_destination());

        let other = SubDestination::new("Information Desk", "id5");
        assert!(!other.is_primary_destination());
    }

    #[test]
    fn test_icon() {
        let terminal = SubDestination::new("Terminal B", "id1");
        assert_eq!(terminal.icon(), "🚪");

        let parking = SubDestination::new("Parking Deck", "id2");
        assert_eq!(parking.icon(), "🅿️");

        let building = SubDestination::new("Office Building", "id3");
        assert_eq!(building.icon(), "🏢");

        let store = SubDestination::new("Gift Shop", "id4");
        assert_eq!(store.icon(), "🏪");

        let other = SubDestination::new("Fountain", "id5");
        assert_eq!(other.icon(), "📍");
    }

    #[test]
    fn test_short_description() {
        let terminal = SubDestination::new("Terminal C", "id1");
        assert_eq!(terminal.short_description(), "🚪 Terminal C");

        let parking = SubDestination::new("Level 2 Parking", "id2");
        assert_eq!(parking.short_description(), "🅿️ Level 2 Parking");

        let other = SubDestination::new("Information", "id3");
        assert_eq!(other.short_description(), "Information");
    }

    #[test]
    fn test_search_keywords() {
        let terminal = SubDestination::new("Terminal A Gate 5", "id1");
        let keywords = terminal.search_keywords();
        assert!(keywords.contains(&"terminal".to_string()));
        assert!(keywords.contains(&"gate".to_string()));
        assert!(keywords.contains(&"departure".to_string()));
        assert!(keywords.len() > 3);

        let parking = SubDestination::new("North Parking Garage", "id2");
        let keywords = parking.search_keywords();
        assert!(keywords.contains(&"north".to_string()));
        assert!(keywords.contains(&"parking".to_string()));
        assert!(keywords.contains(&"garage".to_string()));
    }

    #[test]
    fn test_category_display_name() {
        assert_eq!(SubDestinationCategory::Terminal.display_name(), "Terminal");
        assert_eq!(SubDestinationCategory::Parking.display_name(), "Parking");
        assert_eq!(SubDestinationCategory::Building.display_name(), "Building");
        assert_eq!(SubDestinationCategory::Store.display_name(), "Store");
        assert_eq!(SubDestinationCategory::Other.display_name(), "Other");
    }
}