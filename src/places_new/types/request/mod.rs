#[cfg(any(feature = "places-new-autocomplete", feature = "places-new-nearby-search", feature = "places-new-text-search"))]
pub mod circle;
#[cfg(any(feature = "places-new-autocomplete", feature = "places-new-nearby-search", feature = "places-new-text-search"))]
pub use crate::places_new::types::request::circle::Circle;

#[cfg(any(feature = "places-new-autocomplete", feature = "places-new-nearby-search"))]
pub mod place_type_set;
#[cfg(any(feature = "places-new-autocomplete", feature = "places-new-nearby-search"))]
pub use crate::places_new::types::request::place_type_set::PlaceTypeSet;

#[cfg(any(feature = "places-new-autocomplete", feature = "places-new-text-search"))]
pub mod location_bias;
#[cfg(any(feature = "places-new-autocomplete", feature = "places-new-text-search"))]
pub use crate::places_new::types::request::location_bias::LocationBias;

#[cfg(any(feature = "places-new-autocomplete", feature = "places-new-nearby-search", feature = "places-new-text-search"))]
pub mod location_restriction;
#[cfg(any(feature = "places-new-autocomplete", feature = "places-new-nearby-search", feature = "places-new-text-search"))]
pub use crate::places_new::types::request::location_restriction::LocationRestriction;

#[cfg(any(feature = "places-new-nearby-search", feature = "places-new-text-search"))]
pub mod rank_preference;
#[cfg(any(feature = "places-new-nearby-search", feature = "places-new-text-search"))]
pub use crate::places_new::types::request::rank_preference::RankPreference;