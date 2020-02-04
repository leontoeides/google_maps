//! Contains the `ApiRate` struct and its associated traits. `ApiRate` contains
//! the user's specified request rate and the effective current request rate.

use crate::request_rate::current_rate::CurrentRate;
use crate::request_rate::target_rate::TargetRate;

/// Contains the user's specified request rate and the effective current request
/// rate.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ApiRate {
    pub target_rate: TargetRate,
    pub current_rate: CurrentRate,
} // struct

impl std::default::Default for ApiRate {
    /// Returns a reasonable default population for the `ApiRate` struct type.
    fn default() -> Self {
        ApiRate {
            target_rate: TargetRate::default(),
            current_rate: CurrentRate::default(),
        } // struct
    } // fn
} // impl