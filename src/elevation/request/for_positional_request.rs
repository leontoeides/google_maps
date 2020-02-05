use crate::elevation::request::{
    locations::Locations,
    Request,
}; // use

impl<'a> Request<'a> {

    /// Adds the _positional request_ parameters to the Elevation API query.
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
    /// .for_positional_request(ElevationLocations::LatLngs(vec![
    ///     // Denver, Colorado, the "Mile High City"
    ///     LatLng::try_from(39.7391536, -104.9847034).unwrap(),
    /// ]))
    /// ```

    pub fn for_positional_request(&'a mut self, locations: Locations) -> &'a mut Request {
        // Set the path in Request struct.
        self.locations = Some(locations);
        // Return modified Request struct to caller.
        self
    } // fn

} // impl