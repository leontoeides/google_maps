//! Contains the `Api` enum and its associated traits. The `Api` enum is used to
//! specify a Google Maps Platform API when setting per-API request rate limits.

use serde::{Deserialize, Serialize};

/// `Api` is used to select an API to configure. For example, the Google Maps
/// Client can be set to have different request rates for `Directions` and
/// `Elevation` requests. This `enum` is used to select which Google Maps API
/// you would like to configure.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Api {
    /// This variant is used to select settings that are observed for _all_
    /// APIs. These settings are observed in addition to the per-API settings.
    All,
    Directions,
    DistanceMatrix,
    Elevation,
    Geocoding,
    TimeZone,
} // enum

impl std::convert::From<&Api> for String {
    /// Converts an `Api` enum to a `String` that contains an API name.
    fn from(api: &Api) -> String {
        match api {
            Api::All => String::from("All"),
            Api::Directions => String::from("Directions"),
            Api::DistanceMatrix => String::from("Distance Matrix"),
            Api::Elevation => String::from("Elevation"),
            Api::Geocoding => String::from("Geocoding"),
            Api::TimeZone => String::from("Time Zone"),
        } // match
    } // fn
} // impl

impl std::default::Default for Api {
    /// Returns a reasonable default variant for the `Api` enum.
    fn default() -> Self {
        Api::All
    } // fn
} // impl

impl std::fmt::Display for Api {
    /// Formats an `Api` enum into a string that is presentable to the end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    } // fn
} // impl
