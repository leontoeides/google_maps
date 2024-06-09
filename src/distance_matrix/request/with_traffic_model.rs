use crate::directions::request::traffic_model::TrafficModel;
use crate::distance_matrix::request::Request;

impl<'a> Request<'a> {
    /// Specifies the assumptions to use when calculating time in traffic.
    ///
    /// ## Arguments
    ///
    /// * `traffic_model` ‧ Which traffic model the directions service should
    ///   use when calculating the route and duration in traffic - _best guess_,
    ///   _optimistic_, or _pessimistic_?
    ///
    /// ## Description
    ///
    /// Specifies the assumptions to use when calculating time in traffic. This
    /// setting affects the value returned in the `duration_in_traffic` field in
    /// the response, which contains the predicted time in traffic based on
    /// historical averages. The `traffic_model` parameter may only be specified
    /// for driving directions where the request includes a `departure_time`,
    /// and only if the request includes an API key or a Google Maps Platform
    /// Premium Plan client ID. The available values for this parameter are:
    ///
    /// * `TrafficModel::BestGuess` (default) indicates that the returned
    ///   `duration_in_traffic` should be the best estimate of travel time given
    ///   what is known about both historical traffic conditions and live
    ///   traffic. Live traffic becomes more important the closer the
    ///   `departure_time` is to now.
    ///
    /// * `TrafficModel::Pessimistic` indicates that the returned
    ///   `duration_in_traffic` should be longer than the actual travel time on
    ///   most days, though occasional days with particularly bad traffic
    ///   conditions may exceed this value.
    ///
    /// * `TrafficModel::Optimistic` indicates that the returned
    ///   `duration_in_traffic` should be shorter than the actual travel time on
    ///   most days, though occasional days with particularly good traffic
    ///   conditions may be faster than this value.
    ///
    /// The default value of `BestGuess` will give the most useful predictions
    /// for the vast majority of use cases. It is possible the `BestGuess`
    /// travel time prediction may be _shorter_ than `Optimistic`, or
    /// alternatively, _longer_ than `Pessimistic`, due to the way the
    /// `BestGuess` prediction model integrates live traffic information.
    ///
    /// ## Example
    ///
    /// * Set traffic model to pessimistic:
    /// ```rust
    /// .with_traffic_model(TrafficModel::Pessimistic)
    /// ```

    pub fn with_traffic_model(
        &'a mut self,
        traffic_model: impl Into<TrafficModel>
    ) -> &'a mut Self {
        self.traffic_model = Some(traffic_model.into());
        self
    } // fn
} // impl
