// -------------------------------------------------------------------------------------------------
//
// Structures

pub(super) mod connector_aggregation;
pub use crate::places_new::types::ev_charging::connector_aggregation::ConnectorAggregation;

pub(super) mod ev_charge_options;
pub use crate::places_new::types::ev_charging::ev_charge_options::EvChargeOptions;

// -------------------------------------------------------------------------------------------------
//
// Enumerations

pub(super) mod ev_connector_type;
pub use crate::places_new::types::ev_charging::ev_connector_type::EvConnectorType;