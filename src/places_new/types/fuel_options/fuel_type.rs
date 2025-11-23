use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

// -------------------------------------------------------------------------------------------------
//
/// Represents different types of fuel available at gas stations.
///
/// Fuel types cover various gasoline grades, diesel variants, alternative fuels, and specialized
/// options available at different stations.
///
/// This information helps drivers find stations that carry their required fuel type and compare
/// pricing across different fuel grades and alternative energy sources.
#[derive(
    // std
    Clone,
    Copy,
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
pub enum FuelType {
    /// Unspecified fuel type.
    ///
    /// Used as a fallback when the fuel type cannot be determined or is not provided in the data.
    /// Applications should handle this as unknown fuel information.
    #[default]
    #[serde(rename = "FUEL_TYPE_UNSPECIFIED")]
    Unspecified = 0,

    /// Standard diesel fuel.
    ///
    /// Common diesel fuel used by most diesel vehicles including trucks, buses, and diesel
    /// passenger cars. Typically the most widely available diesel option.
    Diesel = 1,

    /// Premium diesel fuel with additives.
    ///
    /// Enhanced diesel with cleaning agents, performance additives, or improved characteristics.
    /// Often marketed as premium or enhanced diesel with benefits for engine performance and
    /// maintenance.
    DieselPlus = 2,

    /// Regular unleaded gasoline (typically 87 octane in North America).
    ///
    /// The most common and economical gasoline grade, suitable for most passenger vehicles. This is
    /// the baseline fuel that most gas stations carry.
    RegularUnleaded = 3,

    /// Mid-grade gasoline (typically 89 octane in North America).
    ///
    /// An intermediate octane level between regular and premium gasoline. Recommended for some
    /// vehicles that don't require premium but perform better with higher octane than regular.
    Midgrade = 4,

    /// Premium gasoline (typically 91+ octane in North America).
    ///
    /// High-octane gasoline required by many luxury and high-performance vehicles. Helps prevent
    /// engine knock in high-compression engines and may improve performance in vehicles designed
    /// for it.
    Premium = 5,

    /// 91 octane gasoline (primarily used in Australia and some other regions).
    ///
    /// Standard unleaded petrol grade common in Australia and other markets that use the Research
    /// Octane Number (RON) rating system.
    Sp91 = 6,

    /// 91 octane gasoline with 10% ethanol blend.
    ///
    /// 91 octane fuel blended with ethanol, common in regions promoting renewable fuel sources and
    /// reduced emissions through biofuel integration.
    Sp91E10 = 7,

    /// 92 octane gasoline.
    ///
    /// Mid-grade fuel option available in some markets, particularly those using RON rating
    /// systems. Falls between standard and premium grades.
    Sp92 = 8,

    /// 95 octane gasoline (common in Europe and other RON-based markets).
    ///
    /// Standard premium unleaded petrol in many European and international markets, roughly
    /// equivalent to premium gasoline in North America.
    Sp95 = 9,

    /// 95 octane gasoline with 10% ethanol blend.
    ///
    /// Premium gasoline with ethanol blend, combining higher octane rating with renewable fuel
    /// content for environmental and performance benefits.
    Sp95E10 = 10,

    /// 98 octane gasoline (super premium in RON-based markets).
    ///
    /// High-performance fuel for sports cars and high-end vehicles requiring maximum octane rating.
    /// Common in European and other international markets.
    Sp98 = 11,

    /// 99 octane gasoline (ultra-premium fuel).
    ///
    /// Very high octane fuel for racing or high-performance applications. Less commonly available
    /// and typically the most expensive gasoline option.
    Sp99 = 12,

    /// 100 octane gasoline (racing fuel).
    ///
    /// Ultra-high octane fuel primarily for racing or specialized high-performance applications.
    /// Rarely available at standard gas stations.
    Sp100 = 13,

