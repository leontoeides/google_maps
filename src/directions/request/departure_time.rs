use time::PrimitiveDateTime;

/// The desired departure time for the route. The departure time must be set to
/// the current time or some time in the future. It cannot be in the past.
#[derive(Clone, Debug)]
pub enum DepartureTime {
    /// You can specify a value of now, which sets the departure time to the
    /// current time (correct to the nearest second).
    Now,
    /// Specifies the desired time of departure.
    At(PrimitiveDateTime),
} // enum

impl DepartureTime {
    /// Converts a `DepartureTime` enum to a `String` that contains a pretty
    /// source-code-style [departure
    /// time](https://developers.google.com/maps/documentation/directions/intro#optional-parameters)
    /// code for debugging.
    pub fn source_code_print(&self) -> String {
        match self {
            DepartureTime::Now => String::from("DepartureTime::Now"),
            DepartureTime::At(departure_time) => departure_time.format("%F %r"),
        } // match
    } // fn
} // impl

impl From<&DepartureTime> for String {
    /// Converts a `DepartureTime` enum to a `String` that contains a [departure
    /// time](https://developers.google.com/maps/documentation/directions/intro#optional-parameters).
    fn from(departure_time: &DepartureTime) -> String {
        match departure_time {
            DepartureTime::Now => String::from("now"),
            DepartureTime::At(departure_time) => format!("{}", departure_time.timestamp()),
        } // match
    } // fn
} // impl