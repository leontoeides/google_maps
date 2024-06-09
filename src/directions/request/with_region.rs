use crate::{directions::request::Request, types::Region};

impl<'a> Request<'a> {
    /// Specifies the region bias. There is a London in Canada and there is a
    /// London in England. By biasing the region, you help the directions
    /// service choose the London you intended.
    ///
    /// ## Arguments
    ///
    /// * `region` ‧ A country to bias your geocoding results to.
    ///
    /// ## Description
    ///
    /// [Region Biasing](https://developers.google.com/maps/documentation/directions/intro#RegionBiasing)
    ///
    /// You can set the Directions service to return results from a specific
    /// region by using the `region` parameter. You may utilize any domain in
    /// which the main Google Maps application has launched driving directions.
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

    pub fn with_region(
        &'a mut self,
        region: impl Into<Region>
    ) -> &'a mut Self {
        self.region = Some(region.into());
        self
    } // fn
} // impl