    /// Liquefied Petroleum Gas (LPG/Autogas).
    ///
    /// Alternative fuel stored as liquid under pressure, used by vehicles converted to run on LPG.
    /// Common in some regions as an economical and cleaner-burning alternative to gasoline.
    Lpg = 14,

    /// E80 ethanol blend (80% ethanol, 20% gasoline).
    ///
    /// High-ethanol fuel blend for flex-fuel vehicles or vehicles specifically designed to run on
    /// high ethanol content. Less common than E85.
    E80 = 15,

    /// E85 ethanol blend (85% ethanol, 15% gasoline).
    ///
    /// High-ethanol fuel for flex-fuel vehicles, offering renewable fuel benefits and often lower
    /// cost per gallon than regular gasoline.
    E85 = 16,

    /// E100 pure ethanol fuel.
    ///
    /// Nearly pure ethanol fuel used primarily in Brazil and other markets with vehicles designed
    /// specifically for ethanol operation.
    E100 = 17,

    /// Compressed Natural Gas (CNG).
    ///
    /// Alternative fuel stored under high pressure, used by vehicles converted to natural gas
    /// operation. Offers environmental benefits and cost savings in areas with abundant natural
    /// gas.
    Methane = 18,

    /// Biodiesel or diesel made from renewable sources.
    ///
    /// Diesel fuel produced from vegetable oils, animal fats, or other renewable sources. Can often
    //// be used in regular diesel engines with little or no modification.
    BioDiesel = 19,

