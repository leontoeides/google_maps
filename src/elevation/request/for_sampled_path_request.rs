use crate::elevation::request::{
    locations::Locations,
    Request,
}; // use

impl<'a> Request<'a> {

    /// Adds the _sampled path request_ parameters to the Elevation API query.
    ///
    /// ## Arguments:
    ///
    /// * `path` ‧ Defines a path on the earth for which to return elevation
    /// data. This parameter defines a set of two or more ordered
    /// latitude/longitude pairs defining a path along the surface of the earth.
    /// For more information, see [Specifying
    /// Paths](https://developers.google.com/maps/documentation/elevation/intro#Paths).
    ///
    /// * `samples` ‧ Specifies the number of sample points along a path for
    /// which to return elevation data. The samples parameter divides the given
    /// path into an ordered set of equidistant points along the path.
    ///
    /// ## Examples:
    ///
    /// * 2 elevation samples between two points:
    /// ```rust
    /// .for_sampled_path_request(
    ///     ElevationLocations::LatLngs(vec![
    ///         // Denver, Colorado
    ///         LatLng::try_from(dec!(40.714728), dec!(-73.998672)).unwrap(),
    ///         // Death Valley, California
    ///         LatLng::try_from(dec!(-34.397), dec!(-116.866667)).unwrap(),
    ///     ]),
    ///     // Number of samples
    ///     2
    /// )
    /// ```
    ///
    /// * 4 elevation samples along a polyline:
    /// ```rust
    /// .for_sampled_path_request(
    ///     ElevationLocations::Polyline(String::from("gfo}EtohhUxD@bAxJmGF")),
    ///     // Number of samples
    ///     4
    /// )
    /// ```

    pub fn for_sampled_path_request(&'a mut self, path: Locations, samples: u8) -> &'a mut  Request {
        // Set the path in Request struct.
        self.path = Some(path);
        // Set the sample number in Request struct.
        self.samples = Some(samples);
        // Return modified Request struct to caller.
        self
    } // fn

} // impl