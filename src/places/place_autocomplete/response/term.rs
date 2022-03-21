use serde::{Deserialize, Serialize};

/// ----------------------------------------------------------------------------
//
/// [PlaceAutocompleteTerm](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlaceAutocompleteTerm)

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

/// ----------------------------------------------------------------------------

impl std::str::FromStr for Term {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Places API _Place Autocomplete_ JSON `Term` response
    /// into a usable `Term` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    } // fn from_str
}  // impl FromStr