//! Contains the `TargetRate` struct and its associated traits. `TargetRate`
//! contains the user's specified request rate.

use crate::request_rate::rate_to_string::rate_to_string;
use serde::{Serialize, Deserialize};
use std::time::Duration;

/// Contains the user's specified request rate in the form of _requests_ per
/// _duration_.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TargetRate {
    /// Limit the maximum number of requests.
    pub requests: u16,
    /// Limit the maximum number of requests per user-specified _duration_. For
    /// example this field may define _requests_ per _second_, per _minute_,
    /// per _day_, and so on. This is done by using the `std::time::Duration`
    /// methods.
    pub duration: Duration,
} // struct

impl std::convert::From<&TargetRate> for String {
    /// Converts a `TargetRate` enum to a `String` that contains a
    /// human-friendly & readable rate.
    fn from(target_rate: &TargetRate) -> String {
        rate_to_string(target_rate.requests as u64, target_rate.duration, "request", "requests")
    } // fn
} // impl

impl std::default::Default for TargetRate {
    /// Returns a reasonable default values for the `TargetRate` struct.
    fn default() -> Self {
        TargetRate {
            requests: 1,
            duration: Duration::from_secs(10),
        } // struct
    } // fn
} // impl

impl std::fmt::Display for TargetRate {
    /// Formats a `TargetRate` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    } // fn
} // impl