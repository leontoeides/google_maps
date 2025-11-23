use iso_currency::Currency;
use serde::Deserialize;

// -------------------------------------------------------------------------------------------------
//
/// Represents an amount of money with its currency type.
///
/// Money provides a standardized way to handle monetary values with proper currency identification
/// using ISO 4217 currency codes via the `iso_currency::Currency` type.
///
/// The amount is stored as whole units and nano-fractions to ensure precise decimal representation
/// without floating-point precision issues. Uses `i64` for units to enable proper ordering,
/// arithmetic, and comparison operations.
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
pub struct Money {
    /// The currency type using ISO 4217 standard.
    ///
    /// Provides access to currency metadata including symbol, decimal places, and full name.
    /// Enables type-safe currency operations and proper formatting based on currency conventions.
    #[serde(rename = "currencyCode")]
    #[getset(get = "pub")]
    pub currency: Currency,

    /// The whole units of the amount.
    ///
    /// The integer portion of the monetary amount. For example, if the amount is $123.45, this
    /// field contains 123. The units can be negative for representing debts, refunds, or credit
    /// amounts. Note: Google's API uses strings, but we deserialize to i64 for usability.
    #[serde(deserialize_with = "deserialize_units")]
    #[serde(serialize_with = "serialize_units")]
    #[getset(get_copy = "pub")]
    pub units: i64,

    /// Number of nano (10^-9) units of the amount.
    ///
    /// The fractional portion expressed in billionths for precise decimal representation. For
    /// $123.45, this would be 450,000,000 (0.45 * 10^9). The value must be between -999,999,999 and
    /// +999,999,999 inclusive and have the same sign as units.
    #[getset(get_copy = "pub")]
    pub nanos: Option<i32>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Money {
    /// Creates a new Money with the specified currency, units, and nanos.
    ///
    /// Used to construct a Money instance with precise control over all components. The nanos value
    /// should be between -999,999,999 and +999,999,999 and have the same sign as units for valid
    /// monetary representation.
    #[must_use]
    pub const fn new(currency: Currency, units: i64, nanos: Option<i32>) -> Self {
        Self {
            currency,
            units,
            nanos,
        }
    }

    /// Creates Money from a decimal amount and currency code.
    ///
    /// Converts a floating-point amount to the precise units + nanos representation. This is
    /// convenient for creating Money from user input or calculations, though direct construction is
    /// preferred for precision-critical applications.
    ///
    /// # Errors
    /// Returns an error if the currency code is not a valid ISO 4217 code.
    #[allow(clippy::cast_possible_truncation)]
    pub fn from_amount(
        currency_code: &str,
        amount: f64
    ) -> Result<Self, iso_currency::ParseCurrencyError> {
        let currency = currency_code.parse::<Currency>()?;
        let units = amount.trunc() as i64;
        let fractional = amount.fract();
        let nanos = (fractional * 1_000_000_000.0).round() as i32;

        Ok(Self {
            currency,
            units,
            nanos: Some(nanos),
        })
    }

    /// Creates Money representing zero amount in the specified currency.
    ///
    /// Used for initializing monetary values, representing free items, or as a base for monetary
    /// calculations and accumulations.
    ///
    /// # Errors
    /// Returns an error if the currency code is not a valid ISO 4217 code.
    pub fn zero(currency_code: &str) -> Result<Self, iso_currency::ParseCurrencyError> {
        let currency = currency_code.parse::<Currency>()?;
        Ok(Self {
            currency,
            units: 0,
            nanos: Some(0),
        })
    }

    /// Gets the currency code as a string.
    ///
    /// Returns the three-letter ISO 4217 currency code for compatibility with APIs and
    /// string-based operations.
    #[must_use]
    pub fn currency_code(&self) -> &str {
        self.currency.code()
    }

    /// Gets the currency symbol for display purposes.
    ///
    /// Returns the standard currency symbol (e.g., "$", "€", "£") for user-friendly formatting.
    #[must_use]
    pub fn currency_symbol(&self) -> String {
        self.currency.symbol().symbol
    }

    /// Gets the number of decimal places for this currency.
    ///
    /// Returns the standard number of decimal places used by this currency (e.g., `2` for USD/EUR,
    /// `0` for JPY, `3` for some Middle Eastern currencies).
    #[must_use]
    pub fn decimal_places(&self) -> u16 {
        self.currency.exponent().unwrap_or(2)
    }

    /// Gets the currency's full name for display.
    ///
    /// Returns the official currency name (e.g., "United States dollar", "Euro", "British Pound")
    /// for detailed displays, tooltips, or educational interfaces.
    #[must_use]
    pub fn currency_name(&self) -> &str {
        self.currency.name()
    }

    /// Converts to a decimal amount as f64.
    ///
    /// Combines units and nanos into a single floating-point value for display or calculations.
    /// Note that this may introduce floating-point precision issues and should be used carefully
    /// for financial calculations. If nanos is not specified, treats it as zero.
    #[must_use]
    pub fn to_amount(&self) -> f64 {
        let nanos_value = self.nanos.unwrap_or(0);
        self.units as f64 + (f64::from(nanos_value) / 1_000_000_000.0)
    }

    /// Returns whether this represents a positive amount.
    ///
    /// Used for validation, display logic, and business rules that need to distinguish between
    /// positive amounts, zero, and negative amounts. If nanos is not specified, only checks units.
    #[must_use]
    pub fn is_positive(&self) -> bool {
        let nanos_value = self.nanos.unwrap_or(0);
        self.units > 0 || (self.units == 0 && nanos_value > 0)
    }

    /// Returns whether this represents zero amount.
    ///
    /// Used to identify free items, no-cost services, or zeroed balances in financial calculations
    /// and display logic. If nanos is not specified, treats it as zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        let nanos_value = self.nanos.unwrap_or(0);
        self.units == 0 && nanos_value == 0
    }

