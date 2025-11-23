// -------------------------------------------------------------------------------------------------
//
/// Information about parking options for a place.
///
/// Parking options provide details about the different types of parking available at or near a
/// business location.
///
/// This information helps customers plan their visit, understand parking costs, and choose
/// appropriate transportation methods. A parking facility can support multiple options
/// simultaneously.
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
#[serde(rename_all = "camelCase")]
pub struct ParkingOptions {
    /// Place offers free parking lots.
    ///
    /// Indicates whether the business provides complimentary parking in dedicated parking lots or
    /// areas. This is often the most convenient and cost-effective parking option for customers
    /// driving to the location.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub free_parking_lot: Option<bool>,

    /// Place offers paid parking lots.
    ///
    /// Indicates whether the business has paid parking lots available, either operated by the
    /// business itself or by third-party parking companies. Fees may vary by duration, time of day,
    /// or event schedules.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub paid_parking_lot: Option<bool>,

    /// Place offers free street parking.
    ///
    /// Indicates whether free on-street parking is available near the location. This may include
    /// metered spots during certain hours or unmetered public street parking, subject to local
    /// regulations and time limits.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub free_street_parking: Option<bool>,

    /// Place offers paid street parking.
    ///
    /// Indicates whether paid on-street parking is available near the location. This typically
    /// includes metered parking spots or paid parking zones managed by municipal authorities or
    /// parking companies.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub paid_street_parking: Option<bool>,

    /// Place offers valet parking.
    ///
    /// Indicates whether the business provides valet parking services where attendants park and
    /// retrieve vehicles for customers. This is common at upscale restaurants, hotels, and
    /// entertainment venues.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub valet_parking: Option<bool>,

    /// Place offers free garage parking.
    ///
    /// Indicates whether free parking is available in covered or enclosed parking structures. This
    /// provides protection from weather and may be more secure than surface parking options.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub free_garage_parking: Option<bool>,

