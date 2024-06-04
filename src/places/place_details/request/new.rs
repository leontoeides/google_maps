use crate::client::GoogleMapsClient;
use crate::places::place_details::request::Request;

// =============================================================================

impl<'a> Request<'a> {
    // -------------------------------------------------------------------------
    //
    /// Initializes the builder pattern for a Places API _Place Details_ query
    /// with the required, non-optional parameters.
    ///
    /// ## Arguments
    ///
    /// * `client` ‧ Your application's Google Maps API client struct.
    ///
    /// * `place_id` ‧ A textual identifier that uniquely identifies a place,
    ///   returned from a
    ///   [Place Search](https://developers.google.com/maps/documentation/places/web-service/search).
    ///   For more information about place IDs, see the
    ///   [place ID overview](https://developers.google.com/maps/documentation/places/web-service/place-id).

    #[must_use]
    pub fn new(client: &GoogleMapsClient, place_id: impl Into<String>) -> Request {
        // Instantiate struct and return it to caller:
        Request {
            // Required parameters:
            client,
            place_id: place_id.into(),
            // Optional parameters:
            fields: Vec::new(),
            language: None,
            region: None,
            reviews_no_translations: None,
            reviews_sort: None,
            sessiontoken: None,
            // Internal use only:
            query: None,
        } // struct
    } // fn
} // impl
