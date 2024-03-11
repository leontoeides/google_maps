//! Contains the `TrafficModel` enum and its associated traits. It is used to
//! select a traffic model that is as accurate as possible, optimistic, or
//! pessimistic.

use crate::directions::error::Error as DirectionsError;
use crate::error::Error as GoogleMapsError;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------

/// Specifies the [traffic
/// model](https://developers.google.com/maps/documentation/directions/intro#optional-parameters)
/// to use when calculating time in traffic.
///
/// This setting affects the value returned in the `duration_in_traffic` field
/// in the response, which contains the predicted time in traffic based on
/// historical averages. The `traffic_model` parameter may only be specified for
/// driving directions where the request includes a `departure_time`, and only
/// if the request includes an API key or a Google Maps Platform Premium Plan
/// client ID.
///
/// The default value of `best_guess` will give the most useful predictions for
/// the vast majority of use cases. It is possible the `best_guess` travel time
/// prediction may be _shorter_ than `optimistic`, or alternatively, _longer_
/// than `pessimistic`, due to the way the `best_guess` prediction model
/// integrates live traffic information.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum TrafficModel {
    /// Indicates that the returned `duration_in_traffic` should be the best
    /// estimate of travel time given what is known about both historical
    /// traffic conditions and live traffic. Live traffic becomes more important
    /// the closer the `departure_time` is to now.
    #[default]
    BestGuess = 0,
    /// Indicates that the returned `duration_in_traffic` should be shorter than
    /// the actual travel time on most days, though occasional days with
    /// particularly good traffic conditions may be faster than this value.
    Optimistic = 1,
    /// Indicates that the returned `duration_in_traffic` should be longer than
    /// the actual travel time on most days, though occasional days with
    /// particularly bad traffic conditions may exceed this value.
    Pessimistic = 2,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for TrafficModel {
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

impl Serialize for TrafficModel {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&TrafficModel> for &str {
    /// Converts a `TrafficModel` enum to a `String` that contains a [traffic
    /// model](https://developers.google.com/maps/documentation/javascript/reference/directions#TrafficModel)
    /// code.
    fn from(traffic_model: &TrafficModel) -> Self {
        match traffic_model {
            TrafficModel::BestGuess => "best_guess",
            TrafficModel::Optimistic => "optimistic",
            TrafficModel::Pessimistic => "pessimistic",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for TrafficModel {
    /// Converts a `TrafficModel` enum to a `String` that contains a [traffic
    /// model](https://developers.google.com/maps/documentation/javascript/reference/directions#TrafficModel)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&TrafficModel> for String {
    /// Converts a `TrafficModel` enum to a `String` that contains a [traffic
    /// model](https://developers.google.com/maps/documentation/javascript/reference/directions#TrafficModel)
    /// code.
    fn from(traffic_model: &TrafficModel) -> Self {
        std::convert::Into::<&str>::into(traffic_model).to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

static TRAFFIC_MODELS_BY_CODE: phf::Map<&'static str, TrafficModel> = phf_map! {
    "best_guess" => TrafficModel::BestGuess,
    "optimistic" => TrafficModel::Optimistic,
    "pessimistic" => TrafficModel::Pessimistic,
};

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for TrafficModel {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = GoogleMapsError;
    /// Gets a `TrafficModel` enum from a `String` that contains a valid
    /// [traffic
    /// model](https://developers.google.com/maps/documentation/javascript/reference/directions#TrafficModel)
    /// code.
    fn try_from(traffic_model_code: &str) -> Result<Self, Self::Error> {
        Ok(TRAFFIC_MODELS_BY_CODE
            .get(traffic_model_code)
            .cloned()
            .ok_or_else(|| {
                DirectionsError::InvalidTrafficModelCode(traffic_model_code.to_string())
            })?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for TrafficModel {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Err = GoogleMapsError;
    /// Gets a `TrafficModel` enum from a `String` that contains a valid
    /// [traffic
    /// model](https://developers.google.com/maps/documentation/javascript/reference/directions#TrafficModel)
    /// code.
    fn from_str(traffic_model_code: &str) -> Result<Self, Self::Err> {
        Ok(TRAFFIC_MODELS_BY_CODE
            .get(traffic_model_code)
            .cloned()
            .ok_or_else(|| {
                DirectionsError::InvalidTrafficModelCode(traffic_model_code.to_string())
            })?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TrafficModel {
    /// Formats a `TrafficModel` enum into a string that is presentable to the
    /// end user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::BestGuess => "Best Guess",
            Self::Optimistic => "Optimistic",
            Self::Pessimistic => "Pessimistic",
        } // match
    } // fn
} // impl