    /// Returns whether this represents a negative amount.
    ///
    /// Used to identify debts, refunds, credits, or discounts in financial applications and
    /// business logic. If nanos is not specified, only checks units.
    #[must_use]
    pub fn is_negative(&self) -> bool {
        let nanos_value = self.nanos.unwrap_or(0);
        self.units < 0 || (self.units == 0 && nanos_value < 0)
    }

    /// Formats the money amount for display.
    ///
    /// Creates a user-friendly representation of the monetary amount with appropriate currency
    /// symbol and decimal formatting based on the currency's standard conventions.
    #[must_use]
    pub fn format(&self) -> String {
        let amount = self.to_amount();
        let symbol = self.currency_symbol();
        let decimal_places = self.decimal_places();

        format!("{}{}", symbol, Self::format_amount_with_decimals(amount, decimal_places))
    }

    /// Formats the money amount without currency symbol.
    ///
    /// Provides numerical formatting with appropriate decimal places but without currency symbol,
    /// useful for tables, calculations display, or when currency is indicated elsewhere.
    #[must_use]
    pub fn format_amount_only(&self) -> String {
        let amount = self.to_amount();
        let decimal_places = self.decimal_places();
        Self::format_amount_with_decimals(amount, decimal_places)
    }

    /// Helper method to format amount with variable decimal places.
    ///
    /// Handles the runtime decimal formatting by building the format string dynamically, since
    /// Rust's format! macro requires compile-time precision.
    fn format_amount_with_decimals(amount: f64, decimal_places: u16) -> String {
        match decimal_places {
            0 => format!("{amount:.0}"),
            1 => format!("{amount:.1}"),
            3 => format!("{amount:.3}"),
            4 => format!("{amount:.4}"),
            5 => format!("{amount:.5}"),
            6 => format!("{amount:.6}"),
            _ => format!("{amount:.2}"), // Default to 2 decimal places
        }
    }

