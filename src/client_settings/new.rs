use chrono::{
    DateTime,
    Utc,
}; // chrono
use crate::{
    client_settings::ClientSettings,
    directions::request::location::Location,
    directions::request::waypoint::Waypoint,
    latlng::LatLng,
    request_rate::RequestRate,
}; // use

impl ClientSettings {

    /// Initialize the settings needed for a Google Cloud Maps API transaction.
    pub fn new(key: &str) -> ClientSettings {
        ClientSettings {
            key: key.to_string(),
            max_retries: 20,
            max_backoff: 32000,
            rate_limit: RequestRate::default(),
        } // ClientSettings
    } // fn

    /// The Directions API is a service that calculates directions between
    /// locations. You can search for directions for several modes of
    /// transportation, including transit, driving, walking, or cycling.
    ///
    /// ```rust
    /// directions(
    ///     // Origin: Canadian Museum of Nature
    ///     Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
    ///     // Destination: Canada Science and Technology Museum
    ///     Location::LatLng(LatLng::try_from(dec!(45.403_509), dec!(-75.618_904)).unwrap()),
    /// )
    /// ```

    pub fn directions(&mut self, origin: Location, destination: Location) -> crate::directions::request::Request {
        crate::directions::request::Request::new(self, origin, destination)
    } // fn

    /// The Distance Matrix API is a service that provides travel distance and
    /// time for a matrix of origins and destinations, based on the recommended
    /// route between start and end points.
    ///
    /// ```rust
    /// distance_matrix(
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
    ///         Waypoint::LatLng(LatLng::try_from(dec!(37.387_316), dec!(-122.060_008)).unwrap()),
    ///     ],
    /// )
    /// ```

    pub fn distance_matrix(&mut self, origins: Vec<Waypoint>, destinations: Vec<Waypoint>) -> crate::distance_matrix::request::Request {
        crate::distance_matrix::request::Request::new(self, origins, destinations)
    } // fn

    /// The Elevation API provides elevation data for all locations on the
    /// surface of the earth, including depth locations on the ocean floor
    /// (which return negative values).

    pub fn elevation(&mut self) -> crate::elevation::request::Request {
        crate::elevation::request::Request::new(self)
    } // fn

    /// The Geocoding API is a service that provides geocoding and reverse
    /// geocoding of addresses. Geocoding is the process of converting addresses
    /// (like a street address) into geographic coordinates (like latitude and
    /// longitude), which you can use to place markers on a map, or position the
    /// map.

    pub fn geocoding(&mut self) -> crate::geocoding::forward::ForwardRequest {
        crate::geocoding::forward::ForwardRequest::new(self)
    } // fn

    /// The Geocoding API is a service that provides geocoding and reverse
    /// geocoding of addresses. Reverse geocoding is the process of converting
    /// geographic coordinates into a human-readable address.
    ///
    /// ```rust
    /// reverse_geocoding(
    ///     // 10 Downing St, Westminster, London
    ///     LatLng::try_from(dec!(51.503_364), dec!(-0.127_625)).unwrap(),
    /// )
    /// ```

    pub fn reverse_geocoding(&mut self, latlng: LatLng) -> crate::geocoding::reverse::ReverseRequest {
        crate::geocoding::reverse::ReverseRequest::new(self, latlng)
    } // fn

    /// The Time Zone API provides time offset data for locations on the surface
    /// of the earth. You request the time zone information for a specific
    /// latitude/longitude pair and date. The API returns the name of that time
    /// zone, the time offset from UTC, and the daylight savings offset.
    ///
    /// ```rust
    /// time_zone(
    ///      // St. Vitus Cathedral in Prague, Czechia
    ///      LatLng::try_from(dec!(50.090_903), dec!(14.400_512)).unwrap(),
    ///      // Tuesday February 23, 2020 @ 6:00:00 pm
    ///      NaiveDate::from_ymd(2020, 2, 23).and_hms(18, 00, 0)
    /// )
    /// ```

    pub fn time_zone(&mut self, location: LatLng, timestamp: DateTime<Utc>) -> crate::time_zone::request::Request {
        crate::time_zone::request::Request::new(self, location, timestamp)
    } // fn

} // impl