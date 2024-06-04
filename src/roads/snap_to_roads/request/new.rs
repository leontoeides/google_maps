use crate::client::GoogleMapsClient;
use crate::roads::snap_to_roads::request::Request;
use crate::types::LatLng;

// =============================================================================

impl<'a> Request<'a> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Snap To Roads query with the
    /// required, non-optional parameters.
    ///
    /// ## Arguments
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    ///
    /// * `path` ‧ The path to be snapped. Note: The snapping algorithm works
    ///   best for points that are not too far apart. If you observe odd
    ///   snapping behavior, try creating paths that have points closer
    ///   together. To ensure the best snap-to-road quality, you should aim to
    ///   provide paths on which consecutive pairs of points are within 300m of
    ///   each other. This will also help in handling any isolated, long jumps
    ///   between consecutive points caused by GPS signal loss, or noise.

    #[must_use]
    pub fn new(client: &GoogleMapsClient, path: Vec<LatLng>) -> Request {
        // Instantiate struct and return it to caller:
        Request {
            // Required parameters:
            client,
            path,
            // Optional parameters:
            interpolate: None,
            // Internal use only:
            query: None,
        } // struct
    } // fn
} // impl
