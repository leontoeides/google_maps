use chrono::NaiveDateTime;

// -----------------------------------------------------------------------------

impl crate::distance_matrix::Request<'_> {
    /// Specifies the desired arrival time.
    ///
    /// ## Arguments
    ///
    /// * `arrival_time` ‧ The time the passenger should arrive at their final
    ///   destination by.
    ///
    /// ## Description
    ///
    /// Specifies the desired time of arrival for _transit_ distances. You can
    /// use either the `.with_departure_time()` or the `.with_arrival_time()`
    /// method, but not both together.
    ///
    /// ## Example
    ///
    /// * Arriving by January 1, 2019 at 12:00:00 AM:
    /// ```rust
    /// .with_arrival_time(NaiveDate::from_ymd(2019, 1, 1).and_hms(0, 00, 0))
    /// ```
    #[must_use] pub fn with_arrival_time(
        mut self,
        arrival_time: impl Into<NaiveDateTime>
    ) -> Self {
        let arrival_time: NaiveDateTime = arrival_time.into();
        self.arrival_time = Some(arrival_time);
        self
    } // fn
} // impl