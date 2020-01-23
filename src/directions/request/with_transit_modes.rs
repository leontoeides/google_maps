use crate::directions::request::{
    Request,
    transit_mode::TransitMode,
}; // use

impl Request {

    /// Specifies preferred modes of transit.
    ///
    /// ## Arguments
    ///
    /// * `transit_modes` · The preference of the transit rider; what mode of
    /// transit should the directions service prioritize? _Bus_, _Subway_,
    /// _Train_, _Tram_, or _Rail_?
    ///
    /// ## Description
    ///
    /// Specifies one or more preferred modes of transit. This parameter may
    /// only be specified for transit directions, and only if the request
    /// includes an API key or a Google Maps Platform Premium Plan client ID.
    /// The parameter supports the following arguments:
    ///
    /// * `TransitMode::Bus` indicates that the calculated route should prefer
    /// travel by bus.
    ///
    /// * `TransitMode::Subway` indicates that the calculated route should
    /// prefer travel by subway.
    ///
    /// * `TransitMode::Train` indicates that the calculated route should prefer
    /// travel by train.
    ///
    /// * `TransitMode::Tram` indicates that the calculated route should prefer
    /// travel by tram and light rail.
    ///
    /// * `TransitMode::Rail` indicates that the calculated route should prefer
    /// travel by train, tram, light rail, and subway. This is equivalent to
    /// `TransitMode::Train|Tram|Subway`.
    ///
    /// ## Examples:
    ///
    /// * Set preferred transit mode to rail:
    /// ```
    /// .with_transit_mode((vec![TransitMode::Rail])
    /// ```
    ///
    /// * Set preferred transit modes to bus and subway:
    /// ```
    /// .with_transit_modes(vec![
    ///     TransitMode::Bus,
    ///     TransitMode::Subway
    /// ])
    /// ```

    pub fn with_transit_modes(&mut self, transit_modes: Vec<TransitMode>) -> &mut Request {
        self.transit_modes = Some(transit_modes);
        self
    } // fn

} // impl