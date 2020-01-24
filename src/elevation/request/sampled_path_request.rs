use crate::elevation::request::{
    locations::Locations,
    Request,
}; // use

impl Request {

    /// Adds the _sampled path request_ parameters to the Elevation API query.
    ///
    /// ## Arguments:
    ///
    /// * `locations` ‧ Defines the location(s) on the earth from which to
    /// return elevation data. This parameter takes either a single location,
    /// as a latitude/longitude pair, multiple latitude/longitude pairs, or an
    /// encoded polyline. For more information, see [Specifying
    /// Locations](https://developers.google.com/maps/documentation/elevation/intro#Locations).
    ///
    /// ## Example:
    ///
    /// ```
    /// let elevation = ElevationRequest::new(GOOGLE_API_KEY)
    /// .sampled_path_request(
    ///     ElevationLocations::LatLngs(vec![
    ///         // Denver, Colorado
    ///         LatLng { lat: 40.714728, lng: -73.998672 },
    ///         // Death Valley, California
    ///         LatLng { lat: -34.397, lng: -116.866667 },
    ///     ]),
    ///     // Number of samples
    ///     4
    /// ).validate().unwrap().build().unwrap().get().unwrap();
    /// ```

    pub fn sampled_path_request(&mut self, path: Locations, samples: u8) -> &mut Request {
        // Set the path in Request struct.
        self.path = Some(path);
        // Set the sample number in Request struct.
        self.samples = Some(samples);
        // Return modified Request struct to caller.
        self
    } // fn

} // impl