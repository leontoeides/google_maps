//! The `"matched_subtrings"` field contains a list of substrings that describe
//! the location of the entered term in the prediction result text, so that the
//! term can be highlighted if desired.

use serde::{Deserialize, Serialize};

/// ----------------------------------------------------------------------------
//
/// Contains a substring that describes the location of the entered term in the
/// prediction result text, so that the term can be highlighted if desired.
///
/// See also: [PlaceAutocompleteMatchedSubstring](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlaceAutocompleteMatchedSubstring)

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct MatchedSubstring {
    /// Length of the matched substring in the prediction result text.
    #[serde(alias = "length")]
    pub length: usize,
    /// Start location of the matched substring in the prediction result text.
    #[serde(alias = "offset")]
    pub offset: usize,
} // struct MatchedSubstring

/// ----------------------------------------------------------------------------

impl std::str::FromStr for MatchedSubstring {
    type Err = simd_json::Error;
    /// Parse a Google Maps Places API _Place Autocomplete_ JSON
    /// `MatchedSubstring` response into a usable `MatchedSubstring` struct.
    fn from_str(s: &str) -> Result<Self, simd_json::Error> {
        let mut bytes = s.to_string().into_bytes();
        simd_json::serde::from_slice(&mut bytes)
    } // fn from_str
} // impl FromStr
