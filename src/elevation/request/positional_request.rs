use crate::elevation::request::{
    locations::Locations,
    Request,
}; // use

impl Request {

    /// Adds the _positional request_ parameters to the Elevation API query.
    ///
    /// # Arguments:
    ///
    /// * `path` - Defines a path on the earth for which to return elevation
    /// data. This parameter defines a set of two or more ordered
    /// latitude/longitude pairs defining a path along the surface of the earth.
    /// For more information, see [Specifying
    /// Paths](https://developers.google.com/maps/documentation/elevation/intro#Paths).
    /// * `samples` - Specifies the number of sample points along a path for
    /// which to return elevation data. The samples parameter divides the given
    /// path into an ordered set of equidistant points along the path.
    ///
    /// # Example:
    ///
    /// ```
    /// let response = ElevationRequest::new(GOOGLE_API_KEY)
    /// .positional_request(ElevationLocations::LatLngs(vec![
    ///     // Denver, Colorado, the "Mile High City"
    ///     LatLng { lat: 39.7391536, lng: -104.9847034 },
    /// ])).validate().unwrap().build().unwrap().get().unwrap();
    /// let elevation = response.results.unwrap()[0].elevation;
    /// ```

    pub fn positional_request(&mut self, locations: Locations) -> &mut Request {
        // Set the path in Request struct.
        self.locations = Some(locations);
        // Return modified Request struct to caller.
        self
    } // fn

} // impl