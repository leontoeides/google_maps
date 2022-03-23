use crate::client_settings::ClientSettings;
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

impl ClientSettings {

    /// Initialize the settings needed for a Google Cloud Maps API transaction.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    #[cfg(feature = "enable-reqwest")]
    pub fn new(key: &str) -> ClientSettings {
        ClientSettings {
            key: key.to_string(),
            rate_limit: RequestRate::default(),
            reqwest_client: None,
        } // ClientSettings
    } // fn

    /// Initialize the settings needed for a Google Cloud Maps API transaction.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    #[cfg(not(feature = "enable-reqwest"))]
    pub fn new(key: &str) -> ClientSettings {
        ClientSettings {
            key: key.to_string(),
        } // ClientSettings
    } // fn

    /// The Directions API is a service that calculates directions between
    /// locations. You can search for directions for several modes of
    /// transportation, including transit, driving, walking, or cycling.
    ///
    /// ```rust
    /// use google_maps::prelude::*;
    /// directions(
    ///     // Origin: Canadian Museum of Nature
    ///     Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
    ///     // Destination: Canada Science and Technology Museum
    ///     Location::LatLng(LatLng::try_from(dec!(45.403_509), dec!(-75.618_904))?),
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

    /// The Distance Matrix API is a service that provides travel distance and
    /// time for a matrix of origins and destinations, based on the recommended
    /// route between start and end points.
    ///
    /// ```rust
    /// use google_maps::prelude::*;
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
    ///         Waypoint::LatLng(LatLng::try_from(dec!(37.387_316), dec!(-122.060_008))?),
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

    /// The Elevation API provides elevation data for all locations on the
    /// surface of the earth, including depth locations on the ocean floor
    /// (which return negative values).

    #[cfg(feature = "elevation")]
    pub fn elevation(&self) -> crate::elevation::request::Request {
        crate::elevation::request::Request::new(self)
    } // fn

    /// The Geocoding API is a service that provides geocoding and reverse
    /// geocoding of addresses. Geocoding is the process of converting addresses
    /// (like a street address) into geographic coordinates (like latitude and
    /// longitude), which you can use to place markers on a map, or position the
    /// map.

    #[cfg(feature = "geocoding")]
    pub fn geocoding(&self) -> crate::geocoding::forward::ForwardRequest {
        crate::geocoding::forward::ForwardRequest::new(self)
    } // fn

    /// The Geocoding API is a service that provides geocoding and reverse
    /// geocoding of addresses. Reverse geocoding is the process of converting
    /// geographic coordinates into a human-readable address.
    ///
    /// ```rust
    /// use google_maps::LatLng;
    /// reverse_geocoding(
    ///     // 10 Downing St, Westminster, London
    ///     LatLng::try_from(dec!(51.503_364), dec!(-0.127_625))?,
    /// )
    /// ```

    #[cfg(feature = "geocoding")]
    pub fn reverse_geocoding(
        &self,
        latlng: LatLng,
    ) -> crate::geocoding::reverse::ReverseRequest {
        crate::geocoding::reverse::ReverseRequest::new(self, latlng)
    } // fn

    /// The Time Zone API provides time offset data for locations on the surface
    /// of the earth. You request the time zone information for a specific
    /// latitude/longitude pair and date. The API returns the name of that time
    /// zone, the time offset from UTC, and the daylight savings offset.
    ///
    /// ```rust
    /// use google_maps::{LatLng, NaiveDate};
    /// time_zone(
    ///      // St. Vitus Cathedral in Prague, Czechia
    ///      LatLng::try_from(dec!(50.090_903), dec!(14.400_512))?,
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

    /// The Place API _Place Autocomplete_ service returns place predictions.
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

    /// The Place API _Query Autocomplete_ service allows you to add on-the-fly
    /// geographic query predictions to your application. Instead of searching
    /// for a specific location, a user can type in a categorical search, such
    /// as "pizza near New York" and the service responds with a list of
    /// suggested queries matching the string. As the Query Autocomplete service
    /// can match on both full words and substrings, applications can send
    /// queries as the user types to provide on-the-fly predictions.

    #[cfg(feature = "autocomplete")]
    pub fn query_autocomplete(
        &self,
        input: String,
    ) -> crate::places::query_autocomplete::request::Request {
        crate::places::query_autocomplete::request::Request::new(self, input)
    } // fn

} // impl