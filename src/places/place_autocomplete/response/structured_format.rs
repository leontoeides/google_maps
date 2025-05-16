//! The `"structured_formatting"` field provides pre-formatted text that can be
//! shown in your autocomplete results. This content is meant to be read as-is.
//! Do not programmatically parse the formatted address.

use crate::places::place_autocomplete::response::matched_substring::MatchedSubstring;
use serde::{Deserialize, Serialize};

/// ----------------------------------------------------------------------------
//
/// Provides pre-formatted text that can be shown in your autocomplete results.
/// This content is meant to be read as-is. Do not programmatically parse the
/// formatted address.
///
/// See also: [PlaceAutocompleteStructuredFormat](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlaceAutocompleteStructuredFormat)
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct StructuredFormat {
    /// Contains the main text of a prediction, usually the name of the place.
    #[serde(alias = "main_text")]
    pub main_text: String,

    /// Contains an array with `offset` value and `length`. These describe the
    /// location of the entered term in the prediction result text, so that the
    /// term can be highlighted if desired.
    ///
    /// See [PlaceAutocompleteMatchedSubstring](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlaceAutocompleteMatchedSubstring)
    /// for more information.
    #[serde(alias = "main_text_matched_substrings")]
    #[serde(default)]
    pub main_text_matched_substrings: Vec<MatchedSubstring>,

    /// Contains the secondary text of a prediction, usually the location of the
    /// place.
    #[serde(alias = "secondary_text")]
    pub secondary_text: String,

    /// Contains an array with `offset` value and `length`. These describe the
    /// location of the entered term in the prediction result text, so that the
    /// term can be highlighted if desired.
    ///
    /// See [PlaceAutocompleteMatchedSubstring](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlaceAutocompleteMatchedSubstring)
    /// for more information.
    #[serde(alias = "secondary_text_matched_substrings")]
    #[serde(default)]
    pub secondary_text_matched_substrings: Vec<MatchedSubstring>,
} // struct StructuredFormat

// -----------------------------------------------------------------------------

impl std::str::FromStr for StructuredFormat {
    type Err = serde_json::Error;
    /// Parse a Google Maps Places API _Place Autocomplete_ JSON
    /// `StructuredFormat` response into a usable `StructuredFormat` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        let bytes = s.to_string().into_bytes();
        serde_json::from_slice(&bytes)
    } // fn from_str
} // impl FromStr
