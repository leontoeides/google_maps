use crate::places_new::Money;

// -------------------------------------------------------------------------------------------------
//
/// The price range associated with a place.
///
/// Price ranges provide cost expectations for places like restaurants, hotels, or services by
/// defining a minimum price point and optionally an upper bound. The start price indicates the
/// minimum cost a customer should expect, while the end price establishes an upper limit when
/// specified.
///
/// When no end price is provided, the range indicates "starting from" the start price without
/// an upper bound, useful for businesses with highly variable pricing or open-ended cost structures.
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
pub struct PriceRange {
    /// The low end of the price range (inclusive).
    ///
    /// Represents the minimum price customers should expect to pay at this place. This establishes
    /// the baseline cost and helps users understand the entry-level pricing for goods or services
    /// offered at this location.
    #[getset(get = "pub")]
    pub start_price: Money,

    /// The high end of the price range (exclusive).
    ///
    /// Represents the upper limit of pricing when specified. Prices should be lower than this amount.
    /// When not provided, the range indicates "starting from" the start price without an established
    /// upper bound, suitable for businesses with unlimited or highly variable top-tier pricing.
    #[serde(default)]
    #[getset(get = "pub")]
    pub end_price: Option<Money>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl PriceRange {
    /// Creates a new `PriceRange` with start and optional end prices.
    ///
    /// Used to construct price ranges with defined bounds. Pass `None` for `end_price` to indicate
    /// an open-ended range starting from the minimum price.
    #[must_use]
    pub const fn new(start_price: Money, end_price: Option<Money>) -> Self {
        Self {
            start_price,
            end_price,
        }
    }

    /// Creates a price range starting from the specified amount without upper bound.
    ///
    /// Used for businesses with variable or unlimited pricing where only a minimum cost can be
    /// established. Common for services, custom work, or premium establishments.
    #[must_use]
    pub const fn starting_from(start_price: Money) -> Self {
        Self {
            start_price,
            end_price: None,
        }
    }

    /// Creates a price range with both start and end bounds.
    ///
    /// Used for businesses with well-defined pricing tiers and established maximum costs.
    /// Provides customers with clear cost expectations within a specific range.
    #[must_use]
    pub const fn bounded(start_price: Money, end_price: Money) -> Self {
        Self {
            start_price,
            end_price: Some(end_price),
        }
    }

    /// Returns whether this price range has an upper bound.
    ///
    /// Used to determine if the range provides complete pricing information or represents
    /// open-ended pricing starting from a minimum amount.
    #[must_use]
    pub const fn has_upper_bound(&self) -> bool {
        self.end_price.is_some()
    }

    /// Returns whether this represents an open-ended price range.
    ///
    /// Used to identify pricing that starts from a minimum but has no established upper limit,
    /// requiring different display and budgeting considerations.
    #[must_use]
    pub const fn is_open_ended(&self) -> bool {
        self.end_price.is_none()
    }

    /// Gets the price spread between start and end prices.
    ///
    /// Returns the difference between the upper and lower bounds when both are available and in
    /// the same currency. Useful for understanding price variability and range width.
    #[must_use]
    pub fn price_spread(&self) -> Option<f64> {
        let end_price = self.end_price.as_ref()?;
        
        // Only calculate spread for same currency
        if end_price.currency_code() != self.start_price.currency_code() {
            return None;
        }
        
        let start_amount = self.start_price.to_amount();
        let end_amount = end_price.to_amount();
        Some(end_amount - start_amount)
    }

    /// Returns whether the price range spans a significant cost difference.
    ///
    /// Used to identify price ranges with substantial variability, indicating diverse pricing
    /// options or significant service/quality tiers within the establishment.
    #[must_use]
    pub fn has_wide_range(&self) -> bool {
        self.price_spread()
            .is_some_and(|spread| {
                let start_amount = self.start_price.to_amount();
                spread > start_amount * 0.5 // >50% increase
            })
    }

    /// Gets the midpoint price when both bounds are available.
    ///
    /// Returns the average of start and end prices for budget estimation and comparison purposes.
    /// Only works when both prices are in the same currency.
    #[must_use]
    pub fn midpoint_price(&self) -> Option<f64> {
        let end_price = self.end_price.as_ref()?;

        // Only calculate midpoint for same currency
        if end_price.currency_code() != self.start_price.currency_code() {
            return None;
        }

        let start_amount = self.start_price.to_amount();
        let end_amount = end_price.to_amount();
        Some((start_amount + end_amount) / 2.0)
    }

    /// Returns whether this price range is considered budget-friendly.
    ///
    /// Used to categorize establishments based on affordability, helping users filter options
    /// based on budget constraints. Uses a simple heuristic based on start price.
    #[must_use]
    pub fn is_budget_friendly(&self) -> bool {
        let amount = self.start_price.to_amount();
        // Simple heuristic - could be made configurable or currency-aware
        match self.start_price.currency_code() {
            "USD" | "EUR" | "GBP" | "CAD" | "AUD" => amount < 20.0,
            "JPY" => amount < 2000.0,
            _ => amount < 25.0, // Conservative default
        }
    }

    /// Returns whether this price range is considered premium pricing.
    ///
    /// Used to identify high-end establishments for luxury market filtering and premium service
    /// expectations. Based on start price thresholds that vary by currency.
    #[must_use]
    pub fn is_premium(&self) -> bool {
        let amount = self.start_price.to_amount();
        // Currency-specific premium thresholds
        match self.start_price.currency_code() {
            "USD" | "EUR" | "GBP" | "CAD" | "AUD" => amount >= 100.0,
            "JPY" => amount >= 10000.0,
            _ => amount >= 120.0, // Conservative default
        }
    }

    /// Gets a formatted display string for the price range.
    ///
    /// Creates user-friendly text representation of the pricing, handling both bounded and
    /// open-ended ranges appropriately. Uses currency formatting from the start price.
    #[must_use]
    pub fn display_range(&self) -> String {
        self.end_price.as_ref().map_or_else(|| format!("from {}", self.start_price.format()), |end_price| if end_price.currency_code() == self.start_price.currency_code() {
                    format!("{} - {}", self.start_price.format(), end_price.format())
                } else {
                    // Different currencies - show both separately
                    format!("{} to {}", self.start_price.format(), end_price.format())
                })
    }

    /// Gets a compact display string suitable for space-constrained interfaces.
    ///
    /// Provides abbreviated formatting for mobile displays, map overlays, or dashboard views
    /// where space is limited but pricing information is essential.
    #[must_use]
    pub fn compact_display(&self) -> String {
        self.end_price.as_ref().map_or_else(|| format!("{}+", self.start_price.compact_format()), |end_price| if end_price.currency_code() == self.start_price.currency_code() {
                    format!("{}-{}",
                        self.start_price.compact_format(),
                        end_price.compact_format())
                } else {
                    format!("{}+", self.start_price.compact_format())
                })
    }

    /// Returns the currency code for this price range.
    ///
    /// Gets the currency from the start price, which is always present. Useful for currency
    /// filtering, formatting decisions, and validation when mixing with other monetary values.
    #[must_use]
    pub fn currency_code(&self) -> &str {
        self.start_price.currency_code()
    }

    /// Returns whether both prices use the same currency.
    ///
    /// Used to validate price range consistency and determine whether mathematical operations
    /// like spread calculation or midpoint determination are meaningful.
    #[must_use]
    pub fn has_consistent_currency(&self) -> bool {
        self.end_price.as_ref()
            .map_or(true, |end| end.currency_code() == self.start_price.currency_code())
    }

    /// Gets pricing category as a string for filtering and display.
    ///
    /// Returns a simple categorization of the price range for user interfaces, filtering systems,
    /// and business analytics based on the start price level.
    #[must_use]
    pub fn price_category(&self) -> &'static str {
        if self.is_budget_friendly() {
            "Budget"
        } else if self.is_premium() {
            "Premium"
        } else {
            "Moderate"
        }
    }

