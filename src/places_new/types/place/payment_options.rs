// -------------------------------------------------------------------------------------------------
//
/// Payment options that a place accepts.
///
/// Payment options provide information about the different payment methods available at a business,
/// helping customers plan their visit and understand what payment types are accepted.
///
/// This is particularly useful for travelers, customers without certain payment methods, or those
/// preferring specific payment types.
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
#[allow(clippy::doc_markdown, clippy::struct_field_names)]
pub struct PaymentOptions {
    /// Place accepts credit cards as payment.
    ///
    /// Indicates whether the business accepts major credit cards such as Visa, MasterCard, American
    /// Express, and Discover. This is the most common electronic payment method at most businesses.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub accepts_credit_cards: Option<bool>,

    /// Place accepts debit cards as payment.
    ///
    /// Indicates whether the business accepts debit cards, which directly withdraw funds from a
    /// customer's bank account. Some businesses may accept credit cards but not debit cards due to
    /// different processing requirements.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub accepts_debit_cards: Option<bool>,

    /// Place accepts cash only as payment.
    ///
    /// Indicates whether the business only accepts cash payments. Note that places with this
    /// attribute may still accept other payment methods in addition to cash. This field primarily
    /// indicates that cash is definitely accepted.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub accepts_cash_only: Option<bool>,

