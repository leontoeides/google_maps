use crate::latlng::LatLng;
use crate::places::place_search::text_search::request::Request;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {

    /// Adds the location and radius parameters to the Places API _Text Search_
    /// query.
    ///
    /// ## Arguments:
    ///
    /// * `location` ‧ The point around which to retrieve place information.
    /// This must be specified as `latitude,longitude`.
    ///
    /// The `location` parameter may be overriden if the `query` contains an
    /// explicit location such as `Market in Barcelona`. Using quotes around the
    /// query may also influence the weight given to the `location` and
    /// `radius`.

    pub fn with_location(
        &'a mut self,
        location: LatLng,
    ) -> &'a mut Request {
        // Set location in Request struct.
        self.location = Some(location);
        // Return modified Request struct to caller.
        self
    } // fn

    /// Adds the location and radius parameters to the Places API _Text Search_
    /// query.
    ///
    /// ## Arguments:
    ///
    /// * `location` ‧ The point around which to retrieve place information.
    /// This must be specified as `latitude,longitude`.
    ///
    /// The `location` parameter may be overriden if the `query` contains an
    /// explicit location such as `Market in Barcelona`. Using quotes around the
    /// query may also influence the weight given to the `location` and
    /// `radius`.
    ///
    /// * `radius` ‧ Defines the distance (in meters) within which to return
    /// place results. You may bias results to a specified circle by passing a
    /// `location` and a `radius` parameter. Doing so instructs the Places
    /// service to prefer showing results within that circle; results outside of
    /// the defined area may still be displayed.
    ///
    /// The radius will automatically be clamped to a maximum value depending on
    /// the type of search and other parameters.
    ///
    /// * Autocomplete: 50,000 meters
    /// * Nearby Search:
    ///     * with `keyword` or `name`: 50,000 meters
    ///     * without `keyword` or `name`
    ///         * Up to 50,000 meters, adjusted dynamically based on area
    ///         density, independent of `rankby` parameter.
    ///         * When using `rankby=distance`, the radius parameter will not be
    ///         accepted, and will result in an `INVALID_REQUEST`.
    /// * Query Autocomplete: 50,000 meters
    /// * Text Search: 50,000 meters

    pub fn with_location_and_radius(
        &'a mut self,
        location: LatLng,
        radius: u32,
    ) -> &'a mut Request {
        // Set location in Request struct.
        self.location = Some(location);
        // Set radius in Request struct.
        self.radius = Some(radius);
        // Return modified Request struct to caller.
        self
    } // fn

} // impl