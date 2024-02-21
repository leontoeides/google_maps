//! Contains the `Location` enum and its associated traits. It is used to
//! specify origin and destination locations in the form of a text address,
//! latitude/longitude pair, or Google Place ID.

#[cfg(feature = "geo")]
mod geo_conversions;

// -----------------------------------------------------------------------------

use crate::types::LatLng;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

// -----------------------------------------------------------------------------
//
/// Used to specify the address, latitude/longitude, or place ID for the origin
/// and destination.

#[cfg(not(feature = "geo"))]
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum Location {
    /// If you pass an address, the Directions service geocodes the string and
    /// converts it to a latitude/longitude coordinate to calculate directions.
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
    /// see [Place Autocomplete and Directions](https://developers.google.com/maps/documentation/javascript/examples/places-autocomplete-directions).
    /// For more about place IDs, see the [Place ID overview](https://developers.google.com/places/place-id).
    PlaceId(String),
} // enum

// -----------------------------------------------------------------------------

#[cfg(not(feature = "geo"))]
impl std::convert::From<&Location> for String {
    /// Converts a `Location` enum to a `String` that contains a URL-encoded
    /// [location](https://developers.google.com/maps/documentation/directions/intro#required-parameters)
    /// value.
    fn from(location: &Location) -> Self {
        match location {
            Location::Address(address) => {
                utf8_percent_encode(address, NON_ALPHANUMERIC).to_string()
            }

            Location::LatLng(latlng) => {
                utf8_percent_encode(&Self::from(latlng), NON_ALPHANUMERIC).to_string()
            }

            Location::PlaceId(place_id) => {
                utf8_percent_encode(&format!("place_id:{place_id}"), NON_ALPHANUMERIC).to_string()
            }
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------
//
/// Used to specify the address, latitude/longitude, or place ID for the origin
/// and destination.

#[cfg(feature = "geo")]
#[derive(Clone, Debug, PartialEq)]
pub enum Location {
    /// If you pass an address, the Directions service geocodes the string and
    /// converts it to a latitude/longitude coordinate to calculate directions.
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
    /// see [Place Autocomplete and Directions](https://developers.google.com/maps/documentation/javascript/examples/places-autocomplete-directions).
    /// For more about place IDs, see the [Place ID overview](https://developers.google.com/places/place-id).
    PlaceId(String),
    /// If you pass coordinates, they are used unchanged to calculate
    /// directions. This variant supports the
    /// [geo](https://crates.io/crates/geo) crate's
    /// [Coord](https://docs.rs/geo/latest/geo/geometry/struct.Coord.html) type.
    Coord(geo_types::geometry::Coord),
    /// If you pass a point, it is used unchanged to calculate directions. This
    /// variant supports the [geo](https://crates.io/crates/geo) crate's
    /// [Point](https://docs.rs/geo/latest/geo/geometry/struct.Point.html) type.
    Point(geo_types::geometry::Point),
} // enum

// -----------------------------------------------------------------------------

#[cfg(feature = "geo")]
impl std::convert::From<&Location> for String {
    /// Converts a `Location` enum to a `String` that contains a URL-encoded
    /// [location](https://developers.google.com/maps/documentation/directions/intro#required-parameters)
    /// value.
    fn from(location: &Location) -> Self {
        match location {
            Location::Address(address) => {
                utf8_percent_encode(address, NON_ALPHANUMERIC).to_string()
            }

            Location::LatLng(latlng) => {
                utf8_percent_encode(&Self::from(latlng), NON_ALPHANUMERIC).to_string()
            }

            Location::PlaceId(place_id) => {
                utf8_percent_encode(&format!("place_id:{place_id}"), NON_ALPHANUMERIC).to_string()
            }

            Location::Coord(coordinate) => utf8_percent_encode(
                &format!(
                    "{latitude},{longitude}",
                    latitude = coordinate.y,
                    longitude = coordinate.x,
                ),
                NON_ALPHANUMERIC,
            )
            .to_string(),

            Location::Point(point) => utf8_percent_encode(
                &format!(
                    "{latitude},{longitude}",
                    latitude = point.y(),
                    longitude = point.x()
                ),
                NON_ALPHANUMERIC,
            )
            .to_string(),
        } // match
    } // fn
} // impl
