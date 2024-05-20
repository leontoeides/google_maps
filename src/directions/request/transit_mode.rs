//! Contains the `TransitMode` enum and its associated traits. It is used to
//! prioritize certain transit modes such as _bus_ or _subway_ when generating
//! transit directions.

use crate::directions::error::Error as DirectionsError;
use crate::error::Error as GoogleMapsError;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------

/// Specifies one or more preferred [modes of
/// transit](https://developers.google.com/maps/documentation/directions/intro#optional-parameters).
///
/// This parameter may only be specified for transit directions, and only if the
/// request includes an API key or a Google Maps Platform Premium Plan client
/// ID.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum TransitMode {
    /// Indicates that the calculated route should prefer travel by bus.
    #[default]
    Bus = 0,
    /// Indicates that the calculated route should prefer travel by train, tram,
    /// light rail, and subway. This is equivalent to
    /// `transit_mode=train|tram|subway`.
    Rail = 1,
    /// Indicates that the calculated route should prefer travel by subway.
    Subway = 2,
    /// Indicates that the calculated route should prefer travel by train.
    Train = 3,
    /// Indicates that the calculated route should prefer travel by tram and
    /// light rail.
    /// Indicates that the calculated route should prefer travel by tram and
    /// light rail.
    Tram = 4,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for TransitMode {
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

impl Serialize for TransitMode {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&TransitMode> for &str {
    /// Converts a `TransitMode` enum to a `String` that contains a [transit
    /// mode](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitMode)
    /// code.
    fn from(transit_mode: &TransitMode) -> Self {
        match transit_mode {
            TransitMode::Bus => "bus",
            TransitMode::Rail => "rail",
            TransitMode::Subway => "subway",
            TransitMode::Train => "train",
            TransitMode::Tram => "tram",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for TransitMode {
    /// Converts a `TransitMode` enum to a `String` that contains a [transit
    /// mode](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitMode)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&TransitMode> for String {
    /// Converts a `TransitMode` enum to a `String` that contains a [transit
    /// mode](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitMode)
    /// code.
    fn from(transit_mode: &TransitMode) -> Self {
        std::convert::Into::<&str>::into(transit_mode).to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Self> for TransitMode {
    /// Converts a borrowed `&TransitMode` enum into an owned `TransitMode` enum
    /// by cloning it.
    fn from(transit_mode: &Self) -> Self {
        transit_mode.clone()
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

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for TransitMode {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = GoogleMapsError;
    /// Gets a `TransitMode` enum from a `String` that contains a valid [transit
    /// mode](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitMode)
    /// code.
    fn try_from(transit_mode_code: &str) -> Result<Self, Self::Error> {
        Ok(TRANSIT_MODES_BY_CODE
            .get(transit_mode_code)
            .cloned()
            .ok_or_else(|| {
                DirectionsError::InvalidTransitModeCode(transit_mode_code.to_string())
            })?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for TransitMode {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Err = GoogleMapsError;
    /// Gets a `TransitMode` enum from a `String` that contains a valid [transit
    /// mode](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitMode)
    /// code.
    fn from_str(transit_mode_code: &str) -> Result<Self, Self::Err> {
        Ok(TRANSIT_MODES_BY_CODE
            .get(transit_mode_code)
            .cloned()
            .ok_or_else(|| {
                DirectionsError::InvalidTransitModeCode(transit_mode_code.to_string())
            })?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TransitMode {
    /// Formats a `TransitMode` enum into a string that is presentable to the
    /// end user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::Bus => "Bus",
            Self::Rail => "Rail",
            Self::Subway => "Subway",
            Self::Train => "Train",
            Self::Tram => "Tram",
        } // match
    } // fn
} // impl
