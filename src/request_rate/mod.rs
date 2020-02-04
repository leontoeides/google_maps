//! Contains the `RequestRate` struct and its associated traits. `RequestRate`
//! is used to specify Google Maps Platform and per-API request rate limits.

mod current_rate;
mod duration;
mod duration_unit;
mod limit;
mod new;
mod rate;
mod target_rate;
mod with_rate;
pub mod api;
pub mod api_rate;

use crate::request_rate::api_rate::ApiRate;

/// Contains the request rates for the Google Maps Platform and the individual
/// Google Maps APIs.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RequestRate {
    /// Used to specify the request rate for _all_ APIs in addition to the
    /// per-API request rates. The `Api::All` request rate will be observed,
    /// then the per-API request rate such as `Api::Directions` will be observed
    /// afterward.
    pub all: Option<ApiRate>,
    pub directions: Option<ApiRate>,
    pub distance_matrix: Option<ApiRate>,
    pub elevation: Option<ApiRate>,
    pub geocoding: Option<ApiRate>,
    pub time_zone: Option<ApiRate>,
} // struct

impl std::default::Default for RequestRate {
    /// Returns a reasonable default population for the `RequestRate` struct.
    fn default() -> Self {
        RequestRate {
            all: None,
            directions: None,
            distance_matrix: None,
            elevation: None,
            geocoding: None,
            time_zone: None,
        } // struct
    } // fn
} // impl