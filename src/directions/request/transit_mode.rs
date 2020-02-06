//! Contains the `TransitMode` enum and its associated traits. It is used to
//! prioritize certain transit modes such as _bus_ or _subway_ when generating
//! transit directions.

use crate::directions::error::Error;
use serde::{Serialize, Deserialize};

/// Specifies one or more preferred [modes of
/// transit](https://developers.google.com/maps/documentation/directions/intro#optional-parameters).
///
/// This parameter may only be specified for transit directions, and only if the
/// request includes an API key or a Google Maps Platform Premium Plan client
/// ID.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
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

impl std::convert::TryFrom<String> for TransitMode {

    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;

    /// Gets a `TransitMode` enum from a `String` that contains a valid [transit
    /// mode](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitMode)
    /// code.
    fn try_from(transit_mode: String) -> Result<TransitMode, Error> {
        match transit_mode.as_ref() {
            "bus" => Ok(TransitMode::Bus),
            "rail" => Ok(TransitMode::Rail),
            "subway" => Ok(TransitMode::Subway),
            "train" => Ok(TransitMode::Train),
            "tram" => Ok(TransitMode::Tram),
            _ => Err(Error::InvalidTransitModeCode(transit_mode)),
        } // match
    } // fn

} // impl

impl std::default::Default for TransitMode {
    /// Returns a reasonable default variant for the `TransitMode` enum type.
    fn default() -> Self {
        TransitMode::Bus
    } // fn
} // impl

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