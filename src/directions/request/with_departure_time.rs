use crate::directions::request::{departure_time::DepartureTime, Request};

impl<'a> Request<'a> {
    /// Specifies the desired departure time.
    ///
    /// ## Arguments
    ///
    /// * `departure_time` ‧ The soonest time the passenger intends to depart.
    ///   May be "now" or a specified time.
    ///
    /// ## Description
    ///
    /// Specifies the desired time of departure. Alternatively, you can specify
    /// a value of now, which sets the departure time to the current time
    /// (correct to the nearest second). The departure time may be specified in
    /// two cases:
    ///
    /// * For requests where the travel mode is transit: You can optionally
    ///   use one of `.with_departure_time()` or `.with_arrival_time()` methods.
    ///   If neither time is specified, the departure time defaults to now (that
    ///   is, the departure time defaults to the current time).
    ///
    /// * For requests where the travel mode is driving: You can specify the
    ///   `departure_time` to receive a route and trip duration (response field:
    ///   `duration_in_traffic`) that take traffic conditions into account. This
    ///   option is only available if the request contains a valid API key, or a
    ///   valid Google Maps Platform Premium Plan client ID and signature. The
    ///   departure time must be set to the current time or some time in the
    ///   future. It cannot be in the past.
    ///
    /// * Note: If departure time is not specified, choice of route and duration
    ///   are based on road network and average time-independent traffic
    ///   conditions. Results for a given request may vary over time due to
    ///   changes in the road network, updated average traffic conditions, and the
    ///   distributed nature of the service. Results may also vary between
    ///   nearly-equivalent routes at any time or frequency.
    ///
    /// ## Examples:
    ///
    /// * Departing now:
    /// ```rust
    /// .with_departure_time(DepartureTime::Now)
    /// ```
    ///
    /// * Departing on Tuesday February 22, 2022 at 1:00:00 PM:
    /// ```rust
    /// .with_departure_time(DepartureTime::At(
    ///     NaiveDate::from_ymd(2022, 2, 22).and_hms(13, 00, 0)
    /// ))
    /// ```
    ///
    /// * Departing on Tuesday January 1, 2030 at 12:30:00 PM:
    /// ```rust
    /// .with_departure_time(DepartureTime::At(
    ///     NaiveDate::from_ymd(2030, 1, 1).and_hms(12, 30, 0)
    /// ))
    /// ```

    pub fn with_departure_time(
        &'a mut self,
        departure_time: impl Into<DepartureTime>
    ) -> &'a mut Self {
        self.departure_time = Some(departure_time.into());
        self
    } // fn
} // impl
