//! Contains the `Avoid` enum and its associated traits. It is used to route
//! around features such as ferries, highways, and tolls.

use crate::directions::error::Error;
use phf::phf_map;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------

/// Used to specify features that calculated routes should
/// [avoid](https://developers.google.com/maps/documentation/directions/intro#Restrictions).
///
/// Directions may be calculated that adhere to certain restrictions.
/// Restrictions are indicated by use of the `avoid` parameter, and an argument to
/// that parameter indicating the restriction to avoid. The following
/// restrictions are supported:
///
/// * Tolls
/// * Highways
/// * Ferries
///
/// It's possible to request a route that avoids any combination of tolls,
/// highways and ferries by passing both restrictions to the avoid parameter.
/// For example: `avoid=tolls|highways|ferries`.
///
/// Note: the addition of restrictions does not preclude routes that include the
/// restricted feature; it simply biases the result to more favorable routes.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Avoid {
    /// Indicates that the calculated route should avoid ferries.
    #[serde(alias = "ferries")]
    Ferries,
    /// Indicates that the calculated route should avoid highways.
    #[serde(alias = "highways")]
    Highways,
    /// Indicates that the calculated route should avoid indoor steps for
    /// walking and transit directions. Only requests that include an API key or
    /// a Google Maps Platform Premium Plan client ID will receive indoor steps
    /// by default.
    #[serde(alias = "indoor")]
    Indoor,
    /// Indicates that the calculated route should avoid toll roads/bridges.
    #[serde(alias = "tolls")]
    Tolls,
} // enum

// -----------------------------------------------------------------------------

impl std::convert::From<&Avoid> for String {
    /// Converts an `Avoid` enum to a `String` that contains a
    /// [restrictions](https://developers.google.com/maps/documentation/directions/intro#Restrictions)
    /// code.
    fn from(avoid: &Avoid) -> String {
        match avoid {
            Avoid::Ferries => String::from("ferries"),
            Avoid::Highways => String::from("highways"),
            Avoid::Indoor => String::from("indoor"),
            Avoid::Tolls => String::from("tolls"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static RESTRICTIONS_BY_CODE: phf::Map<&'static str, Avoid> = phf_map! {
    "ferries" => Avoid::Ferries,
    "highways" => Avoid::Highways,
    "indoor" => Avoid::Indoor,
    "tolls" => Avoid::Tolls,
};

impl std::convert::TryFrom<&str> for Avoid {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;
    /// Gets an `Avoid` enum from a `String` that contains a valid
    /// [restrictions](https://developers.google.com/maps/documentation/directions/intro#Restrictions)
    /// code.
    fn try_from(restriction_code: &str) -> Result<Avoid, Error> {
        RESTRICTIONS_BY_CODE
            .get(restriction_code)
            .cloned()
            .ok_or_else(|| Error::InvalidAvoidCode(restriction_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for Avoid {
    /// Returns a reasonable default variant for the `Avoid` enum type.
    fn default() -> Self {
        Avoid::Tolls
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for Avoid {
    /// Formats a `Avoid` enum into a string that is presentable to the end
    /// user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Avoid::Ferries => write!(f, "Ferries"),
            Avoid::Highways => write!(f, "Highways"),
            Avoid::Indoor => write!(f, "Indoor"),
            Avoid::Tolls => write!(f, "Tolls"),
        } // match
    } // fn
} // impl