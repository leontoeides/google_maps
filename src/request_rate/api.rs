//! Contains the `Api` enum and its associated traits. The `Api` enum is used to
//! specify a Google Maps Platform API when setting per-API request rate limits.

use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
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
    Places,
    Roads,
    AddressValidation,
} // enum

// -----------------------------------------------------------------------------

impl std::convert::From<&Api> for String {
    /// Converts an `Api` enum to a `String` that contains an API name.
    fn from(api: &Api) -> Self {
        match api {
            Api::All => Self::from("All"),
            Api::Directions => Self::from("Directions"),
            Api::DistanceMatrix => Self::from("Distance Matrix"),
            Api::Elevation => Self::from("Elevation"),
            Api::Geocoding => Self::from("Geocoding"),
            Api::TimeZone => Self::from("Time Zone"),
            Api::Places => Self::from("Places"),
            Api::Roads => Self::from("Roads"),
            Api::AddressValidation => Self::from("Address Validation"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for Api {
    /// Returns a reasonable default variant for the `Api` enum.
    fn default() -> Self {
        Self::All
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for Api {
    /// Formats an `Api` enum into a string that is presentable to the end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    } // fn
} // impl
