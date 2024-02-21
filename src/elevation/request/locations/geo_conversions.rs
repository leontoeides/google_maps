//! Provides some `Locations` conversion methods for the
//! [geo](https://crates.io/crates/geo) crate.

use crate::elevation::request::locations::Locations;
use crate::LatLng;
use geo_types::geometry::{Line, LineString};

// -----------------------------------------------------------------------------

impl TryFrom<&Line> for Locations {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a `geo_types::geometry::Line` struct to a
    /// `google_maps::elevation::Locations` struct.
    fn try_from(line: &Line) -> Result<Self, Self::Error> {
        let lat_lngs: Vec<LatLng> =
            vec![LatLng::try_from(&line.start)?, LatLng::try_from(&line.end)?];

        Ok(Self::LatLngs(lat_lngs))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl TryFrom<&LineString> for Locations {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a `geo_types::geometry::LineString` struct to a
    /// `google_maps::elevation::Locations` struct.
    fn try_from(line_string: &LineString) -> Result<Self, Self::Error> {
        let lat_lngs: Vec<LatLng> = line_string
            .coords()
            .map(LatLng::try_from)
            .collect::<Result<Vec<LatLng>, _>>()?;

        Ok(Self::LatLngs(lat_lngs))
    } // fn
} // impl
