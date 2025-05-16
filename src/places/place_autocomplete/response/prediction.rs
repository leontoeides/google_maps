//! The `"predictions"` are the results from a search.

use crate::places::place_autocomplete::response::{
    matched_substring::MatchedSubstring, structured_format::StructuredFormat, term::Term,
};
use crate::types::PlaceType;
use serde::{Deserialize, Serialize};

/// ----------------------------------------------------------------------------
//
/// When the Places service returns results from a search, it places them within
/// a `predictions` array. Even if the service returns no results (such as if
/// the `location` is remote) it still returns an empty `predictions`
/// array.
///
/// See also: [PlaceAutocompletePrediction](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlaceAutocompletePrediction)
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Prediction {
    /// Contains the human-readable name for the returned result. For
    /// `establishment` results, this is usually the business name. This content
    /// is meant to be read as-is. Do not programmatically parse the formatted
    /// address.
    #[serde(alias = "description")]
    pub description: String,

    /// A list of substrings that describe the location of the entered term in
    /// the prediction result text, so that the term can be highlighted if
    /// desired.
    ///
    /// See [PlaceAutocompleteMatchedSubstring](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlaceAutocompleteMatchedSubstring)
    /// for more information.
    #[serde(alias = "matched_substrings")]
    #[serde(default)]
    pub matched_substrings: Vec<MatchedSubstring>,

    /// Provides pre-formatted text that can be shown in your autocomplete
    /// results. This content is meant to be read as-is. Do not programmatically
    /// parse the formatted address.
    ///
    /// See [PlaceAutocompleteStructuredFormat](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlaceAutocompleteStructuredFormat)
    /// for more information.
    #[serde(alias = "structured_formatting")]
    pub structured_formatting: StructuredFormat,

    /// The straight-line distance in meters from the origin.
    /// This field is only returned for requests made with an `origin`.
    #[serde(default)]
    pub distance_meters: Option<u64>,

    /// Contains an array of terms identifying each section of the returned
    /// description (a section of the description is generally terminated with a
    /// comma). Each entry in the array has a `value` field, containing the text
    /// of the term, and an `offset` field, defining the start position of this
    /// term in the description, measured in Unicode characters.
    ///
    /// See [PlaceAutocompleteTerm](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlaceAutocompleteTerm)
    /// for more information.
    #[serde(alias = "terms")]
    #[serde(default)]
    pub terms: Vec<Term>,

    /// A textual identifier that uniquely identifies a place. To retrieve
    /// information about the place, pass this identifier in the placeId field
    /// of a Places API request. For more information about place IDs, see the
    /// [Place IDs](https://developers.google.com/maps/documentation/places/web-service/place-id)
    /// overview.
    #[serde(alias = "place_id")]
    #[serde(default)]
    pub place_id: Option<String>,

    /// Contains an array of types that apply to this place. For example:
    /// `[ "political", "locality" ] or [ "establishment", "geocode",
    /// "beauty_salon" ]`. The array can contain multiple values. Learn more
    /// about [Place types](https://developers.google.com/maps/documentation/places/web-service/supported_types).
    #[serde(alias = "types")]
    #[serde(default)]
    pub types: Vec<PlaceType>,
} // struct Prediction

// -----------------------------------------------------------------------------

impl std::str::FromStr for Prediction {
    type Err = serde_json::Error;
    /// Parse a Google Maps Places API _Place Autocomplete_ JSON
    /// `Prediction` response into a usable `Prediction` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        let bytes = s.to_string().into_bytes();
        serde_json::from_slice(&bytes)
    } // fn from_str
} // impl FromStr
