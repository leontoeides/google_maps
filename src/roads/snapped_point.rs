//! Resources (enums, structs) for processing the _Snap To Roads_ response from
//! the Google Maps Platform. Look in here for more information about the data
//! returned from Google's server and how to parse it with your program.

// -----------------------------------------------------------------------------

use crate::types::LatLng;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// [SnappedPoint](https://developers.google.com/maps/documentation/roads/snap#SnappedPoint)

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SnappedPoint {
    /// See [LatitudeLongitudeLiteral](https://developers.google.com/maps/documentation/roads/snap#LatitudeLongitudeLiteral)
    /// for more information.
    pub location: LatLng,

    /// A unique identifier for a place. **All place IDs returned by the Roads
    /// API correspond to road segments.**
    #[serde(alias = "placeId")]
    pub place_id: Option<String>,

    /// An integer that indicates the corresponding value in the original
    /// request. Each value in the request should map to a snapped value in the
    /// response. However, if you've set interpolate=true, then it's possible
    /// that the response will contain more coordinates than the request.
    /// Interpolated values will not have an `originalIndex`. These values are
    /// indexed from `0`, so a point with an originalIndex of `4` will be the
    /// snapped value of the 5th latitude/longitude passed to the path
    /// parameter.
    #[serde(alias = "originalIndex")]
    pub origin_index: Option<usize>,
} // struct
