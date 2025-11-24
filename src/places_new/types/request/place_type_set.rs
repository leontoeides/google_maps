use crate::places_new::PlaceType;
use std::collections::{BTreeSet, HashSet};

// -------------------------------------------------------------------------------------------------
//
/// Filter for place types in search queries.
///
/// Wraps a collection of place types for use in nearby search, text search, and other queries.
/// Accepts various input formats through conversion traits for ergonomic usage.
#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PlaceTypeSet(BTreeSet<PlaceType>);

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl PlaceTypeSet {
    /// Creates a new place type filter from a vector.
    #[must_use]
    pub const fn new(b_tree_set: BTreeSet<PlaceType>) -> Self {
        Self(b_tree_set)
    }

    /// Returns the underlying vector.
    #[must_use]
    pub fn into_inner(self) -> BTreeSet<PlaceType> {
        self.0
    }

    /// Returns the number of place types in the filter.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the filter is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl From<Vec<PlaceType>> for PlaceTypeSet {
    fn from(vec: Vec<PlaceType>) -> Self {
        Self(vec.into_iter().collect())
    }
}

impl From<Vec<&PlaceType>> for PlaceTypeSet {
    fn from(vec: Vec<&PlaceType>) -> Self {
        Self(vec.into_iter().copied().collect())
    }
}

impl From<&Vec<PlaceType>> for PlaceTypeSet {
    fn from(vec: &Vec<PlaceType>) -> Self {
        Self(vec.iter().copied().collect())
    }
}

impl From<&Vec<&PlaceType>> for PlaceTypeSet {
    fn from(vec: &Vec<&PlaceType>) -> Self {
        Self(vec.iter().copied().copied().collect())
    }
}

impl From<&[PlaceType]> for PlaceTypeSet {
    fn from(slice: &[PlaceType]) -> Self {
        Self(slice.iter().copied().collect())
    }
}

impl From<&[&PlaceType]> for PlaceTypeSet {
    fn from(slice: &[&PlaceType]) -> Self {
        Self(slice.iter().copied().copied().collect())
    }
}

impl<const N: usize> From<[PlaceType; N]> for PlaceTypeSet {
    fn from(array: [PlaceType; N]) -> Self {
        Self(array.into_iter().collect())
    }
}

impl<const N: usize> From<&[PlaceType; N]> for PlaceTypeSet {
    fn from(array: &[PlaceType; N]) -> Self {
        Self(array.iter().copied().collect())
    }
}

impl<const N: usize> From<&[&PlaceType; N]> for PlaceTypeSet {
    fn from(array: &[&PlaceType; N]) -> Self {
        Self(array.iter().copied().copied().collect())
    }
}

impl From<BTreeSet<PlaceType>> for PlaceTypeSet {
    fn from(b_tree_set: BTreeSet<PlaceType>) -> Self {
        Self(b_tree_set.into_iter().collect())
    }
}

impl From<BTreeSet<&PlaceType>> for PlaceTypeSet {
    fn from(b_tree_set: BTreeSet<&PlaceType>) -> Self {
        Self(b_tree_set.into_iter().copied().collect())
    }
}

impl From<&BTreeSet<&PlaceType>> for PlaceTypeSet {
    fn from(b_tree_set: &BTreeSet<&PlaceType>) -> Self {
        Self(b_tree_set.iter().copied().copied().collect())
    }
}

impl From<HashSet<PlaceType>> for PlaceTypeSet {
    fn from(hash_set: HashSet<PlaceType>) -> Self {
        Self(hash_set.into_iter().collect())
    }
}

impl From<HashSet<&PlaceType>> for PlaceTypeSet {
    fn from(hash_set: HashSet<&PlaceType>) -> Self {
        Self(hash_set.into_iter().copied().collect())
    }
}

impl From<&HashSet<&PlaceType>> for PlaceTypeSet {
    fn from(hash_set: &HashSet<&PlaceType>) -> Self {
        Self(hash_set.iter().copied().copied().collect())
    }
}

impl From<PlaceType> for PlaceTypeSet {
    fn from(place_type: PlaceType) -> Self {
        let mut b_tree_set = BTreeSet::<PlaceType>::default();
        b_tree_set.insert(place_type);
        Self(b_tree_set)
    }
}

impl<I> FromIterator<I> for PlaceTypeSet
where
    I: Into<PlaceType>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

impl std::ops::Deref for PlaceTypeSet {
    type Target = BTreeSet<PlaceType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<BTreeSet<PlaceType>> for PlaceTypeSet {
    fn as_ref(&self) -> &BTreeSet<PlaceType> {
        &self.0
    }
}

impl IntoIterator for PlaceTypeSet {
    type Item = PlaceType;
    type IntoIter = std::collections::btree_set::IntoIter<PlaceType>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a PlaceTypeSet {
    type Item = &'a PlaceType;
    type IntoIter = std::collections::btree_set::Iter<'a, PlaceType>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl std::fmt::Display for PlaceTypeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "(empty)");
        }
        
        let types: Vec<String> = self.0.iter().map(|t| format!("{t:?}")).collect();
        write!(f, "{}", types.join(", "))
    }
}