    /// Formats with explicit currency code.
    ///
    /// Creates a display format that includes the ISO currency code, useful for international
    /// applications or when currency symbols might be ambiguous (e.g., "123.45 USD").
    #[must_use]
    pub fn format_with_code(&self) -> String {
        format!("{} {}", self.format_amount_only(), self.currency_code())
    }

    /// Returns whether this money is valid for financial operations.
    ///
    /// Validates that the nanos value, if present, is within the valid range for proper monetary
    /// representation. Currency is guaranteed to be valid since it's typed as `Currency`. Returns
    /// true if nanos is not specified.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.nanos
            .map_or(true, |n| (-999_999_999..=999_999_999)
            .contains(&n))
    }

    /// Returns whether this currency is a major trading currency.
    ///
    /// Used to identify widely-used international currencies for special handling in currency
    /// conversion, trading applications, or financial analysis.
    #[must_use]
    pub fn is_major_currency(&self) -> bool {
        matches!(self.currency_code(),
            "USD" | "EUR" | "GBP" | "JPY" | "CHF" | "CAD" | "AUD" | "CNY"
        )
    }

    /// Returns whether this is a cryptocurrency.
    ///
    /// Identifies digital currencies for specialized handling, though most cryptocurrencies may not
    /// be in the ISO 4217 standard and would need custom handling beyond this basic check.
    #[must_use]
    pub fn is_cryptocurrency(&self) -> bool {
        // Note: Most cryptocurrencies are not in ISO 4217
        // This is a basic check that could be extended
        matches!(self.currency_code(), "XBT")
    }

    /// Compares amounts assuming same currency.
    ///
    /// Provides ordering comparison for sorting and filtering operations. Only compares amounts,
    /// not currency types - use with caution for mixed-currency collections. Treats missing nanos
    /// as zero for comparison purposes.
    #[must_use]
    pub fn compare_amount(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp(other)
    }

    /// Returns a compact representation for space-constrained displays.
    ///
    /// Provides abbreviated formatting suitable for mobile interfaces, tables, or dashboard
    /// displays where space is limited.
    #[must_use]
    pub fn compact_format(&self) -> String {
        let amount = self.to_amount();
        let symbol = self.currency_symbol();

        // For large amounts, use compact notation
        if amount >= 1_000_000.0 {
            format!("{}{:.1}M", symbol, amount / 1_000_000.0)
        } else if amount >= 1_000.0 {
            format!("{}{:.1}K", symbol, amount / 1_000.0)
        } else {
            self.format()
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl PartialOrd for Money {
    /// Compares money amounts for ordering.
    ///
    /// Compares only the monetary amounts (units and nanos), not currency types. This enables
    /// sorting and filtering operations on money values. Missing nanos values are treated as zero
    /// for comparison purposes.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Money {
    /// Provides total ordering for money amounts.
    ///
    /// Orders money values by their amounts (units and nanos) for sorting operations. Does not
    /// consider currency in the comparison - use with caution when working with mixed-currency
    /// collections. Missing nanos values are treated as zero.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_nanos = self.nanos.unwrap_or(0);
        let other_nanos = other.nanos.unwrap_or(0);
        let self_amount = (self.units, self_nanos);
        let other_amount = (other.units, other_nanos);
        self_amount.cmp(&other_amount)
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl From<Money> for Currency {
    /// Converts Money to its Currency type.
    ///
    /// Enables extraction of currency metadata from `Money` instances for currency-specific
    /// operations.
    fn from(money: Money) -> Self {
        money.currency
    }
}

impl From<&Money> for Currency {
    /// Converts Money reference to its Currency type.
    ///
    /// Convenience implementation for extracting currency information without taking ownership.
    fn from(money: &Money) -> Self {
        money.currency
    }
}

// -------------------------------------------------------------------------------------------------
//
// Serde Helpers

fn deserialize_units<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let s = String::deserialize(deserializer)?;
    s.parse::<i64>().map_err(D::Error::custom)
}

#[allow(clippy::trivially_copy_pass_by_ref, reason = "serde interface")]
fn serialize_units<S>(units: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&units.to_string())
}