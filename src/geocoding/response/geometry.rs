use crate::{
    bounds::Bounds,
    geocoding::location_type::LocationType,
    latlng::LatLng,
}; // use
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Geometry {

    /// Contains the geocoded latitude, longitude value. For normal address
    /// lookups, this field is typically the most important.
    pub location: LatLng,

    /// Stores additional data about the specified location.
    pub location_type: LocationType,

    /// Contains the recommended viewport for displaying the returned result,
    /// specified as two latitude/longitude values defining the southwest and
    /// northeast corner of the viewport bounding box. Generally the viewport is
    /// used to frame a result when displaying it to a user.
    pub viewport: Bounds,

    /// Stores the bounding box which can fully contain the returned result.
    /// Note that these bounds may not match the recommended viewport. (For
    /// example, San Francisco includes the [Farallon
    /// islands](https://en.wikipedia.org/wiki/Farallon_Islands), which are
    /// technically part of the city, but probably should not be returned in the
    /// viewport.)
    pub bounds: Option<Bounds>,

} // struct

