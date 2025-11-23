use crate::places_new::{LocalizedText, SpatialRelationship};
use icu_locale::Locale;

// -------------------------------------------------------------------------------------------------
//
/// Basic landmark information and the landmark's relationship with the target location.
///
/// Landmarks are prominent places that can be used to describe a location in relational terms. They
/// provide context and navigation aids by establishing spatial relationships between a target
/// location and well-known reference points. This helps users understand and navigate to locations
/// using familiar nearby landmarks.
#[derive(
    //std
    Clone,
    Debug,
    // getset
    getset::Getters,
    getset::CopyGetters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[serde(rename_all = "camelCase")]
pub struct Landmark {
    /// The landmark's resource name.
    ///
    /// Google Places API resource identifier that can be used in subsequent Place Details requests
    /// to fetch comprehensive information about this landmark, including detailed location data,
    /// hours, amenities, and other place-specific information.
    #[getset(get = "pub")]
    pub name: String,

    /// The landmark's place ID.
    ///
    /// Unique identifier that can be used across Google Maps Platform APIs to reference this
    /// landmark. This ID provides a stable way to identify the landmark for navigation, search, and
    /// integration with other location services.
    #[getset(get = "pub")]
    pub place_id: String,

    /// The landmark's display name.
    ///
    /// Human-readable name of the landmark in the appropriate language for the request. This is the
    /// name that should be displayed to users when describing the location relationship or
    /// providing navigation context.
    #[getset(get = "pub")]
    pub display_name: LocalizedText,

    /// A set of type tags for this landmark.
    ///
    /// Place types that categorize the landmark, such as `hospital`, `university`,
    /// `shopping_center`, etc. These types help understand what kind of landmark this is and its
    /// significance for navigation and location description.
    #[getset(get = "pub")]
    pub types: Vec<String>,

    /// Defines the spatial relationship between the target location and the landmark.
    ///
    /// Describes how the target location is positioned relative to this landmark, such as `near`,
    /// `across_the_road`, `behind`, etc. This relationship helps provide clear directional context
    /// for navigation and location understanding.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub spatial_relationship: Option<SpatialRelationship>,

    /// The straight line distance, in meters, between the target and the landmark.
    ///
    /// Direct distance measurement that provides objective proximity information. In some
    /// situations, this value can be longer than `travel_distance_meters` due to terrain, roads, or
    /// other navigation constraints.
    #[getset(get_copy = "pub")]
    pub straight_line_distance_meters: f64,

    /// The travel distance, in meters, along the road network from the target to the landmark.
    ///
    /// Actual distance that would need to be traveled along roads, paths, or walkways to reach the
    /// landmark. This distance doesn't account for mode of transportation but provides realistic
    /// navigation distance expectations.
    #[getset(get_copy = "pub")]
    pub travel_distance_meters: Option<f64>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Landmark {
    /// Creates a new Landmark with the specified identifiers and basic information.
    ///
    /// Used to construct a landmark with essential identification and naming information. Spatial
    /// relationship and distance data can be added separately as needed.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        place_id: impl Into<String>,
        display_name: LocalizedText,
        types: Vec<String>,
    ) -> Self {
        Self {
            name: name.into(),
            place_id: place_id.into(),
            display_name,
            types,
            spatial_relationship: None,
            straight_line_distance_meters: 0.0,
            travel_distance_meters: None,
        }
    }

    /// Creates a Landmark with complete spatial relationship information.
    ///
    /// Used when full spatial context is available, providing comprehensive location relationship
    /// data for detailed navigation and positioning.
    #[must_use]
    pub fn with_spatial_context(
        name: impl Into<String>,
        place_id: impl Into<String>,
        display_name: LocalizedText,
        types: Vec<String>,
        spatial_relationship: SpatialRelationship,
        straight_line_distance: f64,
        travel_distance: Option<f64>,
    ) -> Self {
        Self {
            name: name.into(),
            place_id: place_id.into(),
            display_name,
            types,
            spatial_relationship: Some(spatial_relationship),
            straight_line_distance_meters: straight_line_distance,
            travel_distance_meters: travel_distance,
        }
    }

    /// Returns whether this landmark has valid identifiers.
    ///
    /// Used to validate landmark data before using it for API calls or navigation, ensuring both
    /// name and place ID contain meaningful values.
    #[must_use]
    pub fn has_valid_identifiers(&self) -> bool {
        !self.name.trim().is_empty() && !self.place_id.trim().is_empty()
    }

    /// Gets the display name text in the landmark's language.
    ///
    /// Returns the human-readable name suitable for display in user interfaces, navigation
    /// instructions, and location descriptions.
    #[must_use]
    pub fn display_name_text(&self) -> &str {
        self.display_name.text()
    }

