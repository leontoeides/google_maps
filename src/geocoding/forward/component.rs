//! Contains the `Component` enum and its associated traits. It filters the
//! geocoding response to specific types of areas, such as localities (cities)
//! or countries (nations).

use crate::Country;
use serde::{Deserialize, Serialize};

/// The components filter is accepted as an _optional_ parameter if an address
/// is provided. In a Geocoding response, the Geocoding API can return address
/// results restricted to a specific area. You can specify the restriction using
/// the `components` filter. A filter consists of a list of `component:value`
/// pairs separated by a pipe (|). Filter values support the same methods of
/// spelling correction and partial matching as other Geocoding requests. If the
/// geocoder finds a partial match for a component filter, the response will
/// contain a `partial_match` field.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Component {
    // Restrict results:
    // -----------------
    /// Country matches a country name or a two letter [ISO
    /// 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1) country code. The API
    /// follows the ISO standard for defining countries, and the filtering works
    /// best when using the corresponding ISO code of the country.
    Country(Country),

    /// Matches `postal_code` and `postal_code_prefix`.
    PostalCode(String),

    // Bias results:
    // -------------
    /// Matches all the `administrative_area` levels.
    AdministrativeArea(String),

    /// Matches against `locality` and `sublocality` types.
    Locality(String),

    /// Matches the long or short name of a route.
    Route(String),
} // enum

// -----------------------------------------------------------------------------

impl std::convert::From<&Self> for Component {
    /// Converts a borrowed `&Component` enum into an owned `Component` enum by
    /// cloning it.
    fn from(component: &Self) -> Self {
        component.clone()
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Component> for String {
    /// Converts a `Component` struct to a `String` that contains a
    /// component:value pair for filtering results.
    fn from(component: &Component) -> Self {
        match component {
            Component::AdministrativeArea(administrative_area) => {
                format!("administrative_area:{administrative_area}")
            }
            Component::Country(country) => format!("country:{}", Self::from(country)),
            Component::PostalCode(postal_code) => format!("postal_code:{postal_code}"),
            Component::Locality(locality) => format!("locality:{locality}"),
            Component::Route(route) => format!("route:{route}"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for Component {
    /// Returns a reasonable default variant for the `Component` enum type.
    fn default() -> Self {
        Self::Country(Country::UnitedStates)
    } // fn
} // impl
