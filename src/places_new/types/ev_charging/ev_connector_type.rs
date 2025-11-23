use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

// -------------------------------------------------------------------------------------------------
//
/// Represents different types of electric vehicle charging connectors.
///
/// EV connector types define the physical interface between charging stations and electric
/// vehicles. Different regions, manufacturers, and charging standards use various connector types,
/// each with specific voltage, current, and communication capabilities. Understanding connector
/// compatibility is essential for EV drivers to find compatible charging stations.
///
/// See <http://ieeexplore.ieee.org/stamp/stamp.jsp?arnumber=6872107> for additional
/// information/context on EV charging connector types.
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
#[allow(clippy::doc_markdown)]
pub enum EvConnectorType {
    /// Unspecified connector type.
    ///
    /// Used when the connector type cannot be determined or is not provided in the charging station
    /// data. Applications should handle this as unknown connector information.
    #[default]
    #[serde(rename = "EV_CONNECTOR_TYPE_UNSPECIFIED")]
    Unspecified = 0,

    /// Other or non-standard connector types.
    ///
    /// Used for proprietary, legacy, or uncommon connector types that don't fit into the standard
    /// categories. May include manufacturer-specific or regional connectors not covered by major
    /// standards.
    #[serde(rename = "EV_CONNECTOR_TYPE_OTHER")]
    Other = 1,

    /// J1772 Type 1 connector (SAE J1772).
    ///
    /// Standard AC charging connector used primarily in North America and Japan. Supports Level 1
    /// (120V) and Level 2 (240V) AC charging. Common on most electric vehicles and public charging
    /// stations in these regions.
    #[serde(rename = "EV_CONNECTOR_TYPE_J1772")]
    J1772 = 2,

    /// IEC 62196 Type 2 connector (Mennekes).
    ///
    /// European standard AC charging connector, widely used across Europe. Supports single-phase
    /// and three-phase AC charging with higher power capabilities than Type 1. Standard for most
    /// European EV charging infrastructure.
    #[serde(rename = "EV_CONNECTOR_TYPE_TYPE_2")]
    Type2 = 3,

    /// CHAdeMO DC fast charging connector.
    ///
    /// Japanese-developed DC fast charging standard, commonly found on Nissan, Mitsubishi, and
    /// other Japanese EVs. Provides high-power DC charging but is being phased out in favor of CCS
    /// in many markets.
    #[serde(rename = "EV_CONNECTOR_TYPE_CHADEMO")]
    Chademo = 4,

    /// Combined Charging System (CCS) Combo 1.
    ///
    /// North American DC fast charging standard that combines J1772 AC connector with additional DC
    /// pins. Supports both AC and DC charging through the same port, used by most non-Tesla EVs in
    /// North America.
    #[serde(rename = "EV_CONNECTOR_TYPE_CCS_COMBO_1")]
    CcsCombo1 = 5,

    /// Combined Charging System (CCS) Combo 2.
    ///
    /// European DC fast charging standard that combines Type 2 AC connector with additional DC
    /// pins. Dominant fast charging standard in Europe and increasingly adopted globally outside
    /// North America.
    #[serde(rename = "EV_CONNECTOR_TYPE_CCS_COMBO_2")]
    CcsCombo2 = 6,

    /// Tesla connector (proprietary).
    ///
    /// Tesla's proprietary connector system used on Tesla vehicles and Supercharger network. In
    /// North America, this is the NACS standard, while other regions may use different
    /// implementations. Tesla vehicles often require adapters for non-Tesla charging stations.
    #[serde(rename = "EV_CONNECTOR_TYPE_TESLA")]
    Tesla = 7,

    /// GB/T connector (Chinese standard).
    ///
    /// Chinese national standard for EV charging, used throughout China for both AC and DC
    /// charging. Required for electric vehicles sold in the Chinese market and incompatible with
    /// other global standards without adapters.
    #[serde(rename = "EV_CONNECTOR_TYPE_UNSPECIFIED_GB_T")]
    UnspecifiedGbT = 8,

    /// Unspecified wall outlet (standard electrical outlet).
    ///
    /// Generic electrical outlet used for Level 1 charging with portable charging equipment.
    /// Typically household outlets (110V-240V) that require the vehicle's onboard charging cable
    /// and adapter.
    #[serde(rename = "EV_CONNECTOR_TYPE_UNSPECIFIED_WALL_OUTLET")]
    UnspecifiedWallOutlet = 9,

