use crate::client::GoogleMapsClient;
#[cfg(feature = "enable-reqwest")]
use crate::request_rate::RequestRate;
#[cfg(feature = "directions")]
use crate::directions::request::location::Location;
#[cfg(feature = "distance_matrix")]
use crate::directions::request::waypoint::Waypoint;
#[cfg(any(feature = "geocoding", feature = "time_zone"))]
use crate::latlng::LatLng;
#[cfg(feature = "time_zone")]
use chrono::{DateTime, Utc};

// =============================================================================

impl GoogleMapsClient {

    // -------------------------------------------------------------------------
    //
    /// Initialize the settings needed for a Google Cloud Maps API transaction.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    #[cfg(feature = "enable-reqwest")]
    pub fn new(key: &str) -> GoogleMapsClient {

        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let reqwest_client = reqwest::Client::builder()
            .user_agent(format!("Google Maps Rust Client {VERSION}"))
            .build()
            .unwrap();

        GoogleMapsClient {
            key: key.to_string(),
            rate_limit: RequestRate::default(),
            reqwest_client,
        } // GoogleMapsClient

    } // fn

    // -------------------------------------------------------------------------
    //
    /// Initialize the settings needed for a Google Cloud Maps API transaction.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    #[cfg(not(feature = "enable-reqwest"))]
    pub fn new(key: &str) -> GoogleMapsClient {
        GoogleMapsClient {
            key: key.to_string(),
        } // GoogleMapsClient
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Directions API is a service that calculates directions between
    /// locations. You can search for directions for several modes of
    /// transportation, including transit, driving, walking, or cycling.
    ///
    /// ```rust
    /// use google_maps::prelude::*;
    /// use rust_decimal_macros::dec;
    ///
    /// let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
    ///
    /// let directions = google_maps_client.directions(
    ///     // Origin: Canadian Museum of Nature
    ///     Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
    ///     // Destination: Canada Science and Technology Museum
    ///     Location::LatLng(LatLng::try_from_dec(dec!(45.403_509), dec!(-75.618_904))?),
    /// )
    /// ```

    #[cfg(feature = "directions")]
    pub fn directions(
        &self,
        origin: Location,
        destination: Location,
    ) -> crate::directions::request::Request {
        crate::directions::request::Request::new(self, origin, destination)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Distance Matrix API is a service that provides travel distance and
    /// time for a matrix of origins and destinations, based on the recommended
    /// route between start and end points.
    ///
    /// ```rust
    /// use google_maps::prelude::*;
    /// use rust_decimal_macros::dec;
    ///
    /// let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
    ///
    /// let distances = google_maps_client.distance_matrix(
    ///     // Origins
    ///     vec![
    ///         // Microsoft
    ///         Waypoint::Address(String::from("One Microsoft Way, Redmond, WA 98052, United States")),
    ///         // Cloudflare
    ///         Waypoint::Address(String::from("101 Townsend St, San Francisco, CA 94107, United States")),
    ///     ],
    ///     // Destinations
    ///     vec![
    ///         // Google
    ///         Waypoint::PlaceId(String::from("ChIJj61dQgK6j4AR4GeTYWZsKWw")),
    ///         // Mozilla
    ///         Waypoint::LatLng(LatLng::try_from_dec(dec!(37.387_316), dec!(-122.060_008))?),
    ///     ],
    /// )
    /// ```

    #[cfg(feature = "distance_matrix")]
    pub fn distance_matrix(
        &self,
        origins: Vec<Waypoint>,
        destinations: Vec<Waypoint>,
    ) -> crate::distance_matrix::request::Request {
        crate::distance_matrix::request::Request::new(self, origins, destinations)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Elevation API provides elevation data for all locations on the
    /// surface of the earth, including depth locations on the ocean floor
    /// (which return negative values).

    #[cfg(feature = "elevation")]
    pub fn elevation(&self) -> crate::elevation::request::Request {
        crate::elevation::request::Request::new(self)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Geocoding API is a service that provides geocoding and reverse
    /// geocoding of addresses. **Geocoding** is the process of converting
    /// addresses (like a street address) into geographic coordinates (like
    /// latitude and longitude), which you can use to place markers on a map, or
    /// position the map.

    #[cfg(feature = "geocoding")]
    pub fn geocoding(&self) -> crate::geocoding::forward::ForwardRequest {
        crate::geocoding::forward::ForwardRequest::new(self)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Geocoding API is a service that provides geocoding and reverse
    /// geocoding of addresses. **Reverse geocoding** is the process of
    /// converting geographic coordinates into a human-readable address.
    ///
    /// ```rust
    /// use google_maps::prelude::*;
    /// use rust_decimal_macros::dec;
    ///
    /// let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
    ///
    /// let address = google_maps_client.reverse_geocoding(
    ///     // 10 Downing St, Westminster, London
    ///     LatLng::try_from_dec(dec!(51.503_364), dec!(-0.127_625))?,
    /// )
    /// ```

    #[cfg(feature = "geocoding")]
    pub fn reverse_geocoding(
        &self,
        latlng: LatLng,
    ) -> crate::geocoding::reverse::ReverseRequest {
        crate::geocoding::reverse::ReverseRequest::new(self, latlng)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The **Time Zone API** provides time offset data for locations on the
    /// surface of the earth. You request the time zone information for a
    /// specific latitude/longitude pair and date. The API returns the name of
    /// that time zone, the time offset from UTC, and the daylight savings
    /// offset.
    ///
    /// ```rust
    /// use google_maps::prelude::*;
    /// use rust_decimal_macros::dec;
    ///
    /// let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
    ///
    /// let time_zone = google_maps_client.time_zone(
    ///      // St. Vitus Cathedral in Prague, Czechia
    ///      LatLng::try_from_dec(dec!(50.090_903), dec!(14.400_512))?,
    ///      // Tuesday February 23, 2020 @ 6:00:00 pm
    ///      NaiveDate::from_ymd(2020, 2, 23).and_hms(18, 00, 0)
    /// )
    /// ```

    #[cfg(feature = "time_zone")]
    pub fn time_zone(
        &self,
        location: LatLng,
        timestamp: DateTime<Utc>,
    ) -> crate::time_zone::request::Request {
        crate::time_zone::request::Request::new(self, location, timestamp)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Place API **Place Autocomplete** service returns place predictions.
    /// The request specifies a textual search string and optional geographic
    /// bounds. The service can be used to provide autocomplete functionality
    /// for text-based geographic searches, by returning places such as
    /// businesses, addresses and points of interest as a user types.

    #[cfg(feature = "autocomplete")]
    pub fn place_autocomplete(
        &self,
        input: String,
    ) -> crate::places::place_autocomplete::request::Request {
        crate::places::place_autocomplete::request::Request::new(self, input)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Place API **Query Autocomplete** service allows you to add
    /// on-the-fly geographic query predictions to your application. Instead of
    /// searching for a specific location, a user can type in a categorical
    /// search, such as "pizza near New York" and the service responds with a
    /// list of suggested queries matching the string. As the Query Autocomplete
    /// service can match on both full words and substrings, applications can
    /// send queries as the user types to provide on-the-fly predictions.
    ///
    ///
    /// ```rust
    /// use google_maps::prelude::*;
    /// use rust_decimal_macros::dec;
    ///
    /// let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
    ///
    /// let predictions = google_maps_client.place_autocomplete("51".to_string())
    ///     .with_location_and_radius(LatLng::try_from_dec(dec!(54), dec!(-114))?, 1_000)
    ///     .with_type(AutocompleteType::Address)
    ///     .execute()
    ///     .await?;
    ///
    /// println!("{:#?}", predictions);
    /// ```

    #[cfg(feature = "autocomplete")]
    pub fn query_autocomplete(
        &self,
        input: String,
    ) -> crate::places::query_autocomplete::request::Request {
        crate::places::query_autocomplete::request::Request::new(self, input)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Roads API **Snap To Roads** service takes up to 100 GPS points
    /// collected along a route, and returns a similar set of data, with the
    /// points snapped to the most likely roads the vehicle was traveling along.
    /// Optionally, you can request that the points be interpolated, resulting
    /// in a path that smoothly follows the geometry of the road.
    ///
    /// ```rust
    /// use google_maps::prelude::*;
    /// use rust_decimal_macros::dec;
    ///
    /// let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
    ///
    /// let snapped_points = google_maps_client.snap_to_roads(vec![
    ///     LatLng::try_from_dec(dec!(-35.27801), dec!(149.12958))?,
    ///     LatLng::try_from_dec(dec!(-35.28032), dec!(149.12907))?,
    ///     LatLng::try_from_dec(dec!(-35.28099), dec!(149.12929))?,
    ///     LatLng::try_from_dec(dec!(-35.28144), dec!(149.12984))?,
    ///     LatLng::try_from_dec(dec!(-35.28194), dec!(149.13003))?,
    ///     LatLng::try_from_dec(dec!(-35.28282), dec!(149.12956))?,
    ///     LatLng::try_from_dec(dec!(-35.28302), dec!(149.12881))?,
    ///     LatLng::try_from_dec(dec!(-35.28473), dec!(149.12836))?,
    /// ])
    /// .with_interpolation(true)
    /// .execute()
    /// .await?;
    /// ```

    #[cfg(feature = "roads")]
    pub fn snap_to_roads(
        &self,
        path: Vec<LatLng>,
    ) -> crate::roads::snap_to_roads::request::Request {
        crate::roads::snap_to_roads::request::Request::new(self, path)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Roads API **Nearest Roads** service returns individual road segments
    /// for a given set of GPS coordinates. This services takes up to 100 GPS
    /// points and returns the closest road segment for each point. The points
    /// passed do not need to be part of a continuous path.
    ///
    /// **If you are working with sequential GPS points, use Nearest Roads.**
    ///
    /// ```rust
    /// use google_maps::prelude::*;
    /// use rust_decimal_macros::dec;
    ///
    /// let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
    ///
    /// let snapped_points = google_maps_client.nearest_roads(vec![
    ///     LatLng::try_from_dec(dec!(-35.27801), dec!(149.12958))?,
    ///     LatLng::try_from_dec(dec!(60.170880), dec!(24.942795))?,
    ///     LatLng::try_from_dec(dec!(60.170879), dec!(24.942796))?,
    ///     LatLng::try_from_dec(dec!(60.170877), dec!(24.942796))?,
    /// ])
    /// .execute()
    /// .await?;
    /// ```

    #[cfg(feature = "roads")]
    pub fn nearest_roads(
        &self,
        path: Vec<LatLng>,
    ) -> crate::roads::snap_to_roads::request::Request {
        crate::roads::snap_to_roads::request::Request::new(self, path)
    } // fn

} // impl