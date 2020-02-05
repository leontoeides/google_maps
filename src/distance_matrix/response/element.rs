use crate::{
    directions::response::{
        distance::Distance,
        duration::Duration,
        fare::Fare,
    }, // directions::response
    distance_matrix::response::element_status::ElementStatus,
}; // use
use serde::{Serialize, Deserialize};

/// The information about each origin-destination pairing is returned in an
/// element entry.

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Element {

    /// The total distance of this route, expressed in meters (`value`) and as
    /// `text`. The textual value uses the unit system specified with the unit
    /// parameter of the original request, or the origin's region.
    distance: Option<Distance>,

    /// The length of time it takes to travel this route, expressed in seconds
    /// (the `value` field) and as `text`. The textual representation is
    /// localized according to the query's language parameter.
    duration: Option<Duration>,

    /// The length of time it takes to travel this route, based on current and
    /// historical traffic conditions. See the `traffic_model` request parameter
    /// for the options you can use to request that the returned value is
    /// optimistic, pessimistic, or a best-guess estimate. The duration is
    /// expressed in seconds (the `value` field) and as `text`. The textual
    /// representation is localized according to the query's language parameter.
    duration_in_traffic: Option<Duration>,

    /// If present, contains the total fare (that is, the total ticket costs) on
    /// this route. This property is only returned for transit requests and only
    /// for transit providers where fare information is available.
    fare: Option<Fare>,

    /// See [Status
    /// Codes](https://developers.google.com/maps/documentation/distance-matrix/intro#StatusCodes)
    /// for a list of possible status codes.
    status: ElementStatus,

} // struct