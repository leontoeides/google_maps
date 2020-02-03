//! Contains the `ClientSettings` struct and its associated traits.
//! `ClientSettings` is used to set your API key and the settings for governing
//! your requests.

mod duration;
mod duration_unit;
mod limit;
mod new;
mod rate;
mod with_rate_limit;
pub mod api;
pub mod api_limit;

use crate::rate_limit::api_limit::ApiLimit;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RateLimit {
    pub all: Option<ApiLimit>,
    pub directions: Option<ApiLimit>,
    pub distance_matrix: Option<ApiLimit>,
    pub elevation: Option<ApiLimit>,
    pub geocoding: Option<ApiLimit>,
    pub time_zone: Option<ApiLimit>,
} // struct

impl std::default::Default for RateLimit {
    /// Returns a reasonable default population for the `RateLimit` struct.
    fn default() -> Self {
        RateLimit {
            all: None,
            directions: None,
            distance_matrix: None,
            elevation: None,
            geocoding: None,
            time_zone: None,
        } // struct
    } // fn
} // impl