use crate::places::place_autocomplete::response::matched_substring::MatchedSubstring;
use serde::{Deserialize, Serialize};

/// ----------------------------------------------------------------------------
//
/// [PlaceAutocompleteStructuredFormat](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlaceAutocompleteStructuredFormat)

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
    pub main_text_matched_substrings: Vec<MatchedSubstring>,

    /// Contains the secondary text of a prediction, usually the location of the
    /// place.
    #[serde(alias = "secondary_text")]
    pub secondary_text: usize,

    /// Contains an array with `offset` value and `length`. These describe the
    /// location of the entered term in the prediction result text, so that the
    /// term can be highlighted if desired.
    /// 
    /// See [PlaceAutocompleteMatchedSubstring](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlaceAutocompleteMatchedSubstring)
    /// for more information.
    #[serde(alias = "secondary_text_matched_substrings")]
    pub secondary_text_matched_substrings: Option<Vec<MatchedSubstring>>,
} // struct StructuredFormat

/// ----------------------------------------------------------------------------

impl std::str::FromStr for StructuredFormat {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Places API _Place Autocomplete_ JSON
    /// `StructuredFormat` response into a usable `StructuredFormat` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    } // fn from_str
}  // impl FromStr