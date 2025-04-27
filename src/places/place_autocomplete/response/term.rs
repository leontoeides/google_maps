//! The `"terms"` field contains an array of terms identifying each section of
//! the returned description (a section of the description is generally
//! terminated with a comma). Each entry in the array has a value field,
//! containing the text of the term, and an offset field, defining the start
//! position of this term in the description, measured in Unicode characters.

use serde::{Deserialize, Serialize};

/// ----------------------------------------------------------------------------
//
/// Contains term identifying the returned description (a section of the
/// description is generally terminated with a comma). Each entry in the array
/// has a value field, containing the text of the term, and an offset field,
/// defining the start position of this term in the description, measured in
/// Unicode characters.
///
/// See also: [PlaceAutocompleteTerm](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlaceAutocompleteTerm)

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Term {
    /// Defines the start position of this term in the description, measured in
    /// Unicode characters.
    #[serde(alias = "offset")]
    pub offset: usize,
    /// The text of the term.
    #[serde(alias = "value")]
    pub value: String,
} // struct