    /// Diesel fuel specifically formulated for commercial trucks.
    ///
    /// Specialized diesel for heavy-duty vehicles and commercial trucking. May have different
    /// additives or specifications compared to standard diesel.
    TruckDiesel = 20,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl FuelType {
    /// Returns whether this is a diesel-based fuel type.
    ///
    /// Used to categorize fuel types for diesel vehicles and filter stations that serve
    /// diesel-powered vehicles including trucks and passenger cars.
    #[must_use]
    pub const fn is_diesel(self) -> bool {
        matches!(self, Self::Diesel | Self::DieselPlus | Self::BioDiesel | Self::TruckDiesel)
    }

    /// Returns whether this is a gasoline-based fuel type.
    ///
    /// Used to identify traditional gasoline fuels for gasoline-powered vehicles, excluding
    /// alternative fuels and ethanol blends above standard levels.
    #[must_use]
    pub const fn is_gasoline(self) -> bool {
        matches!(
            self,
            Self::RegularUnleaded 
            | Self::Midgrade 
            | Self::Premium 
            | Self::Sp91 
            | Self::Sp92 
            | Self::Sp95 
            | Self::Sp98 
            | Self::Sp99 
            | Self::Sp100
        )
    }

    /// Returns whether this is an ethanol blend or pure ethanol fuel.
    ///
    /// Used to identify renewable fuel options and fuels suitable for flex-fuel vehicles or
    /// ethanol-compatible engines.
    #[must_use]
    pub const fn is_ethanol_blend(self) -> bool {
        matches!(
            self,
            Self::Sp91E10 | Self::Sp95E10 | Self::E80 | Self::E85 | Self::E100
        )
    }

    /// Returns whether this is an alternative fuel (non-traditional petroleum).
    ///
    /// Used to identify eco-friendly or alternative energy options including natural gas, LPG,
    /// high-ethanol blends, and biodiesel.
    #[must_use]
    pub const fn is_alternative_fuel(self) -> bool {
        matches!(
            self,
            Self::Lpg | Self::E80 | Self::E85 | Self::E100 | Self::Methane | Self::BioDiesel
        )
    }

    /// Returns whether this fuel requires specialized vehicle compatibility.
    ///
    /// Used to identify fuels that require specific vehicle modifications or compatibility, helping
    ///users avoid incompatible fuel types.
    #[must_use]
    pub const fn requires_special_compatibility(self) -> bool {
        matches!(
            self,
            Self::Lpg | Self::E80 | Self::E85 | Self::E100 | Self::Methane
        )
    }

    /// Returns the approximate octane rating for gasoline types.
    ///
    /// Provides octane numbers for comparison and vehicle compatibility checking. Returns `None`
    /// for non-gasoline fuels or when octane rating doesn't apply.
    #[must_use]
    pub const fn octane_rating(self) -> Option<u8> {
        match self {
            Self::RegularUnleaded => Some(87),
            Self::Midgrade => Some(89),
            Self::Premium | Self::Sp91 | Self::Sp91E10 => Some(91),
            Self::Sp92 => Some(92),
            Self::Sp95 | Self::Sp95E10 => Some(95),
            Self::Sp98 => Some(98),
            Self::Sp99 => Some(99),
            Self::Sp100 => Some(100),
            _ => None,
        }
    }

    /// Returns a human-readable display name for the fuel type.
    ///
    /// Provides user-friendly names suitable for displaying in applications, maps, and user
    /// interfaces. Uses common terminology that drivers recognize.
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Unspecified => "Unknown fuel",
            Self::Diesel => "Diesel",
            Self::DieselPlus => "Diesel Plus",
            Self::RegularUnleaded => "Regular",
            Self::Midgrade => "Midgrade",
            Self::Premium => "Premium",
            Self::Sp91 => "91 Octane",
            Self::Sp91E10 => "91 E10",
            Self::Sp92 => "92 Octane",
            Self::Sp95 => "95 Octane",
            Self::Sp95E10 => "95 E10",
            Self::Sp98 => "98 Octane",
            Self::Sp99 => "99 Octane",
            Self::Sp100 => "100 Octane",
            Self::Lpg => "LPG",
            Self::E80 => "E80 Ethanol",
            Self::E85 => "E85 Ethanol",
            Self::E100 => "E100 Ethanol",
            Self::Methane => "CNG",
            Self::BioDiesel => "Biodiesel",
            Self::TruckDiesel => "Truck Diesel",
        }
    }

    /// Returns a short abbreviation for compact displays.
    ///
    /// Provides brief identifiers suitable for maps, mobile interfaces, or anywhere space is
    /// limited but fuel identification is needed.
    #[must_use]
    pub const fn abbreviation(self) -> &'static str {
        match self {
            Self::Unspecified => "?",
            Self::Diesel => "D",
            Self::DieselPlus => "D+",
            Self::RegularUnleaded => "87",
            Self::Midgrade => "89",
            Self::Premium => "91+",
            Self::Sp91 => "91",
            Self::Sp91E10 => "91E10",
            Self::Sp92 => "92",
            Self::Sp95 => "95",
            Self::Sp95E10 => "95E10",
            Self::Sp98 => "98",
            Self::Sp99 => "99",
            Self::Sp100 => "100",
            Self::Lpg => "LPG",
            Self::E80 => "E80",
            Self::E85 => "E85",
            Self::E100 => "E100",
            Self::Methane => "CNG",
            Self::BioDiesel => "BD",
            Self::TruckDiesel => "TD",
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
        let fuel = FuelType::RegularUnleaded;
        let json = serde_json::to_string(&fuel).unwrap();
        assert_eq!(json, r#""REGULAR_UNLEADED""#);

        let fuel = FuelType::Sp95E10;
        let json = serde_json::to_string(&fuel).unwrap();
        assert_eq!(json, r#""SP95_E10""#);
    }

    #[test]
    fn test_deserialization() {
        let json = r#""DIESEL_PLUS""#;
        let fuel: FuelType = serde_json::from_str(json).unwrap();
        assert_eq!(fuel, FuelType::DieselPlus);

        let json = r#""FUEL_TYPE_UNSPECIFIED""#;
        let fuel: FuelType = serde_json::from_str(json).unwrap();
        assert_eq!(fuel, FuelType::Unspecified);
    }

    #[test]
    fn test_default() {
        let default_fuel = FuelType::default();
        assert_eq!(default_fuel, FuelType::Unspecified);
    }

    #[test]
    fn test_diesel_classification() {
        assert!(FuelType::Diesel.is_diesel());
        assert!(FuelType::DieselPlus.is_diesel());
        assert!(FuelType::BioDiesel.is_diesel());
        assert!(FuelType::TruckDiesel.is_diesel());
        assert!(!FuelType::RegularUnleaded.is_diesel());
        assert!(!FuelType::E85.is_diesel());
    }

    #[test]
    fn test_gasoline_classification() {
        assert!(FuelType::RegularUnleaded.is_gasoline());
        assert!(FuelType::Premium.is_gasoline());
        assert!(FuelType::Sp95.is_gasoline());
        assert!(!FuelType::Sp95E10.is_gasoline()); // Ethanol blend, not pure gasoline
        assert!(!FuelType::Diesel.is_gasoline());
        assert!(!FuelType::E85.is_gasoline());
    }

    #[test]
    fn test_ethanol_blend_classification() {
        assert!(FuelType::Sp91E10.is_ethanol_blend());
        assert!(FuelType::Sp95E10.is_ethanol_blend());
        assert!(FuelType::E85.is_ethanol_blend());
        assert!(FuelType::E100.is_ethanol_blend());
        assert!(!FuelType::RegularUnleaded.is_ethanol_blend());
        assert!(!FuelType::Diesel.is_ethanol_blend());
    }

    #[test]
    fn test_alternative_fuel_classification() {
        assert!(FuelType::Lpg.is_alternative_fuel());
        assert!(FuelType::Methane.is_alternative_fuel());
        assert!(FuelType::E85.is_alternative_fuel());
        assert!(FuelType::BioDiesel.is_alternative_fuel());
        assert!(!FuelType::RegularUnleaded.is_alternative_fuel());
        assert!(!FuelType::Diesel.is_alternative_fuel());
    }

    #[test]
    fn test_special_compatibility() {
        assert!(FuelType::E85.requires_special_compatibility());
        assert!(FuelType::Lpg.requires_special_compatibility());
        assert!(FuelType::Methane.requires_special_compatibility());
        assert!(!FuelType::RegularUnleaded.requires_special_compatibility());
        assert!(!FuelType::Diesel.requires_special_compatibility());
    }

    #[test]
    fn test_octane_ratings() {
        assert_eq!(FuelType::RegularUnleaded.octane_rating(), Some(87));
        assert_eq!(FuelType::Midgrade.octane_rating(), Some(89));
        assert_eq!(FuelType::Premium.octane_rating(), Some(91));
        assert_eq!(FuelType::Sp95.octane_rating(), Some(95));
        assert_eq!(FuelType::Sp98.octane_rating(), Some(98));
        assert_eq!(FuelType::Diesel.octane_rating(), None);
        assert_eq!(FuelType::E85.octane_rating(), None);
    }

    #[test]
    fn test_display_names() {
        assert_eq!(FuelType::RegularUnleaded.display_name(), "Regular");
        assert_eq!(FuelType::Premium.display_name(), "Premium");
        assert_eq!(FuelType::E85.display_name(), "E85 Ethanol");
        assert_eq!(FuelType::Methane.display_name(), "CNG");
        assert_eq!(FuelType::Unspecified.display_name(), "Unknown fuel");
    }

    #[test]
    fn test_abbreviations() {
        assert_eq!(FuelType::RegularUnleaded.abbreviation(), "87");
        assert_eq!(FuelType::Premium.abbreviation(), "91+");
        assert_eq!(FuelType::Sp95E10.abbreviation(), "95E10");
        assert_eq!(FuelType::E85.abbreviation(), "E85");
        assert_eq!(FuelType::Diesel.abbreviation(), "D");
        assert_eq!(FuelType::Unspecified.abbreviation(), "?");
    }
}