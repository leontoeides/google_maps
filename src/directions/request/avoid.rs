//! Contains the `Avoid` enum and its associated traits. It is used to route
//! around features such as ferries, highways, and tolls.

use crate::directions::error::Error as DirectionsError;
use crate::error::Error as GoogleMapsError;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Avoid {
    /// Indicates that the calculated route should avoid ferries.
    Ferries = 0,
    /// Indicates that the calculated route should avoid highways.
    Highways = 1,
    /// Indicates that the calculated route should avoid indoor steps for
    /// walking and transit directions. Only requests that include an API key or
    /// a Google Maps Platform Premium Plan client ID will receive indoor steps
    /// by default.
    Indoor = 2,
    /// Indicates that the calculated route should avoid toll roads/bridges.
    #[default]
    Tolls = 3,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for Avoid {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match Self::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string())),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl Serialize for Avoid {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Avoid> for &str {
    /// Converts an `Avoid` enum to a `String` that contains a
    /// [restrictions](https://developers.google.com/maps/documentation/directions/intro#Restrictions)
    /// code.
    fn from(avoid: &Avoid) -> Self {
        match avoid {
            Avoid::Ferries => "ferries",
            Avoid::Highways => "highways",
            Avoid::Indoor => "indoor",
            Avoid::Tolls => "tolls",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for Avoid {
    /// Converts an `Avoid` enum to a `String` that contains a
    /// [restrictions](https://developers.google.com/maps/documentation/directions/intro#Restrictions)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Self> for Avoid {
    /// Converts a borrowed `&Avoid` enum into an owned `Avoid` enum by cloning
    /// it.
    fn from(avoid: &Self) -> Self {
        avoid.clone()
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Avoid> for String {
    /// Converts an `Avoid` enum to a `String` that contains a
    /// [restrictions](https://developers.google.com/maps/documentation/directions/intro#Restrictions)
    /// code.
    fn from(avoid: &Avoid) -> Self {
        std::convert::Into::<&str>::into(avoid).to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

static RESTRICTIONS_BY_CODE: phf::Map<&'static str, Avoid> = phf_map! {
    "ferries" => Avoid::Ferries,
    "highways" => Avoid::Highways,
    "indoor" => Avoid::Indoor,
    "tolls" => Avoid::Tolls,
};

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for Avoid {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = GoogleMapsError;
    /// Gets an `Avoid` enum from a `String` that contains a valid
    /// [restrictions](https://developers.google.com/maps/documentation/directions/intro#Restrictions)
    /// code.
    fn try_from(restriction_code: &str) -> Result<Self, Self::Error> {
        Ok(RESTRICTIONS_BY_CODE
            .get(restriction_code)
            .cloned()
            .ok_or_else(|| DirectionsError::InvalidAvoidCode(restriction_code.to_string()))?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for Avoid {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Err = GoogleMapsError;
    /// Gets an `Avoid` enum from a `String` that contains a valid
    /// [restrictions](https://developers.google.com/maps/documentation/directions/intro#Restrictions)
    /// code.
    fn from_str(restriction_code: &str) -> Result<Self, Self::Err> {
        Ok(RESTRICTIONS_BY_CODE
            .get(restriction_code)
            .cloned()
            .ok_or_else(|| DirectionsError::InvalidAvoidCode(restriction_code.to_string()))?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl Avoid {
    /// Formats a `Avoid` enum into a string that is presentable to the end
    /// user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::Ferries => "Ferries",
            Self::Highways => "Highways",
            Self::Indoor => "Indoor",
            Self::Tolls => "Tolls",
        } // match
    } // fn
} // impl