    /// Place offers paid garage parking.
    ///
    /// Indicates whether paid parking is available in covered or enclosed parking structures.
    /// Garage parking often offers better security and weather protection, typically at higher
    /// rates than surface parking.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub paid_garage_parking: Option<bool>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl ParkingOptions {
    /// Creates a new `ParkingOptions` with all options unspecified.
    ///
    /// Used to create a parking options structure where parking information will be added
    /// individually based on available data about the location.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            free_parking_lot: None,
            paid_parking_lot: None,
            free_street_parking: None,
            paid_street_parking: None,
            valet_parking: None,
            free_garage_parking: None,
            paid_garage_parking: None,
        }
    }

    /// Creates `ParkingOptions` with only free parking available.
    ///
    /// Used for locations that offer complimentary parking in lots and garages, common for suburban
    /// businesses, shopping centers, and customer-focused venues.
    #[must_use]
    pub const fn free_only() -> Self {
        Self {
            free_parking_lot: Some(true),
            paid_parking_lot: Some(false),
            free_street_parking: Some(true),
            paid_street_parking: Some(false),
            valet_parking: Some(false),
            free_garage_parking: Some(true),
            paid_garage_parking: Some(false),
        }
    }

    /// Creates `ParkingOptions` for premium locations with valet service.
    ///
    /// Used for upscale establishments that offer valet parking along with other premium parking
    /// options, typical of luxury hotels and restaurants.
    #[must_use]
    pub const fn with_valet() -> Self {
        Self {
            free_parking_lot: Some(false),
            paid_parking_lot: Some(true),
            free_street_parking: Some(false),
            paid_street_parking: Some(true),
            valet_parking: Some(true),
            free_garage_parking: Some(false),
            paid_garage_parking: Some(true),
        }
    }

    /// Creates `ParkingOptions` for urban locations with limited parking.
    ///
    /// Used for city center businesses where parking is primarily street-based and paid, reflecting
    /// typical urban parking constraints and pricing.
    #[must_use]
    pub const fn urban_limited() -> Self {
        Self {
            free_parking_lot: Some(false),
            paid_parking_lot: Some(false),
            free_street_parking: Some(false),
            paid_street_parking: Some(true),
            valet_parking: Some(false),
            free_garage_parking: Some(false),
            paid_garage_parking: Some(true),
        }
    }

    /// Returns whether any parking information is available.
    ///
    /// Used to determine if the business has provided any parking information, helping decide
    /// whether to display parking details to users.
    #[must_use]
    pub const fn has_parking_info(self) -> bool {
        self.free_parking_lot.is_some()
            || self.paid_parking_lot.is_some()
            || self.free_street_parking.is_some()
            || self.paid_street_parking.is_some()
            || self.valet_parking.is_some()
            || self.free_garage_parking.is_some()
            || self.paid_garage_parking.is_some()
    }

    /// Returns whether any free parking options are available.
    ///
    /// Used to identify locations where customers can park without cost, important for
    /// budget-conscious visitors and cost-sensitive decisions.
    #[must_use]
    pub fn has_free_parking(self) -> Option<bool> {
        let free_options = [
            self.free_parking_lot,
            self.free_street_parking,
            self.free_garage_parking,
        ];

        if free_options.contains(&Some(true)) {
            Some(true)
        } else if free_options.iter().all(|&opt| opt == Some(false)) {
            Some(false)
        } else {
            None // Insufficient information
        }
    }

    /// Returns whether any paid parking options are available.
    ///
    /// Used to identify locations where parking fees should be expected, helping customers budget
    /// for their visit and plan accordingly.
    #[must_use]
    pub fn has_paid_parking(self) -> Option<bool> {
        let paid_options = [
            self.paid_parking_lot,
            self.paid_street_parking,
            self.valet_parking,
            self.paid_garage_parking,
        ];

        if paid_options.contains(&Some(true)) {
            Some(true)
        } else if paid_options.iter().all(|&opt| opt == Some(false)) {
            Some(false)
        } else {
            None
        }
    }

    /// Returns whether covered parking (garage) is available.
    ///
    /// Used to identify locations offering weather protection and potentially more secure parking,
    /// important for valuable vehicles or inclement weather.
    #[must_use]
    pub const fn has_covered_parking(self) -> Option<bool> {
        match (self.free_garage_parking, self.paid_garage_parking) {
            (Some(true), _) | (_, Some(true)) => Some(true),
            (Some(false), Some(false)) => Some(false),
            _ => None,
        }
    }

    /// Returns whether convenient parking (lot or valet) is available.
    ///
    /// Used to identify locations with easy parking access, excluding street parking which may
    /// require walking and searching for available spots.
    #[must_use]
    pub fn has_convenient_parking(self) -> Option<bool> {
        let convenient_options = [
            self.free_parking_lot,
            self.paid_parking_lot,
            self.valet_parking,
            self.free_garage_parking,
            self.paid_garage_parking,
        ];

        if convenient_options.contains(&Some(true)) {
            Some(true)
        } else if convenient_options.iter().all(|&opt| opt == Some(false)) {
            Some(false)
        } else {
            None
        }
    }

    /// Returns whether premium parking services are available.
    ///
    /// Used to identify locations offering high-end parking services like valet or covered garage
    /// parking, indicating upscale venue amenities.
    #[must_use]
    pub const fn has_premium_parking(self) -> Option<bool> {
        match (self.valet_parking, self.free_garage_parking, self.paid_garage_parking) {
            (Some(true), _, _) | (_, Some(true), _) | (_, _, Some(true)) => Some(true),
            (Some(false), Some(false), Some(false)) => Some(false),
            _ => None,
        }
    }

    /// Gets a list of available parking types for display.
    ///
    /// Returns a vector of user-friendly parking option names that can be displayed in user
    /// interfaces, filtered to only confirmed options.
    #[must_use]
    pub fn available_options(self) -> Vec<&'static str> {
        let mut options = Vec::new();

        if self.free_parking_lot == Some(true) {
            options.push("Free Lot");
        }
        if self.paid_parking_lot == Some(true) {
            options.push("Paid Lot");
        }
        if self.free_street_parking == Some(true) {
            options.push("Free Street");
        }
        if self.paid_street_parking == Some(true) {
            options.push("Paid Street");
        }
        if self.valet_parking == Some(true) {
            options.push("Valet");
        }
        if self.free_garage_parking == Some(true) {
            options.push("Free Garage");
        }
        if self.paid_garage_parking == Some(true) {
            options.push("Paid Garage");
        }

        options
    }

    /// Gets a user-friendly description of parking options.
    ///
    /// Provides a human-readable summary of parking availability suitable for displaying in search
    /// results, business listings, or detailed information views.
    #[must_use]
    pub fn description(self) -> String {
        let options = self.available_options();

        match options.len() {
            1 => format!("{} parking available", options[0].to_lowercase()),
            2 => format!("{} and {} parking available", 
                options[0].to_lowercase(), options[1].to_lowercase()),
            _ => options.last().map_or_else(
                || "Parking information unavailable".to_string(),
                |last| {
                    let others = options[..options.len()-1].join(", ").to_lowercase();
                    format!("{}, and {} parking available", others, last.to_lowercase())
                })
        }
    }

    /// Gets a short parking summary for compact displays.
    ///
    /// Provides brief parking information suitable for mobile interfaces, map overlays, or
    /// space-constrained displays with essential information.
    #[must_use]
    pub fn short_description(self) -> String {
        if self.has_free_parking() == Some(true) {
            if self.has_paid_parking() == Some(true) {
                "Free & Paid Parking".to_string()
            } else {
                "Free Parking".to_string()
            }
        } else if self.has_paid_parking() == Some(true) {
            "Paid Parking".to_string()
        } else if self.has_parking_info() {
            "Limited Parking".to_string()
        } else {
            "Parking Unknown".to_string()
        }
    }

    /// Returns parking amenity icons for visual display.
    ///
    /// Provides commonly recognized icons/symbols for parking amenities, useful for visual
    /// interfaces, maps, and quick recognition of parking features.
    #[must_use]
    pub fn parking_icons(self) -> Vec<&'static str> {
        let mut icons = Vec::new();

        if self.has_free_parking() == Some(true) {
            icons.push("🅿️"); // Free parking indicator
        }
        if self.has_covered_parking() == Some(true) {
            icons.push("🏢"); // Covered parking
        }
        if self.valet_parking == Some(true) {
            icons.push("🛎️"); // Valet service
        }
        if self.has_paid_parking() == Some(true) && self.has_free_parking() != Some(true) {
            icons.push("💰"); // Paid parking only
        }

        icons
    }

    /// Returns whether parking is suitable for events or large gatherings.
    ///
    /// Used to identify locations with adequate parking capacity for events, considering convenient
    /// access and multiple parking options.
    #[must_use]
    pub fn is_event_suitable(self) -> Option<bool> {
        // Events benefit from lots, garages, or valet service
        let event_friendly = [
            self.free_parking_lot,
            self.paid_parking_lot,
            self.free_garage_parking,
            self.paid_garage_parking,
            self.valet_parking,
        ];

        if (event_friendly.iter().filter(|&&opt| opt == Some(true)).count() >= 2)
            | event_friendly.contains(&Some(true)) {
                Some(true)
        } else if event_friendly.iter().all(|&opt| opt == Some(false)) {
            Some(false) // No convenient options
        } else {
            None // Insufficient information
        }
    }

    /// Returns whether parking options are budget-friendly.
    ///
    /// Used to identify locations where parking costs won't significantly impact the total cost of
    /// visiting, important for price-sensitive customers.
    #[must_use]
    pub fn is_budget_friendly(self) -> Option<bool> {
        match (self.has_free_parking(), self.has_paid_parking()) {
            (Some(true), _) => Some(true), // Any free parking is budget-friendly
            (Some(false), Some(true | false)) => Some(false), // Only paid parking
            // No parking at all
            _ => None, // Insufficient information
        }
    }

    /// Returns the parking convenience score (0-5 scale).
    ///
    /// Provides a numerical assessment of parking convenience, with higher scores indicating easier
    /// and more convenient parking access.
    #[must_use]
    pub fn convenience_score(self) -> Option<u8> {
        if !self.has_parking_info() {
            return None;
        }

        let mut score = 0u8;

        // Valet parking gets highest convenience points
        if self.valet_parking == Some(true) {
            score += 2;
        }

        // Lot parking is very convenient
        if self.free_parking_lot == Some(true) || self.paid_parking_lot == Some(true) {
            score += 2;
        }

        // Garage parking is convenient and covered
        if self.free_garage_parking == Some(true) || self.paid_garage_parking == Some(true) {
            score += 1;
        }

        // Street parking is less convenient
        if self.free_street_parking == Some(true) || self.paid_street_parking == Some(true) {
            score += 1;
        }

        Some(score.min(5)) // Cap at 5
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
        let parking = ParkingOptions {
            free_parking_lot: Some(true),
            paid_parking_lot: Some(false),
            free_street_parking: Some(true),
            paid_street_parking: Some(true),
            valet_parking: Some(false),
            free_garage_parking: None,
            paid_garage_parking: Some(true),
        };

        let json = serde_json::to_string(&parking).unwrap();
        assert!(json.contains("freeParkingLot"));
        assert!(json.contains("paidParkingLot"));
        assert!(json.contains("valetParking"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "freeParkingLot": true,
            "paidParkingLot": false,
            "freeStreetParking": null,
            "paidStreetParking": true,
            "valetParking": false,
            "freeGarageParking": true,
            "paidGarageParking": false
        }"#;

        let parking: ParkingOptions = serde_json::from_str(json).unwrap();
        assert_eq!(parking.free_parking_lot, Some(true));
        assert_eq!(parking.paid_parking_lot, Some(false));
        assert_eq!(parking.free_street_parking, None);
        assert_eq!(parking.valet_parking, Some(false));
    }

    #[test]
    fn test_constructors() {
        let new = ParkingOptions::new();
        assert!(!new.has_parking_info());

        let free_only = ParkingOptions::free_only();
        assert_eq!(free_only.has_free_parking(), Some(true));
        assert_eq!(free_only.has_paid_parking(), Some(false));

        let with_valet = ParkingOptions::with_valet();
        assert_eq!(with_valet.valet_parking, Some(true));
        assert_eq!(with_valet.has_premium_parking(), Some(true));

        let urban = ParkingOptions::urban_limited();
        assert_eq!(urban.has_free_parking(), Some(false));
        assert_eq!(urban.paid_street_parking, Some(true));
    }

    #[test]
    fn test_has_parking_info() {
        let empty = ParkingOptions::new();
        assert!(!empty.has_parking_info());

        let partial = ParkingOptions {
            free_parking_lot: Some(true),
            ..ParkingOptions::new()
        };
        assert!(partial.has_parking_info());
    }

    #[test]
    fn test_has_free_parking() {
        let free_lot = ParkingOptions {
            free_parking_lot: Some(true),
            free_street_parking: Some(false),
            free_garage_parking: Some(false),
            ..ParkingOptions::new()
        };
        assert_eq!(free_lot.has_free_parking(), Some(true));

        let no_free = ParkingOptions {
            free_parking_lot: Some(false),
            free_street_parking: Some(false),
            free_garage_parking: Some(false),
            ..ParkingOptions::new()
        };
        assert_eq!(no_free.has_free_parking(), Some(false));

        let unknown = ParkingOptions::new();
        assert_eq!(unknown.has_free_parking(), None);
    }

    #[test]
    fn test_has_paid_parking() {
        let paid_lot = ParkingOptions {
            paid_parking_lot: Some(true),
            paid_street_parking: Some(false),
            valet_parking: Some(false),
            paid_garage_parking: Some(false),
            ..ParkingOptions::new()
        };
        assert_eq!(paid_lot.has_paid_parking(), Some(true));

        let no_paid = ParkingOptions {
            paid_parking_lot: Some(false),
            paid_street_parking: Some(false),
            valet_parking: Some(false),
            paid_garage_parking: Some(false),
            ..ParkingOptions::new()
        };
        assert_eq!(no_paid.has_paid_parking(), Some(false));
    }

    #[test]
    fn test_has_covered_parking() {
        let covered = ParkingOptions {
            free_garage_parking: Some(true),
            paid_garage_parking: Some(false),
            ..ParkingOptions::new()
        };
        assert_eq!(covered.has_covered_parking(), Some(true));

        let no_covered = ParkingOptions {
            free_garage_parking: Some(false),
            paid_garage_parking: Some(false),
            ..ParkingOptions::new()
        };
        assert_eq!(no_covered.has_covered_parking(), Some(false));
    }

    #[test]
    fn test_has_convenient_parking() {
        let convenient = ParkingOptions {
            free_parking_lot: Some(true),
            ..ParkingOptions::new()
        };
        assert_eq!(convenient.has_convenient_parking(), Some(true));

        let street_only = ParkingOptions {
            free_street_parking: Some(true),
            free_parking_lot: Some(false),
            paid_parking_lot: Some(false),
            valet_parking: Some(false),
            free_garage_parking: Some(false),
            paid_garage_parking: Some(false),
            ..ParkingOptions::new()
        };
        assert_eq!(street_only.has_convenient_parking(), Some(false));
    }

    #[test]
    fn test_has_premium_parking() {
        let valet = ParkingOptions {
            valet_parking: Some(true),
            ..ParkingOptions::new()
        };
        assert_eq!(valet.has_premium_parking(), Some(true));

        let garage = ParkingOptions {
            paid_garage_parking: Some(true),
            valet_parking: Some(false),
            free_garage_parking: Some(false),
            ..ParkingOptions::new()
        };
        assert_eq!(garage.has_premium_parking(), Some(true));

        let basic = ParkingOptions {
            valet_parking: Some(false),
            free_garage_parking: Some(false),
            paid_garage_parking: Some(false),
            ..ParkingOptions::new()
        };
        assert_eq!(basic.has_premium_parking(), Some(false));
    }

    #[test]
    fn test_available_options() {
        let mixed = ParkingOptions {
            free_parking_lot: Some(true),
            valet_parking: Some(true),
            paid_street_parking: Some(true),
            ..ParkingOptions::new()
        };
        let options = mixed.available_options();
        assert_eq!(options.len(), 3);
        assert!(options.contains(&"Free Lot"));
        assert!(options.contains(&"Valet"));
        assert!(options.contains(&"Paid Street"));

        let none = ParkingOptions::new();
        assert!(none.available_options().is_empty());
    }

    #[test]
    fn test_description() {
        let single = ParkingOptions {
            free_parking_lot: Some(true),
            ..ParkingOptions::new()
        };
        assert_eq!(single.description(), "free lot parking available");

        let double = ParkingOptions {
            free_parking_lot: Some(true),
            valet_parking: Some(true),
            ..ParkingOptions::new()
        };
        assert_eq!(double.description(), "free lot and valet parking available");

        let none = ParkingOptions::new();
        assert_eq!(none.description(), "Parking information unavailable");
    }

    #[test]
    fn test_short_description() {
        let free_only = ParkingOptions::free_only();
        assert_eq!(free_only.short_description(), "Free Parking");

        let paid_only = ParkingOptions {
            paid_parking_lot: Some(true),
            free_parking_lot: Some(false),
            free_street_parking: Some(false),
            free_garage_parking: Some(false),
            ..ParkingOptions::new()
        };
        assert_eq!(paid_only.short_description(), "Paid Parking");

        let mixed = ParkingOptions {
            free_parking_lot: Some(true),
            paid_parking_lot: Some(true),
            ..ParkingOptions::new()
        };
        assert_eq!(mixed.short_description(), "Free & Paid Parking");

        let unknown = ParkingOptions::new();
        assert_eq!(unknown.short_description(), "Parking Unknown");
    }

    #[test]
    fn test_parking_icons() {
        let comprehensive = ParkingOptions {
            free_parking_lot: Some(true),
            paid_garage_parking: Some(true),
            valet_parking: Some(true),
            ..ParkingOptions::new()
        };
        let icons = comprehensive.parking_icons();
        assert!(icons.contains(&"🅿️")); // Free parking
        assert!(icons.contains(&"🏢")); // Covered parking
        assert!(icons.contains(&"🛎️")); // Valet

        let paid_only = ParkingOptions {
            paid_parking_lot: Some(true),
            free_parking_lot: Some(false),
            free_street_parking: Some(false),
            free_garage_parking: Some(false),
            ..ParkingOptions::new()
        };
        let icons = paid_only.parking_icons();
        assert!(icons.contains(&"💰")); // Paid parking only
    }

    #[test]
    fn test_is_event_suitable() {
        let multiple_options = ParkingOptions {
            free_parking_lot: Some(true),
            paid_garage_parking: Some(true),
            ..ParkingOptions::new()
        };
        assert_eq!(multiple_options.is_event_suitable(), Some(true));

        let street_only = ParkingOptions {
            free_street_parking: Some(true),
            free_parking_lot: Some(false),
            paid_parking_lot: Some(false),
            free_garage_parking: Some(false),
            paid_garage_parking: Some(false),
            valet_parking: Some(false),
            ..ParkingOptions::new()
        };
        assert_eq!(street_only.is_event_suitable(), Some(false));
    }

    #[test]
    fn test_is_budget_friendly() {
        let free_available = ParkingOptions {
            free_parking_lot: Some(true),
            paid_parking_lot: Some(true),
            ..ParkingOptions::new()
        };
        assert_eq!(free_available.is_budget_friendly(), Some(true));

        let paid_only = ParkingOptions {
            paid_parking_lot: Some(true),
            free_parking_lot: Some(false),
            free_street_parking: Some(false),
            free_garage_parking: Some(false),
            ..ParkingOptions::new()
        };
        assert_eq!(paid_only.is_budget_friendly(), Some(false));
    }

    #[test]
    fn test_convenience_score() {
        let premium = ParkingOptions {
            valet_parking: Some(true),
            free_parking_lot: Some(true),
            paid_garage_parking: Some(true),
            ..ParkingOptions::new()
        };
        let score = premium.convenience_score().unwrap();
        assert!(score >= 4); // High convenience

        let street_only = ParkingOptions {
            free_street_parking: Some(true),
            ..ParkingOptions::new()
        };
        let score = street_only.convenience_score().unwrap();
        assert_eq!(score, 1); // Low convenience

        let no_info = ParkingOptions::new();
        assert_eq!(no_info.convenience_score(), None);
    }

    #[test]
    fn test_default() {
        let default_parking = ParkingOptions::default();
        assert!(!default_parking.has_parking_info());
        assert_eq!(default_parking.convenience_score(), None);
    }
}