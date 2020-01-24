use crate::directions::response::distance::Distance;
use crate::directions::response::duration::Duration;
use crate::directions::response::fare::Fare;
use crate::distance_matrix::response::element_status::ElementStatus;
use serde::{Serialize, Deserialize};

/// The information about each origin-destination pairing is returned in an
/// element entry.

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Element {

    distance: Distance,
    duration: Duration,
    duration_in_traffic: Option<Duration>,
    fare: Option<Fare>,
    status: ElementStatus,

} // struct