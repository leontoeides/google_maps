use crate::types::LatLng;

// =============================================================================

impl<'r> crate::geocoding::reverse::ReverseRequest<'r> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Geolocation API query with the
    /// required, non-optional parameters.
    ///
    /// # Arguments:
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    ///
    /// * `latlng` ‧ The latitude and longitude values specifying the location
    ///   for which you wish to obtain the closest, human-readable address.
    #[must_use]
    pub const fn new(
        client: &'r crate::client::Client,
        latlng: LatLng
    ) -> Self {
        // Instantiate struct and return it to caller:
        Self {
            // Required parameters:
            client,
            latlng,
            // Optional parameters:
            language: None,
            location_types: Vec::new(),
            result_types: Vec::new(),
        } // struct
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Geolocation API query with the
    /// required, non-optional parameters.
    ///
    /// This function is the same as `new` but it supports
    /// the [geo](https://crates.io/crates/geo) crate's
    /// [Coord](https://docs.rs/geo/latest/geo/geometry/struct.Coord.html) type.
    ///
    /// # Arguments:
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    /// * `coordinate` ‧ The `Coord` specifying the location for which you
    ///   wish to obtain the closest, human-readable address.
    #[cfg(feature = "geo")]
    pub fn try_new_coordinate(
        client: &'r crate::client::Client,
        coordinate: &geo_types::Coord
    ) -> Result<Self, crate::error::Error> {
        // Instantiate struct and return it to caller:
        Ok(Self {
            // Required parameters:
            client,
            latlng: LatLng::try_from(coordinate)?,
            // Optional parameters:
            language: None,
            location_types: Vec::new(),
            result_types: Vec::new(),
        }) // struct
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Geolocation API query with the
    /// required, non-optional parameters.
    ///
    /// This function is the same as `new` but it supports
    /// the [geo](https://crates.io/crates/geo) crate's
    /// [Point](https://docs.rs/geo/latest/geo/geometry/struct.Point.html) type.
    ///
    /// # Arguments:
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    /// * `point` ‧ The `Point` specifying the location for which you wish to
    ///   obtain the closest, human-readable address.
    #[cfg(feature = "geo")]
    pub fn try_new_point(
        client: &'r crate::client::Client,
        point: &geo_types::Point
    ) -> Result<Self, crate::error::Error> {
        // Instantiate struct and return it to caller:
        Ok(Self {
            // Required parameters:
            client,
            latlng: LatLng::try_from(point)?,
            // Optional parameters:
            language: None,
            location_types: Vec::new(),
            result_types: Vec::new(),
        }) // struct
    } // fn
} // impl
