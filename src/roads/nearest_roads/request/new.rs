use crate::client_settings::ClientSettings;
use crate::latlng::LatLng;
use crate::roads::nearest_roads::request::Request;

// =============================================================================

impl<'a> Request<'a> {

    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Nearest Roads query with the
    /// required parameters.
    ///
    /// ## Arguments:
    ///
    /// * `client_settings` ‧ Your application's Google Maps API client struct.
    ///
    /// * `points` ‧ The points to be snapped. The points parameter accepts a
    /// list of latitude/longitude pairs.
    ///
    /// Note: The snapping algorithm works best for points that are not too far
    /// apart. If you observe odd snapping behavior, try creating paths that
    /// have points closer together. To ensure the best snap-to-road quality,
    /// you should aim to provide paths on which consecutive pairs of points are
    /// within 300m of each other. This will also help in handling any isolated,
    /// long jumps between consecutive points caused by GPS signal loss, or
    /// noise.

    pub fn new(
        client_settings: &ClientSettings,
        points: Vec<LatLng>,
    ) -> Request {

        // Instantiate struct and return it to caller:
        Request {
            // Required parameters:
            client_settings,
            points,
            // Internal use only:
            query: None,
        } // struct

    } // fn

} // impl