    /// Returns whether this price range is suitable for a given budget.
    ///
    /// Determines if the price range fits within a specified budget amount, considering that
    /// open-ended ranges may exceed the budget despite having acceptable start prices.
    #[must_use]
    pub fn fits_budget(&self, budget_amount: f64, currency_code: &str) -> Option<bool> {
        // Only compare if currencies match
        if self.start_price.currency_code() != currency_code {
            return None;
        }

        let start_amount = self.start_price.to_amount();

        // If start price exceeds budget, definitely doesn't fit
        if start_amount > budget_amount {
            return Some(false);
        }

        // If no end price, we can't guarantee it fits, but start price is acceptable
        let Some(end_price) = &self.end_price else {
            return Some(true); // Optimistic - start price fits
        };

        // Check if end price also fits budget
        let end_amount = end_price.to_amount();
        Some(end_amount <= budget_amount)
    }

    /// Gets a detailed description including pricing context and category.
    ///
    /// Provides comprehensive information suitable for detailed place listings, including
    /// price range, category, and contextual information about the pricing structure.
    #[must_use]
    pub fn detailed_description(&self) -> String {
        let range_display = self.display_range();
        let category = self.price_category();
        
        if self.is_open_ended() {
            format!("{range_display} ({category}+ category)")
        } else {
            format!("{range_display} ({category} category)")
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::fmt::Display for PriceRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_range())
    }
}