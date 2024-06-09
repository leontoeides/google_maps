use crate::directions::travel_mode::TravelMode;
use crate::distance_matrix::request::Request;

impl<'a> Request<'a> {
    /// Specify the mode of transportation.
    ///
    /// ## Arguments
    ///
    /// * `travel_mode` ‧ The mode of transportation that distance matrix should
    ///   be calculated for. For example, _transit_ distances or car _driving_
    ///   distances.
    ///
    /// ## Description
    ///
    /// [Travel Modes](https://developers.google.com/maps/documentation/distance-matrix/intro#travel_modes)
    ///
    /// For the calculation of distances, you may specify the transportation
    /// mode to use. By default, distances are calculated for driving mode.
    ///
    /// * `TravelMode::Driving` (default) indicates distance calculation using
    ///   the road network.
    ///
    /// * `TravelMode::Walking` requests distance calculation for walking via
    ///   pedestrian paths & sidewalks (where available).
    ///
    /// * `TravelMode::Bicycling` requests distance calculation for bicycling
    ///   via bicycle paths & preferred streets (where available).
    ///
    /// * `TravelMode::Transit` requests distance calculation via public transit
    ///   routes (where available). If you set the travel mode to
    ///   `TravelMode::Transit`, you can optionally use either
    ///   `with_departure_time()` or `with_arrival_time()` methods. If neither
    ///   time is specified, the departure time defaults to now (that is, the
    ///   departure time defaults to the current time). You can also optionally
    ///   use `with_transit_mode()` and/or `with_transit_route_preference()`
    ///   methods.
    ///
    /// _Note_: Both walking and bicycling routes may sometimes not include
    /// clear pedestrian or bicycling paths, so these responses will return
    /// `warnings` in the returned result which you must display to the user.
    ///
    /// ## Example
    ///
    /// * Set travel mode to transit:
    /// ```rust
    /// .with_travel_mode(TravelMode::Transit)
    /// ```

    pub fn with_travel_mode(
        &'a mut self,
        travel_mode: impl Into<TravelMode>
    ) -> &'a mut Self {
        self.travel_mode = Some(travel_mode.into());
        self
    } // fn
} // impl
