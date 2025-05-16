//! Transit directions return additional information.

use crate::directions::response::{
    transit_line::TransitLine, transit_stop::TransitStop, transit_time::TransitTime,
}; // use crate::directions::response
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Transit directions return additional information that is not relevant for
/// other modes of transportation. These additional properties are exposed
/// through the `transit_details` object, returned as a field of an element in
/// the `steps[]` array. From the `TransitDetails` object you can access
/// additional information about the transit stop, transit line and transit
/// agency.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TransitDetails {
    /// Contains information about the stop/station for this part of the trip.
    pub arrival_stop: TransitStop,

    /// Contains the arrival for this leg of the journey.
    pub arrival_time: TransitTime,

    /// Contains information about the stop/station for this part of the trip.
    pub departure_stop: TransitStop,

    /// Contains the departure for this leg of the journey.
    pub departure_time: TransitTime,

    /// Specifies the direction in which to travel on this line, as it is marked
    /// on the vehicle or at the departure stop. This will often be the terminus
    /// station.
    pub headsign: String,

    /// Specifies the expected number of seconds between departures from the
    /// same stop at this time. For example, with a headway value of 600, you
    /// would expect a ten minute wait if you should miss your bus.
    #[serde(default)]
    pub headway: Option<u16>,

    /// Contains information about the transit line used in this step.
    pub line: TransitLine,

    /// Contains the number of stops in this step, counting the arrival stop,
    /// but not the departure stop. For example, if your directions involve
    /// leaving from Stop A, passing through stops B and C, and arriving at stop
    /// D, `num_stops` will return 3.
    pub num_stops: u8,

    /// Contains the text that appears in schedules and sign boards to identify
    /// a transit trip to passengers. The text should uniquely identify a trip
    /// within a service day. For example, "538" is the `trip_short_name` of the
    /// Amtrak train that leaves San Jose, CA at 15:10 on weekdays to
    /// Sacramento, CA.
    #[serde(default)]
    pub trip_short_name: Option<String>,
} // struct
