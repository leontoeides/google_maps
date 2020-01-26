use crate::latlng::LatLng;
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

/// Defines the
/// [location(s)](https://developers.google.com/maps/documentation/elevation/intro#Locations)
/// on the earth from which to return elevation data.
///
/// This parameter takes either a single location as a latitude/longitude
/// pair, multiple latitude/longitude pairs, or an encoded polyline.

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Locations {

    /// A single or multiple
    /// [latitude/longitude](https://developers.google.com/maps/documentation/elevation/intro#Locations)
    /// pairs.
    LatLngs(Vec<LatLng>),

    /// An [encoded
    /// polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm).
    Polyline(String),

} // enum

impl std::convert::From<&Locations> for String {
    /// Converts a `Locations` enum to a `String` that contains
    /// [locations](https://developers.google.com/maps/documentation/elevation/intro#Locations).
    fn from(locations: &Locations) -> String {
        match locations {
            Locations::LatLngs(latlngs) => String::from(
                latlngs.iter().map(|latlng| String::try_from(latlng).unwrap() + "|").collect::<String>().trim_end_matches('|')
            ),
            Locations::Polyline(polyline) => format!("enc:{}", polyline),
        } // match
    } // fn
} // impl

impl std::default::Default for Locations {
    /// Returns a reasonable default variant for the `Locations` enum type.
    fn default() -> Self {
        Locations::LatLngs(vec![LatLng { lat: 0.0, lng: 0.0 }])
    } // fn
} // impl