use crate::directions::request::{transit_mode::TransitMode, Request};

impl<'a> Request<'a> {

    /// Specify the preferred mode of transit.
    ///
    /// ## Arguments
    ///
    /// * `transit_modes` ‧ The preference of the transit rider; what mode of
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
    /// ```rust
    /// .with_transit_mode((vec![TransitMode::Rail]))
    /// ```
    ///
    /// * Set preferred transit modes to bus and subway:
    /// ```rust
    /// .with_transit_modes(vec![
    ///     TransitMode::Bus,
    ///     TransitMode::Subway
    /// ])
    /// ```

    pub fn with_transit_mode(
        &'a mut self,
        transit_mode: TransitMode
    ) -> &'a mut Request {
        // Add restiction to Request struct.
        match &mut self.transit_modes {
            // If there are no transit modes in the request struct, initialize:
            None => self.transit_modes = Some(vec![transit_mode]),
            // If there are already transit modes, append to them:
            Some(transit_modes) => transit_modes.push(transit_mode),
        } // match
        // Return modified Request struct to caller.
        self
    } // fn

    /// Specifies preferred modes of transit.
    ///
    /// # Example:
    ///
    /// * Alternatively, multiple transit modes may be passed in a single method
    /// call by passing a Vec. This example sets preferred transit modes to bus
    /// and subway:
    ///
    /// ```rust
    /// .with_transit_modes(vec![
    ///     TransitMode::Bus,
    ///     TransitMode::Subway,
    /// ])
    /// ```

    pub fn with_transit_modes(
        &'a mut self,
        transit_modes_slice: &[TransitMode],
    ) -> &'a mut Request {
        // Add transit_modes to Request struct.
        match &mut self.transit_modes {
            // If there are no transit modes in the request struct, initialize:
            None => self.transit_modes = Some(transit_modes_slice.to_vec()),
            // If there are already transit modes, append to them:
            Some(transit_modes) => transit_modes_slice.iter().for_each(|transit_mode|
                transit_modes.push(transit_mode.to_owned())
            ), // iter
        } // match
        // Return modified Request struct to caller.
        self
    } // fn

} // impl