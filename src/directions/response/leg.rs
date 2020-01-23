use crate::{
    directions::response::{
        distance::Distance,
        duration::Duration,
        step::Step,
        time::Time,
    }, // directions::response
    latlng::LatLng,
}; // use
use serde::{Serialize, Deserialize};

/// A single leg consisting of a set of steps in a DirectionsResult. Some fields
/// in the leg may not be returned for all requests. Note that though this
/// result is "JSON-like," it is not strictly JSON, as it directly and
/// indirectly includes LatLng objects.

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Leg {

    /// An estimated arrival time for this leg. Only applicable for
    /// `TravelMode::Transit` requests.
    pub arrival_time: Option<Time>,

    /// An estimated departure time for this leg. Only applicable for
    /// `TravelMode::Transit` requests.
    pub departure_time: Option<Time>,

    /// The total distance covered by this leg. This property may be undefined
    /// as the distance may be unknown.
    pub distance: Distance,

    /// The total duration of this leg. This property may be undefined as the
    /// duration may be unknown.
    pub duration: Duration,

    /// The total duration of this leg, taking into account the traffic
    /// conditions indicated by the `with_traffic_model()` method. This property
    /// may be undefined as the duration may be unknown. Only available to
    /// Premium Plan customers.
    pub duration_in_traffic: Option<Duration>,

    /// The address of the destination of this leg.
    pub end_address: String,

    /// The Directions Service calculates directions between locations by using
    /// the nearest transportation option (usually a road) at the start and end
    /// locations. `end_location` indicates the actual geocoded destination,
    /// which may be different than the end_location of the last step if, for
    /// example, the road is not near the destination of this leg.
    pub end_location: LatLng,

    /// The address of the origin of this leg.
    pub start_address: String,

    /// The Directions Service calculates directions between locations by using
    /// the nearest transportation option (usually a road) at the start and end
    /// locations. `start_location` indicates the actual geocoded origin, which
    /// may be different than the `start_location` of the first step if, for
    /// example, the road is not near the origin of this leg.
    pub start_location: LatLng,

    /// An array of `Steps`, each of which contains information about the
    /// individual steps in this leg.
    pub steps: Vec<Step>,

} // struct