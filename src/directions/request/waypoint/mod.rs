//! Contains the `Waypoint` enum and its associated traits. It is used to
//! specify intermediate locations in the form of a text address,
//! a latitude & longitude pair, a Google Place ID, or as an Encoded Polyline.

#[cfg(feature = "geo")]
mod geo;

// -----------------------------------------------------------------------------

use crate::types::LatLng;
use crate::GoogleMapsError;
use rust_decimal::Decimal;

// -----------------------------------------------------------------------------
//
/// Used to specify pass throughs or stopovers at intermediate locations.

#[cfg(not(feature = "geo"))]
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum Waypoint {
    /// If you pass an address, the Directions service geocodes the string and
    /// converts it to latitude & longitude coordinates to calculate directions.
    /// This coordinate may be different from that returned by the Geocoding
    /// API, for example a building entrance rather than its center.
    Address(String),
    /// If you pass coordinates, they are used unchanged to calculate
    /// directions.
    LatLng(LatLng),
    /// The place ID may only be specified if the request includes an API key or
    /// a Google Maps Platform Premium Plan client ID. You can retrieve place
    /// IDs from the Geocoding API and the Places API (including Place
    /// Autocomplete). For an example using place IDs from Place Autocomplete,
    /// see [Place Autocomplete and
    /// Directions](https://developers.google.com/maps/documentation/javascript/examples/places-autocomplete-directions).
    /// For more about place IDs, see the [Place ID
    /// overview](https://developers.google.com/places/place-id).
    PlaceId(String),
    /// Alternatively, you can supply an encoded set of points using the
    /// [Encoded Polyline
    /// Algorithm](https://developers.google.com/maps/documentation/utilities/polylinealgorithm).
    /// You will find an encoded set is useful for a large number of waypoints,
    /// because the URL is significantly shorter. All web services have a URL
    /// limit of 8192 characters.
    ///
    /// See also: the Google Encoded Polyline encoding & decoding crate called
    /// [polyline](https://crates.io/crates/polyline).
    Polyline(String),
} // enum

// -----------------------------------------------------------------------------