    /// Gets the language code of the display name.
    ///
    /// Returns the BCP-47 language code for the landmark name, useful for internationalization and
    /// language-specific processing.
    #[must_use]
    pub fn display_name_language(&self) -> &Locale {
        self.display_name.language()
    }

    /// Returns whether the landmark is in a specific language.
    ///
    /// Used for language-based filtering and localization, helping applications show landmarks with
    /// names in users' preferred languages.
    #[must_use]
    pub fn is_in_language(&self, language_code: &str) -> bool {
        self.display_name_language().normalizing_eq(language_code)
    }

    /// Returns whether this landmark is of a specific type.
    ///
    /// Used to filter landmarks by category, such as finding only hospitals, schools, or
    /// transportation hubs for specific navigation contexts.
    #[must_use]
    pub fn is_of_type(&self, place_type: &str) -> bool {
        self.types.iter().any(|t| t.eq_ignore_ascii_case(place_type))
    }

    /// Returns whether this landmark is close to the target.
    ///
    /// Used to identify landmarks that are in immediate proximity, based on straight-line distance
    /// or spatial relationship indicators. Returns true if the landmark is within 100 meters or has
    /// a close proximity spatial relationship.
    #[must_use]
    pub fn is_close_proximity(&self) -> bool {
        let has_close_relationship = self.spatial_relationship
            .is_some_and(SpatialRelationship::is_close_proximity);

        has_close_relationship || self.straight_line_distance_meters <= 100.0
    }

    /// Returns whether this landmark requires navigation to reach.
    ///
    /// Used to identify landmarks that would require turns or complex routing to reach from the
    /// target location, affecting navigation instructions. Returns false if no spatial relationship
    /// is specified.
    #[must_use]
    pub fn requires_navigation(&self) -> bool {
        self.spatial_relationship.is_some_and(SpatialRelationship::requires_navigation)
    }

    /// Gets the most relevant distance for display purposes.
    ///
    /// Returns travel distance if available (as it's more practical), otherwise falls back to
    /// straight-line distance for basic proximity information.
    #[must_use]
    pub fn display_distance(&self) -> f64 {
        self.travel_distance_meters.unwrap_or(self.straight_line_distance_meters)
    }

    /// Formats distance for user display with appropriate units.
    ///
    /// Returns distance in user-friendly format with meters for short distances and kilometers for
    /// longer distances, suitable for navigation interfaces.
    #[must_use]
    pub fn formatted_distance(&self) -> String {
        let dist = self.display_distance();
        if dist < 1_000.0 {
            format!("{dist:.0}m")
        } else {
            format!("{dist:.1}km", dist = dist / 1000.0)
        }
    }

    /// Gets the primary landmark type for categorization.
    ///
    /// Returns the first place type as the primary category, useful for grouping landmarks and
    /// applying type-specific logic or display rules.
    #[must_use]
    pub fn primary_type(&self) -> Option<&String> {
        self.types.first()
    }

    /// Returns whether this landmark is suitable for pedestrian navigation.
    ///
    /// Used to identify landmarks appropriate for walking directions, based on spatial relationship
    /// and distance characteristics. Returns true if within 500 meters or has a pedestrian-friendly
    /// spatial relationship.
    #[must_use]
    pub fn is_pedestrian_suitable(&self) -> bool {
        let is_pedestrian_friendly = self.spatial_relationship
            .is_some_and(SpatialRelationship::is_pedestrian_friendly);

        is_pedestrian_friendly || self.straight_line_distance_meters <= 500.0
    }

    /// Creates a location description using this landmark.
    ///
    /// Generates a human-readable description of the target location's relationship to this
    /// landmark, suitable for directions and location explanations. If no spatial relationship is
    /// specified, uses distance only.
    #[must_use]
    pub fn create_location_description(&self) -> String {
        let landmark_name = self.display_name_text();
        let distance = self.formatted_distance();

        let description = self.spatial_relationship
            .map_or_else(
                || format!("{landmark_name} ({distance})"),
                |sr| format!("{} {landmark_name} ({distance})", sr.description())
            );

        description
    }

    /// Gets a compact description suitable for mobile displays.
    ///
    /// Provides brief landmark reference suitable for mobile interfaces, map overlays, or
    /// space-constrained displays.
    #[must_use]
    pub fn compact_description(&self) -> String {
        format!(
            "{name} ({distance})",
            name = self.display_name_text(),
            distance = self.formatted_distance()
        )
    }