    /// North American Charging Standard (NACS).
    ///
    /// Tesla's connector design adopted as the North American Charging Standard (SAE J3400).
    /// Increasingly being adopted by other manufacturers and charging networks in North America as
    /// the unified standard.
    #[serde(rename = "EV_CONNECTOR_TYPE_NACS")]
    Nacs = 10,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl EvConnectorType {
    /// Returns whether this is a DC fast charging connector.
    ///
    /// Used to identify high-speed charging options that can rapidly charge EV batteries, typically
    /// providing 50kW+ charging power for quick stops.
    #[must_use]
    pub const fn is_dc_fast_charging(self) -> bool {
        matches!(
            self,
            Self::Chademo
            | Self::CcsCombo1
            | Self::CcsCombo2
        )
    }

    /// Returns whether this is an AC charging connector.
    ///
    /// Used to identify slower, steady charging options typically used for overnight charging at
    /// home or workplace, or extended parking sessions.
    #[must_use]
    pub const fn is_ac_charging(self) -> bool {
        matches!(
            self,
            Self::J1772
            | Self::Type2
            | Self::UnspecifiedWallOutlet
        )
    }

    /// Returns whether this connector supports both AC and DC charging.
    ///
    /// Used to identify versatile connectors that can handle both slow AC charging and fast DC
    /// charging through the same physical connection.
    #[must_use]
    pub const fn supports_both_ac_and_dc(self) -> bool {
        matches!(
            self,
            Self::CcsCombo1
            | Self::CcsCombo2
            | Self::Nacs
        )
    }

    /// Returns whether this is a regional or proprietary standard.
    ///
    /// Used to identify connectors that may have limited compatibility or require specific
    /// adapters, helping users understand potential limitations.
    #[must_use]
    pub const fn is_regional_or_proprietary(self) -> bool {
        matches!(
            self,
            Self::Tesla
            | Self::UnspecifiedGbT
            | Self::Nacs
            | Self::Other
        )
    }

    /// Returns the primary geographical region where this connector is standard.
    ///
    /// Provides information about where drivers are most likely to find this connector type, useful
    /// for travel planning and compatibility checking.
    #[must_use]
    pub const fn primary_region(self) -> Option<&'static str> {
        match self {
            Self::J1772 => Some("North America & Japan"),
            Self::Type2 => Some("Europe"),
            Self::Chademo => Some("Japan & Legacy Global"),
            Self::CcsCombo1 | Self::Nacs => Some("North America"),
            Self::CcsCombo2 => Some("Europe & Global"),
            Self::Tesla => Some("Global (Tesla Network)"),
            Self::UnspecifiedGbT => Some("China"),
            Self::UnspecifiedWallOutlet => Some("Global"),
            Self::Unspecified | Self::Other => None,
        }
    }

