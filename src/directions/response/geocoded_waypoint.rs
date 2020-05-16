use crate::{directions::response::geocoder_status::GeocoderStatus, place_type::PlaceType}; // use
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GeocodedWaypoint {
    /// Indicates the status code resulting from the geocoding operation.
    pub geocoder_status: Option<GeocoderStatus>,

    /// Whether the geocoder did not return an exact match for the original
    /// waypoint, though it was able to match part of the requested address.
    pub partial_match: Option<bool>,

    /// The place ID associated with the waypoint. Place IDs uniquely identify a
    /// place in the Google Places database and on Google Maps. Learn more about
    /// [Place IDs](https://developers.google.com/places/place-id) in the Places
    /// API developer guide.
    pub place_id: Option<String>,

    /// An array of strings denoting the type of the returned geocoded element.
    /// For a list of possible strings, refer to the
    /// [Address Component Types](https://developers.google.com/maps/documentation/javascript/geocoding#GeocodingAddressTypes)
    /// section of the Developer's Guide.
    pub types: Option<Vec<PlaceType>>,
} // struct