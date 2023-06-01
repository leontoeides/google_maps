//! Common types used across several Google Maps API interfaces.

pub(super) mod bounds;
pub(super) mod country;
pub(super) mod error;
pub(super) mod language;
pub(super) mod latlng;
pub(super) mod place_type;
pub(super) mod region;

// -----------------------------------------------------------------------------

pub use crate::types::bounds::Bounds;
pub use crate::types::country::Country;
pub use crate::types::error::Error;
pub use crate::types::language::Language;
pub use crate::types::latlng::LatLng;
pub use crate::types::place_type::PlaceType;
pub use crate::types::region::Region;