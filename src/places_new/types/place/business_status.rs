use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

// -------------------------------------------------------------------------------------------------
//
/// Indicates the operational status of a business or place.
///
/// Business status provides information about whether a place is currently operating, temporarily
/// closed, or permanently shut down.
///
/// This is useful for determining if a business is available for customers and helps applications
/// provide accurate information about place availability.
#[derive(
    // std
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
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
pub enum BusinessStatus {
    /// Default value when business status is unspecified or unknown.
    ///
    /// This value is used as a fallback when the business status cannot be determined from
    /// available data. Applications should handle this as an unknown state.
    #[default]
    #[serde(rename = "BUSINESS_STATUS_UNSPECIFIED")]
    Unspecified = 0,

    /// The establishment is operational and may be open for business.
    ///
    /// This indicates the business is actively operating, though it doesn't guarantee the business
    //// is currently open (check opening hours for current availability). The business is accepting
    /// customers during its normal operating hours.
    Operational = 1,

    /// The establishment is temporarily closed for business.
    ///
    /// This indicates the business is not currently serving customers but is expected to reopen in
    /// the future. Common reasons include renovations, staff shortages, seasonal closures, or
    /// temporary disruptions like holidays or emergencies.
    ClosedTemporarily = 2,

    /// The establishment is permanently closed and will not reopen.
    ///
    /// This indicates the business has ceased operations permanently. The location may be vacant,
    /// demolished, or occupied by a different business. Applications should avoid directing users
    /// to permanently closed establishments.
    ClosedPermanently = 3,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl BusinessStatus {
    /// Returns whether the business is currently accepting customers.
    ///
    /// Used to determine if a business is available for customer visits, though this doesn't
    /// account for current opening hours - only operational status.
    #[must_use]
    pub const fn is_accepting_customers(self) -> Option<bool> {
        match self {
            Self::Operational => Some(true),
            Self::ClosedTemporarily | Self::ClosedPermanently => Some(false),
            Self::Unspecified => None,
        }
    }

    /// Returns whether the business closure is permanent.
    ///
    /// Used to distinguish between temporary disruptions and permanent closures, which is important
    /// for business directory maintenance and user guidance.
    #[must_use]
    pub const fn is_permanently_closed(self) -> bool {
        matches!(self, Self::ClosedPermanently)
    }

    /// Returns whether the business closure is temporary.
    ///
    /// Used to identify businesses that may reopen, which can be useful for showing different
    /// messaging to users or scheduling re-checks of status.
    #[must_use]
    pub const fn is_temporarily_closed(self) -> bool {
        matches!(self, Self::ClosedTemporarily)
    }

    /// Returns whether the business is in any closed state.
    ///
    /// Used to quickly identify businesses that are not serving customers, regardless of whether
    /// the closure is temporary or permanent.
    #[must_use]
    pub const fn is_closed(self) -> bool {
        matches!(self, Self::ClosedTemporarily | Self::ClosedPermanently)
    }

    /// Returns whether the business status is known and definitive.
    ///
    /// Used to determine if the status information is reliable enough to make decisions about
    /// business availability or user recommendations.
    #[must_use]
    pub const fn is_status_known(self) -> bool {
        !matches!(self, Self::Unspecified)
    }

    /// Returns a human-readable description of the business status.
    ///
    /// Provides user-friendly text that can be displayed in UIs to inform users about the current
    /// state of a business.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Unspecified => "Status unknown",
            Self::Operational => "Open for business",
            Self::ClosedTemporarily => "Temporarily closed",
            Self::ClosedPermanently => "Permanently closed",
        }
    }

    /// Returns a short status indicator suitable for compact displays.
    ///
    /// Provides brief status text that can be used in lists, maps, or other space-constrained
    /// interfaces where full descriptions are too long.
    #[must_use]
    pub const fn short_description(self) -> &'static str {
        match self {
            Self::Unspecified => "Unknown",
            Self::Operational => "Open",
            Self::ClosedTemporarily => "Temp. Closed",
            Self::ClosedPermanently => "Closed",
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
        let status = BusinessStatus::Operational;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""OPERATIONAL""#);

        let status = BusinessStatus::ClosedPermanently;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""CLOSED_PERMANENTLY""#);
    }

    #[test]
    fn test_deserialization() {
        let json = r#""CLOSED_TEMPORARILY""#;
        let status: BusinessStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, BusinessStatus::ClosedTemporarily);

        let json = r#""BUSINESS_STATUS_UNSPECIFIED""#;
        let status: BusinessStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, BusinessStatus::Unspecified);
    }

    #[test]
    fn test_default() {
        let default_status = BusinessStatus::default();
        assert_eq!(default_status, BusinessStatus::Unspecified);
    }

    #[test]
    fn test_is_accepting_customers() {
        assert_eq!(BusinessStatus::Operational.is_accepting_customers(), Some(true));
        assert_eq!(BusinessStatus::ClosedTemporarily.is_accepting_customers(), Some(false));
        assert_eq!(BusinessStatus::ClosedPermanently.is_accepting_customers(), Some(false));
        assert_eq!(BusinessStatus::Unspecified.is_accepting_customers(), None);
    }

    #[test]
    fn test_closure_status() {
        assert!(BusinessStatus::ClosedPermanently.is_permanently_closed());
        assert!(!BusinessStatus::Operational.is_permanently_closed());
        
        assert!(BusinessStatus::ClosedTemporarily.is_temporarily_closed());
        assert!(!BusinessStatus::Operational.is_temporarily_closed());

        assert!(BusinessStatus::ClosedTemporarily.is_closed());
        assert!(BusinessStatus::ClosedPermanently.is_closed());
        assert!(!BusinessStatus::Operational.is_closed());
    }

    #[test]
    fn test_status_known() {
        assert!(BusinessStatus::Operational.is_status_known());
        assert!(BusinessStatus::ClosedTemporarily.is_status_known());
        assert!(BusinessStatus::ClosedPermanently.is_status_known());
        assert!(!BusinessStatus::Unspecified.is_status_known());
    }

    #[test]
    fn test_descriptions() {
        assert_eq!(BusinessStatus::Operational.description(), "Open for business");
        assert_eq!(BusinessStatus::ClosedTemporarily.description(), "Temporarily closed");
        assert_eq!(BusinessStatus::ClosedPermanently.description(), "Permanently closed");
        assert_eq!(BusinessStatus::Unspecified.description(), "Status unknown");

        assert_eq!(BusinessStatus::Operational.short_description(), "Open");
        assert_eq!(BusinessStatus::ClosedTemporarily.short_description(), "Temp. Closed");
        assert_eq!(BusinessStatus::ClosedPermanently.short_description(), "Closed");
        assert_eq!(BusinessStatus::Unspecified.short_description(), "Unknown");
    }
}