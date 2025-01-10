//! Common types used across several Google Maps API interfaces.

#[cfg(any(feature = "geocoding", feature = "places"))]
pub(super) mod address_component;

#[cfg(any(
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
pub(super) mod bounds;

pub(super) mod classified_error;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "geocoding"
))]
pub(super) mod country;

pub(super) mod error;

#[cfg(any(feature = "geocoding", feature = "places"))]
pub(super) mod geometry;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places",
    feature = "time_zone"
))]
pub(super) mod language;

#[cfg(any(
    feature = "address_validation",
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "elevation",
    feature = "geocoding",
    feature = "places",
    feature = "roads",
    feature = "time_zone"
))]
pub(super) mod latlng;

#[cfg(any(feature = "geocoding", feature = "places"))]
pub(super) mod location_type;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
pub(super) mod place_type;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
pub(super) mod region;

// -----------------------------------------------------------------------------

#[cfg(any(feature = "geocoding", feature = "places"))]
pub use crate::types::address_component::AddressComponent;

#[cfg(any(
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
pub use crate::types::bounds::Bounds;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "geocoding"
))]
pub use crate::types::country::Country;

pub use crate::types::error::Error;

#[cfg(any(feature = "geocoding", feature = "places"))]
pub use crate::types::geometry::Geometry;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places",
    feature = "time_zone"
))]
pub use crate::types::language::Language;

#[cfg(any(
    feature = "address_validation",
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "elevation",
    feature = "geocoding",
    feature = "places",
    feature = "roads",
    feature = "time_zone"
))]
pub use crate::types::latlng::LatLng;

#[cfg(any(feature = "geocoding", feature = "places"))]
pub use crate::types::location_type::LocationType;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
pub use crate::types::place_type::PlaceType;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
pub use crate::types::region::Region;