    /// Returns whether this landmark provides line of sight to the target.
    ///
    /// Used to determine if the landmark can be visually referenced from the target location, which
    /// affects the usefulness for visual navigation. Returns false if no spatial relationship is
    /// specified.
    #[must_use]
    pub fn has_line_of_sight(&self) -> bool {
        self.spatial_relationship
            .is_some_and(super::spatial_relationship::SpatialRelationship::has_line_of_sight)
    }

    /// Gets navigation complexity for this landmark relationship.
    ///
    /// Returns a complexity score indicating how difficult navigation between the target and
    /// landmark would be, useful for routing decisions. Returns 0 if no spatial relationship is
    /// specified.
    #[must_use]
    pub fn navigation_complexity(&self) -> u8 {
        self.spatial_relationship
            .map_or(0, super::spatial_relationship::SpatialRelationship::navigation_complexity)
    }

    /// Returns landmark importance based on type and relationship.
    ///
    /// Calculates relative importance for prioritizing landmarks in navigation and location
    /// descriptions, considering both landmark type and proximity.
    #[must_use]
    pub fn importance_score(&self) -> u32 {
        let mut score = 0u32;
        
        // Type-based importance
        if let Some(primary_type) = self.primary_type() {
            score += match primary_type.as_str() {
                "hospital" | "airport" | "train_station" => 100,
                "university" | "shopping_center" | "stadium" => 80,
                "school" | "park" | "library" => 60,
                "restaurant" | "gas_station" => 40,
                _ => 20,
            };
        }
        
        // Proximity bonus
        if self.is_close_proximity() {
            score += 50;
        }
        
        // Navigation simplicity bonus
        match self.navigation_complexity() {
            1 => score += 30,
            2 => score += 15,
            _ => {},
        }
        
        score
    }

    /// Creates a Google Maps URL for this landmark.
    ///
    /// Generates a direct link to view this landmark in Google Maps, useful for "View in Google
    /// Maps" functionality and external navigation.
    #[must_use]
    pub fn google_maps_url(&self) -> String {
        format!("https://maps.google.com/maps/place/?q=place_id:{}", self.place_id)
    }

    /// Returns appropriate navigation instruction prefix.
    ///
    /// Provides the appropriate preposition or directional phrase for creating natural language
    /// navigation instructions using this landmark. If no spatial relationship is specified, uses a
    /// generic "near" phrasing.
    #[must_use]
    pub fn navigation_instruction_prefix(&self) -> String {
        let preposition = self.spatial_relationship
            .map_or("near", super::spatial_relationship::SpatialRelationship::preposition);

        format!("Look for the location {preposition}")
    }

    /// Groups landmark types into broader categories.
    ///
    /// Returns a category classification that groups related place types, useful for filtering and
    /// organizing landmarks in user interfaces.
    #[must_use]
    pub fn type_category(&self) -> LandmarkCategory {
        self.primary_type()
            .map_or(LandmarkCategory::Other, |primary_type| match primary_type.as_str() {
                "hospital" | "pharmacy" | "doctor" | "dentist" => LandmarkCategory::Healthcare,
                "school" | "university" | "library" => LandmarkCategory::Education,
                "restaurant" | "cafe" | "bar" | "food" => LandmarkCategory::Dining,
                "shopping_center" | "store" | "department_store" => LandmarkCategory::Shopping,
                "airport" | "train_station" | "bus_station" | "subway_station" => LandmarkCategory::Transportation,
                "park" | "zoo" | "amusement_park" | "museum" => LandmarkCategory::Recreation,
                "bank" | "atm" | "post_office" | "courthouse" => LandmarkCategory::Services,
                "church" | "mosque" | "synagogue" | "temple" => LandmarkCategory::Religious,
                _ => LandmarkCategory::Other,
            })
    }
}

// -------------------------------------------------------------------------------------------------
//
/// Categories for landmark classification.
///
/// Used to group landmarks by their primary function or purpose, enabling category-based filtering
/// and logical organization in navigation and location description systems.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum LandmarkCategory {
    /// Healthcare facilities and medical services.
    Healthcare,
    /// Educational institutions and learning facilities.
    Education,
    /// Restaurants, cafes, and food establishments.
    Dining,
    /// Shopping centers and retail establishments.
    Shopping,
    /// Transportation hubs and transit facilities.
    Transportation,
    /// Parks, entertainment, and recreational facilities.
    Recreation,
    /// Government services and business facilities.
    Services,
    /// Religious buildings and places of worship.
    Religious,
    /// Other or unclassified landmarks.
    Other,
}