#[cfg(not(feature = "geo"))]
impl std::convert::From<&Waypoint> for String {
    /// Converts a `Waypoint` enum to a `String` that contains a
    /// [waypoint](https://developers.google.com/maps/documentation/directions/intro#Waypoints)
    /// value.
    fn from(waypoint: &Waypoint) -> Self {
        match waypoint {
            Waypoint::Address(address) => address.clone(),
            Waypoint::LatLng(latlng) => Self::from(latlng),
            Waypoint::PlaceId(place_id) => format!("place_id:{place_id}"),
            Waypoint::Polyline(polyline) => format!("enc:{polyline}:"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Self> for Waypoint {
    /// Converts a borrowed `&Waypoint` enum into an owned `Waypoint` enum by
    /// cloning it.
    fn from(waypoint: &Self) -> Self {
        waypoint.clone()
    } // fn
} // impl

// -----------------------------------------------------------------------------
//
/// Used to specify pass throughs or stopovers at intermediate locations.

#[cfg(feature = "geo")]
#[derive(Clone, Debug, PartialEq)]
pub enum Waypoint {
    /// If you pass an address, the Directions service geocodes the string and
    /// converts it to latitude & longitude coordinates to calculate directions.
    /// This coordinate may be different from that returned by the Geocoding
    /// API, for example a building entrance rather than its center.
    Address(String),
    /// If you pass coordinates, they are used unchanged to calculate
    /// directions.
    LatLng(LatLng),
    /// The place ID may only be specified if the request includes an API key or
    /// a Google Maps Platform Premium Plan client ID. You can retrieve place
    /// IDs from the Geocoding API and the Places API (including Place
    /// Autocomplete). For an example using place IDs from Place Autocomplete,
    /// see [Place Autocomplete and
    /// Directions](https://developers.google.com/maps/documentation/javascript/examples/places-autocomplete-directions).
    /// For more about place IDs, see the [Place ID
    /// overview](https://developers.google.com/places/place-id).
    PlaceId(String),
    /// Alternatively, you can supply an encoded set of points using the
    /// [Encoded Polyline
    /// Algorithm](https://developers.google.com/maps/documentation/utilities/polylinealgorithm).
    /// You will find an encoded set is useful for a large number of waypoints,
    /// because the URL is significantly shorter. All web services have a URL
    /// limit of 8192 characters.
    ///
    /// See also: the Google Encoded Polyline encoding & decoding crate called
    /// [polyline](https://crates.io/crates/polyline).
    Polyline(String),
    /// If you pass coordinates, they are used unchanged to calculate
    /// directions. This variant supports the
    /// [geo](https://crates.io/crates/geo) crate's
    /// [Coord](https://docs.rs/geo/latest/geo/geometry/struct.Coord.html) type.
    Coord(geo_types::geometry::Coord),
    /// If you pass coordinates, they are used unchanged to calculate
    /// directions. This variant supports the
    /// [geo](https://crates.io/crates/geo) crate's
    /// [Point](https://docs.rs/geo/latest/geo/geometry/struct.Point.html) type.
    Point(geo_types::geometry::Point),
} // enum

// -----------------------------------------------------------------------------

#[cfg(feature = "geo")]
impl std::convert::From<&Waypoint> for String {
    /// Converts a `Waypoint` enum to a `String` that contains a
    /// [waypoint](https://developers.google.com/maps/documentation/directions/intro#Waypoints)
    /// value.
    fn from(waypoint: &Waypoint) -> Self {
        match waypoint {
            Waypoint::Address(address) => address.clone(),

            Waypoint::LatLng(latlng) => Self::from(latlng),

            Waypoint::PlaceId(place_id) => format!("place_id:{place_id}"),

            Waypoint::Polyline(polyline) => format!("enc:{polyline}:"),

            Waypoint::Coord(coordinate) => format!(
                "{latitude},{longitude}",
                latitude = coordinate.y,
                longitude = coordinate.x
            ),

            Waypoint::Point(point) => format!(
                "{latitude},{longitude}",
                latitude = point.y(),
                longitude = point.x()
            ),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl Waypoint {
    /// If you pass an address, the Directions service geocodes the string and
    /// converts it to latitude & longitude coordinates to calculate directions.
    /// This coordinate may be different from that returned by the Geocoding
    /// API, for example a building entrance rather than its center.
    pub fn from_address(address: impl Into<String>) -> Self {
        Self::Address(address.into())
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl Waypoint {
    /// The place ID may only be specified if the request includes an API key or
    /// a Google Maps Platform Premium Plan client ID. You can retrieve place
    /// IDs from the Geocoding API and the Places API (including Place
    /// Autocomplete). For an example using place IDs from Place Autocomplete,
    /// see [Place Autocomplete and
    /// Directions](https://developers.google.com/maps/documentation/javascript/examples/places-autocomplete-directions).
    /// For more about place IDs, see the [Place ID
    /// overview](https://developers.google.com/places/place-id).
    pub fn from_place_id(place_id: impl Into<String>) -> Self {
        Self::PlaceId(place_id.into())
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl Waypoint {
    /// Alternatively, you can supply an encoded set of points using the
    /// [Encoded Polyline
    /// Algorithm](https://developers.google.com/maps/documentation/utilities/polylinealgorithm).
    /// You will find an encoded set is useful for a large number of waypoints,
    /// because the URL is significantly shorter. All web services have a URL
    /// limit of 8192 characters.
    ///
    /// See also: the Google Encoded Polyline encoding & decoding crate called
    /// [polyline](https://crates.io/crates/polyline).
    pub fn from_polyline(polyline: impl Into<String>) -> Self {
        Self::Polyline(polyline.into())
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl Waypoint {
    /// Takes individual latitude & longitude `Decimal` coordinates and
    /// converts them into a `Waypoint` structure. If either the latitude
    /// (-90.0 to +90.0) or longitude (-180.0 to +180.0) are out of range, this
    /// function will return an error.
    pub fn try_from_dec(latitude: Decimal, longitude: Decimal) -> Result<Self, GoogleMapsError> {
        let latlng = LatLng::try_from_dec(latitude, longitude)?;
        Ok(Self::LatLng(latlng))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl Waypoint {
    /// Takes individual latitude & longitude `f32` coordinates and
    /// converts them into a `Waypoint` structure. If either the latitude
    /// (-90.0 to +90.0) or longitude (-180.0 to +180.0) are out of range, this
    /// function will return an error.
    pub fn try_from_f32(latitude: f32, longitude: f32) -> Result<Self, GoogleMapsError> {
        let latlng = LatLng::try_from_f32(latitude, longitude)?;
        Ok(Self::LatLng(latlng))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl Waypoint {
    /// Takes individual latitude & longitude `f64` coordinates and
    /// converts them into a `Waypoint` structure. If either the latitude
    /// (-90.0 to +90.0) or longitude (-180.0 to +180.0) are out of range, this
    /// function will return an error.
    pub fn try_from_f64(latitude: f64, longitude: f64) -> Result<Self, GoogleMapsError> {
        let latlng = LatLng::try_from_f64(latitude, longitude)?;
        Ok(Self::LatLng(latlng))
    } // fn
} // impl
