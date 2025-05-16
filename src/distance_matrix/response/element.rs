use crate::{
    directions::response::{
        directions_distance::DirectionsDistance, directions_duration::DirectionsDuration,
        transit_fare::TransitFare,
    },
    distance_matrix::response::element_status::ElementStatus,
};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// The information about each origin-destination pairing is returned in an
/// element entry.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Element {
    /// The total distance of this route, expressed in meters (`value`) and as
    /// `text`. The textual value uses the unit system specified with the unit
    /// parameter of the original request, or the origin's region.
    #[serde(default)]
    pub distance: Option<DirectionsDistance>,

    /// The length of time it takes to travel this route, expressed in seconds
    /// (the `value` field) and as `text`. The textual representation is
    /// localized according to the query's language parameter.
    #[serde(default)]
    pub duration: Option<DirectionsDuration>,

    /// The length of time it takes to travel this route, based on current and
    /// historical traffic conditions. See the `traffic_model` request parameter
    /// for the options you can use to request that the returned value is
    /// optimistic, pessimistic, or a best-guess estimate. The duration is
    /// expressed in seconds (the `value` field) and as `text`. The textual
    /// representation is localized according to the query's language parameter.
    #[serde(default)]
    pub duration_in_traffic: Option<DirectionsDuration>,

    /// If present, contains the total fare (that is, the total ticket costs) on
    /// this route. This property is only returned for transit requests and only
    /// for transit providers where fare information is available.
    #[serde(default)]
    pub fare: Option<TransitFare>,

    /// See [Status
    /// Codes](https://developers.google.com/maps/documentation/distance-matrix/intro#StatusCodes)
    /// for a list of possible status codes.
    pub status: ElementStatus,
} // struct