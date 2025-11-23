use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

// -------------------------------------------------------------------------------------------------
//
/// Identifies the type of secondary hours for a place.
///
/// Secondary hours are different from a business's main operating hours and provide specific
/// information about specialized services or access times.
///
/// For example, a restaurant might have different hours for drive-through service versus dine-in
/// service, or a pharmacy might have specific hours for prescription pickup versus general
/// shopping.
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
pub enum SecondaryHoursType {
    /// Default value when secondary hour type is not specified.
    ///
    /// This is used as a fallback when the secondary hours type cannot be determined or is not
    /// provided in the API response.
    #[default]
    #[serde(rename = "SECONDARY_HOURS_TYPE_UNSPECIFIED")]
    Unspecified = 0,

    /// Drive-through service hours for banks, restaurants, or pharmacies.
    ///
    /// These hours indicate when customers can use drive-through services, which may differ from
    /// regular business hours. Common for fast food restaurants, banks, and pharmacies that offer
    /// drive-through windows.
    DriveThrough = 1,

    /// Happy hour service times.
    ///
    /// Special pricing or promotional hours typically offered by bars, restaurants, or
    /// entertainment venues. These hours usually feature discounted drinks, food, or other
    /// promotions.
    HappyHour = 2,

    /// Delivery service availability hours.
    ///
    /// Hours when the business offers delivery services, which may be more limited than regular
    /// operating hours due to staffing or logistical constraints.
    Delivery = 3,

    /// Takeout service availability hours.
    ///
    /// Hours when customers can order food for pickup, which may extend beyond regular dining hours
    /// or be available when dine-in service is not offered.
    Takeout = 4,

    /// Kitchen operating hours.
    ///
    /// Hours when the kitchen is actively preparing food, which may be shorter than the
    /// restaurant's overall operating hours. Useful for establishments that stay open for drinks
    /// after kitchen closure.
    Kitchen = 5,

    /// Breakfast service hours.
    ///
    /// Specific hours when breakfast items are available, which may be a subset of the restaurant's
    /// total operating hours.
    Breakfast = 6,

    /// Lunch service hours.
    ///
    /// Specific hours when lunch items are available, which may differ from overall restaurant
    /// hours or other meal service times.
    Lunch = 7,

    /// Dinner service hours.
    ///
    /// Specific hours when dinner items are available, which may be later than lunch service and
    /// represent the main evening dining period.
    Dinner = 8,

    /// Brunch service hours.
    ///
    /// Special weekend or holiday hours when brunch items are available, typically combining
    /// breakfast and lunch offerings during late morning hours.
    Brunch = 9,

    /// Pickup service availability hours.
    ///
    /// Hours when customers can collect pre-ordered items, which may differ from regular business
    /// hours and could extend beyond normal operating times.
    Pickup = 10,

    /// Access hours for storage or specialized facilities.
    ///
    /// Hours when customers can access storage units, lockers, or other facilities that may have
    /// different access times than the main business office hours.
    Access = 11,

    /// Special hours designated for senior citizens.
    ///
    /// Dedicated time periods when businesses offer special services, discounts, or priority access
    /// for senior customers, often during less busy periods.
    SeniorHours = 12,

