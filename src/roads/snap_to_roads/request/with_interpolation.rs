use crate::roads::snap_to_roads::request::Request;

// =============================================================================

impl<'a> Request<'a> {
    // -------------------------------------------------------------------------
    //
    /// Whether to interpolate a path to include all points forming the full
    /// road-geometry.
    ///
    /// ## Arguments
    ///
    /// * `interpolate` ‧ When `true`, additional interpolated points will also
    ///   be returned, resulting in a path that smoothly follows the geometry of
    ///   the road, even around corners and through tunnels. Interpolated paths
    ///   will most likely contain more points than the original path. Defaults
    ///   to `false`.
    ///
    /// ## Example
    ///
    /// * Turn on interpolation:
    /// ```rust
    /// .with_interpolation(true)
    /// ```

    pub fn with_interpolation(
        &'a mut self,
        interpolate: impl Into<bool>
    ) -> &'a mut Self {
        // Set language in Request struct.
        self.interpolate = Some(interpolate.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
