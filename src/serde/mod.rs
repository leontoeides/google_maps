//! This module contains custom serializers and deserializers for Serde.

#[cfg(any(feature = "directions", feature = "distance_matrix"))]
pub mod duration_to_seconds;
#[cfg(any(feature = "directions", feature = "distance_matrix"))]
pub mod seconds_to_duration;
