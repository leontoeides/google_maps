//! This module contains custom serializers and deserializers for Serde.

#[cfg(feature = "distance_matrix")]
pub mod duration_to_seconds;
#[cfg(feature = "distance_matrix")]
pub mod seconds_to_duration;