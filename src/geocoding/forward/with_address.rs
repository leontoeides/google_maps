use crate::geocoding::forward::ForwardRequest;

impl ForwardRequest {

    /// Specifies the street address to geocode.
    ///
    /// # Arguments:
    ///
    /// * `address` - The street address that you want to geocode, in the format
    /// used by the national postal service of the country concerned. Additional
    /// address elements such as business names and unit, suite or floor numbers
    /// should be avoided. Please refer to [the
    /// FAQ](https://developers.google.com/maps/faq#geocoder_queryformat) for
    /// additional guidance.
    ///
    /// # Example:
    ///
    /// ```
    /// .with_address(String::from(
    ///     "1313 Disneyland Dr, Anaheim, CA 92802, United States"
    /// ))
    /// ```

    pub fn with_address(&mut self, address: &str) -> &mut ForwardRequest {
        // Set address in ForwardRequest struct.
        self.address = Some(String::from(address));
        // Return modified ForwardRequest struct to caller.
        self
    } // fn

} // impl