impl LandmarkCategory {
    /// Returns a user-friendly name for this category.
    ///
    /// Provides descriptive text suitable for display in category filters, grouping headers, or
    /// navigation interfaces.
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Healthcare => "Healthcare",
            Self::Education => "Education",
            Self::Dining => "Dining",
            Self::Shopping => "Shopping",
            Self::Transportation => "Transportation",
            Self::Recreation => "Recreation",
            Self::Services => "Services",
            Self::Religious => "Religious",
            Self::Other => "Other",
        }
    }

    /// Returns an icon representation for this category.
    ///
    /// Provides appropriate symbols for visual representation of landmark categories in maps and
    /// user interfaces.
    #[must_use]
    pub const fn icon(self) -> &'static str {
        match self {
            Self::Healthcare => "🏥",
            Self::Education => "🎓",
            Self::Dining => "🍽️",
            Self::Shopping => "🛍️",
            Self::Transportation => "🚇",
            Self::Recreation => "🎪",
            Self::Services => "🏛️",
            Self::Religious => "⛪",
            Self::Other => "📍",
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

    fn create_test_landmark() -> Landmark {
        Landmark::new(
            "places/ChIJExample123",
            "ChIJExample123",
            LocalizedText::new("Central Hospital", "en-US").unwrap(),
            vec!["hospital".to_string(), "health".to_string()],
        )
    }

    #[test]
    fn test_serialization() {
        let landmark = Landmark::with_spatial_context(
            "places/ChIJExample123",
            "ChIJExample123",
            LocalizedText::new("Central Park", "en-US").unwrap(),
            vec!["park".to_string()],
            SpatialRelationship::Near,
            150.0,
            Some(200.0),
        );

        let json = serde_json::to_string(&landmark).unwrap();
        assert!(json.contains("ChIJExample123"));
        assert!(json.contains("Central Park"));
        assert!(json.contains("NEAR"));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_deserialization() {
        let json = r#"{
            "name": "places/ChIJTest456",
            "placeId": "ChIJTest456",
            "displayName": {
                "text": "City Library",
                "languageCode": "en-US"
            },
            "types": ["library", "education"],
            "spatialRelationship": "ACROSS_THE_ROAD",
            "straightLineDistanceMeters": 75.5,
            "travelDistanceMeters": 120.0
        }"#;

        let landmark: Landmark = serde_json::from_str(json).unwrap();
        assert_eq!(landmark.place_id, "ChIJTest456");
        assert_eq!(landmark.display_name_text(), "City Library");
        assert_eq!(landmark.spatial_relationship, Some(SpatialRelationship::AcrossTheRoad));
        assert_eq!(landmark.straight_line_distance_meters, 75.5);
    }

    #[test]
    fn test_constructors() {
        let basic = create_test_landmark();
        assert_eq!(basic.display_name_text(), "Central Hospital");
        assert!(basic.has_valid_identifiers());

        let with_spatial = Landmark::with_spatial_context(
            "places/test",
            "test_id",
            LocalizedText::new("Test Landmark", "en").unwrap(),
            vec!["test".to_string()],
            SpatialRelationship::Beside,
            50.0,
            Some(75.0),
        );
        assert_eq!(with_spatial.spatial_relationship, Some(SpatialRelationship::Beside));
    }

    #[test]
    fn test_validation() {
        let valid = create_test_landmark();
        assert!(valid.has_valid_identifiers());

        let invalid = Landmark::new(
            "",
            "valid_id",
            LocalizedText::new("Test", "en").unwrap(),
            vec![],
        );
        assert!(!invalid.has_valid_identifiers());
    }

    #[test]
    fn test_language_detection() {
        let landmark = Landmark::new(
            "places/test",
            "test_id",
            LocalizedText::new("Test Landmark", "en-US").unwrap(),
            vec![],
        );

        assert!(landmark.is_in_language("en-US"));
        assert!(landmark.is_in_language("EN-US")); // Case insensitive
        assert!(!landmark.is_in_language("es-ES"));
        assert_eq!(landmark.display_name_language().to_string(), "en-US".to_string());
    }

    #[test]
    fn test_type_checking() {
        let landmark = create_test_landmark();
        
        assert!(landmark.is_of_type("hospital"));
        assert!(landmark.is_of_type("HOSPITAL")); // Case insensitive
        assert!(landmark.is_of_type("health"));
        assert!(!landmark.is_of_type("restaurant"));
        assert_eq!(landmark.primary_type(), Some(&"hospital".to_string()));
    }

    #[test]
    fn test_proximity_detection() {
        let close = Landmark::with_spatial_context(
            "places/test",
            "test_id",
            LocalizedText::new("Close Landmark", "en").unwrap(),
            vec![],
            SpatialRelationship::Beside,
            50.0,
            None,
        );
        assert!(close.is_close_proximity());

        let far = Landmark::with_spatial_context(
            "places/test",
            "test_id",
            LocalizedText::new("Far Landmark", "en").unwrap(),
            vec![],
            SpatialRelationship::DownTheRoad,
            500.0,
            None,
        );
        assert!(!far.is_close_proximity());
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_distance_formatting() {
        let landmark = Landmark::with_spatial_context(
            "places/test",
            "test_id",
            LocalizedText::new("Test", "en").unwrap(),
            vec![],
            SpatialRelationship::Near,
            150.0,
            Some(200.0),
        );

        assert_eq!(landmark.display_distance(), 200.0); // Prefers travel distance
        assert_eq!(landmark.formatted_distance(), "200m".to_string());

        let far_landmark = Landmark::with_spatial_context(
            "places/test",
            "test_id",
            LocalizedText::new("Test", "en").unwrap(),
            vec![],
            SpatialRelationship::Near,
            0.0,
            Some(1500.0),
        );
        assert_eq!(far_landmark.formatted_distance(), "1.5km".to_string());
    }

    #[test]
    fn test_navigation_properties() {
        let simple = Landmark::with_spatial_context(
            "places/test",
            "test_id",
            LocalizedText::new("Simple", "en").unwrap(),
            vec![],
            SpatialRelationship::Beside,
            50.0,
            None,
        );
        assert!(!simple.requires_navigation());
        assert!(simple.has_line_of_sight());
        assert!(simple.is_pedestrian_suitable());
        assert_eq!(simple.navigation_complexity(), 1);

        let complex = Landmark::with_spatial_context(
            "places/test",
            "test_id",
            LocalizedText::new("Complex", "en").unwrap(),
            vec![],
            SpatialRelationship::AroundTheCorner,
            300.0,
            None,
        );
        assert!(complex.requires_navigation());
        assert!(!complex.has_line_of_sight());
        assert!(complex.is_pedestrian_suitable());
        assert_eq!(complex.navigation_complexity(), 3);
    }

    #[test]
    fn test_descriptions() {
        let landmark = Landmark::with_spatial_context(
            "places/test",
            "test_id",
            LocalizedText::new("Central Library", "en").unwrap(),
            vec!["library".to_string()],
            SpatialRelationship::AcrossTheRoad,
            75.0,
            Some(100.0),
        );

        let location_desc = landmark.create_location_description();
        assert!(location_desc.contains("across the street from"));
        assert!(location_desc.contains("Central Library"));
        assert!(location_desc.contains("100m"));

        let compact = landmark.compact_description();
        assert_eq!(compact, "Central Library (100m)");
    }

    #[test]
    fn test_importance_scoring() {
        let hospital = Landmark::new(
            "places/hospital",
            "hospital_id",
            LocalizedText::new("City Hospital", "en").unwrap(),
            vec!["hospital".to_string()],
        );
        let hospital_score = hospital.importance_score();

        let restaurant = Landmark::new(
            "places/restaurant",
            "restaurant_id",
            LocalizedText::new("Local Cafe", "en").unwrap(),
            vec!["restaurant".to_string()],
        );
        let restaurant_score = restaurant.importance_score();

        assert!(hospital_score > restaurant_score); // Hospitals are more important landmarks
    }

    #[test]
    fn test_landmark_categories() {
        let hospital = create_test_landmark();
        assert_eq!(hospital.type_category(), LandmarkCategory::Healthcare);

        let school = Landmark::new(
            "places/school",
            "school_id",
            LocalizedText::new("Elementary School", "en").unwrap(),
            vec!["school".to_string()],
        );
        assert_eq!(school.type_category(), LandmarkCategory::Education);
    }

    #[test]
    fn test_category_display() {
        assert_eq!(LandmarkCategory::Healthcare.display_name(), "Healthcare");
        assert_eq!(LandmarkCategory::Transportation.icon(), "🚇");
        assert_eq!(LandmarkCategory::Dining.icon(), "🍽️");
    }

    #[test]
    fn test_google_maps_url() {
        let landmark = create_test_landmark();
        let url = landmark.google_maps_url();
        assert!(url.contains("https://maps.google.com/maps/place/"));
        assert!(url.contains("ChIJExample123"));
    }

    #[test]
    fn test_navigation_instructions() {
        let landmark = Landmark::with_spatial_context(
            "places/test",
            "test_id",
            LocalizedText::new("Test", "en").unwrap(),
            vec![],
            SpatialRelationship::Behind,
            0.0,
            None,
        );

        let prefix = landmark.navigation_instruction_prefix();
        assert!(prefix.contains("behind"));
    }
}