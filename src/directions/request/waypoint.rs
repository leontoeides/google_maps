//! Contains the `Waypoint` enum and its associated traits. It is used to
//! specify intermediate locations in the form of a text address,
//! a latitude & longitude pair, a Google Place ID, or as an Encoded Polyline.

use crate::latlng::LatLng;
use serde::{Serialize, Deserialize};

/// Used to specify pass throughs or stopovers at intermediate locations.

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
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
    Polyline(String),

} // enum

impl std::convert::From<&Waypoint> for String {
    /// Converts a `Waypoint` enum to a `String` that contains a
    /// [waypoint](https://developers.google.com/maps/documentation/directions/intro#Waypoints)
    /// value.
    fn from(waypoint: &Waypoint) -> String {
        match waypoint {
            Waypoint::Address(address) => address.clone(),
            Waypoint::LatLng(latlng) => latlng.to_string(),
            Waypoint::PlaceId(place_id) => format!("place_id:{}", place_id),
            Waypoint::Polyline(polyline) => format!("enc:{}:", polyline),
        } // match
    } // fn
} // impl