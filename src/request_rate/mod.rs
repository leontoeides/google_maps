//! Contains the `RequestRate` struct and its associated traits. `RequestRate`
//! is used to specify Google Maps Platform and per-API request rate limits.

mod api_rate;
mod current_rate;
mod duration_to_string;
mod duration_unit;
mod limit;
mod rate_to_string;
mod target_rate;
mod with_rate;
pub mod api;

use crate::request_rate::api_rate::ApiRate;

/// Contains the request rates for the Google Maps Platform and the individual
/// Google Maps APIs.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RequestRate {
    /// Used to specify the request rate for _all_ APIs in addition to the
    /// per-API request rates. The `Api::All` request rate will be observed
    /// first, then the per-API request rate such as `Api::Directions` will be
    /// observed afterward.
    pub all: Option<ApiRate>,
    pub directions: Option<ApiRate>,
    pub distance_matrix: Option<ApiRate>,
    pub elevation: Option<ApiRate>,
    pub geocoding: Option<ApiRate>,
    pub time_zone: Option<ApiRate>,
} // struct

impl std::default::Default for RequestRate {
    /// Returns a reasonable default values for the `RequestRate` struct.
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