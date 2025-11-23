#![allow(clippy::doc_markdown)]

use crate::places_new::EvConnectorType;
use jiff::Timestamp;

// -------------------------------------------------------------------------------------------------
//
/// EV charging information grouped by connector type and maximum charge rate.
///
/// Connector aggregations group charging connectors that have the same type and maximum
/// charge rate, providing efficient representation of charging station capabilities.
/// This helps EV drivers quickly understand available charging options, speeds, and
/// real-time availability without needing to process individual connector details.
#[derive(
    //std
    Clone,
    Debug,
    // getset
    getset::Getters,
    getset::CopyGetters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorAggregation {
    /// The connector type of this aggregation.
    ///
    /// Specifies the physical connector standard (CCS, CHAdeMO, Tesla, etc.) that all connectors in
    /// this aggregation share. This determines vehicle compatibility and charging protocol used.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub connector_type: Option<EvConnectorType>,

    /// The static maximum charging rate in kW of each connector in the aggregation.
    ///
    /// Maximum power output that each connector in this group can deliver under optimal conditions.
    /// Actual charging speeds may be lower due to vehicle limitations, battery state, or station
    /// load balancing.
    #[getset(get_copy = "pub")]
    pub max_charge_rate_kw: f64,

    /// Number of connectors in this aggregation.
    ///
    /// Total count of physical connectors that share the same type and maximum charge rate. More
    /// connectors generally mean better availability and reduced wait times for charging.
    #[getset(get_copy = "pub")]
    pub count: i32,

    /// The timestamp when connector availability information was last updated.
    ///
    /// Indicates when the availability data was last refreshed. More recent timestamps provide more
    /// reliable information about current connector status and availability.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub availability_last_update_time: Timestamp,

    /// Number of connectors in this aggregation that are currently available.
    ///
    /// Count of connectors that are operational and ready for immediate use. This number should be
    /// less than or equal to the total count and helps users understand real-time charging
    /// availability.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub available_count: Option<i32>,

    /// Number of connectors in this aggregation that are currently out of service.
    ///
    /// Count of connectors that are not operational due to maintenance, faults, or other issues.
    /// These connectors cannot be used for charging and reduce total station capacity.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub out_of_service_count: Option<i32>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl ConnectorAggregation {
    /// Creates a new `ConnectorAggregation` with the specified type, rate, and count.
    ///
    /// Used to construct connector aggregation with essential charging information. Availability
    /// data can be added separately as it may not always be available.
    #[must_use]
    pub fn new(
        connector_type: Option<EvConnectorType>,
        max_charge_rate_kw: f64,
        count: i32,
    ) -> Self {
        Self {
            connector_type,
            max_charge_rate_kw,
            count,
            availability_last_update_time: Timestamp::default(),
            available_count: None,
            out_of_service_count: None,
        }
    }

    /// Creates a `ConnectorAggregation` with full availability information.
    ///
    /// Used when complete real-time availability data is available, providing comprehensive
    /// information about connector status and usability.
    #[must_use]
    pub const fn with_availability(
        connector_type: Option<EvConnectorType>,
        max_charge_rate_kw: f64,
        count: i32,
        available_count: i32,
        out_of_service_count: i32,
        update_time: Timestamp,
    ) -> Self {
        Self {
            connector_type,
            max_charge_rate_kw,
            count,
            availability_last_update_time: update_time,
            available_count: Some(available_count),
            out_of_service_count: Some(out_of_service_count),
        }
    }

    /// Returns whether availability information is provided.
    ///
    /// Used to determine if real-time availability data is available for this connector group,
    /// helping applications decide how to present charging options.
    #[must_use]
    pub const fn has_availability_info(&self) -> bool {
        self.available_count.is_some() || self.out_of_service_count.is_some()
    }

    /// Returns whether any connectors are currently available.
    ///
    /// Used to determine if immediate charging is possible with this connector type, helping users
    /// identify stations with immediate availability.
    #[must_use]
    pub const fn has_available_connectors(&self) -> Option<bool> {
        match self.available_count {
            Some(count) => Some(count > 0),
            None => None,
        }
    }

    /// Returns whether all connectors are currently in use or unavailable.
    ///
    /// Used to identify connector groups with no immediate availability, helping users understand
    /// potential wait times or need to find alternatives.
    #[must_use]
    pub const fn is_fully_occupied(&self) -> Option<bool> {
        match self.available_count {
            Some(count) => Some(count == 0),
            None => None,
        }
    }

    /// Returns the number of connectors currently in use.
    ///
    /// Calculates how many connectors are occupied by subtracting available and out-of-service
    /// connectors from the total count.
    #[must_use]
    pub const fn in_use_count(&self) -> Option<i32> {
        match (self.available_count, self.out_of_service_count) {
            (Some(available), Some(out_of_service)) => {
                Some(self.count - available - out_of_service)
            }
            (Some(available), None) => Some(self.count - available),
            _ => None,
        }
    }

    /// Returns the utilization rate as a percentage.
    ///
    /// Calculates what percentage of total connectors are currently in use, useful for
    /// understanding station demand and congestion levels.
    #[must_use]
    pub fn utilization_rate(&self) -> Option<f64> {
        let in_use = self.in_use_count()?;
        if self.count > 0 {
            Some((f64::from(in_use) / f64::from(self.count)) * 100.0)
        } else {
            None
        }
    }

    /// Returns whether this is a fast charging option.
    ///
    /// Used to identify high-speed charging connectors (typically 50kW+) that can provide rapid
    /// charging for quick stops and travel needs.
    #[must_use]
    pub const fn is_fast_charging(&self) -> bool {
        self.max_charge_rate_kw >= 50.0
    }

    /// Returns whether this is a rapid charging option.
    ///
    /// Used to identify ultra-fast charging connectors (typically 150kW+) that provide the fastest
    /// available charging speeds for modern EVs.
    #[must_use]
    pub const fn is_rapid_charging(&self) -> bool {
        self.max_charge_rate_kw >= 150.0
    }

    /// Returns whether this supports AC charging.
    ///
    /// Returns `None` if connector type information is not available. Used to identify slower,
    /// steady charging options typically used for extended parking sessions like work, shopping,
    /// or overnight charging.
    #[must_use]
    pub fn is_ac_charging(&self) -> Option<bool> {
        self.connector_type.map(EvConnectorType::is_ac_charging)
    }

    /// Returns whether this supports DC fast charging.
    ///
    /// Returns `None` if connector type information is not available. Used to identify high-speed
    /// DC charging options for quick stops and travel charging where speed is prioritized over
    /// extended duration.
    #[must_use]
    pub fn is_dc_charging(&self) -> Option<bool> {
        self.connector_type.map(EvConnectorType::is_dc_fast_charging)
    }

    /// Gets a user-friendly description of the charging speed category.
    ///
    /// Provides descriptive text about charging speed suitable for display in user interfaces and
    /// charging station information.
    #[must_use]
    pub const fn speed_category(&self) -> &'static str {
        if self.is_rapid_charging() {
            "Rapid Charging"
        } else if self.is_fast_charging() {
            "Fast Charging"
        } else if self.max_charge_rate_kw >= 22.0 {
            "Fast AC"
        } else if self.max_charge_rate_kw >= 7.0 {
            "Standard AC"
        } else {
            "Slow AC"
        }
    }

    /// Gets the estimated charging time for a typical EV battery.
    ///
    /// Provides rough time estimates for charging from 10% to 80% battery capacity, which
    /// represents the most efficient charging range for most EVs.
    #[must_use]
    pub fn estimated_charging_time(&self, battery_size_kwh: f64) -> String {
        // Rough estimate for 10% to 80% charging (70% of battery capacity)
        let charging_needed = battery_size_kwh * 0.7;
        let effective_rate = self.max_charge_rate_kw.min(battery_size_kwh); // Vehicle may limit rate
        let hours = charging_needed / effective_rate;

        if hours < 1.0 {
            format!("{:.0} minutes", hours * 60.0)
        } else if hours < 2.0 {
            format!("{hours:.1} hours")
        } else {
            format!("{hours:.0} hours")
        }
    }

    /// Returns the age of availability information.
    ///
    /// Calculates how long ago the availability data was last updated, useful for determining data
    /// freshness and reliability.
    #[must_use]
    pub fn availability_age(&self) -> jiff::Span {
        Timestamp::now()
            .since(self.availability_last_update_time)
            .unwrap_or_default()
    }

    /// Returns whether availability information is fresh.
    ///
    /// Used to determine if availability data is recent enough to be reliable for real-time
    /// charging decisions and station selection.
    #[must_use]
    pub fn has_fresh_availability(&self) -> bool {
        self.availability_age().total(jiff::Unit::Minute).unwrap_or(0.0) <= 15.0
    }

    /// Gets a compact description suitable for mobile displays.
    ///
    /// Provides brief connector information suitable for mobile interfaces, map overlays, or
    /// space-constrained displays. If connector type is unknown, displays generic charging info.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, reason = "for display, not for calculations")]
    pub fn compact_description(&self) -> String {
        let connector_label = self.connector_type
            .map_or("Unknown", EvConnectorType::abbreviation);

        format!(
            "{} {}kW ({})",
            connector_label,
            self.max_charge_rate_kw as i32,
            self.count
        )
    }

    /// Gets a detailed description including availability.
    ///
    /// Provides comprehensive connector information suitable for detailed station listings and
    /// charging planning interfaces. If connector type is unknown, shows generic description.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, reason = "for display, not for calculations")]
    pub fn detailed_description(&self) -> String {
        let connector_name = self.connector_type
            .map_or("Unknown Connector", EvConnectorType::display_name);

        let base = format!(
            "{} - {}kW ({} connector{})",
            connector_name,
            self.max_charge_rate_kw as i32,
            self.count,
            if self.count == 1 { "" } else { "s" }
        );

        match self.available_count {
            Some(available) => {
                if available == self.count {
                    format!("{base} - All available")
                } else if available == 0 {
                    format!("{base} - None available")
                } else {
                    format!("{base} - {available} available")
                }
            }
            None => base,
        }
    }

    /// Returns whether this connector type is compatible with a vehicle.
    ///
    /// Returns `None` if connector type information is not available. Used for vehicle-specific
    /// filtering to show only relevant charging options based on the user's EV connector
    /// compatibility.
    #[must_use]
    pub fn is_compatible_with(&self, vehicle_connector: EvConnectorType) -> Option<bool> {
        self.connector_type.map(|ct| ct == vehicle_connector)
    }

    /// Gets availability status for UI indicators.
    ///
    /// Provides status classification suitable for color coding, badges, or other visual indicators
    /// in charging station interfaces.
    #[must_use]
    pub const fn availability_status(&self) -> AvailabilityStatus {
        match self.available_count {
            Some(0) => AvailabilityStatus::Unavailable,
            Some(available) if available == self.count => AvailabilityStatus::FullyAvailable,
            Some(_) => AvailabilityStatus::PartiallyAvailable,
            None => AvailabilityStatus::Unknown,
        }
    }

    /// Returns an icon representation for the connector type.
    ///
    /// Provides visual indicators suitable for maps, mobile interfaces, and quick identification of
    /// charging connector types. Returns generic icon if type is unknown.
    #[must_use]
    pub fn connector_icon(&self) -> &'static str {
        self.connector_type.map_or("⚡", |ct| match ct {
            EvConnectorType::Chademo => "🔌",
            EvConnectorType::Tesla | EvConnectorType::Nacs => "🏎️",
            EvConnectorType::J1772 | EvConnectorType::Type2 => "🔋",
            _ => "⚡",
        })
    }
}

