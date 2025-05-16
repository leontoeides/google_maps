//! Resources (enums, structs) for processing the _Directions API_ response from
//! the Google Maps Platform. Look in here for more information about the data
//! returned from Google's server and how to parse it with your program.

pub mod directions_distance;
pub mod directions_duration;
pub mod driving_maneuver;
pub mod geocoded_waypoint;
pub mod geocoder_status;
pub mod leg;
pub mod overview_polyline;
pub mod polyline;
pub mod route;
pub mod status;
pub mod step;
pub mod transit_agency;
pub mod transit_currency;
pub mod transit_details;
pub mod transit_fare;
pub mod transit_line;
pub mod transit_stop;
pub mod transit_time;
pub mod transit_vehicle;

// -----------------------------------------------------------------------------

use crate::directions::{
    Error,
    response::{
        geocoded_waypoint::GeocodedWaypoint,
        route::Route,
        status::Status
    },
    travel_mode::TravelMode,
};

use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Directions responses contain the following root elements.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// Contains an array of available travel modes. This field is returned when
    /// a request specifies a travel `mode` and gets no results. The array
    /// contains the available travel modes in the countries of the given set of
    /// waypoints. This field is not returned if one or more of the waypoints
    /// are `via:` waypoints.
    #[serde(default)]
    pub available_travel_modes: Vec<TravelMode>,

    /// When the status code is other than `OK`, there may be an additional
    /// `error_message` field within the Directions response object. This field
    /// contains more detailed information about the reasons behind the given
    /// status code.
    ///
    /// **Note**: This field is not guaranteed to be always present, and its
    /// content is subject to change.
    #[serde(default)]
    pub error_message: Option<String>,

    /// Contains an array with details about the geocoding of origin,
    /// destination and waypoints. See [Geocoded
    /// Waypoints](https://developers.google.com/maps/documentation/directions/intro#GeocodedWaypoints).
    #[serde(default)]
    pub geocoded_waypoints: Vec<GeocodedWaypoint>,

    /// Contains an array of routes from the origin to the destination. See
    /// [Routes](https://developers.google.com/maps/documentation/directions/intro#Routes).
    /// Routes consist of nested
    /// [Legs](https://developers.google.com/maps/documentation/directions/intro#Legs)
    /// and [Steps](https://developers.google.com/maps/documentation/directions/intro#Steps).
    #[serde(default)]
    pub routes: Vec<Route>,

    /// Contains metadata on the request. See [Status
    /// Codes](https://developers.google.com/maps/documentation/directions/intro#StatusCodes).
    pub status: Status,
} // struct

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<String> for Response {
    type Error = serde_json::Error;
    /// Convert a Google Maps API [JSON](https://en.wikipedia.org/wiki/JSON)
    /// `String` response into a `Response` struct.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        serde_json::from_slice(&s.into_bytes())
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for Response {
    type Err = serde_json::Error;
    /// Converts a Google Maps API [JSON](https://en.wikipedia.org/wiki/JSON)
    /// `&str` response into a `Response` struct.
    ///
    /// # Notes
    ///
    /// * It's recommended to use the implemented `TryFrom` trait instead.
    ///
    /// * The [serde_json](https://crates.io/crates/simd-json)'s `from_str`
    ///   function implementation is unsafe and it's `from_slice` function
    ///   requires a mutable reference. Therefore this trait clones the `&str`
    ///   into a `String` to give `from_slice` mutable access to the string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.to_string().into_bytes();
        serde_json::from_slice(&bytes)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<Response> for Result<Response, crate::directions::Error> {
    /// Converts a Google Maps API `Response` into a `Result<Response, Error>`
    /// by examining the `status` field inside of the response.
    ///
    /// If the status indicates a success, then an `Ok(response)` will be
    /// returned. If the status indicates an error, then an `Err(error)` will be
    /// returned.
    fn from(response: Response) -> Self {
        match response.status {
            Status::Ok => Ok(response),
            Status::InvalidRequest => Err(Error::InvalidRequest),
            Status::MaxRouteLengthExceeded => Err(Error::MaxRouteLengthExceeded),
            Status::MaxWaypointsExceeded => Err(Error::MaxWaypointsExceeded),
            Status::NotFound => Err(Error::NotFound),
            Status::OverDailyLimit => Err(Error::OverDailyLimit),
            Status::OverQueryLimit => Err(Error::OverQueryLimit),
            Status::RequestDenied => Err(Error::RequestDenied),
            Status::UnknownError => Err(Error::UnknownError),
            Status::ZeroResults => Err(Error::ZeroResults),
        } // match
    } // fn
} // impl