//! Provides some `Locations` conversion methods for the
//! [geo](https://crates.io/crates/geo) crate.

use crate::elevation::request::locations::Locations;
use crate::types::LatLng;
use geo_types::geometry::{Line, LineString};

// -----------------------------------------------------------------------------

impl TryFrom<&Line> for Locations {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a borrowed `&geo_types::geometry::Line` struct to a
    /// `google_maps::elevation::Locations` struct.
    fn try_from(line: &Line) -> Result<Self, Self::Error> {
        let lat_lngs: Vec<LatLng> =
            vec![LatLng::try_from(&line.start)?, LatLng::try_from(&line.end)?];

        Ok(Self::LatLngs(lat_lngs))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<Line> for Locations {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert an owned `geo_types::geometry::Line` struct into a
    /// `google_maps::elevation::Locations` struct.
    fn try_from(line: Line) -> Result<Self, Self::Error> {
        (&line).try_into()
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&LineString> for Locations {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a borrowed `&geo_types::geometry::LineString` struct
    /// to a `google_maps::elevation::Locations` struct.
    fn try_from(line_string: &LineString) -> Result<Self, Self::Error> {
        let lat_lngs: Vec<LatLng> = line_string
            .coords()
            .map(LatLng::try_from)
            .collect::<Result<Vec<LatLng>, _>>()?;

        Ok(Self::LatLngs(lat_lngs))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<LineString> for Locations {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert an owned `geo_types::geometry::LineString` struct
    /// into a `google_maps::elevation::Locations` struct.
    fn try_from(line_string: LineString) -> Result<Self, Self::Error> {
        (&line_string).try_into()
    } // fn
} // impl
