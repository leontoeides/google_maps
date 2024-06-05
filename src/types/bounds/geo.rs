//! Provides some `Bounds` conversion methods for the
//! [geo](https://crates.io/crates/geo) crate.

use crate::{Bounds, LatLng};
use geo_types::geometry::{Coord, Polygon, Rect};

// -----------------------------------------------------------------------------

impl TryFrom<&Rect> for Bounds {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a borrowed `&geo_types::geometry::Rect` struct to a
    /// `google_maps::Bounds` struct.
    fn try_from(rect: &Rect) -> Result<Self, Self::Error> {
        let min_coordinate: Coord = rect.min();
        let max_coordinate: Coord = rect.max();

        let southwest: LatLng = LatLng::try_from_f64(max_coordinate.y, min_coordinate.x)?;
        let northeast: LatLng = LatLng::try_from_f64(min_coordinate.y, max_coordinate.x)?;

        Ok(Self {
            southwest,
            northeast,
        })
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<Rect> for Bounds {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert an owned `geo_types::geometry::Rect` struct into a
    /// `google_maps::Bounds` struct.
    fn try_from(rect: Rect) -> Result<Self, Self::Error> {
        (&rect).try_into()
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&Bounds> for Rect {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a borrowed `&google_maps::Bounds` struct to a
    /// `geo_types::geometry::Rect` struct.
    fn try_from(bounds: &Bounds) -> Result<Self, Self::Error> {
        let c1: Coord = Coord::try_from(&bounds.southwest)?;
        let c2: Coord = Coord::try_from(&bounds.northeast)?;

        Ok(Self::new(c1, c2))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<Bounds> for Rect {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert an owned `google_maps::Bounds` struct into a
    /// `geo_types::geometry::Rect` struct.
    fn try_from(bounds: Bounds) -> Result<Self, Self::Error> {
        (&bounds).try_into()
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&Bounds> for Polygon {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a borrowed `&google_maps::Bounds` struct to a
    /// `geo_types::geometry::Polygon` struct.
    fn try_from(bounds: &Bounds) -> Result<Self, Self::Error> {
        Ok(Rect::try_from(bounds)?.to_polygon())
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<Bounds> for Polygon {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert an owned `google_maps::Bounds` struct into a
    /// `geo_types::geometry::Polygon` struct.
    fn try_from(bounds: Bounds) -> Result<Self, Self::Error> {
        (&bounds).try_into()
    } // fn
} // impl
