// -------------------------------------------------------------------------------------------------
//
// Structures

pub mod fuel_price;

// -------------------------------------------------------------------------------------------------
//
// Enumerations

pub mod fuel_type;

// -------------------------------------------------------------------------------------------------

use crate::places_new::{FuelPrice, FuelType};

// -------------------------------------------------------------------------------------------------
//
/// The most recent information about fuel options in a gas station.
///
/// Fuel options provide comprehensive pricing and availability information for different fuel types
/// at a gas station.
///
/// This information is updated regularly to reflect current market conditions and help drivers
/// compare costs across stations and fuel grades. Each fuel type has its own pricing entry with
/// timestamp information for reliability.
#[derive(
    //std
    Clone,
    Debug,
    Default,
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
pub struct FuelOptions {
    /// The last known fuel price for each type of fuel this station has.
    ///
    /// Contains one entry per fuel type available at this station, with current pricing and update
    /// timestamp information. The order is not guaranteed to be significant, so applications should
    /// sort or filter based on fuel type or price as needed.
    #[serde(default)]
    #[getset(get = "pub")]
    pub fuel_prices: Vec<FuelPrice>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl FuelOptions {
    /// Creates a new `FuelOptions` with the specified fuel prices.
    ///
    /// Used to construct fuel options with a collection of fuel prices. Each fuel type should
    /// appear only once in the collection for clarity.
    #[must_use]
    pub const fn new(fuel_prices: Vec<FuelPrice>) -> Self {
        Self { fuel_prices }
    }

    /// Creates an empty `FuelOptions`.
    ///
    /// Used when initializing fuel options that will be populated later or when a gas station has
    /// no available fuel price information.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            fuel_prices: Vec::new(),
        }
    }

    /// Returns whether this station has any fuel price information.
    ///
    /// Used to determine if the station has provided any fuel pricing data, helping decide whether
    /// to display fuel information to users.
    #[must_use]
    pub fn has_fuel_info(&self) -> bool {
        !self.fuel_prices.is_empty()
    }

    /// Returns the number of different fuel types available.
    ///
    /// Used to determine the variety of fuel options at this station, useful for filtering or
    /// categorizing stations by fuel availability.
    #[must_use]
    pub fn fuel_type_count(&self) -> usize {
        self.fuel_prices.len()
    }

    /// Gets the price for a specific fuel type.
    ///
    /// Returns the fuel price information for a specific fuel type if available at this station.
    /// Used for price comparisons and fuel-specific queries.
    #[must_use]
    pub fn price_for_fuel_type(&self, fuel_type: crate::places_new::FuelType) -> Option<&FuelPrice> {
        self.fuel_prices
            .iter()
            .find(|price| price.fuel_type() == Some(fuel_type))
    }

    /// Gets all fuel prices sorted by price (lowest to highest).
    ///
    /// Returns fuel prices ordered from cheapest to most expensive, useful for price comparison
    /// interfaces and budget-conscious fuel selection.
    #[must_use]
    pub fn prices_by_cost(&self) -> Vec<&FuelPrice> {
        let mut prices: Vec<&FuelPrice> = self.fuel_prices.iter().collect();
        prices.sort_by(|a, b| a.price().compare_amount(b.price()));
        prices
    }

    /// Gets all fuel prices sorted by fuel type.
    ///
    /// Returns fuel prices ordered by fuel type for consistent display and logical grouping in user
    /// interfaces.
    #[must_use]
    pub fn prices_by_fuel_type(&self) -> Vec<&FuelPrice> {
        let mut prices: Vec<&FuelPrice> = self.fuel_prices.iter().collect();
        prices.sort_by_key(|price| price.fuel_type());
        prices
    }

    /// Gets only prices that are considered fresh.
    ///
    /// Returns fuel prices that have been updated recently enough to be reliable for
    /// decision-making, filtering out stale pricing data.
    #[must_use]
    pub fn fresh_prices(&self) -> Vec<&FuelPrice> {
        self.fuel_prices
            .iter()
            .filter(|price| price.is_fresh().unwrap_or(false))
            .collect()
    }

    /// Gets only prices that are considered very recent.
    ///
    /// Returns the most up-to-date fuel prices for users who need the most current and reliable
    /// pricing information available.
    #[must_use]
    pub fn very_recent_prices(&self) -> Vec<&FuelPrice> {
        self.fuel_prices
            .iter()
            .filter(|price| price.is_very_recent().unwrap_or(false))
            .collect()
    }

    /// Returns whether this station has competitive pricing.
    ///
    /// Used to identify stations with notably good prices across their fuel options, useful for
    /// highlighting deals in user interfaces.
    #[must_use]
    pub fn has_competitive_pricing(
        &self,
        market_averages: &std::collections::HashMap<crate::places_new::FuelType, crate::places_new::Money>
    ) -> bool {
        let competitive_count = self
            .fuel_prices
            .iter()
            .filter(|price| price
                .fuel_type()
                .is_some_and(|fuel_type| market_averages
                    .get(&fuel_type)
                    .is_some_and(|avg| price.is_competitive(Some(avg)))))
            .count();

        // Consider competitive if at least half the fuel types are competitively priced
        competitive_count >= self.fuel_prices.len().div_ceil(2)
    }

    /// Gets the cheapest fuel option at this station.
    ///
    /// Returns the lowest-priced fuel available, regardless of fuel type. Useful for
    /// budget-conscious drivers who are flexible about fuel grade.
    #[must_use]
    pub fn cheapest_fuel(&self) -> Option<&FuelPrice> {
        self.fuel_prices
            .iter()
            .min_by(|a, b| a.price().compare_amount(b.price()))
    }

    /// Gets the most expensive fuel option at this station.
    ///
    /// Returns the highest-priced fuel available, typically premium grades or specialty fuels.
    /// Useful for understanding the price range at a station.
    #[must_use]
    pub fn most_expensive_fuel(&self) -> Option<&FuelPrice> {
        self.fuel_prices
            .iter()
            .max_by(|a, b| a.price().compare_amount(b.price()))
    }

    /// Returns whether this station offers alternative fuels.
    ///
    /// Used to identify stations that provide eco-friendly or alternative fuel options beyond
    /// traditional gasoline and diesel.
    #[must_use]
    pub fn has_alternative_fuels(&self) -> bool {
        self.fuel_prices
            .iter()
            .any(|price| price
                .fuel_type()
                .is_some_and(FuelType::is_alternative_fuel))
    }

    /// Returns whether this station offers ethanol blends.
    ///
    /// Used to identify stations with ethanol fuel options for flex-fuel vehicles or drivers
    /// seeking renewable fuel alternatives.
    #[must_use]
    pub fn has_ethanol_blends(&self) -> bool {
        self.fuel_prices
            .iter()
            .any(|price| price
                .fuel_type()
                .is_some_and(FuelType::is_ethanol_blend))
    }

    /// Returns whether this station offers diesel fuels.
    ///
    /// Used to identify stations suitable for diesel vehicles, including trucks, buses, and diesel
    /// passenger cars.
    #[must_use]
    pub fn has_diesel(&self) -> bool {
        self.fuel_prices
            .iter()
            .any(|price| price
                .fuel_type()
                .is_some_and(FuelType::is_diesel))
    }

    /// Gets the price range at this station.
    ///
    /// Returns the difference between the most expensive and cheapest fuels, useful for
    /// understanding the pricing spread and station positioning.
    #[must_use]
    pub fn price_range(&self) -> Option<f64> {
        let cheapest = self.cheapest_fuel()?.price().to_amount();
        let most_expensive = self.most_expensive_fuel()?.price().to_amount();
        Some(most_expensive - cheapest)
    }

    /// Gets a summary of available fuel types.
    ///
    /// Returns a list of fuel type names available at this station, suitable for display in user
    /// interfaces and station descriptions.
    #[must_use]
    pub fn available_fuel_types(&self) -> Vec<String> {
        self.fuel_prices
            .iter()
            .map(FuelPrice::fuel_type_name)
            .collect()
    }

    /// Gets a compact description of fuel options.
    ///
    /// Provides a brief summary suitable for mobile interfaces, map overlays, or space-constrained
    /// displays showing fuel availability.
    #[must_use]
    pub fn compact_description(&self) -> String {
        match self.fuel_type_count() {
            0 => "No fuel info".to_string(),
            1 => "1 fuel type".to_string(),
            count => format!("{count} fuel types"),
        }
    }

    /// Gets a detailed description including price range.
    ///
    /// Provides comprehensive information suitable for detailed station listings, including fuel
    /// variety and pricing information.
    #[must_use]
    pub fn detailed_description(&self) -> String {
        if self.fuel_prices.is_empty() {
            return "No fuel information available".to_string();
        }

        let fuel_count = self.fuel_type_count();
        let description = match fuel_count {
            1 => "1 fuel type".to_string(),
            count => format!("{count} fuel types"),
        };

        if let (Some(cheapest), Some(most_expensive)) = (self.cheapest_fuel(), self.most_expensive_fuel()) {
            if cheapest.fuel_type() == most_expensive.fuel_type() {
                format!("{} ({})", description, cheapest.formatted_price())
            } else {
                format!(
                    "{} ({} - {})",
                    description,
                    cheapest.formatted_price(),
                    most_expensive.formatted_price()
                )
            }
        } else {
            description
        }
    }

    /// Returns whether all fuel prices are fresh.
    ///
    /// Used to determine if all pricing data at this station is recent enough to be reliable for
    /// user decision-making.
    #[must_use]
    pub fn all_prices_fresh(&self) -> bool {
        !self.fuel_prices.is_empty()
            && self
                .fuel_prices
                .iter()
                .all(|price| price.is_fresh().unwrap_or(false))
    }

    /// Returns whether any fuel prices are stale.
    ///
    /// Used to identify stations with outdated pricing that may need updates or should be flagged
    /// as potentially unreliable or inconsistent.
    #[must_use]
    pub fn has_stale_prices(&self) -> bool {
        self.fuel_prices
            .iter()
            .any(|price| price.is_stale().unwrap_or(true))
    }

    /// Gets fuel options filtered by currency.
    ///
    /// Returns only fuel prices in the specified currency, useful for international applications or
    /// currency-specific filtering.
    #[must_use]
    pub fn prices_in_currency(&self, currency_code: &str) -> Vec<&FuelPrice> {
        self.fuel_prices
            .iter()
            .filter(|price| price.price().currency_code() == currency_code)
            .collect()
    }

    /// Returns the average price across all fuels.
    ///
    /// Calculates the mean price of all available fuels, useful for understanding overall station
    /// pricing and regional comparisons.
    #[must_use]
    pub fn average_price(&self) -> Option<f64> {
        if self.fuel_prices.is_empty() {
            return None;
        }

        let sum: f64 = self
            .fuel_prices
            .iter()
            .map(|price| price.price().to_amount())
            .sum();

        let count = self.fuel_prices.len();

        if count > 0 {
            Some(sum / count as f64)
        } else {
            None
        }
    }

    /// Returns whether this station is suitable for commercial vehicles.
    ///
    /// Used to identify stations appropriate for trucks and commercial fleets, typically requiring
    /// diesel availability and competitive pricing.
    #[must_use]
    pub fn is_commercial_vehicle_friendly(&self) -> bool {
        self.has_diesel() && self.fuel_type_count() >= 2
    }
}

