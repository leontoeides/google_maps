use crate::directions::{request::Request, travel_mode::TravelMode};

impl<'a> Request<'a> {
    /// Specify the mode of transportation.
    ///
    /// ## Arguments
    ///
    /// * `travel_mode` ‧ The mode of transportation that directions should be
    ///   calculated for. For example, _transit_ directions or car _driving_
    ///   directions.
    ///
    /// ## Description
    ///
    /// [Travel Modes](https://developers.google.com/maps/documentation/directions/intro#TravelModes)
    ///
    /// When you calculate directions, you may specify the transportation mode
    /// to use. By default, directions are calculated as driving directions. The
    /// following travel modes are supported:
    ///
    /// * `TravelMode::Driving` (default) indicates standard driving directions
    ///   using the road network.
    ///
    /// * `TravelMode::Walking` requests walking directions via pedestrian paths
    ///   & sidewalks (where available).
    ///
    /// * `TravelMode::Bicycling` requests bicycling directions via bicycle
    ///   paths & preferred streets (where available).
    ///
    /// * `TravelMode::Transit` requests directions via public transit routes
    ///   (where available). If you set the travel mode to
    ///   `TravelMode::Transit`, you can optionally use either
    ///   `with_departure_time()` or `with_arrival_time()` methods. If neither
    ///   time is specified, the departure time defaults to now (that is, the
    ///   departure time defaults to the current time). You can also optionally
    ///   use `with_transit_mode()` and/or `with_transit_route_preference()`
    ///   methods.
    ///
    /// _Note_: Both walking and bicycling directions may sometimes not include
    /// clear pedestrian or bicycling paths, so these directions will return
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
