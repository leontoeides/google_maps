// -------------------------------------------------------------------------------------------------
//
// Structures

pub mod area;
pub use crate::places_new::types::address_descriptor::area::Area;

pub mod landmark;
pub use crate::places_new::types::address_descriptor::landmark::Landmark;

// -------------------------------------------------------------------------------------------------
//
// Enumerations

pub mod containment;
pub use crate::places_new::types::address_descriptor::containment::Containment;

pub mod spatial_relationship;
pub use crate::places_new::types::address_descriptor::spatial_relationship::SpatialRelationship;

// -------------------------------------------------------------------------------------------------
//
/// A relational description of a location using landmarks and containing areas.
///
/// Address descriptors provide rich contextual information about a location by describing its
/// relationship to nearby landmarks and the areas that contain or are adjacent to it. This helps
/// users understand where a place is situated using familiar reference points and geographic
/// boundaries.
///
/// The landmarks and areas are ranked by recognizability and relevance, with the most useful
/// contextual information appearing first in their respective lists.
#[derive(
    //std
    Clone,
    Debug,
    Default,
    // getset
    getset::Getters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[serde(rename_all = "camelCase")]
pub struct AddressDescriptor {
    /// A ranked list of nearby landmarks that help describe the location.
    ///
    /// Contains prominent places that can be used as reference points for describing the location.
    /// Landmarks are ordered by recognizability and proximity, with the most useful references
    /// appearing first. Each landmark includes spatial relationship information.
    #[serde(default)]
    #[getset(get = "pub")]
    pub landmarks: Vec<Landmark>,

    /// A ranked list of containing or adjacent areas that provide geographic context.
    ///
    /// Contains precise sublocalities, neighborhoods, and geographic regions that help establish
    /// where the location sits within the broader area structure. Areas are ordered by precision
    /// and recognizability, with the most specific and well-known areas listed first.
    #[serde(default)]
    #[getset(get = "pub")]
    pub areas: Vec<Area>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl AddressDescriptor {
    /// Creates a new `AddressDescriptor` with the specified landmarks and areas.
    ///
    /// Used to construct address descriptors with curated lists of contextual references.
    /// Both landmarks and areas should be pre-sorted by relevance and recognizability.
    #[must_use]
    pub const fn new(landmarks: Vec<Landmark>, areas: Vec<Area>) -> Self {
        Self { landmarks, areas }
    }

    /// Creates an empty `AddressDescriptor` with no contextual information.
    ///
    /// Used when initializing address descriptors that will be populated later or when
    /// no landmark or area context is available for a location.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            landmarks: Vec::new(),
            areas: Vec::new(),
        }
    }

    /// Returns whether this descriptor has any contextual information.
    ///
    /// Used to determine if the descriptor provides any useful location context through
    /// landmarks or areas, helping decide whether to display contextual information.
    #[must_use]
    pub fn has_context(&self) -> bool {
        !self.landmarks.is_empty() || !self.areas.is_empty()
    }

    /// Returns the number of landmarks available for reference.
    ///
    /// Used to understand the richness of landmark-based context available for describing
    /// this location to users.
    #[must_use]
    pub fn landmark_count(&self) -> usize {
        self.landmarks.len()
    }

    /// Returns the number of areas that provide geographic context.
    ///
    /// Used to understand how much area-based geographic context is available for
    /// situating this location within broader regions.
    #[must_use]
    pub fn area_count(&self) -> usize {
        self.areas.len()
    }
}