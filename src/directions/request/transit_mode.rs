//! Contains the `TransitMode` enum and its associated traits. It is used to
//! prioritize certain transit modes such as _bus_ or _subway_ when generating
//! transit directions.

use crate::directions::error::Error;
use phf::phf_map;
use serde::{Deserialize, Serialize, Deserializer};

// -----------------------------------------------------------------------------

/// Specifies one or more preferred [modes of
/// transit](https://developers.google.com/maps/documentation/directions/intro#optional-parameters).
///
/// This parameter may only be specified for transit directions, and only if the
/// request includes an API key or a Google Maps Platform Premium Plan client
/// ID.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TransitMode {
    /// Indicates that the calculated route should prefer travel by bus.
    #[serde(alias = "bus")]
    Bus,
    /// Indicates that the calculated route should prefer travel by train, tram,
    /// light rail, and subway. This is equivalent to
    /// `transit_mode=train|tram|subway`.
    #[serde(alias = "rail")]
    Rail,
    /// Indicates that the calculated route should prefer travel by subway.
    #[serde(alias = "subway")]
    Subway,
    /// Indicates that the calculated route should prefer travel by train.
    #[serde(alias = "train")]
    Train,
    /// Indicates that the calculated route should prefer travel by tram and
    /// light rail.
    /// Indicates that the calculated route should prefer travel by tram and
    /// light rail.
    #[serde(alias = "tram")]
    Tram,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for TransitMode {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match TransitMode::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string()))
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&TransitMode> for String {
    /// Converts a `TransitMode` enum to a `String` that contains a [transit
    /// mode](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitMode)
    /// code.
    fn from(transit_mode: &TransitMode) -> String {
        match transit_mode {
            TransitMode::Bus => String::from("bus"),
            TransitMode::Rail => String::from("rail"),
            TransitMode::Subway => String::from("subway"),
            TransitMode::Train => String::from("train"),
            TransitMode::Tram => String::from("tram"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static TRANSIT_MODES_BY_CODE: phf::Map<&'static str, TransitMode> = phf_map! {
    "bus" => TransitMode::Bus,
    "rail" => TransitMode::Rail,
    "subway" => TransitMode::Subway,
    "train" => TransitMode::Train,
    "tram" => TransitMode::Tram,
};

impl std::convert::TryFrom<&str> for TransitMode {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;
    /// Gets a `TransitMode` enum from a `String` that contains a valid [transit
    /// mode](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitMode)
    /// code.
    fn try_from(transit_mode_code: &str) -> Result<TransitMode, Error> {
        TRANSIT_MODES_BY_CODE
            .get(transit_mode_code)
            .cloned()
            .ok_or_else(|| Error::InvalidTransitModeCode(transit_mode_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for TransitMode {
    /// Returns a reasonable default variant for the `TransitMode` enum type.
    fn default() -> Self {
        TransitMode::Bus
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for TransitMode {
    /// Formats a `TransitMode` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TransitMode::Bus => write!(f, "Bus"),
            TransitMode::Rail => write!(f, "Rail"),
            TransitMode::Subway => write!(f, "Subway"),
            TransitMode::Train => write!(f, "Train"),
            TransitMode::Tram => write!(f, "Tram"),
        } // match
    } // fn
} // impl