    /// Returns typical power levels supported by this connector type.
    ///
    /// Provides guidance on expected charging speeds, though actual power depends on the specific
    /// charging station and vehicle capabilities.
    #[must_use]
    pub const fn typical_power_range(self) -> Option<&'static str> {
        match self {
            Self::J1772 => Some("1.4-19.2 kW AC"),
            Self::Type2 => Some("3.7-43 kW AC"),
            Self::Chademo => Some("50-400 kW DC"),
            Self::CcsCombo1 => Some("50-350 kW DC, 1.4-19.2 kW AC"),
            Self::CcsCombo2 => Some("50-350 kW DC, 3.7-43 kW AC"),
            Self::Tesla => Some("1.4-250 kW (varies by generation)"),
            Self::UnspecifiedGbT => Some("3.3-237 kW (AC/DC)"),
            Self::Nacs => Some("1.4-250 kW (AC/DC)"),
            Self::UnspecifiedWallOutlet => Some("1.4-3.7 kW AC"),
            Self::Unspecified | Self::Other => None,
        }
    }

    /// Returns a user-friendly display name for the connector type.
    ///
    /// Provides names that EV drivers recognize and understand, suitable for display in charging
    /// station finders, mobile apps, and navigation systems.
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Unspecified => "Unknown Connector",
            Self::Other => "Other Connector",
            Self::J1772 => "J1772",
            Self::Type2 => "Type 2 (Mennekes)",
            Self::Chademo => "CHAdeMO",
            Self::CcsCombo1 => "CCS Combo 1",
            Self::CcsCombo2 => "CCS Combo 2",
            Self::Tesla => "Tesla Connector",
            Self::UnspecifiedGbT => "GB/T",
            Self::UnspecifiedWallOutlet => "Wall Outlet",
            Self::Nacs => "NACS",
        }
    }

    /// Returns a short abbreviation for compact displays.
    ///
    /// Provides brief identifiers suitable for mobile interfaces, map pins, or anywhere space is
    /// limited but connector identification is needed.
    #[must_use]
    pub const fn abbreviation(self) -> &'static str {
        match self {
            Self::Unspecified => "?",
            Self::Other => "Other",
            Self::J1772 => "J1772",
            Self::Type2 => "Type2",
            Self::Chademo => "CHAdeMO",
            Self::CcsCombo1 => "CCS1",
            Self::CcsCombo2 => "CCS2",
            Self::Tesla => "Tesla",
            Self::UnspecifiedGbT => "GB/T",
            Self::UnspecifiedWallOutlet => "Outlet",
            Self::Nacs => "NACS",
        }
    }

    /// Returns whether this connector is likely to be compatible with most EVs in a region.
    ///
    /// Used to identify "universal" connectors that work with the majority of electric vehicles in
    /// their primary market, helping prioritize charging options.
    #[must_use]
    pub const fn is_widely_compatible(self) -> bool {
        matches!(
            self,
            Self::J1772
            | Self::Type2
            | Self::CcsCombo1
            | Self::CcsCombo2
            | Self::Nacs
        )
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
        let connector = EvConnectorType::CcsCombo2;
        let json = serde_json::to_string(&connector).unwrap();
        assert_eq!(json, r#""EV_CONNECTOR_TYPE_CCS_COMBO_2""#);

        let connector = EvConnectorType::Nacs;
        let json = serde_json::to_string(&connector).unwrap();
        assert_eq!(json, r#""EV_CONNECTOR_TYPE_NACS""#);
    }

    #[test]
    fn test_deserialization() {
        let json = r#""EV_CONNECTOR_TYPE_J1772""#;
        let connector: EvConnectorType = serde_json::from_str(json).unwrap();
        assert_eq!(connector, EvConnectorType::J1772);

        let json = r#""EV_CONNECTOR_TYPE_UNSPECIFIED""#;
        let connector: EvConnectorType = serde_json::from_str(json).unwrap();
        assert_eq!(connector, EvConnectorType::Unspecified);
    }

    #[test]
    fn test_default() {
        let default_connector = EvConnectorType::default();
        assert_eq!(default_connector, EvConnectorType::Unspecified);
    }

    #[test]
    fn test_dc_fast_charging() {
        assert!(EvConnectorType::Chademo.is_dc_fast_charging());
        assert!(EvConnectorType::CcsCombo1.is_dc_fast_charging());
        assert!(EvConnectorType::CcsCombo2.is_dc_fast_charging());
        assert!(!EvConnectorType::J1772.is_dc_fast_charging());
        assert!(!EvConnectorType::Type2.is_dc_fast_charging());
    }

    #[test]
    fn test_ac_charging() {
        assert!(EvConnectorType::J1772.is_ac_charging());
        assert!(EvConnectorType::Type2.is_ac_charging());
        assert!(EvConnectorType::UnspecifiedWallOutlet.is_ac_charging());
        assert!(!EvConnectorType::Chademo.is_ac_charging());
    }

    #[test]
    fn test_both_ac_and_dc() {
        assert!(EvConnectorType::CcsCombo1.supports_both_ac_and_dc());
        assert!(EvConnectorType::CcsCombo2.supports_both_ac_and_dc());
        assert!(EvConnectorType::Nacs.supports_both_ac_and_dc());
        assert!(!EvConnectorType::J1772.supports_both_ac_and_dc());
        assert!(!EvConnectorType::Chademo.supports_both_ac_and_dc());
    }

    #[test]
    fn test_regional_or_proprietary() {
        assert!(EvConnectorType::Tesla.is_regional_or_proprietary());
        assert!(EvConnectorType::UnspecifiedGbT.is_regional_or_proprietary());
        assert!(EvConnectorType::Nacs.is_regional_or_proprietary());
        assert!(!EvConnectorType::J1772.is_regional_or_proprietary());
        assert!(!EvConnectorType::CcsCombo2.is_regional_or_proprietary());
    }

    #[test]
    fn test_primary_regions() {
        assert_eq!(
            EvConnectorType::J1772.primary_region(),
            Some("North America & Japan")
        );
        assert_eq!(
            EvConnectorType::Type2.primary_region(),
            Some("Europe")
        );
        assert_eq!(
            EvConnectorType::UnspecifiedGbT.primary_region(),
            Some("China")
        );
        assert_eq!(
            EvConnectorType::Unspecified.primary_region(),
            None
        );
    }

    #[test]
    fn test_typical_power_ranges() {
        assert!(EvConnectorType::J1772.typical_power_range().unwrap().contains("kW"));
        assert!(EvConnectorType::CcsCombo2.typical_power_range().unwrap().contains("DC"));
        assert_eq!(EvConnectorType::Unspecified.typical_power_range(), None);
    }

    #[test]
    fn test_display_names() {
        assert_eq!(EvConnectorType::J1772.display_name(), "J1772");
        assert_eq!(EvConnectorType::Type2.display_name(), "Type 2 (Mennekes)");
        assert_eq!(EvConnectorType::CcsCombo2.display_name(), "CCS Combo 2");
        assert_eq!(EvConnectorType::Nacs.display_name(), "NACS");
    }

    #[test]
    fn test_abbreviations() {
        assert_eq!(EvConnectorType::J1772.abbreviation(), "J1772");
        assert_eq!(EvConnectorType::CcsCombo1.abbreviation(), "CCS1");
        assert_eq!(EvConnectorType::CcsCombo2.abbreviation(), "CCS2");
        assert_eq!(EvConnectorType::Unspecified.abbreviation(), "?");
    }

    #[test]
    fn test_widely_compatible() {
        assert!(EvConnectorType::J1772.is_widely_compatible());
        assert!(EvConnectorType::CcsCombo2.is_widely_compatible());
        assert!(EvConnectorType::Nacs.is_widely_compatible());
        assert!(!EvConnectorType::Tesla.is_widely_compatible());
        assert!(!EvConnectorType::UnspecifiedGbT.is_widely_compatible());
    }
}