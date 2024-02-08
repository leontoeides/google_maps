//! Contains the `TransitRoutePreference` enum and its associated traits. It is
//! used to prioritize _fewer bus transfers_ or _less walking_ is when
//! generating transit directions.

use crate::directions::error::Error as DirectionsError;
use crate::error::Error as GoogleMapsError;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------

/// Specifies preferences for [transit
/// routes](https://developers.google.com/maps/documentation/directions/intro#optional-parameters).
///
/// Using this parameter, you can bias the options returned, rather than
/// accepting the default best route chosen by the API. This parameter may only
/// be specified for transit directions, and only if the request includes an API
/// key or a Google Maps Platform Premium Plan client ID.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum TransitRoutePreference {
    /// Indicates that the calculated route should prefer a limited number of
    /// transfers.
    #[default]
    FewerTransfers = 0,
    /// Indicates that the calculated route should prefer limited amounts of
    /// walking.
    LessWalking = 1,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for TransitRoutePreference {
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

impl Serialize for TransitRoutePreference {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&TransitRoutePreference> for &str {
    /// Converts a `TransitRoutePreference` enum to a `String` that contains a
    /// [transit route
    /// preference](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitRoutePreferences)
    /// code.
    fn from(transit_route_preference: &TransitRoutePreference) -> Self {
        match transit_route_preference {
            TransitRoutePreference::FewerTransfers => "fewer_transfers",
            TransitRoutePreference::LessWalking => "less_walking",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for TransitRoutePreference {
    /// Converts a `TransitRoutePreference` enum to a `String` that contains a
    /// [transit route
    /// preference](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitRoutePreferences)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&TransitRoutePreference> for String {
    /// Converts a `TransitRoutePreference` enum to a `String` that contains a
    /// [transit route
    /// preference](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitRoutePreferences)
    /// code.
    fn from(transit_route_preference: &TransitRoutePreference) -> Self {
        std::convert::Into::<&str>::into(transit_route_preference).to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

static TRANSIT_ROUTE_PREFERENCE_BY_CODE: phf::Map<&'static str, TransitRoutePreference> = phf_map! {
    "fewer_transfers" => TransitRoutePreference::FewerTransfers,
    "less_walking" => TransitRoutePreference::LessWalking,
};

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for TransitRoutePreference {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = GoogleMapsError;
    /// Gets a `TransitRoutePreference` enum from a `String` that contains a
    /// valid [transit route
    /// preference](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitRoutePreference)
    /// code.
    fn try_from(transit_route_preference_code: &str) -> Result<Self, Self::Error> {
        Ok(TRANSIT_ROUTE_PREFERENCE_BY_CODE
            .get(transit_route_preference_code)
            .cloned()
            .ok_or_else(|| {
                DirectionsError::InvalidTransitRoutePreferenceCode(
                    transit_route_preference_code.to_string(),
                )
            })?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for TransitRoutePreference {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Err = GoogleMapsError;
    /// Gets a `TransitRoutePreference` enum from a `String` that contains a
    /// valid [transit route
    /// preference](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitRoutePreference)
    /// code.
    fn from_str(transit_route_preference_code: &str) -> Result<Self, Self::Err> {
        Ok(TRANSIT_ROUTE_PREFERENCE_BY_CODE
            .get(transit_route_preference_code)
            .cloned()
            .ok_or_else(|| {
                DirectionsError::InvalidTransitRoutePreferenceCode(
                    transit_route_preference_code.to_string(),
                )
            })?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TransitRoutePreference {
    /// Formats a `TransitRoutePreference` enum into a string that is
    /// presentable to the end user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::FewerTransfers => "Fewer Transfers",
            Self::LessWalking => "Less Walking",
        } // match
    } // fn
} // impl
