impl crate::geocoding::ForwardRequest<'_> {
    /// Specifies the region bias.
    ///
    /// ## Arguments
    ///
    /// * `region` ‧ The region to prefer in search results. This parameter will
    ///   only influence, not fully restrict, results from the geocoder.
    ///
    /// ## Description
    ///
    /// [Region
    /// Biasing](https://developers.google.com/maps/documentation/geocoding/intro#RegionCodes)
    ///
    /// In a Geocoding request, you can instruct the Geocoding service to return
    /// results biased to a particular region by using the `region` parameter.
    ///
    /// Geocoding results can be biased for every domain in which the main
    /// Google Maps application is officially launched. Note that biasing only
    /// _prefers_ results for a specific domain; if more relevant results exist
    /// outside of this domain, they may be included.
    ///
    /// For example, a directions request for "Toledo" to "Madrid" returns
    /// appropriate results when `region` is set to `Region::Spain` and "Toledo"
    /// is then interpreted as the Spanish city. A directions request for
    /// "Toledo" to "Madrid" sent without a `region` parameter does not return
    /// results, because "Toledo" is interpreted as the city in Ohio and not
    /// Spain.
    ///
    /// ## Example
    ///
    /// * Bias region to Canada:
    /// ```rust
    /// .with_region(Region::Canada)
    /// ```
    #[must_use] pub fn with_region(
        mut self,
        region: impl Into<crate::types::Region>
    ) -> Self {
        // Set region in ForwardRequest struct.
        self.region = Some(region.into());
        // Return modified ForwardRequest struct to caller.
        self
    } // fn
} // impl