    /// Online service availability hours.
    ///
    /// Hours when online services, customer support, or digital platforms are actively staffed and
    /// available, which may differ from physical location hours.
    OnlineServiceHours = 13,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl SecondaryHoursType {
    /// Returns whether this is a food service related hours type.
    ///
    /// Used to identify secondary hours that relate to food preparation or service, which can be
    /// useful for filtering or categorizing different types of hours.
    #[must_use]
    pub const fn is_food_service(&self) -> bool {
        matches!(
            self,
            Self::Kitchen
                | Self::Breakfast
                | Self::Lunch
                | Self::Dinner
                | Self::Brunch
                | Self::Delivery
                | Self::Takeout
        )
    }

    /// Returns whether this is a customer service related hours type.
    ///
    /// Used to identify secondary hours that relate to customer-facing services rather than
    /// operational or internal business functions.
    #[must_use]
    pub const fn is_customer_service(&self) -> bool {
        matches!(
            self,
            Self::DriveThrough
                | Self::Pickup
                | Self::Access
                | Self::SeniorHours
                | Self::OnlineServiceHours
        )
    }

    /// Returns whether this represents special promotional hours.
    ///
    /// Used to identify hours that are primarily for promotional or special pricing purposes rather
    /// than standard service availability.
    #[must_use]
    pub const fn is_promotional(&self) -> bool {
        matches!(self, Self::HappyHour | Self::SeniorHours)
    }

    /// Returns a human-readable description of this hours type.
    ///
    /// Provides a user-friendly description that can be displayed in UIs or used for logging and
    /// debugging purposes.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Unspecified => "Unspecified hours type",
            Self::DriveThrough => "Drive-through hours",
            Self::HappyHour => "Happy hour",
            Self::Delivery => "Delivery hours",
            Self::Takeout => "Takeout hours",
            Self::Kitchen => "Kitchen hours",
            Self::Breakfast => "Breakfast hours",
            Self::Lunch => "Lunch hours",
            Self::Dinner => "Dinner hours",
            Self::Brunch => "Brunch hours",
            Self::Pickup => "Pickup hours",
            Self::Access => "Access hours",
            Self::SeniorHours => "Senior hours",
            Self::OnlineServiceHours => "Online service hours",
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
        let hours_type = SecondaryHoursType::DriveThrough;
        let json = serde_json::to_string(&hours_type).unwrap();
        assert_eq!(json, r#""DRIVE_THROUGH""#);

        let hours_type = SecondaryHoursType::OnlineServiceHours;
        let json = serde_json::to_string(&hours_type).unwrap();
        assert_eq!(json, r#""ONLINE_SERVICE_HOURS""#);
    }

    #[test]
    fn test_deserialization() {
        let json = r#""HAPPY_HOUR""#;
        let hours_type: SecondaryHoursType = serde_json::from_str(json).unwrap();
        assert_eq!(hours_type, SecondaryHoursType::HappyHour);

        let json = r#""SECONDARY_HOURS_TYPE_UNSPECIFIED""#;
        let hours_type: SecondaryHoursType = serde_json::from_str(json).unwrap();
        assert_eq!(hours_type, SecondaryHoursType::Unspecified);
    }

    #[test]
    fn test_default() {
        let default_type = SecondaryHoursType::default();
        assert_eq!(default_type, SecondaryHoursType::Unspecified);
    }

    #[test]
    fn test_food_service_classification() {
        assert!(SecondaryHoursType::Kitchen.is_food_service());
        assert!(SecondaryHoursType::Breakfast.is_food_service());
        assert!(SecondaryHoursType::Delivery.is_food_service());
        assert!(!SecondaryHoursType::DriveThrough.is_food_service());
        assert!(!SecondaryHoursType::Access.is_food_service());
    }

    #[test]
    fn test_customer_service_classification() {
        assert!(SecondaryHoursType::DriveThrough.is_customer_service());
        assert!(SecondaryHoursType::Pickup.is_customer_service());
        assert!(SecondaryHoursType::OnlineServiceHours.is_customer_service());
        assert!(!SecondaryHoursType::Kitchen.is_customer_service());
        assert!(!SecondaryHoursType::Breakfast.is_customer_service());
    }

    #[test]
    fn test_promotional_classification() {
        assert!(SecondaryHoursType::HappyHour.is_promotional());
        assert!(SecondaryHoursType::SeniorHours.is_promotional());
        assert!(!SecondaryHoursType::Kitchen.is_promotional());
        assert!(!SecondaryHoursType::DriveThrough.is_promotional());
    }

    #[test]
    fn test_description() {
        assert_eq!(SecondaryHoursType::DriveThrough.description(), "Drive-through hours");
        assert_eq!(SecondaryHoursType::HappyHour.description(), "Happy hour");
        assert_eq!(SecondaryHoursType::OnlineServiceHours.description(), "Online service hours");
    }
}