// -------------------------------------------------------------------------------------------------
//
/// Indicates the availability status of connectors in an aggregation.
///
/// Used for color coding, user interface indicators, and filtering to help users quickly understand
/// charging availability at stations.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum AvailabilityStatus {
    /// All connectors in the aggregation are available for use.
    FullyAvailable,
    /// Some but not all connectors are available.
    PartiallyAvailable,
    /// No connectors are currently available.
    Unavailable,
    /// Availability status is unknown or not provided.
    Unknown,
}

impl AvailabilityStatus {
    /// Returns a user-friendly description of the availability status.
    ///
    /// Provides text suitable for tooltips, status indicators, or detailed information displays
    /// about charging availability.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::FullyAvailable => "All connectors available",
            Self::PartiallyAvailable => "Some connectors available",
            Self::Unavailable => "No connectors available",
            Self::Unknown => "Availability unknown",
        }
    }

    /// Returns a color indicator for UI styling.
    ///
    /// Provides color codes suitable for CSS styling, status badges, or visual indicators in
    /// charging station interfaces.
    #[must_use]
    pub const fn color(self) -> &'static str {
        match self {
            Self::FullyAvailable => "green",
            Self::PartiallyAvailable => "orange",
            Self::Unavailable => "red",
            Self::Unknown => "gray",
        }
    }

    /// Returns an emoji indicator for compact displays.
    ///
    /// Provides visual availability indicators suitable for mobile interfaces or anywhere compact
    /// visual representation is needed.
    #[must_use]
    pub const fn emoji(self) -> &'static str {
        match self {
            Self::FullyAvailable => "🟢",
            Self::PartiallyAvailable => "🟡",
            Self::Unavailable => "🔴",
            Self::Unknown => "⚪",
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
        let aggregation = ConnectorAggregation {
            connector_type: Some(EvConnectorType::CcsCombo2),
            max_charge_rate_kw: 150.0,
            count: 4,
            availability_last_update_time: Timestamp::now(),
            available_count: Some(2),
            out_of_service_count: Some(1),
        };

        let json = serde_json::to_string(&aggregation).unwrap();
        assert!(json.contains("EV_CONNECTOR_TYPE_CCS_COMBO_2"));
        assert!(json.contains("150"));
    }

    #[test]
    #[allow(clippy::float_cmp, reason = "test only")]
    fn test_constructors() {
        let basic = ConnectorAggregation::new(
            Some(EvConnectorType::CcsCombo1),
            100.0,
            3,
        );
        assert_eq!(basic.connector_type, Some(EvConnectorType::CcsCombo1));
        assert_eq!(basic.max_charge_rate_kw, 100.0);
        assert_eq!(basic.count, 3);
        assert!(!basic.has_availability_info());

        let with_availability = ConnectorAggregation::with_availability(
            Some(EvConnectorType::CcsCombo2),
            150.0,
            4,
            2,
            1,
            Timestamp::now(),
        );
        assert!(with_availability.has_availability_info());
        assert_eq!(with_availability.available_count, Some(2));
        assert_eq!(with_availability.out_of_service_count, Some(1));
    }

    #[test]
    fn test_availability_status() {
        let fully_available = ConnectorAggregation::with_availability(
            Some(EvConnectorType::CcsCombo2),
            150.0,
            4,
            4,
            0,
            Timestamp::now(),
        );
        assert_eq!(fully_available.availability_status(), AvailabilityStatus::FullyAvailable);
        assert!(fully_available.has_available_connectors().unwrap());
        assert!(!fully_available.is_fully_occupied().unwrap());

        let unavailable = ConnectorAggregation::with_availability(
            Some(EvConnectorType::Chademo),
            50.0,
            2,
            0,
            0,
            Timestamp::now(),
        );
        assert_eq!(unavailable.availability_status(), AvailabilityStatus::Unavailable);
        assert!(!unavailable.has_available_connectors().unwrap());
        assert!(unavailable.is_fully_occupied().unwrap());

        let partial = ConnectorAggregation::with_availability(
            Some(EvConnectorType::J1772),
            22.0,
            3,
            1,
            1,
            Timestamp::now(),
        );
        assert_eq!(partial.availability_status(), AvailabilityStatus::PartiallyAvailable);
    }

    #[test]
    fn test_utilization_calculations() {
        let aggregation = ConnectorAggregation::with_availability(
            Some(EvConnectorType::CcsCombo2),
            150.0,
            4,
            1,
            1,
            Timestamp::now(),
        );

        assert_eq!(aggregation.in_use_count(), Some(2)); // 4 total - 1 available - 1 out of service
        let utilization = aggregation.utilization_rate().unwrap();
        assert!((utilization - 50.0).abs() < 0.01); // 2/4 = 50%
    }

    #[test]
    fn test_charging_speed_categories() {
        let slow_ac = ConnectorAggregation::new(
            Some(EvConnectorType::J1772),
            3.7,
            2,
        );
        assert_eq!(slow_ac.speed_category(), "Slow AC");
        assert!(!slow_ac.is_fast_charging());
        assert!(!slow_ac.is_rapid_charging());

        let fast_charging = ConnectorAggregation::new(
            Some(EvConnectorType::CcsCombo2),
            100.0,
            4,
        );
        assert_eq!(fast_charging.speed_category(), "Fast Charging");
        assert!(fast_charging.is_fast_charging());
        assert!(!fast_charging.is_rapid_charging());

        let rapid_charging = ConnectorAggregation::new(
            Some(EvConnectorType::CcsCombo2),
            250.0,
            2,
        );
        assert_eq!(rapid_charging.speed_category(), "Rapid Charging");
        assert!(rapid_charging.is_fast_charging());
        assert!(rapid_charging.is_rapid_charging());
    }

    #[test]
    fn test_charging_type_detection() {
        let ac_connector = ConnectorAggregation::new(
            Some(EvConnectorType::J1772),
            7.4,
            2,
        );
        assert_eq!(ac_connector.is_ac_charging(), Some(true));
        assert_eq!(ac_connector.is_dc_charging(), Some(false));

        let dc_connector = ConnectorAggregation::new(
            Some(EvConnectorType::CcsCombo2),
            150.0,
            4,
        );
        assert_eq!(dc_connector.is_ac_charging(), Some(false));
        assert_eq!(dc_connector.is_dc_charging(), Some(true));

        let unknown_connector = ConnectorAggregation::new(
            None,
            50.0,
            2,
        );
        assert_eq!(unknown_connector.is_ac_charging(), None);
        assert_eq!(unknown_connector.is_dc_charging(), None);
    }

    #[test]
    fn test_estimated_charging_time() {
        let fast_charger = ConnectorAggregation::new(
            Some(EvConnectorType::CcsCombo2),
            150.0,
            2,
        );

        let time_estimate = fast_charger.estimated_charging_time(60.0); // 60 kWh battery
        assert!(time_estimate.contains("minutes") || time_estimate.contains("hours"));

        let slow_charger = ConnectorAggregation::new(
            Some(EvConnectorType::J1772),
            7.4,
            1,
        );

        let slow_estimate = slow_charger.estimated_charging_time(60.0);
        assert!(slow_estimate.contains("hours"));
    }

    #[test]
    fn test_descriptions() {
        let aggregation = ConnectorAggregation::with_availability(
            Some(EvConnectorType::CcsCombo2),
            150.0,
            4,
            2,
            1,
            Timestamp::now(),
        );

        let compact = aggregation.compact_description();
        assert!(compact.contains("CCS2"));
        assert!(compact.contains("150kW"));
        assert!(compact.contains('4'));

        let detailed = aggregation.detailed_description();
        assert!(detailed.contains("CCS Combo 2"));
        assert!(detailed.contains("150kW"));
        assert!(detailed.contains("4 connectors"));
        assert!(detailed.contains("2 available"));
    }

    #[test]
    fn test_compatibility() {
        let ccs_aggregation = ConnectorAggregation::new(
            Some(EvConnectorType::CcsCombo2),
            150.0,
            4,
        );

        assert_eq!(ccs_aggregation.is_compatible_with(EvConnectorType::CcsCombo2), Some(true));
        assert_eq!(ccs_aggregation.is_compatible_with(EvConnectorType::Chademo), Some(false));

        let unknown_aggregation = ConnectorAggregation::new(None, 150.0, 4);
        assert_eq!(unknown_aggregation.is_compatible_with(EvConnectorType::CcsCombo2), None);
    }

    #[test]
    fn test_availability_status_methods() {
        assert_eq!(AvailabilityStatus::FullyAvailable.description(), "All connectors available");
        assert_eq!(AvailabilityStatus::Unavailable.color(), "red");
        assert_eq!(AvailabilityStatus::PartiallyAvailable.emoji(), "🟡");
    }

    #[test]
    fn test_connector_icons() {
        let ccs = ConnectorAggregation::new(Some(EvConnectorType::CcsCombo2), 150.0, 2);
        assert_eq!(ccs.connector_icon(), "⚡");

        let tesla = ConnectorAggregation::new(Some(EvConnectorType::Tesla), 120.0, 8);
        assert_eq!(tesla.connector_icon(), "🏎️");

        let chademo = ConnectorAggregation::new(Some(EvConnectorType::Chademo), 50.0, 1);
        assert_eq!(chademo.connector_icon(), "🔌");

        let unknown = ConnectorAggregation::new(None, 50.0, 1);
        assert_eq!(unknown.connector_icon(), "⚡");
    }
}