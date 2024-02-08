//! Contains the `RequestRate` struct and its associated traits. It is used to
//! specify Google Maps Platform and per-API request rate limits. **Do not use
//! this module to set request rates. Use the `GoogleMapsClient` methods
//! instead.**

pub mod api;
mod api_rate;
pub mod api_rate_limit;
mod current_rate;
mod duration_to_string;
mod duration_unit;
mod limit;
mod rate_to_string;
mod target_rate;
mod with_rate;

// -----------------------------------------------------------------------------

use crate::request_rate::api::Api;
use crate::request_rate::api_rate::ApiRate;
use std::collections::HashMap;

// -----------------------------------------------------------------------------
//
/// Contains the request rates for the Google Maps Platform and the individual
/// Google Maps APIs.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestRate {
    /// Used to specify the request rate for _all_ APIs in addition to the
    /// per-API request rates. The `Api::All` request rate will be observed
    /// first, then the per-API request rate such as `Api::Directions` will be
    /// observed afterward.
    pub rate_map: HashMap<Api, ApiRate>,
} // struct

// -----------------------------------------------------------------------------

impl std::default::Default for RequestRate {
    /// Returns default values (empty) for the `RequestRate` struct.
    fn default() -> Self {
        Self {
            rate_map: HashMap::new(),
        } // struct
    } // fn
} // impl
