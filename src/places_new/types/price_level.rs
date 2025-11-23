use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

// -------------------------------------------------------------------------------------------------
//
/// Indicates the relative price level of a place on a scale from free to very expensive.
///
/// Price level provides a standardized way to categorize the cost of goods or services at a
/// business.
///
/// This is typically represented as a scale (often shown as dollar signs in UIs) and helps users
/// filter places based on their budget preferences.
#[derive(
    // std
    Copy,
    Clone,
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
pub enum PriceLevel {
    /// Price level is unspecified or unknown.
    ///
    /// This value is used when price information is not available or cannot be determined.
    /// Applications should handle this as missing price data rather than assuming any particular
    /// price range.
    #[default]
    #[serde(rename = "PRICE_LEVEL_UNSPECIFIED")]
    Unspecified = 0,

    /// Place provides free services or goods.
    ///
    /// This indicates the business offers services or goods at no cost to customers. Common for
    /// public facilities, free museums, parks, or businesses with free basic services.
    #[serde(rename = "PRICE_LEVEL_FREE")]
    Free = 1,

    /// Place provides inexpensive services or goods.
    ///
    /// This represents the most affordable paid tier, typically budget-friendly options. Common for
    /// fast food, casual dining, budget accommodations, or discount retailers. Often displayed as a
    /// single `$` symbol in user interfaces.
    #[serde(rename = "PRICE_LEVEL_INEXPENSIVE")]
    Inexpensive = 2,

    /// Place provides moderately priced services or goods.
    ///
    /// This represents mid-range pricing that's neither budget nor luxury. Common for casual dining
    /// restaurants, mid-tier hotels, or standard retail stores. Often displayed as `$$` symbols in
    /// user interfaces.
    #[serde(rename = "PRICE_LEVEL_MODERATE")]
    Moderate = 3,

    /// Place provides expensive services or goods.
    ///
    /// This represents higher-end pricing for premium services or goods. Common for upscale
    /// restaurants, luxury hotels, or high-end retail stores. Often displayed as `$$$` symbols in
    /// user interfaces.
    #[serde(rename = "PRICE_LEVEL_EXPENSIVE")]
    Expensive = 4,

