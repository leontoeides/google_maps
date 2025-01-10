impl crate::places::place_autocomplete::Request<'_> {
    /// Adds the origin parameter to the Place API _Place Autocomplete_ query.
    ///
    /// ## Arguments
    ///
    /// * `origin` ‧ The origin point from which to calculate straight-line
    ///   distance to the destination (returned as `distance_meters`). If this
    ///   value is omitted, straight-line distance will not be returned.
    #[must_use] pub fn with_origin(
        mut self,
        origin: impl Into<crate::types::LatLng>
    ) -> Self {
        // Set origin in Request struct.
        self.origin = Some(origin.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl