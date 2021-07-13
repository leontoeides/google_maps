//! Contains the `ApiRate` struct and its associated traits. `ApiRate` contains
//! the user's specified request rate and the system calculated effective
//! current request rate.

use crate::request_rate::target_rate::TargetRate; // use
use stream_throttle::ThrottlePool;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Contains the user's specified request rate and the effective current request
/// rate.
#[derive(Clone)]
pub struct ApiRate {
    pub target_rate: TargetRate,
    pub throttle_pool: Option<ThrottlePool>,
} // struct

impl std::default::Default for ApiRate {
    /// Returns a reasonable default values for the `ApiRate` type.
    fn default() -> Self {
        ApiRate {
            target_rate: TargetRate::default(),
            throttle_pool: None,
        } // struct
    } // fn
} // impl

impl fmt::Debug for ApiRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ApiRate")
            .field("target_rate", &self.target_rate.to_string())
            //.field("current_rate", &self.current_rate)
            .finish()
    }
}

impl PartialEq for ApiRate {
    fn eq(&self, other: &Self) -> bool {
        self.target_rate == other.target_rate // && self.current_rate == other.current_rate
    }
}

impl Eq for ApiRate {}


impl Hash for ApiRate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // self.current_rate.hash(state);
        self.target_rate.hash(state);
    }
}