    /// Place provides very expensive services or goods.
    ///
    /// This represents the highest price tier for luxury or exclusive services. Common for fine
    /// dining establishments, luxury resorts, or premium boutiques. Often displayed as `$$$$`
    /// symbols in user interfaces.
    #[serde(rename = "PRICE_LEVEL_VERY_EXPENSIVE")]
    VeryExpensive = 5,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl PriceLevel {
    /// Returns the number of dollar signs typically used to represent this price level.
    ///
    /// Used for creating visual representations of price levels in user interfaces, where dollar
    /// signs provide an intuitive indication of relative cost.
    #[must_use]
    pub const fn dollar_sign_count(self) -> Option<u8> {
        match self {
            Self::Unspecified => None,
            Self::Free => Some(0),
            Self::Inexpensive => Some(1),
            Self::Moderate => Some(2),
            Self::Expensive => Some(3),
            Self::VeryExpensive => Some(4),
        }
    }

    /// Returns a dollar sign representation of the price level.
    ///
    /// Provides the standard visual representation using dollar signs that users expect to see in
    /// restaurant reviews, travel sites, and business listings.
    #[must_use]
    pub fn dollar_signs(self) -> Option<String> {
        self.dollar_sign_count().map(|count| "$".repeat(count as usize))
    }

    /// Returns whether this represents a paid service level.
    ///
    /// Used to distinguish between free and paid services, which can be useful for filtering or
    /// categorizing businesses based on cost structure.
    #[must_use]
    pub const fn is_paid(self) -> Option<bool> {
        match self {
            Self::Free => Some(false),
            Self::Inexpensive
            | Self::Moderate
            | Self::Expensive
            | Self::VeryExpensive => Some(true),
            Self::Unspecified => None,
        }
    }

    /// Returns whether this represents a budget-friendly option.
    ///
    /// Used to identify places that are affordable for budget-conscious users, including both free
    /// and inexpensive options.
    #[must_use]
    pub const fn is_budget_friendly(self) -> bool {
        matches!(self, Self::Free | Self::Inexpensive)
    }

    /// Returns whether this represents a luxury or high-end option.
    ///
    /// Used to identify premium establishments that cater to users seeking upscale experiences
    /// regardless of cost.
    #[must_use]
    pub const fn is_luxury(self) -> bool {
        matches!(self, Self::Expensive | Self::VeryExpensive)
    }

    /// Returns a human-readable description of the price level.
    ///
    /// Provides clear text descriptions that can be used in accessibility features, detailed
    /// listings, or when dollar signs are not appropriate for the context.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Unspecified => "Price level unknown",
            Self::Free => "Free",
            Self::Inexpensive => "Inexpensive",
            Self::Moderate => "Moderate",
            Self::Expensive => "Expensive",
            Self::VeryExpensive => "Very expensive",
        }
    }

    /// Returns a numeric value for the price level (0-4).
    ///
    /// Used for mathematical operations, sorting, or integration with systems that work with
    /// numeric price scales rather than enum variants.
    #[must_use]
    pub const fn numeric_value(self) -> Option<u8> {
        match self {
            Self::Unspecified => None,
            Self::Free => Some(0),
            Self::Inexpensive => Some(1),
            Self::Moderate => Some(2),
            Self::Expensive => Some(3),
            Self::VeryExpensive => Some(4),
        }
    }

    /// Creates a `PriceLevel` from a numeric value (0-4).
    ///
    /// Used for converting from numeric representations back to the enum, useful when working with
    /// APIs or databases that store price as numbers.
    #[must_use]
    pub const fn from_numeric_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Free),
            1 => Some(Self::Inexpensive),
            2 => Some(Self::Moderate),
            3 => Some(Self::Expensive),
            4 => Some(Self::VeryExpensive),
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
        let price = PriceLevel::Moderate;
        let json = serde_json::to_string(&price).unwrap();
        assert_eq!(json, r#""PRICE_LEVEL_MODERATE""#);

        let price = PriceLevel::VeryExpensive;
        let json = serde_json::to_string(&price).unwrap();
        assert_eq!(json, r#""PRICE_LEVEL_VERY_EXPENSIVE""#);
    }

    #[test]
    fn test_deserialization() {
        let json = r#""PRICE_LEVEL_FREE""#;
        let price: PriceLevel = serde_json::from_str(json).unwrap();
        assert_eq!(price, PriceLevel::Free);

        let json = r#""PRICE_LEVEL_UNSPECIFIED""#;
        let price: PriceLevel = serde_json::from_str(json).unwrap();
        assert_eq!(price, PriceLevel::Unspecified);
    }

    #[test]
    fn test_default() {
        let default_price = PriceLevel::default();
        assert_eq!(default_price, PriceLevel::Unspecified);
    }

    #[test]
    fn test_ordering() {
        assert!(PriceLevel::Free < PriceLevel::Inexpensive);
        assert!(PriceLevel::Moderate < PriceLevel::Expensive);
        assert!(PriceLevel::Expensive < PriceLevel::VeryExpensive);
    }

    #[test]
    fn test_dollar_sign_count() {
        assert_eq!(PriceLevel::Unspecified.dollar_sign_count(), None);
        assert_eq!(PriceLevel::Free.dollar_sign_count(), Some(0));
        assert_eq!(PriceLevel::Inexpensive.dollar_sign_count(), Some(1));
        assert_eq!(PriceLevel::Moderate.dollar_sign_count(), Some(2));
        assert_eq!(PriceLevel::Expensive.dollar_sign_count(), Some(3));
        assert_eq!(PriceLevel::VeryExpensive.dollar_sign_count(), Some(4));
    }

    #[test]
    fn test_dollar_signs() {
        assert_eq!(PriceLevel::Unspecified.dollar_signs(), None);
        assert_eq!(PriceLevel::Free.dollar_signs(), Some(String::new()));
        assert_eq!(PriceLevel::Inexpensive.dollar_signs(), Some("$".to_string()));
        assert_eq!(PriceLevel::Moderate.dollar_signs(), Some("$$".to_string()));
        assert_eq!(PriceLevel::Expensive.dollar_signs(), Some("$$$".to_string()));
        assert_eq!(PriceLevel::VeryExpensive.dollar_signs(), Some("$$$$".to_string()));
    }

    #[test]
    fn test_is_paid() {
        assert_eq!(PriceLevel::Unspecified.is_paid(), None);
        assert_eq!(PriceLevel::Free.is_paid(), Some(false));
        assert_eq!(PriceLevel::Inexpensive.is_paid(), Some(true));
        assert_eq!(PriceLevel::Moderate.is_paid(), Some(true));
    }

    #[test]
    fn test_budget_friendly() {
        assert!(PriceLevel::Free.is_budget_friendly());
        assert!(PriceLevel::Inexpensive.is_budget_friendly());
        assert!(!PriceLevel::Moderate.is_budget_friendly());
        assert!(!PriceLevel::Expensive.is_budget_friendly());
    }

    #[test]
    fn test_luxury() {
        assert!(!PriceLevel::Free.is_luxury());
        assert!(!PriceLevel::Inexpensive.is_luxury());
        assert!(!PriceLevel::Moderate.is_luxury());
        assert!(PriceLevel::Expensive.is_luxury());
        assert!(PriceLevel::VeryExpensive.is_luxury());
    }

    #[test]
    fn test_numeric_conversion() {
        assert_eq!(PriceLevel::Free.numeric_value(), Some(0));
        assert_eq!(PriceLevel::Inexpensive.numeric_value(), Some(1));
        assert_eq!(PriceLevel::Moderate.numeric_value(), Some(2));
        assert_eq!(PriceLevel::Expensive.numeric_value(), Some(3));
        assert_eq!(PriceLevel::VeryExpensive.numeric_value(), Some(4));
        assert_eq!(PriceLevel::Unspecified.numeric_value(), None);

        assert_eq!(PriceLevel::from_numeric_value(0), Some(PriceLevel::Free));
        assert_eq!(PriceLevel::from_numeric_value(2), Some(PriceLevel::Moderate));
        assert_eq!(PriceLevel::from_numeric_value(4), Some(PriceLevel::VeryExpensive));
        assert_eq!(PriceLevel::from_numeric_value(5), None);
    }

    #[test]
    fn test_descriptions() {
        assert_eq!(PriceLevel::Free.description(), "Free");
        assert_eq!(PriceLevel::Moderate.description(), "Moderate");
        assert_eq!(PriceLevel::VeryExpensive.description(), "Very expensive");
        assert_eq!(PriceLevel::Unspecified.description(), "Price level unknown");
    }
}