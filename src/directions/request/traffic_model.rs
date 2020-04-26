//! Contains the `TrafficModel` enum and its associated traits. It is used to
//! select a traffic model that is as accurate as possible, optimistic, or
//! pessimistic.

use crate::directions::error::Error;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum TrafficModel {
    /// Indicates that the returned `duration_in_traffic` should be the best
    /// estimate of travel time given what is known about both historical
    /// traffic conditions and live traffic. Live traffic becomes more important
    /// the closer the `departure_time` is to now.
    #[serde(alias = "best_guess")]
    BestGuess,

    /// Indicates that the returned duration_in_traffic should be shorter than
    /// the actual travel time on most days, though occasional days with
    /// particularly good traffic conditions may be faster than this value.
    #[serde(alias = "optimistic")]
    Optimistic,

    /// Indicates that the returned `duration_in_traffic` should be longer than
    /// the actual travel time on most days, though occasional days with
    /// particularly bad traffic conditions may exceed this value.
    #[serde(alias = "pessimistic")]
    Pessimistic,
} // enum

impl std::convert::From<&TrafficModel> for String {
    /// Converts a `TrafficModel` enum to a `String` that contains a [traffic
    /// model](https://developers.google.com/maps/documentation/javascript/reference/directions#TrafficModel)
    /// code.
    fn from(traffic_model: &TrafficModel) -> String {
        match traffic_model {
            TrafficModel::BestGuess => String::from("best_guess"),
            TrafficModel::Optimistic => String::from("optimistic"),
            TrafficModel::Pessimistic => String::from("pessimistic"),
        } // match
    } // fn
} // impl

impl std::convert::TryFrom<&str> for TrafficModel {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;

    /// Gets a `TrafficModel` enum from a `String` that contains a valid
    /// [traffic
    /// model](https://developers.google.com/maps/documentation/javascript/reference/directions#TrafficModel)
    /// code.
    fn try_from(traffic_model: &str) -> Result<TrafficModel, Error> {
        match traffic_model {
            "best_guess" => Ok(TrafficModel::BestGuess),
            "optimistic" => Ok(TrafficModel::Optimistic),
            "pessimistic" => Ok(TrafficModel::Pessimistic),
            _ => Err(Error::InvalidTrafficModelCode(traffic_model.to_string())),
        } // match
    } // fn
} // impl

impl std::default::Default for TrafficModel {
    /// Returns a reasonable default variant for the `TrafficModel` enum type.
    fn default() -> Self {
        TrafficModel::BestGuess
    } // fn
} // impl

impl std::fmt::Display for TrafficModel {
    /// Formats a `TrafficModel` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TrafficModel::BestGuess => write!(f, "Best Guess"),
            TrafficModel::Optimistic => write!(f, "Optimistic"),
            TrafficModel::Pessimistic => write!(f, "Pessimistic"),
        } // match
    } // fn
} // impl
