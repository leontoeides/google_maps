use crate::address_validation::{PlusCode, Viewport};
use crate::types::LatLng;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Contains information about the place the input was geocoded to.
#[allow(clippy::doc_markdown)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, CopyGetters, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct Geocode {
    /// The geocoded location of the input.
    ///
    /// Using place IDs is preferred over using addresses, latitude/longitude
    /// coordinates, or plus codes. Using coordinates when routing or
    /// calculating driving directions will always result in the point being
    /// snapped to the road nearest to those coordinates. This may not be a road
    /// that will quickly or safely lead to the destination and may not be near
    /// an access point to the property. Additionally, when a location is
    /// reverse geocoded, there is no guarantee that the returned address will
    /// match the original.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub location: LatLng,

    /// The plus code corresponding to the `location`.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub plus_code: PlusCode,

    /// The bounds of the geocoded place.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub bounds: Viewport,

    /// The size of the geocoded place, in meters. This is another measure of
    /// the coarseness of the geocoded location, but in physical size rather
    /// than in semantic meaning.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub feature_size_meters: Option<rust_decimal::Decimal>,

    /// The PlaceID of the place this input geocodes to.
    ///
    /// For more information about Place IDs see here.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub place_id: String,

    /// The type(s) of place that the input geocoded to. For example,
    /// `['locality', 'political']`. The full list of types can be found
    /// [here](https://developers.google.com/maps/documentation/geocoding/requests-geocoding#Types).
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub place_types: Vec<String>,
} // struct Geocode