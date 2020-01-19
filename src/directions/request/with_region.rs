use crate::{
    directions::request::Request,
    region::Region,
}; // use

impl Request {

    /// Specifies the region bias.
    ///
    /// [Region Biasing](https://developers.google.com/maps/documentation/directions/intro#RegionBiasing)
    /// -------------------------------------------------------------------------------------------------
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
    /// Example:
    /// ---------
    ///
    /// * Bias region to Canada:
    /// ```
    /// .with_region(Region::Canada)
    /// ```

    pub fn with_region(&mut self, region: Region) -> &mut Request {
        self.region = Some(region);
        self
    } // fn

} // impl