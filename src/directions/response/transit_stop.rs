//! Contains information about the stop/station for this part of the trip.

use crate::types::LatLng;
use serde::{Deserialize, Serialize};

/// Contains information about the stop/station for this part of the trip.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TransitStop {
    /// The name of the transit station/stop. eg. "Union Square".
    pub name: String,
    /// The location of the transit station/stop, represented as a `lat` and
    /// `lng` field.
    pub location: LatLng,
} // struct