    /// Place accepts NFC (Near Field Communication) payments.
    ///
    /// Indicates whether the business accepts contactless payments such as Apple Pay, Google Pay,
    /// Samsung Pay, or contactless credit/debit cards. This is increasingly important for
    /// contactless transactions.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub accepts_nfc: Option<bool>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl PaymentOptions {
    /// Creates a new `PaymentOptions` with all methods unspecified.
    ///
    /// Used to create a payment options structure where payment method information will be added
    /// individually based on available data.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            accepts_credit_cards: None,
            accepts_debit_cards: None,
            accepts_cash_only: None,
            accepts_nfc: None,
        }
    }

    /// Creates `PaymentOptions` accepting all common payment methods.
    ///
    /// Used for businesses that accept all major payment types, providing a convenient constructor
    /// for full payment flexibility.
    #[must_use]
    pub const fn accepts_all() -> Self {
        Self {
            accepts_credit_cards: Some(true),
            accepts_debit_cards: Some(true),
            accepts_cash_only: Some(true),
            accepts_nfc: Some(true),
        }
    }

    /// Creates `PaymentOptions` for a cash-only business.
    ///
    /// Used for businesses that only accept cash payments, such as some food trucks, small vendors,
    /// or businesses in areas with limited electronic payment infrastructure.
    #[must_use]
    pub const fn cash_only() -> Self {
        Self {
            accepts_credit_cards: Some(false),
            accepts_debit_cards: Some(false),
            accepts_cash_only: Some(true),
            accepts_nfc: Some(false),
        }
    }

    /// Creates `PaymentOptions` for businesses accepting cards but not cash.
    ///
    /// Used for modern businesses that have gone cashless, accepting only electronic payment
    /// methods for speed and security reasons.
    #[must_use]
    pub const fn cards_only() -> Self {
        Self {
            accepts_credit_cards: Some(true),
            accepts_debit_cards: Some(true),
            accepts_cash_only: Some(false),
            accepts_nfc: Some(true),
        }
    }

    /// Returns whether any payment method information is available.
    ///
    /// Used to determine if the business has provided any payment method information, helping
    /// decide whether to display payment options to users.
    #[must_use]
    pub const fn has_payment_info(self) -> bool {
        self.accepts_credit_cards.is_some() 
            || self.accepts_debit_cards.is_some()
            || self.accepts_cash_only.is_some() 
            || self.accepts_nfc.is_some()
    }

    /// Returns whether the business accepts any electronic payment methods.
    ///
    /// Used to identify businesses that support card or digital payments, useful for customers who
    /// prefer not to carry cash.
    #[must_use]
    pub const fn accepts_electronic_payments(self) -> Option<bool> {
        match (self.accepts_credit_cards, self.accepts_debit_cards, self.accepts_nfc) {
            (Some(true), _, _) | (_, Some(true), _) | (_, _, Some(true)) => Some(true),
            (Some(false), Some(false), Some(false)) => Some(false),
            _ => None, // Insufficient information
        }
    }

    /// Returns whether the business is likely cash-preferred or cash-only.
    ///
    /// Used to identify businesses where customers should be prepared to pay with cash, either
    /// because it's required or strongly preferred.
    #[must_use]
    pub const fn is_cash_preferred(self) -> Option<bool> {
        match self.accepts_cash_only {
            Some(true) => Some(true),
            Some(false) => Some(false),
            // If no electronic payments are accepted, cash is likely preferred
            None => match self.accepts_electronic_payments() {
                Some(false) => Some(true),
                _ => None,
            }
        }
    }

    /// Returns whether the business supports contactless payments.
    ///
    /// Used to identify businesses that support modern contactless payment methods, important for
    /// hygiene, speed, and customer convenience.
    #[must_use]
    pub const fn supports_contactless(self) -> Option<bool> {
        self.accepts_nfc
    }

    /// Returns whether the business accepts traditional card payments.
    ///
    /// Used to identify businesses that accept physical credit or debit cards, regardless of
    /// whether they support contactless options.
    #[must_use]
    pub const fn accepts_traditional_cards(self) -> Option<bool> {
        match (self.accepts_credit_cards, self.accepts_debit_cards) {
            (Some(true), _) | (_, Some(true)) => Some(true),
            (Some(false), Some(false)) => Some(false),
            _ => None,
        }
    }

    /// Gets a list of accepted payment methods for display.
    ///
    /// Returns a vector of user-friendly payment method names that can be displayed in user
    /// interfaces, filtered to only confirmed methods.
    #[must_use]
    pub fn accepted_methods(self) -> Vec<&'static str> {
        let mut methods = Vec::new();

        if self.accepts_credit_cards == Some(true) {
            methods.push("Credit Cards");
        }
        if self.accepts_debit_cards == Some(true) {
            methods.push("Debit Cards");
        }
        if self.accepts_cash_only == Some(true) {
            methods.push("Cash");
        }
        if self.accepts_nfc == Some(true) {
            methods.push("Contactless");
        }

        methods
    }

    /// Gets a user-friendly description of payment options.
    ///
    /// Provides a human-readable summary of payment methods suitable for displaying in search
    /// results, business listings, or detailed views.
    #[must_use]
    pub fn description(self) -> String {
        let methods = self.accepted_methods();
        
        match methods.len() {
            1 => format!("Accepts {}", methods[0].to_lowercase()),
            2 => format!("Accepts {} and {}", methods[0].to_lowercase(), methods[1].to_lowercase()),
            _ => methods.last().map_or_else(
                || "Payment methods unknown".to_string(),
                |last| {
                    let others = methods[..methods.len() - 1].join(", ").to_lowercase();
                    format!("Accepts {}, and {}", others, last.to_lowercase())
                })
        }
    }

    /// Gets a short payment summary for compact displays.
    ///
    /// Provides brief payment information suitable for mobile interfaces, map overlays, or
    /// space-constrained displays.
    #[must_use]
    pub fn short_description(self) -> String {
        let methods = self.accepted_methods();
        
        match methods.len() {
            0 => "Payment unknown".to_string(),
            1 => methods[0].to_string(),
            2 => format!("{} & {}", methods[0], methods[1]),
            _ => format!("{} payment methods", methods.len()),
        }
    }

    /// Returns payment method icons for visual display.
    ///
    /// Provides commonly recognized icons/symbols for payment methods, useful for visual interfaces
    /// and international applications.
    #[must_use]
    pub fn payment_icons(self) -> Vec<&'static str> {
        let mut icons = Vec::new();

        if self.accepts_credit_cards == Some(true) {
            icons.push("💳");
        }
        if self.accepts_debit_cards == Some(true) && self.accepts_credit_cards != Some(true) {
            icons.push("💳"); // Same icon as credit, but only if credit cards not already added
        }
        if self.accepts_cash_only == Some(true) {
            icons.push("💵");
        }
        if self.accepts_nfc == Some(true) {
            icons.push("📱");
        }

        // Remove duplicates while preserving order
        icons.dedup();
        icons
    }

    /// Returns whether payment options are suitable for tourists.
    ///
    /// Used to identify businesses that accept payment methods commonly available to travelers,
    /// such as major credit cards and contactless payments.
    #[must_use]
    pub const fn is_tourist_friendly(self) -> Option<bool> {
        match (self.accepts_credit_cards, self.accepts_nfc) {
            (Some(true), _) | (_, Some(true)) => Some(true),
            (Some(false), Some(false)) => {
                // If only cash/debit, less tourist-friendly
                Some(false)
            }
            _ => None,
        }
    }

    /// Returns whether the business has modern payment capabilities.
    ///
    /// Used to identify businesses with up-to-date payment processing, including contactless and
    /// electronic payment support.
    #[must_use]
    pub const fn has_modern_payments(self) -> Option<bool> {
        // Modern payments include NFC or at least credit cards
        match (self.accepts_nfc, self.accepts_credit_cards) {
            (Some(false), Some(true)) | (Some(true), _) => Some(true),
            (Some(false), Some(false)) => Some(false),
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
        let payment_options = PaymentOptions {
            accepts_credit_cards: Some(true),
            accepts_debit_cards: Some(true),
            accepts_cash_only: Some(false),
            accepts_nfc: Some(true),
        };

        let json = serde_json::to_string(&payment_options).unwrap();
        assert!(json.contains("acceptsCreditCards"));
        assert!(json.contains("acceptsDebitCards"));
        assert!(json.contains("acceptsCashOnly"));
        assert!(json.contains("acceptsNfc"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "acceptsCreditCards": true,
            "acceptsDebitCards": false,
            "acceptsCashOnly": true,
            "acceptsNfc": null
        }"#;

        let payment_options: PaymentOptions = serde_json::from_str(json).unwrap();
        assert_eq!(payment_options.accepts_credit_cards, Some(true));
        assert_eq!(payment_options.accepts_debit_cards, Some(false));
        assert_eq!(payment_options.accepts_cash_only, Some(true));
        assert_eq!(payment_options.accepts_nfc, None);
    }

    #[test]
    fn test_constructors() {
        let new = PaymentOptions::new();
        assert_eq!(new.accepts_credit_cards, None);
        assert_eq!(new.accepts_debit_cards, None);
        assert_eq!(new.accepts_cash_only, None);
        assert_eq!(new.accepts_nfc, None);

        let all = PaymentOptions::accepts_all();
        assert_eq!(all.accepts_credit_cards, Some(true));
        assert_eq!(all.accepts_debit_cards, Some(true));
        assert_eq!(all.accepts_cash_only, Some(true));
        assert_eq!(all.accepts_nfc, Some(true));

        let cash_only = PaymentOptions::cash_only();
        assert_eq!(cash_only.accepts_credit_cards, Some(false));
        assert_eq!(cash_only.accepts_debit_cards, Some(false));
        assert_eq!(cash_only.accepts_cash_only, Some(true));
        assert_eq!(cash_only.accepts_nfc, Some(false));

        let cards_only = PaymentOptions::cards_only();
        assert_eq!(cards_only.accepts_credit_cards, Some(true));
        assert_eq!(cards_only.accepts_debit_cards, Some(true));
        assert_eq!(cards_only.accepts_cash_only, Some(false));
        assert_eq!(cards_only.accepts_nfc, Some(true));
    }

    #[test]
    fn test_has_payment_info() {
        let empty = PaymentOptions::new();
        assert!(!empty.has_payment_info());

        let partial = PaymentOptions {
            accepts_credit_cards: Some(true),
            ..PaymentOptions::new()
        };
        assert!(partial.has_payment_info());
    }

    #[test]
    fn test_accepts_electronic_payments() {
        let electronic = PaymentOptions {
            accepts_credit_cards: Some(true),
            accepts_debit_cards: Some(false),
            accepts_cash_only: Some(true),
            accepts_nfc: Some(false),
        };
        assert_eq!(electronic.accepts_electronic_payments(), Some(true));

        let cash_only = PaymentOptions {
            accepts_credit_cards: Some(false),
            accepts_debit_cards: Some(false),
            accepts_cash_only: Some(true),
            accepts_nfc: Some(false),
        };
        assert_eq!(cash_only.accepts_electronic_payments(), Some(false));

        let unknown = PaymentOptions::new();
        assert_eq!(unknown.accepts_electronic_payments(), None);
    }

    #[test]
    fn test_is_cash_preferred() {
        let cash_only = PaymentOptions::cash_only();
        assert_eq!(cash_only.is_cash_preferred(), Some(true));

        let cards_only = PaymentOptions::cards_only();
        assert_eq!(cards_only.is_cash_preferred(), Some(false));

        let no_electronic = PaymentOptions {
            accepts_credit_cards: Some(false),
            accepts_debit_cards: Some(false),
            accepts_cash_only: None,
            accepts_nfc: Some(false),
        };
        assert_eq!(no_electronic.is_cash_preferred(), Some(true));
    }

    #[test]
    fn test_supports_contactless() {
        let with_nfc = PaymentOptions {
            accepts_nfc: Some(true),
            ..PaymentOptions::new()
        };
        assert_eq!(with_nfc.supports_contactless(), Some(true));

        let without_nfc = PaymentOptions {
            accepts_nfc: Some(false),
            ..PaymentOptions::new()
        };
        assert_eq!(without_nfc.supports_contactless(), Some(false));
    }

    #[test]
    fn test_accepts_traditional_cards() {
        let credit_only = PaymentOptions {
            accepts_credit_cards: Some(true),
            accepts_debit_cards: Some(false),
            ..PaymentOptions::new()
        };
        assert_eq!(credit_only.accepts_traditional_cards(), Some(true));

        let no_cards = PaymentOptions {
            accepts_credit_cards: Some(false),
            accepts_debit_cards: Some(false),
            ..PaymentOptions::new()
        };
        assert_eq!(no_cards.accepts_traditional_cards(), Some(false));
    }

    #[test]
    fn test_accepted_methods() {
        let all = PaymentOptions::accepts_all();
        let methods = all.accepted_methods();
        assert_eq!(methods.len(), 4);
        assert!(methods.contains(&"Credit Cards"));
        assert!(methods.contains(&"Debit Cards"));
        assert!(methods.contains(&"Cash"));
        assert!(methods.contains(&"Contactless"));

        let cash_only = PaymentOptions::cash_only();
        let methods = cash_only.accepted_methods();
        assert_eq!(methods, vec!["Cash"]);
    }

    #[test]
    fn test_description() {
        let cash_only = PaymentOptions::cash_only();
        assert_eq!(cash_only.description(), "Accepts cash");

        let cards_and_cash = PaymentOptions {
            accepts_credit_cards: Some(true),
            accepts_cash_only: Some(true),
            ..PaymentOptions::new()
        };
        assert_eq!(cards_and_cash.description(), "Accepts credit cards and cash");

        let empty = PaymentOptions::new();
        assert_eq!(empty.description(), "Payment methods unknown");
    }

    #[test]
    fn test_short_description() {
        let all = PaymentOptions::accepts_all();
        assert_eq!(all.short_description(), "4 payment methods");

        let cash_only = PaymentOptions::cash_only();
        assert_eq!(cash_only.short_description(), "Cash");

        let two_methods = PaymentOptions {
            accepts_credit_cards: Some(true),
            accepts_nfc: Some(true),
            ..PaymentOptions::new()
        };
        assert_eq!(two_methods.short_description(), "Credit Cards & Contactless");
    }

    #[test]
    fn test_payment_icons() {
        let all = PaymentOptions::accepts_all();
        let icons = all.payment_icons();
        assert!(icons.contains(&"💳"));
        assert!(icons.contains(&"💵"));
        assert!(icons.contains(&"📱"));
        // Should not have duplicates
        assert_eq!(icons.len(), 3);

        let cash_only = PaymentOptions::cash_only();
        let icons = cash_only.payment_icons();
        assert_eq!(icons, vec!["💵"]);
    }

    #[test]
    fn test_is_tourist_friendly() {
        let credit_cards = PaymentOptions {
            accepts_credit_cards: Some(true),
            ..PaymentOptions::new()
        };
        assert_eq!(credit_cards.is_tourist_friendly(), Some(true));

        let cash_only = PaymentOptions::cash_only();
        assert_eq!(cash_only.is_tourist_friendly(), Some(false));

        let contactless = PaymentOptions {
            accepts_nfc: Some(true),
            ..PaymentOptions::new()
        };
        assert_eq!(contactless.is_tourist_friendly(), Some(true));
    }

    #[test]
    fn test_has_modern_payments() {
        let nfc = PaymentOptions {
            accepts_nfc: Some(true),
            ..PaymentOptions::new()
        };
        assert_eq!(nfc.has_modern_payments(), Some(true));

        let credit_no_nfc = PaymentOptions {
            accepts_credit_cards: Some(true),
            accepts_nfc: Some(false),
            ..PaymentOptions::new()
        };
        assert_eq!(credit_no_nfc.has_modern_payments(), Some(true));

        let cash_only = PaymentOptions {
            accepts_credit_cards: Some(false),
            accepts_nfc: Some(false),
            ..PaymentOptions::new()
        };
        assert_eq!(cash_only.has_modern_payments(), Some(false));
    }

    #[test]
    fn test_default() {
        let default_payment = PaymentOptions::default();
        assert_eq!(default_payment.accepts_credit_cards, None);
        assert_eq!(default_payment.accepts_debit_cards, None);
        assert_eq!(default_payment.accepts_cash_only, None);
        assert_eq!(default_payment.accepts_nfc, None);
        assert!(!default_payment.has_payment_info());
    }
}