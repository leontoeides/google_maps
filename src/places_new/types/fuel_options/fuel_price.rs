use crate::places_new::{FuelType, Money};
use jiff::Timestamp;

// -------------------------------------------------------------------------------------------------
//
/// Fuel price information for a given type of fuel at a gas station.
///
/// Fuel prices provide current pricing information for specific fuel types, helping drivers compare
/// costs across different stations and fuel grades.
///
/// Prices include timestamp information to indicate freshness and reliability of the pricing data.
/// This information is updated regularly to reflect current market conditions.
#[derive(
    //std
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    // getset
    getset::Getters,
    getset::CopyGetters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[serde(rename_all = "camelCase")]
pub struct FuelPrice {
    /// The type of fuel this price represents.
    ///
    /// Specifies the exact fuel grade or type (regular, premium, diesel, etc.) that this pricing
    /// information applies to. Different fuel types at the same station will have separate
    /// `FuelPrice` entries.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub fuel_type: Option<FuelType>,

    /// The price of the fuel.
    ///
    /// Current price per unit (typically per gallon or liter depending on region) for this specific
    /// fuel type. Includes currency information for international compatibility and precise
    /// monetary representation.
    #[getset(get = "pub")]
    pub price: Money,

    /// The time when this fuel price was last updated.
    ///
    /// Timestamp indicating when this price information was last verified or updated. More recent
    /// timestamps indicate more reliable pricing data, which is crucial for fuel price comparisons
    /// and decision-making.
    #[getset(get_copy = "pub")]
    pub update_time: Timestamp,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl FuelPrice {
    /// Creates a new `FuelPrice` with the specified fuel type, price, and update time.
    ///
    /// Used to construct fuel price information with all essential components. The update time
    /// should reflect when the price was last verified to ensure data freshness and reliability for
    /// users.
    #[must_use]
    pub const fn new(fuel_type: Option<FuelType>, price: Money, update_time: Timestamp) -> Self {
        Self {
            fuel_type,
            price,
            update_time,
        }
    }

    /// Creates a `FuelPrice` with current timestamp.
    ///
    /// Convenience constructor that automatically sets the update time to now, useful when
    /// recording fresh price data from gas station APIs or manual input.
    #[must_use]
    pub fn with_current_time(fuel_type: Option<FuelType>, price: Money) -> Self {
        Self {
            fuel_type,
            price,
            update_time: Timestamp::now(),
        }
    }

    /// Returns the age of this price information.
    ///
    /// Calculates how long ago this price was last updated, useful for determining data freshness
    /// and reliability. Older prices may be less accurate due to market fluctuations and should be
    /// used with caution.
    pub fn age(&self) -> Result<jiff::Span, jiff::Error> {
        Timestamp::now().since(self.update_time)
    }

    /// Returns whether this price information is very recent.
    ///
    /// Used to identify the most current and reliable price data, typically updated within the last
    /// few hours for optimal accuracy.
    pub fn is_very_recent(&self) -> Result<bool, jiff::Error> {
        self.age().map(|age| age.total(jiff::Unit::Minute).unwrap_or(0.0) <= 120.0) // 2 hours
    }

    /// Returns whether this price information is considered fresh.
    ///
    /// Used to determine if the price data is recent enough to be reliable for user
    /// decision-making. Fuel prices can change frequently, so freshness is important for accurate
    /// comparisons.
    pub fn is_fresh(&self) -> Result<bool, jiff::Error> {
        self.age().map(|age| age.total(jiff::Unit::Minute).unwrap_or(0.0) <= 1_440.0) // 1 day
    }

    /// Returns whether this price information is stale.
    ///
    /// Used to identify outdated price data that may not reflect current market conditions. Stale
    /// prices should be updated or marked as potentially inaccurate in user interfaces.
    pub fn is_stale(&self) -> Result<bool, jiff::Error> {
        self.age().map(|age| age.total(jiff::Unit::Minute).unwrap_or(0.0) > 10_080.0) // 1 week
    }

    /// Gets a user-friendly description of when this price was last updated.
    ///
    /// Provides relative time descriptions like "2 hours ago" or "yesterday" that help users
    /// understand the freshness of price information.
    pub fn update_time_description(&self) -> Result<String, jiff::Error> {
        let age = self.age()?;
        
        let total_seconds = age.total(jiff::Unit::Second).unwrap_or(0.0);
        let total_minutes = age.total(jiff::Unit::Minute).unwrap_or(0.0);
        let total_hours = age.total(jiff::Unit::Hour).unwrap_or(0.0);
        let total_days = age.total(jiff::Unit::Day).unwrap_or(0.0);

        Ok(if total_seconds < 60.0 {
            "Just updated".to_string()
        } else if total_minutes < 60.0 {
            format!(
                "{} minute{} ago",
                total_minutes, if (total_minutes - 1.0).abs() < 0.1 { "" } else { "s" }
            )
        } else if total_hours < 24.0 {
            format!(
                "{} hour{} ago",
                total_hours, if (total_hours - 1.0).abs() < 0.1 { "" } else { "s" }
            )
        } else if total_days < 7.0 {
            format!(
                "{} day{} ago",
                total_days, if (total_days - 1.0).abs() < 0.1 { "" } else { "s" }
            )
        } else {
            format!(
                "{} week{} ago",
                total_days / 7.0, if (total_days / 7.0 - 1.0).abs() < 0.1 { "" } else { "s" }
            )
        })
    }

    /// Returns a freshness indicator for display purposes.
    ///
    /// Provides a simple classification of price data freshness that can be used for color coding,
    /// badges, or user interface indicators.
    pub fn freshness_indicator(&self) -> Result<FreshnessIndicator, jiff::Error> {
        if self.is_very_recent()? {
            Ok(FreshnessIndicator::VeryFresh)
        } else if self.is_fresh()? {
            Ok(FreshnessIndicator::Fresh)
        } else if self.is_stale()? {
            Ok(FreshnessIndicator::Stale)
        } else {
            Ok(FreshnessIndicator::Outdated)
        }
    }

    /// Gets the price per unit with currency formatting.
    ///
    /// Returns a formatted price string suitable for display in gas station listings, price
    /// comparison interfaces, and mobile applications.
    #[must_use]
    pub fn formatted_price(&self) -> String {
        self.price.format()
    }

    /// Gets a compact price representation for mobile displays.
    ///
    /// Provides space-efficient price formatting suitable for mobile interfaces, map overlays, or
    /// anywhere screen real estate is limited.
    #[must_use]
    pub fn compact_price(&self) -> String {
        self.price.compact_format()
    }

    /// Returns whether this fuel price represents a competitive rate.
    ///
    /// Used to identify notably good prices for highlighting in user interfaces. This is a
    /// placeholder implementation that could be enhanced with market data for more accurate
    /// competitive analysis.
    #[must_use]
    pub fn is_competitive(&self, average_price: Option<&Money>) -> bool {
        if let Some(avg) = average_price {
            return self.price.to_amount() <= avg.to_amount() * 0.95; // 5% below average
        }
        false // Can't determine without comparison data
    }

    /// Gets a display-friendly fuel type name.
    ///
    /// Returns the human-readable name of the fuel type for user interfaces. If the fuel type is
    /// unknown, returns a generic description.
    #[must_use]
    pub fn fuel_type_name(&self) -> String {
        self.fuel_type
            .map_or_else(
                || "Unknown Fuel".to_string(),
                |ft| ft.display_name().to_string()
            )
    }

    /// Creates a full description combining fuel type and price.
    ///
    /// Provides complete information suitable for detailed listings, tooltips, or comprehensive
    /// price displays that include both fuel type and cost.
    #[must_use]
    pub fn full_description(&self) -> String {
        format!("{}: {}", self.fuel_type_name(), self.formatted_price())
    }

    /// Returns whether this price has valid monetary information.
    ///
    /// Used to validate price data before displaying or using it for comparisons, ensuring the
    /// price contains meaningful and properly formatted monetary values.
    #[must_use]
    pub fn has_valid_price(&self) -> bool {
        self.price.is_valid() && self.price.is_positive()
    }

    /// Compares this price with another for the same fuel type.
    ///
    /// Provides price comparison functionality for sorting and filtering operations. Only
    /// meaningful when comparing prices for the same fuel type and currency.
    #[must_use]
    pub fn compare_price(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.fuel_type == other.fuel_type {
            Some(self.price.compare_amount(&other.price))
        } else {
            None // Different fuel types can't be meaningfully compared
        }
    }

    /// Returns an icon representation for the fuel type.
    ///
    /// Provides visual indicators suitable for maps, mobile interfaces, and quick identification of
    /// fuel types in user interfaces. Returns generic fuel icon if type is unknown.
    #[must_use]
    pub fn fuel_icon(&self) -> &'static str {
        self.fuel_type.map_or("⛽", |ft| match ft {
            FuelType::Diesel | FuelType::DieselPlus | FuelType::BioDiesel | FuelType::TruckDiesel => "🚛",
            FuelType::E85 | FuelType::E80 | FuelType::E100 => "🌱",
            FuelType::Lpg => "🔥",
            FuelType::Methane => "💨",
            _ => "⛽",
        })
    }

