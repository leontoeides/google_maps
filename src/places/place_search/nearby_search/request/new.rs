use crate::client::GoogleMapsClient;
use crate::places::place_search::nearby_search::request::Request;
use crate::LatLng;

// =============================================================================

impl<'a> Request<'a> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Places API _Nearby Search_ query
    /// with the required, non-optional parameters.
    ///
    /// ## Arguments
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    ///
    /// * `location` ‧ The point around which to retrieve place information.
    ///   This must be specified as `latitude,longitude`.
    ///
    /// * `radius` ‧ Defines the distance (in meters) within which to return
    ///   place results. You may bias results to a specified circle by passing a
    ///   `location` and a `radius` parameter. Doing so instructs the Places
    ///   service to prefer showing results within that circle; results outside
    ///   of the defined area may still be displayed.
    ///
    /// The radius will automatically be clamped to a maximum value depending on
    /// the type of search and other parameters.
    ///
    /// * Autocomplete: 50,000 meters
    /// * Nearby Search:
    ///     * with `keyword` or `name`: 50,000 meters
    ///     * without `keyword` or `name`
    ///         * Up to 50,000 meters, adjusted dynamically based on area
    ///           density, independent of `rankby` parameter.
    ///         * When using `rankby=distance`, the radius parameter will not be
    ///           accepted, and will result in an `INVALID_REQUEST`.
    /// * Query Autocomplete: 50,000 meters
    /// * Nearby Search: 50,000 meters

    #[must_use]
    pub const fn new(client: &GoogleMapsClient, location: LatLng, radius: u32) -> Request {
        // Instantiate struct and return it to caller:
        Request {
            // Required parameters:
            client,
            location,
            radius,
            // Optional parameters:
            keyword: None,
            language: None,
            maxprice: None,
            minprice: None,
            opennow: None,
            pagetoken: None,
            rankby: None,
            place_type: None,
            // Internal use only:
            query: None,
        } // struct
    } // fn
} // impl
