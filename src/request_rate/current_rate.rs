//! Contains the `CurrentRate` struct and its associated traits. Used to track
//! the current effective request rate.

use crate::request_rate::rate_to_string::rate_to_string;
use std::time::Instant;

/// Contains the current request rate in the form of _requests_ since _first
/// request_.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CurrentRate {
    /// The _instant_ of the first request to the API. If the API has not been
    /// called, the `Option` value will be `None`.
    pub first_request: Option<Instant>,
    /// The _count_ of total number of requests to the API since the first
    /// request.
    pub request_count: u64,
} // struct

impl std::convert::From<&CurrentRate> for String {
    /// Converts a `CurrentRate` enum to a `String` that contains a
    /// human-friendly & readable rate.
    fn from(current_rate: &CurrentRate) -> Self {
        current_rate.first_request.map_or_else(
            || Self::from("None"),
            |first_request| {
                rate_to_string(
                    current_rate.request_count,
                    &first_request.elapsed(),
                    "request",
                    "requests",
                )
            }, // rate_to_string
        ) // map_or_else
    } // fn
} // impl

impl std::default::Default for CurrentRate {
    /// Returns a reasonable default values for the `CurrentRate` struct.
    fn default() -> Self {
        Self {
            first_request: None,
            request_count: 0,
        } // struct
    } // fn
} // impl

impl std::fmt::Display for CurrentRate {
    /// Formats a `CurrentRate` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    } // fn
} // impl
