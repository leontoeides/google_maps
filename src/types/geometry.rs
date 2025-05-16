//! Contains the geocoded latitude/longitude, recommended viewport for
//! displaying the returned result, the bounding box, and other additional
//! data.

use crate::types::{Bounds, LatLng, LocationType};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Contains the geocoded latitude/longitude, recommended viewport for
/// displaying the returned result, the bounding box, and other additional
/// data.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Geometry {
    /// Contains the geocoded latitude, longitude value. For normal address
    /// lookups, this field is typically the most important.
    pub location: LatLng,

    /// Stores additional data about the specified location.
    #[serde(default)]
    pub location_type: Option<LocationType>,

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
    #[serde(default)]
    pub bounds: Option<Bounds>,
} // struct

// =============================================================================

impl Geometry {
    // -------------------------------------------------------------------------
    //
    /// A helper function for destructuring the optional `bounds` field. If
    /// the _bounds_ field is populated, this function will return the
    /// south-west _latitude_. If the _bounds_ field is empty, this function
    /// will return `None`.
    /// ```rust
    /// let bounds_southwest_lat = geocoding.geometry.get_bounds_southwest_lng();
    /// ```
    #[must_use]
    pub fn get_bounds_southwest_lat(&self) -> Option<Decimal> {
        self.bounds.as_ref().map(|bounds| bounds.southwest.lat)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// A helper function for destructuring the optional `bounds` field. If
    /// the _bounds_ field is populated, this function will return the
    /// south-west _latitude_. If the _bounds_ field is empty, this function
    /// will return `None`.
    /// ```rust
    /// let bounds_southwest_lng = geocoding.geometry.get_bounds_southwest_lng();
    /// ```
    #[must_use]
    pub fn get_bounds_southwest_lng(&self) -> Option<Decimal> {
        self.bounds.as_ref().map(|bounds| bounds.southwest.lng)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// A helper function for destructuring the optional `bounds` field. If
    /// the _bounds_ field is populated, this function will return the
    /// north-east _latitude_. If the _bounds_ field is empty, this function
    /// will return `None`.
    /// ```rust
    /// let bounds_northeast_lat = geocoding.geometry.get_bounds_northeast_lng();
    /// ```
    #[must_use]
    pub fn get_bounds_northeast_lat(&self) -> Option<Decimal> {
        self.bounds.as_ref().map(|bounds| bounds.northeast.lat)
    } // fn

    // -------------------------------------------------------------------------
    //
    /// A helper function for destructuring the optional `bounds` field. If
    /// the _bounds_ field is populated, this function will return the
    /// north-east _latitude_. If the _bounds_ field is empty, this function
    /// will return `None`.
    /// ```rust
    /// let bounds_northeast_lng = geocoding.geometry.get_bounds_northeast_lng();
    /// ```
    #[must_use]
    pub fn get_bounds_northeast_lng(&self) -> Option<Decimal> {
        self.bounds.as_ref().map(|bounds| bounds.northeast.lng)
    } // fn
} // impl
