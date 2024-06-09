use crate::client::GoogleMapsClient;
use crate::types::LatLng;
#[cfg(feature = "directions")]
use crate::directions::request::location::Location;
#[cfg(feature = "distance_matrix")]
use crate::directions::request::waypoint::Waypoint;
#[cfg(feature = "enable-reqwest")]
use crate::request_rate::RequestRate;
#[cfg(feature = "time_zone")]
use chrono::{DateTime, Utc};
#[cfg(feature = "enable-reqwest")]
use reqwest::Response;

// =============================================================================

impl GoogleMapsClient {
    // -------------------------------------------------------------------------
    //
    /// Initialize the settings needed for a Google Cloud Maps API transaction.
    ///
    /// ## Arguments
    ///
    /// * `key` ‧ Your application's API key. This key identifies your
    ///   application for purposes of quota management. Learn how to [get a
    ///   key](https://developers.google.com/maps/documentation/geocoding/get-api-key).

    #[cfg(all(feature = "enable-reqwest", not(feature = "enable-reqwest-middleware")))]
    pub fn try_new(key: impl Into<String>) -> Result<Self, crate::GoogleMapsError> {
        let reqwest_client = reqwest::Client::builder()
            .user_agent(format!(
                "RustGoogleMaps/{version}",
                version = env!("CARGO_PKG_VERSION")
            ))
            .connect_timeout(std::time::Duration::from_secs(10))
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            key: key.into(),
            rate_limit: RequestRate::default(),
            reqwest_client,
        }) // GoogleMapsClient
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Initialize the settings needed for a Google Cloud Maps API transaction.
    ///
    /// ## Arguments
    ///
    /// * `key` ‧ Your application's API key. This key identifies your
    /// application for purposes of quota management. Learn how to [get a
    /// key](https://developers.google.com/maps/documentation/geocoding/get-api-key).

    #[cfg(all(feature = "enable-reqwest", feature = "enable-reqwest-middleware"))]
    pub fn try_new(key: impl Into<String>) -> Result<Self, crate::GoogleMapsError> {
        let reqwest_client = reqwest::Client::builder()
            .user_agent(format!(
                "RustGoogleMaps/{version}",
                version = env!("CARGO_PKG_VERSION")
            ))
            .connect_timeout(std::time::Duration::from_secs(10))
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            key: key.into(),
            rate_limit: RequestRate::default(),
            reqwest_client: crate::reqwest_maybe_middleware::Client::Vanilla(reqwest_client),
        }) // GoogleMapsClient
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Initialize the settings needed for a Google Cloud Maps API transaction.
    ///
    /// ## Arguments
    ///
    /// * `key` ‧ Your application's API key. This key identifies your
    ///   application for purposes of quota management. Learn how to [get a
    ///   key](https://developers.google.com/maps/documentation/geocoding/get-api-key).
    ///
    /// ## Panics
    ///
    /// * This function will panic if the `reqwest` client builder chain fails.
    ///   Realistically this shouldn't happen. However you may want to use
    ///   `try_new` to instantiate a new `GoogleMapsClient` instead.

    #[cfg(feature = "enable-reqwest")]
    #[deprecated(since = "3.4.2", note = "use `try_new` instead")]
    #[must_use]
    pub fn new(key: impl Into<String>) -> Self {
        Self::try_new(key).unwrap()
    }

    // -------------------------------------------------------------------------
    //
    /// Initialize the settings needed for a Google Cloud Maps API transaction.
    ///
    /// ## Arguments
    ///
    /// * `key` ‧ Your application's API key. This key identifies your
    /// application for purposes of quota management. Learn how to [get a
    /// key](https://developers.google.com/maps/documentation/geocoding/get-api-key).

    #[cfg(not(feature = "enable-reqwest"))]
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Directions API is a service that calculates directions between
    /// locations. You can search for directions for several modes of
    /// transportation, including transit, driving, walking, or cycling.
    ///
    /// ## Basic usage
    ///
    /// ```rust
    /// use google_maps::prelude::*;
    /// use rust_decimal_macros::dec;
    ///
    /// let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
    ///
    /// let directions = google_maps_client.directions(
    ///     // Origin: Canadian Museum of Nature
    ///     Location::from_address("240 McLeod St, Ottawa, ON K2P 2R1"),
    ///     // Destination: Canada Science and Technology Museum
    ///     Location::try_from_f32(45.403_509, -75.618_904)?,
    /// )
    /// ```

    #[cfg(feature = "directions")]
    #[must_use]
    pub fn directions(
        &self,
        origin: impl Into<Location>,
        destination: impl Into<Location>
    ) -> crate::directions::request::Request {
        crate::directions::request::Request::new(self, origin.into(), destination.into())
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Distance Matrix API is a service that provides travel distance and
    /// time for a matrix of origins and destinations, based on the recommended
    /// route between start and end points.
    ///
    /// ## Basic usage
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
    ///         Waypoint::from_address("One Microsoft Way, Redmond, WA 98052, United States"),
    ///         // Cloudflare
    ///         Waypoint::from_address("101 Townsend St, San Francisco, CA 94107, United States"),
    ///     ],
    ///     // Destinations
    ///     vec![
    ///         // Google
    ///         Waypoint::from_place_id("ChIJj61dQgK6j4AR4GeTYWZsKWw"),
    ///         // Mozilla
    ///         Waypoint::try_from_f32(37.387_316, -122.060_008)?,
    ///     ],
    /// )
    /// ```
    ///
    /// # Generics
    ///
    /// This method uses generics to improve ergonomics. The `C` generic is
    /// intended to represent any collection that can be iterated over, and the
    /// `W` generic is for any type that can be converted to a `Waypoint` type.

    #[cfg(feature = "distance_matrix")]
    #[must_use]
    pub fn distance_matrix<C, W>(
        &self,
        origins: C,
        destinations: C
    ) -> crate::distance_matrix::request::Request
    where
        C: IntoIterator<Item = W>,
        W: Into<Waypoint> {
        let origins: Vec<Waypoint> = origins.into_iter().map(Into::into).collect();
        let destinations: Vec<Waypoint> = destinations.into_iter().map(Into::into).collect();
        crate::distance_matrix::request::Request::new(self, origins, destinations)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Elevation API provides elevation data for all locations on the
    /// surface of the earth, including depth locations on the ocean floor
    /// (which return negative values).
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments, it initiates a builder pattern. Use
    /// the methods of the resulting type to set the parameters.

    #[cfg(feature = "elevation")]
    #[must_use]
    pub const fn elevation(&self) -> crate::elevation::request::Request {
        crate::elevation::request::Request::new(self)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Geocoding API is a service that provides geocoding and reverse
    /// geocoding of addresses. **Geocoding** is the process of converting
    /// addresses (like a street address) into geographic coordinates (like
    /// latitude and longitude), which you can use to place markers on a map, or
    /// position the map.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments, it initiates a builder pattern. Use
    /// the methods of the resulting type to set the parameters.

    #[cfg(feature = "geocoding")]
    #[must_use]
    pub const fn geocoding(&self) -> crate::geocoding::forward::ForwardRequest {
        crate::geocoding::forward::ForwardRequest::new(self)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Geocoding API is a service that provides geocoding and reverse
    /// geocoding of addresses. **Reverse geocoding** is the process of
    /// converting geographic coordinates into a human-readable address.
    ///
    /// ## Arguments
    ///
    /// * `latlng` ‧ The latitude and longitude values specifying the location
    ///   for which you wish to obtain the closest, human-readable address.
    ///
    /// ## Basic usage
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
    #[must_use]
    pub fn reverse_geocoding(
        &self,
        location: impl Into<LatLng>
    ) -> crate::geocoding::reverse::ReverseRequest {
        crate::geocoding::reverse::ReverseRequest::new(self, location.into())
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The **Time Zone API** provides time offset data for locations on the
    /// surface of the earth. You request the time zone information for a
    /// specific latitude/longitude pair and date. The API returns the name of
    /// that time zone, the time offset from UTC, and the daylight savings
    /// offset.
    ///
    /// ## Arguments
    ///
    /// * `location` ‧ Latitude & longitude of the desired time zone location.
    ///
    /// * `timestamp` ‧ Time is used to determine if Daylight Savings is
    ///   applicable.
    ///
    /// ## Basic usage
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
    #[must_use]
    pub fn time_zone(
        &self,
        location: impl Into<LatLng>,
        timestamp: impl Into<DateTime<Utc>>
    ) -> crate::time_zone::request::Request {
        crate::time_zone::request::Request::new(self, location.into(), timestamp.into())
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Places API **Place Autocomplete** service returns place predictions.
    /// The request specifies a textual search string and optional geographic
    /// bounds. The service can be used to provide autocomplete functionality
    /// for text-based geographic searches, by returning places such as
    /// businesses, addresses and points of interest as a user types.
    ///
    /// ## Arguments
    ///
    /// * `input` ‧ The text string on which to search.

    #[cfg(feature = "autocomplete")]
    #[must_use]
    pub fn place_autocomplete(
        &self,
        input: impl Into<String>
    ) -> crate::places::place_autocomplete::request::Request {
        crate::places::place_autocomplete::request::Request::new(self, input)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Places API **Query Autocomplete** service allows you to add
    /// on-the-fly geographic query predictions to your application. Instead of
    /// searching for a specific location, a user can type in a categorical
    /// search, such as "pizza near New York" and the service responds with a
    /// list of suggested queries matching the string. As the Query Autocomplete
    /// service can match on both full words and substrings, applications can
    /// send queries as the user types to provide on-the-fly predictions.
    ///
    /// ## Arguments
    ///
    /// * `input` ‧ The text string on which to search.
    ///
    /// ## Basic usage
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
    #[must_use]
    pub fn query_autocomplete(
        &self,
        input: impl Into<String>
    ) -> crate::places::query_autocomplete::request::Request {
        crate::places::query_autocomplete::request::Request::new(self, input)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Places API **Text Search** service returns information about a set
    /// of places based on a string — for example "pizza in New York" or "shoe
    /// stores near Ottawa" or "123 Main Street". The service responds with a
    /// list of places matching the text string and any location bias that has
    /// been set.
    ///
    /// ## Arguments
    ///
    /// * `query` ‧ The text string on which to search, for example:
    ///   "restaurant" or "123 Main Street". This must a place name, address, or
    ///   category of establishments. Any other types of input can generate
    ///   errors and are not guaranteed to return valid results. The Google
    ///   Places service will return candidate matches based on this string and
    ///   order the results based on their perceived relevance.
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
    ///
    /// ## Additional information:
    ///
    /// The service is especially useful for making ambiguous address queries in
    /// an automated system, and non-address components of the string may match
    /// businesses as well as addresses. Examples of ambiguous address queries
    /// are poorly-formatted addresses or requests that include non-address
    /// components such as business names. Requests like the first two examples
    /// below may return `ZERO_RESULTS` unless a location bias - such as Region,
    /// Location, or Bounds - is set.
    ///
    /// | "10 High Street, UK" or "123 Main Street, US" | multiple "High Street"s in the UK; multiple "Main Street"s in the US. Query will not return desirable results unless a location restriction is set. |
    /// |---|---|
    /// | "`ChainRestaurant` New York" | multiple "`ChainRestaurant`" locations in New York; no street address or even street name. Query will not return desirable results unless a location restriction is set. |
    /// | "10 High Street, Escher UK" or "123 Main Street, Pleasanton US" | only one "High Street" in the UK city of Escher; only one "Main Street" in the US city of Pleasanton CA. |
    /// | "`UniqueRestaurantName` New York" | only one establishment with this name in New York; no street address needed to differentiate. |
    /// | "pizza restaurants in New York" | this query contains its location restriction, and "pizza restaurants" is a well-defined place type. Will yield multiple results, as is expected. |
    ///
    /// The search response will include a list of places. You can send a Place
    /// Details request for more information about any of the places in the
    /// response.
    ///
    /// * Nearby Search and Text Search return all of the available data fields
    ///   for the selected place (a [subset of the supported
    ///   fields](https://developers.google.com/maps/documentation/places/web-service/place-data-fields#places-api-fields-support)),
    ///   and you will be [billed accordingly](https://developers.google.com/maps/billing/understanding-cost-of-use#nearby-search)
    ///   There is no way to constrain Nearby Search or Text Search to only
    ///   return specific fields. To keep from requesting (and paying for) data
    ///   that you don't need, use a [Find Place
    ///   request](https://developers.google.com/maps/documentation/places/web-service/search#FindPlaceRequests)
    ///   instead.
    ///
    /// ## Basic usage
    ///
    /// ```rust
    /// use google_maps::prelude::*;
    /// use rust_decimal_macros::dec;
    ///
    /// let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
    ///
    /// let search_results = google_maps_client.text_search("123 Main Street".to_string(), 50_000)
    ///     .with_type(PlaceType::Restaurant)
    ///     .execute()
    ///     .await?;
    ///
    /// println!("{:#?}", search_results);
    /// ```

    #[cfg(feature = "places")]
    #[must_use]
    pub fn text_search(
        &self,
        query: impl Into<String>,
        radius: impl Into<u32>
    ) -> crate::places::place_search::text_search::request::Request {
        crate::places::place_search::text_search::request::Request::new(self, query, radius.into())
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Places API **Nearby Search** service lets you search for places
    /// within a specified area. You can refine your search request by supplying
    /// keywords or specifying the type of place you are searching for.
    ///
    /// ## Arguments
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
    ///
    /// ## Basic usage
    ///
    /// ```rust
    /// use google_maps::prelude::*;
    /// use rust_decimal_macros::dec;
    ///
    /// let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
    ///
    /// let search_results = google_maps_client.nearby_search(LatLng::try_from_dec(dec!(53.540_989), dec!(-113.493_768))?, 1_000)
    ///     .with_type(PlaceType::Restaurant)
    ///     .execute()
    ///     .await?;
    ///
    /// println!("{:#?}", search_results);
    /// ```

    #[cfg(feature = "places")]
    #[must_use]
    pub fn nearby_search(
        &self,
        location: impl Into<LatLng>,
        radius: impl Into<u32>
    ) -> crate::places::place_search::nearby_search::request::Request {
        crate::places::place_search::nearby_search::request::Request::new(
            self,
            location.into(),
            radius.into()
        )
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Places API **Place Details** service returns more details about a
    /// particular establishment or point of interest. A Place Details request
    /// returns more comprehensive information about the indicated place such as
    /// its complete address, phone number, user rating and reviews.
    ///
    /// ## Arguments
    ///
    /// * `place_id` ‧ A textual identifier that uniquely identifies a place,
    ///   returned from a
    ///   [Place Search](https://developers.google.com/maps/documentation/places/web-service/search).
    ///   For more information about place IDs, see the
    ///   [place ID overview](https://developers.google.com/maps/documentation/places/web-service/place-id).
    ///
    /// ## Basic usage
    ///
    /// ```rust
    /// use google_maps::prelude::*;
    /// use rust_decimal_macros::dec;
    ///
    /// let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");
    ///
    /// let details = google_maps_client.place_details("ChIJIyEbn74koFMR4xlRm4Ftp6M".to_string())
    ///     .execute()
    ///     .await?;
    ///
    /// println!("{:#?}", details);
    /// ```

    #[cfg(feature = "places")]
    #[must_use]
    pub fn place_details(
        &self,
        place_id: impl Into<String>
    ) -> crate::places::place_details::request::Request {
        crate::places::place_details::request::Request::new(
            self,
            place_id.into()
        )
    } // fn

    // -------------------------------------------------------------------------
    //
    /// The Roads API **Snap To Roads** service takes up to 100 GPS points
    /// collected along a route, and returns a similar set of data, with the
    /// points snapped to the most likely roads the vehicle was traveling along.
    /// Optionally, you can request that the points be interpolated, resulting
    /// in a path that smoothly follows the geometry of the road.
    ///
    /// ## Arguments
    ///
    /// * `path` ‧ The path to be snapped. Note: The snapping algorithm works
    ///   best for points that are not too far apart. If you observe odd
    ///   snapping behavior, try creating paths that have points closer
    ///   together. To ensure the best snap-to-road quality, you should aim to
    ///   provide paths on which consecutive pairs of points are within 300m of
    ///   each other. This will also help in handling any isolated, long jumps
    ///   between consecutive points caused by GPS signal loss, or noise.
    ///
    /// ## Basic usage
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
    ///
    /// # Generics
    ///
    /// This method uses generics to improve ergonomics. The `C` generic is
    /// intended to represent any collection that can be iterated over, and the
    /// `L` generic is for any type that can be converted to `LatLng`
    /// coordinates.

    #[cfg(feature = "roads")]
    #[must_use]
    pub fn snap_to_roads<C, L>(
        &self,
        path: C
    ) -> crate::roads::snap_to_roads::request::Request
    where
        C: IntoIterator<Item = L>,
        L: Into<LatLng> {
        let path: Vec<LatLng> = path.into_iter().map(Into::into).collect();
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
    /// ## Arguments
    ///
    /// * `points` ‧ The points to be snapped. The points parameter accepts a
    ///   list of latitude/longitude pairs.
    ///
    /// ## Basic usage
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
    /// ]).execute().await?;
    /// ```
    ///
    /// # Generics
    ///
    /// This method uses generics to improve ergonomics. The `C` generic is
    /// intended to represent any collection that can be iterated over, and the
    /// `L` generic is for any type that can be converted to `LatLng`
    /// coordinates.

    #[cfg(feature = "roads")]
    #[must_use]
    pub fn nearest_roads<C, L>(
        &self,
        points: C
    ) -> crate::roads::snap_to_roads::request::Request
    where
        C: IntoIterator<Item = L>,
        L: Into<LatLng> {
        let points: Vec<LatLng> = points.into_iter().map(Into::into).collect();
        crate::roads::snap_to_roads::request::Request::new(self, points)
    } // fn

    #[cfg(feature = "enable-reqwest")]
    pub async fn get_request(&self, url: &str) -> Result<Response, crate::ReqError> {
        match self.reqwest_client.get(url).build() {
            Ok(request) => self.reqwest_client.execute(request).await,
            Err(error) => Err(crate::ReqError::from(error)),
        }
    }
} // impl
