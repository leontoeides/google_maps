use crate::geocoding::forward::ForwardRequest;

impl<'a> ForwardRequest<'a> {
    /// Specifies the place id to geocode.
    ///
    /// ## Arguments
    ///
    /// * `place_id` - `TThe` place ID of the place for which you wish to obtain
    ///   the human-readable address. The place ID is a unique identifier that
    ///   can be used with other Google APIs. For example, you can use the
    ///   placeID returned by the [Roads
    ///   API](https://developers.google.com/maps/documentation/roads/snap) to
    ///   get the address for a snapped point.
    ///   For more information about place IDs, see the place [ID
    ///   overview](https://developers.google.com/maps/documentation/places/web-service/place-id).
    ///
    /// ## Example
    ///
    /// ```rust
    /// .with_place_id(
    ///     "ChIJd8BlQ2BZwokRAFUEcm_qrcA"
    /// )
    /// ```

    pub fn with_place_id(
        &'a mut self,
        place_id: impl Into<String>
    ) -> &'a mut Self {
        // Set address in ForwardRequest struct.
        self.place_id = Some(place_id.into());
        // Return modified ForwardRequest struct to caller.
        self
    } // fn
} // impl
