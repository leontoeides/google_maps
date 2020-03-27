//! A representation of duration as a numeric value and a display string.

use chrono::Duration;
use crate::serde::seconds_to_duration::seconds_to_duration;
use serde::Deserialize;

/// A representation of duration as a numeric value and a display string.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize)]
pub struct DirectionsDuration {

    /// A string representation of the duration value.
    pub text: String,

    /// The duration in seconds.
    #[serde(deserialize_with = "seconds_to_duration")]
    pub value: Duration,

} // struct