// -------------------------------------------------------------------------------------------------
//
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::places_new::{FuelType, Money};
    use jiff::Timestamp;
    use std::collections::HashMap;

    fn create_test_fuel_price(fuel_type: FuelType, amount: f64) -> FuelPrice {
        FuelPrice::new(
            Some(fuel_type),
            Money::from_amount("USD", amount).unwrap(),
            Timestamp::now(),
        )
    }

    #[test]
    fn test_serialization() {
        let fuel_options = FuelOptions::new(vec![
            create_test_fuel_price(FuelType::RegularUnleaded, 3.25),
            create_test_fuel_price(FuelType::Premium, 3.65),
        ]);

        let json = serde_json::to_string(&fuel_options).unwrap();
        assert!(json.contains("REGULAR_UNLEADED"));
        assert!(json.contains("PREMIUM"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "fuelPrices": [
                {
                    "fuelType": "REGULAR_UNLEADED",
                    "price": {
                        "currencyCode": "USD",
                        "units": "3",
                        "nanos": 250000000
                    },
                    "updateTime": "2024-01-01T12:00:00Z"
                }
            ]
        }"#;

        let fuel_options: FuelOptions = serde_json::from_str(json).unwrap();
        assert_eq!(fuel_options.fuel_prices.len(), 1);
        assert_eq!(fuel_options.fuel_prices[0].fuel_type(), Some(FuelType::RegularUnleaded));
    }

    #[test]
    fn test_constructors() {
        let prices = vec![
            create_test_fuel_price(FuelType::RegularUnleaded, 3.25),
            create_test_fuel_price(FuelType::Premium, 3.65),
        ];

        let fuel_options = FuelOptions::new(prices);
        assert_eq!(fuel_options.fuel_type_count(), 2);

        let empty = FuelOptions::empty();
        assert!(!empty.has_fuel_info());
    }

    #[test]
    fn test_fuel_info() {
        let fuel_options = FuelOptions::new(vec![
            create_test_fuel_price(FuelType::RegularUnleaded, 3.25),
            create_test_fuel_price(FuelType::Premium, 3.65),
        ]);

        assert!(fuel_options.has_fuel_info());
        assert_eq!(fuel_options.fuel_type_count(), 2);
    }

    #[test]
    fn test_price_for_fuel_type() {
        let fuel_options = FuelOptions::new(vec![
            create_test_fuel_price(FuelType::RegularUnleaded, 3.25),
            create_test_fuel_price(FuelType::Premium, 3.65),
        ]);

        let regular_price = fuel_options.price_for_fuel_type(FuelType::RegularUnleaded);
        assert!(regular_price.is_some());
        assert_eq!(regular_price.unwrap().fuel_type(), Some(FuelType::RegularUnleaded));

        let diesel_price = fuel_options.price_for_fuel_type(FuelType::Diesel);
        assert!(diesel_price.is_none());
    }

    #[test]
    fn test_price_sorting() {
        let fuel_options = FuelOptions::new(vec![
            create_test_fuel_price(FuelType::Premium, 3.65),      // Most expensive
            create_test_fuel_price(FuelType::RegularUnleaded, 3.25), // Cheapest
            create_test_fuel_price(FuelType::Midgrade, 3.45),    // Middle
        ]);

        let by_cost = fuel_options.prices_by_cost();
        assert_eq!(by_cost[0].fuel_type(), Some(FuelType::RegularUnleaded));
        assert_eq!(by_cost[1].fuel_type(), Some(FuelType::Midgrade));
        assert_eq!(by_cost[2].fuel_type(), Some(FuelType::Premium));

        let by_type = fuel_options.prices_by_fuel_type();
        assert_eq!(by_type.len(), 3);
    }

    #[test]
    fn test_cheapest_and_most_expensive() {
        let fuel_options = FuelOptions::new(vec![
            create_test_fuel_price(FuelType::Premium, 3.65),
            create_test_fuel_price(FuelType::RegularUnleaded, 3.25),
            create_test_fuel_price(FuelType::Midgrade, 3.45),
        ]);

        let cheapest = fuel_options.cheapest_fuel().unwrap();
        assert_eq!(cheapest.fuel_type(), Some(FuelType::RegularUnleaded));

        let most_expensive = fuel_options.most_expensive_fuel().unwrap();
        assert_eq!(most_expensive.fuel_type(), Some(FuelType::Premium));

        let price_range = fuel_options.price_range().unwrap();
        assert!((price_range - 0.40).abs() < 0.01); // 3.65 - 3.25 = 0.40
    }

    #[test]
    fn test_fuel_type_detection() {
        let fuel_options = FuelOptions::new(vec![
            create_test_fuel_price(FuelType::RegularUnleaded, 3.25),
            create_test_fuel_price(FuelType::E85, 2.95),
            create_test_fuel_price(FuelType::Diesel, 3.45),
        ]);

        assert!(fuel_options.has_alternative_fuels());
        assert!(fuel_options.has_ethanol_blends());
        assert!(fuel_options.has_diesel());
        assert!(fuel_options.is_commercial_vehicle_friendly());
    }

    #[test]
    fn test_competitive_pricing() {
        let fuel_options = FuelOptions::new(vec![
            create_test_fuel_price(FuelType::RegularUnleaded, 3.00), // Below average
            create_test_fuel_price(FuelType::Premium, 3.40),         // Below average
        ]);

        let mut market_averages = HashMap::new();
        market_averages.insert(FuelType::RegularUnleaded, Money::from_amount("USD", 3.25).unwrap());
        market_averages.insert(FuelType::Premium, Money::from_amount("USD", 3.65).unwrap());

        assert!(fuel_options.has_competitive_pricing(&market_averages));
    }

    #[test]
    fn test_descriptions() {
        let fuel_options = FuelOptions::new(vec![
            create_test_fuel_price(FuelType::RegularUnleaded, 3.25),
            create_test_fuel_price(FuelType::Premium, 3.65),
        ]);

        assert_eq!(fuel_options.compact_description(), "2 fuel types");

        let detailed = fuel_options.detailed_description();
        assert!(detailed.contains("2 fuel types"));
        assert!(detailed.contains("$3.25"));
        assert!(detailed.contains("$3.65"));

        let types = fuel_options.available_fuel_types();
        assert_eq!(types.len(), 2);
        assert!(types.contains(&"Regular".to_string()));
        assert!(types.contains(&"Premium".to_string()));
    }

    #[test]
    fn test_average_price() {
        let fuel_options = FuelOptions::new(vec![
            create_test_fuel_price(FuelType::RegularUnleaded, 3.00),
            create_test_fuel_price(FuelType::Premium, 4.00),
        ]);

        let average = fuel_options.average_price().unwrap();
        assert!((average - 3.50).abs() < 0.01); // (3.00 + 4.00) / 2 = 3.50

        let empty = FuelOptions::empty();
        assert!(empty.average_price().is_none());
    }

    #[test]
    fn test_currency_filtering() {
        let fuel_options = FuelOptions::new(vec![
            create_test_fuel_price(FuelType::RegularUnleaded, 3.25),
            create_test_fuel_price(FuelType::Premium, 3.65),
        ]);

        let usd_prices = fuel_options.prices_in_currency("USD");
        assert_eq!(usd_prices.len(), 2);

        let eur_prices = fuel_options.prices_in_currency("EUR");
        assert_eq!(eur_prices.len(), 0);
    }

    #[test]
    fn test_default() {
        let default_fuel_options = FuelOptions::default();
        assert!(!default_fuel_options.has_fuel_info());
        assert_eq!(default_fuel_options.fuel_type_count(), 0);
    }
}