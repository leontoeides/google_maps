//! Contains the `Locations` enum and its associated traits. It is used to
//! specify a series of location in the form of latitude & longitude pairs,
//! or as an Encoded Polyline.

#[cfg(feature = "geo")]
mod geo;

// -----------------------------------------------------------------------------

use crate::types::LatLng;
use rust_decimal_macros::dec;

// -----------------------------------------------------------------------------
//
/// Null Island: this whimsical term refers to the intersection of 0 degrees
/// latitude and 0 degrees longitude.

const NULL_ISLAND: LatLng = LatLng {
    lat: dec!(0.0),
    lng: dec!(0.0),
};

// -----------------------------------------------------------------------------
//
/// Defines the
/// [location(s)](https://developers.google.com/maps/documentation/elevation/intro#Locations)
/// on the earth from which to return elevation data.
///
/// This parameter takes either a single location as a latitude/longitude
/// pair, multiple latitude/longitude pairs, or an encoded polyline.

#[cfg(not(feature = "geo"))]
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub enum Locations {
    /// A single or multiple
    /// [latitude/longitude](https://developers.google.com/maps/documentation/elevation/intro#Locations)
    /// pairs.
    LatLngs(Vec<LatLng>),
    /// An [encoded
    /// polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm).
    ///
    /// See also: the Google Encoded Polyline encoding & decoding crate called
    /// [polyline](https://crates.io/crates/polyline).
    Polyline(String),
} // enum

// -----------------------------------------------------------------------------

#[cfg(not(feature = "geo"))]
impl std::convert::From<&Locations> for String {
    /// Converts a `Locations` enum to a `String` that contains
    /// [locations](https://developers.google.com/maps/documentation/elevation/intro#Locations).
    fn from(locations: &Locations) -> Self {
        match locations {
            Locations::LatLngs(latlngs) => latlngs
                .iter()
                .map(Self::from)
                .collect::<Vec<Self>>()
                .join("|"),

            Locations::Polyline(polyline) => format!("enc:{polyline}"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------
//
/// Defines the
/// [location(s)](https://developers.google.com/maps/documentation/elevation/intro#Locations)
/// on the earth from which to return elevation data.
///
/// This parameter takes either a single location as a latitude/longitude
/// pair, multiple latitude/longitude pairs, or an encoded polyline.

#[cfg(feature = "geo")]
#[derive(Clone, Debug, PartialEq)]
pub enum Locations {
    /// A single or multiple
    /// [latitude/longitude](https://developers.google.com/maps/documentation/elevation/intro#Locations)
    /// pairs.
    LatLngs(Vec<LatLng>),
    /// An [encoded
    /// polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm).
    ///
    /// See also: the Google Encoded Polyline encoding & decoding crate called
    /// [polyline](https://crates.io/crates/polyline).
    Polyline(String),
    /// This variant supports the
    /// [geo](https://crates.io/crates/geo) crate's
    /// [Line](https://docs.rs/geo/latest/geo/geometry/struct.Line.html) type.
    Line(geo_types::geometry::Line),
    /// This variant supports the
    /// [geo](https://crates.io/crates/geo) crate's
    /// [LineString](https://docs.rs/geo/latest/geo/geometry/struct.LineString.html) type.
    LineString(geo_types::geometry::LineString),
} // enum

// -----------------------------------------------------------------------------

#[cfg(feature = "geo")]
impl std::convert::From<&Locations> for String {
    /// Converts a `Locations` enum to a `String` that contains
    /// [locations](https://developers.google.com/maps/documentation/elevation/intro#Locations).
    fn from(locations: &Locations) -> Self {
        match locations {
            Locations::LatLngs(latlngs) => latlngs
                .iter()
                .map(Self::from)
                .collect::<Vec<Self>>()
                .join("|"),

            Locations::Polyline(polyline) => format!("enc:{polyline}"),

            Locations::Line(line) => format!(
                "{start_lat},{start_lng}|{end_lat},{end_lng}",
                start_lat = line.start.y,
                start_lng = line.start.x,
                end_lat = line.start.y,
                end_lng = line.start.x,
            ), // format!

            Locations::LineString(line_string) => line_string
                .coords()
                .map(|coordinate| format!("{lat},{lng}", lat = coordinate.y, lng = coordinate.x)) // map
                .collect::<Vec<Self>>()
                .join("|"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for Locations {
    /// Returns a reasonable default variant for the `Locations` enum type.
    fn default() -> Self {
        Self::LatLngs(vec![NULL_ISLAND])
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl Locations {
    /// An [encoded
    /// polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm).
    ///
    /// See also: the Google Encoded Polyline encoding & decoding crate called
    /// [polyline](https://crates.io/crates/polyline).
    pub fn from_polyline(polyline: impl Into<String>) -> Self {
        Self::Polyline(polyline.into())
    } // fn
} // impl
