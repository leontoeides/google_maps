//! A representation of distance as a numeric value and a display string.

use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// A representation of distance as a numeric value and a display string.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectionsDistance {
    /// A string representation of the distance value, using the
    /// `with_unit_system()` specified in the request.
    pub text: String,

    /// Indicates the distance in meters.
    pub value: u32,
} // struct
