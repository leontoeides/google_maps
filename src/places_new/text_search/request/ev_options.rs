#![allow(clippy::ref_option, reason = "for the getset crate")]

use crate::places_new::types::ev_charging::EvConnectorType;

// -------------------------------------------------------------------------------------------------
//
/// EV charging requirements for filtering places.
///
/// Specifies the types of charging connectors and minimum charging rate needed. Places not meeting
/// these requirements are filtered out of results. Use this to find charging stations compatible
/// with your electric vehicle.
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
pub struct EvOptions {
    /// Required connector types.
    ///
    /// Only places with at least one of these connector types are returned. Places without any
    /// matching connectors are filtered out.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub connector_types: Option<Vec<EvConnectorType>>,

    /// Minimum charging rate in kilowatts.
    ///
    /// Only places with charging rates at or above this value are returned. Places with slower
    /// charging are filtered out.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub minimum_charging_rate_kw: Option<f64>,
}