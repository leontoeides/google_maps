use crate::directions::request::transit_mode::TransitMode;
use crate::distance_matrix::request::Request;

impl<'a> Request<'a> {
    /// Specify the preferred mode of transit.
    ///
    /// ## Arguments
    ///
    /// * `transit_modes` ‧ The preference of the transit rider; what mode of
    ///   transit should the directions service prioritize? _Bus_, _Subway_,
    ///   _Train_, _Tram_, or _Rail_?
    ///
    /// ## Description
    ///
    /// Specifies one or more preferred modes of transit. This parameter may
    /// only be specified for transit directions, and only if the request
    /// includes an API key or a Google Maps Platform Premium Plan client ID.
    /// The parameter supports the following arguments:
    ///
    /// * `TransitMode::Bus` indicates that the calculated route should prefer
    ///   travel by bus.
    ///
    /// * `TransitMode::Subway` indicates that the calculated route should
    ///   prefer travel by subway.
    ///
    /// * `TransitMode::Train` indicates that the calculated route should prefer
    ///   travel by train.
    ///
    /// * `TransitMode::Tram` indicates that the calculated route should prefer
    ///   travel by tram and light rail.
    ///
    /// * `TransitMode::Rail` indicates that the calculated route should prefer
    ///   travel by train, tram, light rail, and subway. This is equivalent to
    ///   `TransitMode::Train|Tram|Subway`.
    ///
    /// ## Examples:
    ///
    /// * Set preferred transit mode to rail:
    ///
    /// ```rust
    /// .with_transit_mode(TransitMode::Rail)
    /// ```
    ///
    /// * Multiple modes may be stacked together. This example sets preferred
    ///   transit modes to bus and subway:
    ///
    /// ```rust
    /// .with_transit_mode(TransitMode::Bus)
    /// .with_transit_mode(TransitMode::Subway)
    /// ```

    pub fn with_transit_mode(
        &'a mut self,
        transit_mode: impl Into<TransitMode>
    ) -> &'a mut Self {
        // Add restiction to Request struct.
        self.transit_modes = vec![transit_mode.into()];
        // Return modified Request struct to caller.
        self
    } // fn

    /// Specifies preferred modes of transit.
    ///
    /// # Example:
    ///
    /// * Alternatively, multiple transit modes may be passed in a single method
    ///   call by passing a slice. This example sets preferred transit modes to
    ///   bus and subway:
    ///
    /// ```rust
    /// .with_transit_modes(&[
    ///     TransitMode::Bus,
    ///     TransitMode::Subway,
    /// ])
    /// ```
    ///
    /// # Generics
    ///
    /// This method uses generics to improve ergonomics. The `C` generic is
    /// intended to represent any collection that can be iterated over, and the
    /// `T` generic is for any type that can be converted to the `TransitMode`
    /// type.

    pub fn with_transit_modes<C, T>(
        &'a mut self,
        transit_modes: C
    ) -> &'a mut Self
    where
        C: IntoIterator<Item = T>,
        T: Into<TransitMode> {
        // Add transit_modes to Request struct.
        self.transit_modes = transit_modes.into_iter().map(Into::into).collect();
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
