//! Contains the `DepartureTime` enum and its associated traits. `DepartureTime`
//! is used to specify when the user would like to depart for traffic modelling
//! and transit directions.

use time::PrimitiveDateTime;

/// Specifies the desired [time of
/// departure](https://developers.google.com/maps/documentation/directions/intro#optional-parameters).
///
/// You can specify the time or alternatively you can specify a value of now,
/// which sets the departure time to the current time (correct to the nearest
/// second).
///
/// * For requests where the travel mode is transit: You can optionally specify
/// one of `departure_time` or `arrival_time`. If neither time is specified, the
/// `departure_time` defaults to now (that is, the departure time defaults to
/// the current time).
///
/// * For requests where the travel mode is driving: You can specify the
/// `departure_time` to receive a route and trip duration (response field:
/// `duration_in_traffic`) that take traffic conditions into account. This
/// option is only available if the request contains a valid API key, or a valid
/// Google Maps Platform Premium Plan client ID and signature. The
/// `departure_time` must be set to the current time or some time in the future.
/// It cannot be in the past.
///
/// **Note**: If departure time is not specified, choice of route and duration
/// are based on road network and average time-independent traffic conditions.
/// Results for a given request may vary over time due to changes in the road
/// network, updated average traffic conditions, and the distributed nature of
/// the service. Results may also vary between nearly-equivalent routes at any
/// time or frequency.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DepartureTime {

    /// You can specify a value of now, which sets the departure time to the
    /// current time (correct to the nearest second).
    Now,

    /// Specifies the desired time of departure.
    At(PrimitiveDateTime),

} // enum

impl std::convert::From<&DepartureTime> for String {
    /// Converts a `DepartureTime` enum to a `String` that contains a [departure
    /// time](https://developers.google.com/maps/documentation/directions/intro#optional-parameters).
    fn from(departure_time: &DepartureTime) -> String {
        match departure_time {
            DepartureTime::Now => String::from("now"),
            DepartureTime::At(departure_time) => format!("{}", departure_time.timestamp()),
        } // match
    } // fn
} // impl

impl std::default::Default for DepartureTime {
    /// Returns a reasonable default variant for the `DepartureTime` enum type.
    fn default() -> Self {
        DepartureTime::Now
    } // fn
} // impl

impl std::fmt::Display for DepartureTime {
    /// Formats a `DepartureTime` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DepartureTime::Now => write!(f, "Now"),
            DepartureTime::At(departure_time) => write!(f, "At {}", departure_time.format("At %F %r")),
        } // match
    } // fn
} // impl