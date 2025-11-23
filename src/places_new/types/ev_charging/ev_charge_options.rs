use crate::places_new::ConnectorAggregation;

// -------------------------------------------------------------------------------------------------
//
/// Information about EV charge station hosted in a place.
///
/// EV charge options provide comprehensive information about electric vehicle charging
/// infrastructure at a location. This includes connector types, charging speeds, availability, and
/// capacity information.
///
/// The terminology follows standard EV charging conventions: one station has one or more ports, one
/// port can charge one car at a time, and one port has one or more connectors of potentially
/// different types.
#[derive(
    //std
    Clone,
    Debug,
    Default,
    // getset
    getset::Getters,
    getset::CopyGetters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[serde(rename_all = "camelCase")]
pub struct EvChargeOptions {
    /// Number of connectors at this station.
    ///
    /// Total count of individual charging connectors available at this location. Note that because
    /// some ports can have multiple connectors but only charge one car at a time, the number of
    /// connectors may be greater than the total number of cars that can charge simultaneously.
    #[getset(get_copy = "pub")]
    pub connector_count: i32,

    /// A list of EV charging connector aggregations.
    ///
    /// Contains connectors grouped by type and maximum charge rate, providing detailed information
    /// about available charging options, speeds, and real-time availability for each connector
    /// configuration.
    #[serde(default)]
    #[getset(get = "pub")]
    pub connector_aggregation: Vec<ConnectorAggregation>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl EvChargeOptions {
    /// Creates a new `EvChargeOptions` with the specified connector count and aggregations.
    ///
    /// Used to construct EV charging information with complete connector details. The connector
    /// count should match the sum of connectors across all aggregations.
    #[must_use]
    pub const fn new(connector_count: i32, connector_aggregation: Vec<ConnectorAggregation>) -> Self {
        Self {
            connector_count,
            connector_aggregation,
        }
    }

    /// Creates an empty `EvChargeOptions`.
    ///
    /// Used when initializing EV charge options for locations without charging infrastructure or
    /// when charging data is not available.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            connector_count: 0,
            connector_aggregation: Vec::new(),
        }
    }

    /// Returns whether this location has any EV charging infrastructure.
    ///
    /// Used to determine if the location offers electric vehicle charging, helping filter locations
    /// and provide relevant information to EV drivers.
    #[must_use]
    pub fn has_charging(&self) -> bool {
        self.connector_count > 0 && !self.connector_aggregation.is_empty()
    }

    /// Returns the number of different connector types available.
    ///
    /// Used to determine the variety of charging options, which indicates how many different types
    /// of EVs can charge at this location.
    #[must_use]
    pub fn connector_type_count(&self) -> usize {
        self.connector_aggregation.len()
    }

    /// Gets all connector aggregations sorted by charging speed (fastest first).
    ///
    /// Returns charging options ordered from highest to lowest power output, useful for
    /// prioritizing fast charging options for users in a hurry.
    #[must_use]
    pub fn aggregations_by_speed(&self) -> Vec<&ConnectorAggregation> {
        let mut aggregations: Vec<&ConnectorAggregation> = self.connector_aggregation.iter().collect();
        aggregations.sort_by(|a, b| {
            b.max_charge_rate_kw()
                .partial_cmp(&a.max_charge_rate_kw())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        aggregations
    }

    /// Gets all connector aggregations sorted by connector type.
    ///
    /// Returns charging options ordered by connector type for consistent display and logical
    /// grouping in user interfaces.
    #[must_use]
    pub fn aggregations_by_type(&self) -> Vec<&ConnectorAggregation> {
        let mut aggregations: Vec<&ConnectorAggregation> = self.connector_aggregation.iter().collect();
        aggregations.sort_by_key(|agg| agg.connector_type());
        aggregations
    }

    /// Gets only connector aggregations that have available connectors.
    ///
    /// Returns charging options that are immediately usable, filtering out fully occupied or
    /// out-of-service connector groups.
    #[must_use]
    pub fn available_aggregations(&self) -> Vec<&ConnectorAggregation> {
        self.connector_aggregation
            .iter()
            .filter(|agg| agg.has_available_connectors().unwrap_or(false))
            .collect()
    }

    /// Gets connector aggregations suitable for fast charging.
    ///
    /// Returns charging options with high power output (typically 50kW+) suitable for rapid
    /// charging during travel or quick stops.
    #[must_use]
    pub fn fast_charging_aggregations(&self) -> Vec<&ConnectorAggregation> {
        self.connector_aggregation
            .iter()
            .filter(|agg| agg.is_fast_charging())
            .collect()
    }

    /// Gets connector aggregations suitable for AC charging.
    ///
    /// Returns slower charging options typically used for extended parking sessions like work,
    /// shopping, or overnight charging.
    #[must_use]
    pub fn ac_charging_aggregations(&self) -> Vec<&ConnectorAggregation> {
        self.connector_aggregation
            .iter()
            .filter(|agg| agg.is_ac_charging().unwrap_or(false))
            .collect()
    }

    /// Gets connector aggregations compatible with a specific connector type.
    ///
    /// Returns charging options that can be used with a specific vehicle connector, useful for
    /// vehicle-specific charging station filtering.
    #[must_use]
    pub fn compatible_aggregations(&self, connector_type: crate::places_new::EvConnectorType) -> Vec<&ConnectorAggregation> {
        self.connector_aggregation
            .iter()
            .filter(|agg| agg.is_compatible_with(connector_type).unwrap_or(false))
            .collect()
    }

    /// Returns the maximum charging speed available at this station.
    ///
    /// Used to identify the fastest charging option available, useful for highlighting stations
    /// with high-speed charging capabilities.
    #[must_use]
    pub fn max_charging_speed(&self) -> Option<f64> {
        self.connector_aggregation
            .iter()
            .map(ConnectorAggregation::max_charge_rate_kw)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Returns the minimum charging speed available at this station.
    ///
    /// Used to understand the full range of charging options and identify the slowest available
    /// charging speed.
    #[must_use]
    pub fn min_charging_speed(&self) -> Option<f64> {
        self.connector_aggregation
            .iter()
            .map(ConnectorAggregation::max_charge_rate_kw)
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Returns the total number of currently available connectors.
    ///
    /// Calculates how many connectors across all types are immediately available for charging,
    /// providing overall station availability.
    #[must_use]
    pub fn total_available_connectors(&self) -> Option<i32> {
        let available_counts: Vec<i32> = self
            .connector_aggregation
            .iter()
            .filter_map(ConnectorAggregation::available_count)
            .collect();

        if available_counts.len() == self.connector_aggregation.len() {
            Some(available_counts.iter().sum())
        } else {
            None // Not all aggregations have availability data
        }
    }

    /// Returns the total number of out-of-service connectors.
    ///
    /// Calculates how many connectors are not operational, providing insight into station
    /// reliability and maintenance status.
    #[must_use]
    pub fn total_out_of_service_connectors(&self) -> Option<i32> {
        let out_of_service_counts: Vec<i32> = self
            .connector_aggregation
            .iter()
            .filter_map(ConnectorAggregation::out_of_service_count)
            .collect();

        if out_of_service_counts.len() == self.connector_aggregation.len() {
            Some(out_of_service_counts.iter().sum())
        } else {
            None
        }
    }

    /// Returns whether this station has immediate charging availability.
    ///
    /// Used to identify stations where EV drivers can charge immediately without waiting, based on
    /// real-time availability data.
    #[must_use]
    pub fn has_immediate_availability(&self) -> Option<bool> {
        self.total_available_connectors().map(|count| count > 0)
    }

    /// Returns whether this station supports rapid charging.
    ///
    /// Used to identify stations with ultra-fast charging capabilities (typically 150kW+) suitable
    /// for highway travel and quick charging stops.
    #[must_use]
    pub fn supports_rapid_charging(&self) -> bool {
        self.connector_aggregation
            .iter()
            .any(ConnectorAggregation::is_rapid_charging)
    }

    /// Returns whether this station supports multiple connector types.
    ///
    /// Used to identify universal charging stations that can accommodate different types of
    /// electric vehicles with varying connector standards.
    #[must_use]
    pub fn is_universal_station(&self) -> bool {
        self.connector_type_count() >= 2
    }

    /// Gets the overall utilization rate of the charging station.
    ///
    /// Calculates what percentage of total connectors are currently in use, providing insight into
    /// station demand and congestion levels.
    #[must_use]
    pub fn overall_utilization_rate(&self) -> Option<f64> {
        if self.connector_count == 0 {
            return None;
        }

        let total_available = self.total_available_connectors()?;
        let total_out_of_service = self.total_out_of_service_connectors().unwrap_or(0);
        let total_in_use = self.connector_count - total_available - total_out_of_service;

        Some((f64::from(total_in_use) / f64::from(self.connector_count)) * 100.0)
    }

    /// Gets a compact description suitable for mobile displays.
    ///
    /// Provides brief charging information suitable for mobile interfaces, map overlays, or
    /// space-constrained displays showing key charging details.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, reason = "for display, not for calculations")]
    pub fn compact_description(&self) -> String {
        if !self.has_charging() {
            return "No EV charging".to_string();
        }

        match (self.connector_count, self.max_charging_speed()) {
            (count, Some(max_speed)) => {
                format!("{} connectors, up to {}kW", count, max_speed as i32)
            }
            (count, None) => format!("{count} connectors"),
        }
    }

    /// Gets a detailed description including availability and speed range.
    ///
    /// Provides comprehensive charging information suitable for detailed station listings and
    /// charging planning interfaces.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, reason = "for display, not for calculations")]
    #[allow(clippy::format_push_string, reason = "for conditional text building")]
    pub fn detailed_description(&self) -> String {
        if !self.has_charging() {
            return "No EV charging available".to_string();
        }

        let mut description = format!("{} connector", self.connector_count);
        if self.connector_count != 1 {
            description.push('s');
        }

        // Add speed range if available
        if let (Some(min_speed), Some(max_speed)) = (self.min_charging_speed(), self.max_charging_speed()) {
            if (min_speed - max_speed).abs() < 0.1 {
                description.push_str(&format!(" at {}kW", max_speed as i32));
            } else {
                description.push_str(&format!(" ({}-{}kW)", min_speed as i32, max_speed as i32));
            }
        }

        // Add availability if known
        if let Some(available) = self.total_available_connectors() {
            if available == 0 {
                description.push_str(" - None available");
            } else if available == self.connector_count {
                description.push_str(" - All available");
            } else {
                description.push_str(&format!(" - {available} available"));
            }
        }

        description
    }

    /// Gets a list of unique connector types available.
    ///
    /// Returns the different connector standards available at this station, useful for displaying
    /// compatibility information and connector variety.
    #[must_use]
    pub fn available_connector_types(&self) -> Vec<crate::places_new::EvConnectorType> {
        self.connector_aggregation
            .iter()
            .filter_map(ConnectorAggregation::connector_type)
            .collect()
    }

    /// Returns whether this station is suitable for highway travel.
    ///
    /// Used to identify charging stations appropriate for long-distance travel, typically requiring
    /// fast charging capabilities and good availability.
    #[must_use]
    pub fn is_highway_suitable(&self) -> bool {
        self.supports_rapid_charging() && self.connector_count >= 2
    }

    /// Returns whether this station is suitable for urban/destination charging.
    ///
    /// Used to identify stations appropriate for extended parking sessions with AC charging options
    /// for work, shopping, or residential areas.
    #[must_use]
    pub fn is_destination_suitable(&self) -> bool {
        !self.ac_charging_aggregations().is_empty()
    }

    /// Gets aggregations grouped by charging speed category.
    ///
    /// Returns connector aggregations organized by speed categories (slow, fast, rapid) for
    /// structured display and filtering in user interfaces.
    #[must_use]
    pub fn aggregations_by_speed_category(&self) -> std::collections::HashMap<&'static str, Vec<&ConnectorAggregation>> {
        let mut categories = std::collections::HashMap::new();
        
        for aggregation in &self.connector_aggregation {
            let category = aggregation.speed_category();
            categories.entry(category).or_insert_with(Vec::new).push(aggregation);
        }
        
        categories
    }

    /// Returns whether all connector availability data is fresh.
    ///
    /// Used to determine if all availability information is recent enough to be reliable for
    /// real-time charging decisions.
    #[must_use]
    pub fn has_fresh_availability_data(&self) -> bool {
        !self.connector_aggregation.is_empty()
            && self
                .connector_aggregation
                .iter()
                .all(ConnectorAggregation::has_fresh_availability)
    }

    /// Gets the charging station's capability summary.
    ///
    /// Returns a structured summary of the station's charging capabilities including speed ranges,
    /// connector types, and availability status.
    #[must_use]
    pub fn capability_summary(&self) -> ChargingCapabilitySummary {
        ChargingCapabilitySummary {
            total_connectors: self.connector_count,
            connector_types: self.connector_type_count(),
            max_speed_kw: self.max_charging_speed(),
            min_speed_kw: self.min_charging_speed(),
            supports_rapid_charging: self.supports_rapid_charging(),
            supports_ac_charging: !self.ac_charging_aggregations().is_empty(),
            available_connectors: self.total_available_connectors(),
            is_universal: self.is_universal_station(),
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
/// Summary of charging station capabilities and status.
///
/// Provides a structured overview of charging station features, speeds,
/// and availability for easy comparison and filtering of charging options.
#[derive(Clone, Debug, PartialEq)]
pub struct ChargingCapabilitySummary {
    /// Total number of connectors at the station.
    pub total_connectors: i32,
    /// Number of different connector types available.
    pub connector_types: usize,
    /// Maximum charging speed available in kW.
    pub max_speed_kw: Option<f64>,
    /// Minimum charging speed available in kW.
    pub min_speed_kw: Option<f64>,
    /// Whether the station supports rapid charging (150kW+).
    pub supports_rapid_charging: bool,
    /// Whether the station supports AC charging.
    pub supports_ac_charging: bool,
    /// Number of currently available connectors.
    pub available_connectors: Option<i32>,
    /// Whether the station supports multiple connector types.
    pub is_universal: bool,
}

// -------------------------------------------------------------------------------------------------
//
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::places_new::EvConnectorType;
    use jiff::Timestamp;

    fn create_test_aggregation(
        connector_type: EvConnectorType,
        max_rate: f64,
        count: i32,
        available: Option<i32>,
    ) -> ConnectorAggregation {
        available.map_or_else(
            || ConnectorAggregation::new(Some(connector_type), max_rate, count),
            |avail| ConnectorAggregation::with_availability(
                Some(connector_type),
                max_rate,
                count,
                avail,
                0,
                Timestamp::now(),
            )
        )
    }

    #[test]
    fn test_serialization() {
        let charge_options = EvChargeOptions::new(
            6,
            vec![
                create_test_aggregation(EvConnectorType::CcsCombo2, 150.0, 4, Some(2)),
                create_test_aggregation(EvConnectorType::J1772, 22.0, 2, Some(1)),
            ],
        );

        let json = serde_json::to_string(&charge_options).unwrap();
        assert!(json.contains("connectorCount"));
        assert!(json.contains("connectorAggregation"));
    }

    #[test]
    fn test_constructors() {
        let aggregations = vec![
            create_test_aggregation(EvConnectorType::CcsCombo2, 150.0, 4, Some(2)),
            create_test_aggregation(EvConnectorType::J1772, 22.0, 2, Some(1)),
        ];

        let charge_options = EvChargeOptions::new(6, aggregations);
        assert_eq!(charge_options.connector_count, 6);
        assert_eq!(charge_options.connector_type_count(), 2);
        assert!(charge_options.has_charging());

        let empty = EvChargeOptions::empty();
        assert!(!empty.has_charging());
        assert_eq!(empty.connector_count, 0);
    }

    #[test]
    fn test_charging_capabilities() {
        let charge_options = EvChargeOptions::new(
            6,
            vec![
                create_test_aggregation(EvConnectorType::CcsCombo2, 250.0, 4, Some(2)),
                create_test_aggregation(EvConnectorType::J1772, 22.0, 2, Some(1)),
            ],
        );

        assert_eq!(charge_options.max_charging_speed(), Some(250.0));
        assert_eq!(charge_options.min_charging_speed(), Some(22.0));
        assert!(charge_options.supports_rapid_charging());
        assert!(charge_options.is_universal_station());
        assert!(charge_options.is_highway_suitable());
        assert!(charge_options.is_destination_suitable());
    }

    #[test]
    fn test_availability_calculations() {
        let charge_options = EvChargeOptions::new(
            6,
            vec![
                create_test_aggregation(EvConnectorType::CcsCombo2, 150.0, 4, Some(2)),
                create_test_aggregation(EvConnectorType::J1772, 22.0, 2, Some(1)),
            ],
        );

        assert_eq!(charge_options.total_available_connectors(), Some(3));
        assert_eq!(charge_options.has_immediate_availability(), Some(true));

        let utilization = charge_options.overall_utilization_rate().unwrap();
        assert!((utilization - 50.0).abs() < 0.01); // (6 - 3) / 6 = 50%
    }

    #[test]
    fn test_filtering_methods() {
        let charge_options = EvChargeOptions::new(
            6,
            vec![
                create_test_aggregation(EvConnectorType::CcsCombo2, 150.0, 4, Some(2)),
                create_test_aggregation(EvConnectorType::J1772, 22.0, 2, Some(0)),
            ],
        );

        let available = charge_options.available_aggregations();
        assert_eq!(available.len(), 1);
        assert_eq!(available[0].connector_type(), Some(EvConnectorType::CcsCombo2));

        let fast_charging = charge_options.fast_charging_aggregations();
        assert_eq!(fast_charging.len(), 1);

        let ac_charging = charge_options.ac_charging_aggregations();
        assert_eq!(ac_charging.len(), 1);

        let compatible = charge_options.compatible_aggregations(EvConnectorType::CcsCombo2);
        assert_eq!(compatible.len(), 1);
    }

    #[test]
    #[allow(clippy::float_cmp, reason = "for testing only")]
    fn test_sorting_methods() {
        let charge_options = EvChargeOptions::new(
            6,
            vec![
                create_test_aggregation(EvConnectorType::J1772, 22.0, 2, Some(1)),
                create_test_aggregation(EvConnectorType::CcsCombo2, 150.0, 4, Some(2)),
            ],
        );

        let by_speed = charge_options.aggregations_by_speed();
        assert_eq!(by_speed[0].max_charge_rate_kw(), 150.0); // Fastest first
        assert_eq!(by_speed[1].max_charge_rate_kw(), 22.0);

        let by_type = charge_options.aggregations_by_type();
        assert_eq!(by_type.len(), 2);
    }

    #[test]
    fn test_descriptions() {
        let charge_options = EvChargeOptions::new(
            6,
            vec![
                create_test_aggregation(EvConnectorType::CcsCombo2, 150.0, 4, Some(2)),
                create_test_aggregation(EvConnectorType::J1772, 22.0, 2, Some(1)),
            ],
        );

        let compact = charge_options.compact_description();
        assert!(compact.contains("6 connectors"));
        assert!(compact.contains("150kW"));

        let detailed = charge_options.detailed_description();
        assert!(detailed.contains("6 connectors"));
        assert!(detailed.contains("22-150kW"));
        assert!(detailed.contains("3 available"));

        let empty = EvChargeOptions::empty();
        assert_eq!(empty.compact_description(), "No EV charging");
    }

    #[test]
    fn test_connector_types() {
        let charge_options = EvChargeOptions::new(
            4,
            vec![
                create_test_aggregation(EvConnectorType::CcsCombo2, 150.0, 2, None),
                create_test_aggregation(EvConnectorType::J1772, 22.0, 2, None),
            ],
        );

        let types = charge_options.available_connector_types();
        assert_eq!(types.len(), 2);
        assert!(types.contains(&EvConnectorType::CcsCombo2));
        assert!(types.contains(&EvConnectorType::J1772));
    }

    #[test]
    fn test_speed_categories() {
        let charge_options = EvChargeOptions::new(
            6,
            vec![
                create_test_aggregation(EvConnectorType::CcsCombo2, 250.0, 2, None),
                create_test_aggregation(EvConnectorType::CcsCombo1, 100.0, 2, None),
                create_test_aggregation(EvConnectorType::J1772, 22.0, 2, None),
            ],
        );

        let categories = charge_options.aggregations_by_speed_category();
        assert!(categories.contains_key("Rapid Charging"));
        assert!(categories.contains_key("Fast Charging"));
        assert!(categories.contains_key("Fast AC"));
    }

    #[test]
    fn test_capability_summary() {
        let charge_options = EvChargeOptions::new(
            6,
            vec![
                create_test_aggregation(EvConnectorType::CcsCombo2, 250.0, 4, Some(2)),
                create_test_aggregation(EvConnectorType::J1772, 22.0, 2, Some(1)),
            ],
        );

        let summary = charge_options.capability_summary();
        assert_eq!(summary.total_connectors, 6);
        assert_eq!(summary.connector_types, 2);
        assert_eq!(summary.max_speed_kw, Some(250.0));
        assert_eq!(summary.min_speed_kw, Some(22.0));
        assert!(summary.supports_rapid_charging);
        assert!(summary.supports_ac_charging);
        assert_eq!(summary.available_connectors, Some(3));
        assert!(summary.is_universal);
    }

    #[test]
    fn test_default() {
        let default_charge_options = EvChargeOptions::default();
        assert!(!default_charge_options.has_charging());
        assert_eq!(default_charge_options.connector_count, 0);
        assert_eq!(default_charge_options.connector_type_count(), 0);
    }
}