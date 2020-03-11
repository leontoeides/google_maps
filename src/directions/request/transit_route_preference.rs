//! Contains the `TransitRoutePreference` enum and its associated traits. It is
//! used to prioritize _fewer bus transfers_ or _less walking_ is when
//! generating transit directions.

use crate::directions::error::Error;
use serde::{Serialize, Deserialize};

/// Specifies preferences for [transit
/// routes](https://developers.google.com/maps/documentation/directions/intro#optional-parameters).
///
/// Using this parameter, you can bias the options returned, rather than
/// accepting the default best route chosen by the API. This parameter may only
/// be specified for transit directions, and only if the request includes an API
/// key or a Google Maps Platform Premium Plan client ID.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum TransitRoutePreference {

    /// Indicates that the calculated route should prefer a limited number of
    /// transfers.
    #[serde(alias = "fewer_transfers")]
    FewerTransfers,

    /// Indicates that the calculated route should prefer limited amounts of
    /// walking.
    #[serde(alias = "less_walking")]
    LessWalking,

} // enum

impl std::convert::From<&TransitRoutePreference> for String {
    /// Converts a `TransitRoutePreference` enum to a `String` that contains a
    /// [transit route
    /// preference](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitRoutePreferences)
    /// code.
    fn from(transit_route_preference: &TransitRoutePreference) -> String {
        match transit_route_preference {
            TransitRoutePreference::FewerTransfers => String::from("fewer_transfers"),
            TransitRoutePreference::LessWalking => String::from("less_walking"),
        } // match
    } // fn
} // impl

impl std::convert::TryFrom<&str> for TransitRoutePreference {

    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;

    /// Gets a `TransitRoutePreference` enum from a `String` that contains a
    /// valid [transit route
    /// preference](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitRoutePreference)
    /// code.
    fn try_from(transit_route_preference: &str) -> Result<TransitRoutePreference, Error> {
        match transit_route_preference {
            "fewer_transfers" => Ok(TransitRoutePreference::FewerTransfers),
            "less_walking" => Ok(TransitRoutePreference::LessWalking),
            _ => Err(Error::InvalidTransitRoutePreferenceCode(transit_route_preference.to_string())),
        } // match
    } // fn

} // impl

impl std::default::Default for TransitRoutePreference {
    /// Returns a reasonable default variant for the `TransitRoutePreference`
    /// enum type.
    fn default() -> Self {
        TransitRoutePreference::FewerTransfers
    } // fn
} // impl

impl std::fmt::Display for TransitRoutePreference {
    /// Formats a `TransitRoutePreference` enum into a string that is
    /// presentable to the end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TransitRoutePreference::FewerTransfers => write!(f, "Fewer Transfers"),
            TransitRoutePreference::LessWalking => write!(f, "Less Walking"),
        } // match
    } // fn
} // impl