use crate::directions::request::{
    Request,
    transit_route_preference::TransitRoutePreference,
}; // use

impl Request {

    /// Specifies the preferences for transit routes.
    ///
    /// Description
    /// -----------
    ///
    /// Specifies preferences for transit routes. Using this parameter, you can
    /// bias the options returned, rather than accepting the default best route
    /// chosen by the API. This parameter may only be specified for transit
    /// directions, and only if the request includes an API key or a Google Maps
    /// Platform Premium Plan client ID. The parameter supports the following
    /// arguments:
    ///
    /// * `TransitRoutePreference::LessWalking` indicates that the calculated
    /// route should prefer limited amounts of walking.
    ///
    /// * `TransitRoutePreference::FewerTransfers` indicates that the
    /// calculated route should prefer a limited number of transfers.
    ///
    /// Example:
    /// ---------
    ///
    /// * Set transit route preference to fewer transfers:
    /// ```
    /// .with_transit_route_preference(TransitRoutePreference::FewerTransfers)
    /// ```

    pub fn with_transit_route_preference(&mut self, transit_route_preference: TransitRoutePreference) -> &mut Request {
        self.transit_route_preference = Some(transit_route_preference);
        self
    } // fn

} // impl