//! A representation of duration as a numeric value and a display string.

use crate::serde::duration_to_seconds::duration_to_seconds;
use crate::serde::seconds_to_duration::seconds_to_duration;
use chrono::Duration;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// A representation of duration as a numeric value and a display string.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DirectionsDuration {
    /// A string representation of the duration value.
    pub text: String,

    /// The duration in seconds.
    #[serde(
        deserialize_with = "seconds_to_duration",
        serialize_with = "duration_to_seconds"
    )]
    pub value: Duration,
} // struct
