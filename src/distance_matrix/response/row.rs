use crate::distance_matrix::response::element::Element;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// When the Distance Matrix API returns results, it places them within a JSON
/// rows array. Even if no results are returned (such as when the origins and/or
/// destinations don't exist), it still returns an empty array.
///
/// Rows are ordered according to the values in the origin parameter of the
/// request. Each row corresponds to an origin, and each `element` within that
/// row corresponds to a pairing of the origin with a `destination` value.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Row {
    /// Each row array contains one or more `element` entries, which in turn
    /// contain the information about a single origin-destination pairing.
    #[serde(default)]
    pub elements: Vec<Element>,
} // struct
