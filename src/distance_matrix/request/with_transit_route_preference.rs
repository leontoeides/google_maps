use crate::directions::request::transit_route_preference::TransitRoutePreference;
use crate::distance_matrix::request::Request;

impl<'a> Request<'a> {
    /// Specifies the preference for transit routes.
    ///
    /// ## Arguments
    ///
    /// * `transit_route_preference` ‧ The preference of the transit rider;
    ///   should the directions service try to reduce the amount of _walking_ to
    ///   reach the destination, or reduce the number of bus _transfers_?
    ///
    /// ## Description
    ///
    /// Specifies preferences for transit routes. Using this parameter, you can
    /// bias the options returned, rather than accepting the default best route
    /// chosen by the API. This parameter may only be specified for transit
    /// directions, and only if the request includes an API key or a Google Maps
    /// Platform Premium Plan client ID. The parameter supports the following
    /// arguments:
    ///
    /// * `TransitRoutePreference::LessWalking` indicates that the calculated
    ///   route should prefer limited amounts of walking.
    ///
    /// * `TransitRoutePreference::FewerTransfers` indicates that the
    ///   calculated route should prefer a limited number of transfers.
    ///
    /// ## Example
    ///
    /// * Set transit route preference to fewer transfers:
    /// ```rust
    /// .with_transit_route_preference(TransitRoutePreference::FewerTransfers)
    /// ```

    pub fn with_transit_route_preference(
        &'a mut self,
        transit_route_preference: impl Into<TransitRoutePreference>
    ) -> &'a mut Self {
        self.transit_route_preference = Some(transit_route_preference.into());
        self
    } // fn
} // impl