    /// Gets the update time in a specific timezone.
    ///
    /// Converts the update timestamp to a specified timezone for display in local time contexts or
    /// international applications.
    #[must_use]
    pub fn update_time_in_timezone(&self, tz: &jiff::tz::TimeZone) -> jiff::Zoned {
        self.update_time.to_zoned(tz.clone())
    }
}

// -------------------------------------------------------------------------------------------------
//
/// Indicates the freshness level of fuel price data.
///
/// Used for color coding, user interface indicators, and filtering to help users understand the
/// reliability and currency of price information.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum FreshnessIndicator {
    /// Price updated within the last 2 hours - highest reliability.
    VeryFresh,
    /// Price updated within the last 24 hours - good reliability.
    Fresh,
    /// Price is older than 24 hours but less than 7 days - use with caution.
    Outdated,
    /// Price is older than 7 days - may be significantly outdated.
    Stale,
}

impl FreshnessIndicator {
    /// Returns a user-friendly description of the freshness level.
    ///
    /// Provides text suitable for tooltips, status indicators, or detailed information displays
    /// about price data reliability.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::VeryFresh => "Very recent price",
            Self::Fresh => "Recent price",
            Self::Outdated => "Price may be outdated",
            Self::Stale => "Price likely outdated",
        }
    }

    /// Returns a color indicator for UI styling.
    ///
    /// Provides color codes suitable for CSS styling, status badges, or visual indicators in user
    /// interfaces.
    #[must_use]
    pub const fn color(self) -> &'static str {
        match self {
            Self::VeryFresh => "green",
            Self::Fresh => "blue",
            Self::Outdated => "orange",
            Self::Stale => "red",
        }
    }

    /// Returns an emoji indicator for compact displays.
    ///
    /// Provides visual freshness indicators suitable for mobile interfaces or anywhere compact
    /// visual representation is needed.
    #[must_use]
    pub const fn emoji(self) -> &'static str {
        match self {
            Self::VeryFresh => "🟢",
            Self::Fresh => "🔵", 
            Self::Outdated => "🟡",
            Self::Stale => "🔴",
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::places_new::FuelType;

    #[test]
    fn test_serialization() {
        let price = FuelPrice {
            fuel_type: Some(FuelType::RegularUnleaded),
            price: Money::from_amount("USD", 3.45).unwrap(),
            update_time: Timestamp::now(),
        };

        let json = serde_json::to_string(&price).unwrap();
        assert!(json.contains("REGULAR_UNLEADED"));
        assert!(json.contains("USD"));
    }

    #[test]
    fn test_deserialization() {
        let json = format!(r#"{{
            "fuelType": "PREMIUM",
            "price": {{
                "currencyCode": "USD",
                "units": "3",
                "nanos": 650000000
            }},
            "updateTime": "{}"
        }}"#, Timestamp::now());

        let price: FuelPrice = serde_json::from_str(&json).unwrap();
        assert_eq!(price.fuel_type, Some(FuelType::Premium));
        assert_eq!(price.price.currency_code(), "USD");
    }

    #[test]
    fn test_constructors() {
        let now = Timestamp::now();
        let price = Money::from_amount("USD", 3.25).unwrap();

        let fuel_price = FuelPrice::new(Some(FuelType::RegularUnleaded), price.clone(), now);
        assert_eq!(fuel_price.fuel_type, Some(FuelType::RegularUnleaded));
        assert_eq!(fuel_price.update_time, now);

        let current_price = FuelPrice::with_current_time(Some(FuelType::Premium), price);
        assert_eq!(current_price.fuel_type, Some(FuelType::Premium));
        // Current time should be very recent
        assert!(current_price.age().unwrap().get_seconds() < 5);
    }

    #[test]
    fn test_freshness() {
        let now = Timestamp::now();
        let recent = now - jiff::Span::new().hours(1);
        let old = now - jiff::Span::new().hours(240);

        let fresh_price = FuelPrice::new(
            Some(FuelType::RegularUnleaded),
            Money::from_amount("USD", 3.25).unwrap(),
            recent
        );
        assert!(fresh_price.is_fresh().unwrap());
        assert!(fresh_price.is_very_recent().unwrap());
        assert!(!fresh_price.is_stale().unwrap());

        let stale_price = FuelPrice::new(
            Some(FuelType::RegularUnleaded),
            Money::from_amount("USD", 3.25).unwrap(),
            old
        );
        assert!(!stale_price.is_fresh().unwrap());
        assert!(!stale_price.is_very_recent().unwrap());
        assert!(stale_price.is_stale().unwrap());
    }

    #[test]
    fn test_freshness_indicator() {
        let now = Timestamp::now();

        let very_fresh = FuelPrice::new(
            Some(FuelType::RegularUnleaded),
            Money::from_amount("USD", 3.25).unwrap(),
            now - jiff::Span::new().minutes(30)
        );
        assert_eq!(very_fresh.freshness_indicator().unwrap(), FreshnessIndicator::VeryFresh);

        let fresh = FuelPrice::new(
            Some(FuelType::RegularUnleaded),
            Money::from_amount("USD", 3.25).unwrap(),
            now - jiff::Span::new().minutes(720) // 12 hours
        );
        assert_eq!(fresh.freshness_indicator().unwrap(), FreshnessIndicator::Fresh);

        let stale = FuelPrice::new(
            Some(FuelType::RegularUnleaded),
            Money::from_amount("USD", 3.25).unwrap(),
            now - jiff::Span::new().hours(14_400) // 10 days
        );
        assert_eq!(stale.freshness_indicator().unwrap(), FreshnessIndicator::Stale);
    }

    #[test]
    fn test_update_time_description() {
        let now = Timestamp::now();

        let recent = FuelPrice::new(
            Some(FuelType::RegularUnleaded),
            Money::from_amount("USD", 3.25).unwrap(),
            now - jiff::Span::new().minutes(30)
        );
        let desc = recent.update_time_description();
        assert!(desc.unwrap().contains("minute"));

        let hours_ago = FuelPrice::new(
            Some(FuelType::RegularUnleaded),
            Money::from_amount("USD", 3.25).unwrap(),
            now - jiff::Span::new().hours(5)
        );
        let desc = hours_ago.update_time_description();
        assert!(desc.unwrap().contains("hour"));
    }

    #[test]
    fn test_price_formatting() {
        let price = FuelPrice::new(
            Some(FuelType::Premium),
            Money::from_amount("USD", 3.75).unwrap(),
            Timestamp::now()
        );

        assert_eq!(price.formatted_price(), "$3.75");
        assert_eq!(price.fuel_type_name(), "Premium");
        assert_eq!(price.full_description(), "Premium: $3.75");

        let unknown_fuel = FuelPrice::new(
            None,
            Money::from_amount("USD", 3.75).unwrap(),
            Timestamp::now()
        );
        assert_eq!(unknown_fuel.fuel_type_name(), "Unknown Fuel");
    }

    #[test]
    fn test_validation() {
        let valid_price = FuelPrice::new(
            Some(FuelType::RegularUnleaded),
            Money::from_amount("USD", 3.25).unwrap(),
            Timestamp::now()
        );
        assert!(valid_price.has_valid_price());

        let zero_price = FuelPrice::new(
            Some(FuelType::RegularUnleaded),
            Money::zero("USD").unwrap(),
            Timestamp::now()
        );
        assert!(!zero_price.has_valid_price());
    }

    #[test]
    fn test_comparison() {
        let price1 = FuelPrice::new(
            Some(FuelType::RegularUnleaded),
            Money::from_amount("USD", 3.25).unwrap(),
            Timestamp::now()
        );
        let price2 = FuelPrice::new(
            Some(FuelType::RegularUnleaded),
            Money::from_amount("USD", 3.50).unwrap(),
            Timestamp::now()
        );
        let different_fuel = FuelPrice::new(
            Some(FuelType::Premium),
            Money::from_amount("USD", 3.25).unwrap(),
            Timestamp::now()
        );

        let unknown_fuel = FuelPrice::new(
            None,
            Money::from_amount("USD", 3.25).unwrap(),
            Timestamp::now()
        );

        assert_eq!(price1.compare_price(&price2), Some(std::cmp::Ordering::Less));
        assert_eq!(price1.compare_price(&different_fuel), None);
        assert_eq!(price1.compare_price(&unknown_fuel), None);
    }

    #[test]
    fn test_competitive_pricing() {
        let price = FuelPrice::new(
            Some(FuelType::RegularUnleaded),
            Money::from_amount("USD", 3.00).unwrap(),
            Timestamp::now()
        );
        let average = Money::from_amount("USD", 3.25).unwrap();

        assert!(price.is_competitive(Some(&average)));

        let expensive = FuelPrice::new(
            Some(FuelType::RegularUnleaded),
            Money::from_amount("USD", 3.50).unwrap(),
            Timestamp::now()
        );
        assert!(!expensive.is_competitive(Some(&average)));
    }

    #[test]
    fn test_fuel_icons() {
        let gasoline = FuelPrice::new(
            Some(FuelType::RegularUnleaded),
            Money::from_amount("USD", 3.25).unwrap(),
            Timestamp::now()
        );
        assert_eq!(gasoline.fuel_icon(), "⛽");

        let diesel = FuelPrice::new(
            Some(FuelType::Diesel),
            Money::from_amount("USD", 3.45).unwrap(),
            Timestamp::now()
        );
        assert_eq!(diesel.fuel_icon(), "🚛");

        let ethanol = FuelPrice::new(
            Some(FuelType::E85),
            Money::from_amount("USD", 2.95).unwrap(),
            Timestamp::now()
        );
        assert_eq!(ethanol.fuel_icon(), "🌱");

        let unknown = FuelPrice::new(
            None,
            Money::from_amount("USD", 3.25).unwrap(),
            Timestamp::now()
        );
        assert_eq!(unknown.fuel_icon(), "⛽");
    }

    #[test]
    fn test_freshness_indicator_methods() {
        assert_eq!(FreshnessIndicator::VeryFresh.description(), "Very recent price");
        assert_eq!(FreshnessIndicator::Fresh.color(), "blue");
        assert_eq!(FreshnessIndicator::Stale.emoji(), "🔴");
    }
}