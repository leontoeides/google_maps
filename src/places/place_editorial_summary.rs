//! Contains a summary of the place. A summary is comprised of a textual
//! overview, and also includes the language code for these if applicable.
//! Summary text must be presented as-is and can not be modified or altered.

use crate::types::Language;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Contains a summary of the place. A summary is comprised of a textual
/// overview, and also includes the language code for these if applicable.
/// Summary text must be presented as-is and can not be modified or altered.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PlaceEditorialSummary {
    /// The language of the previous fields. May not always be present.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,

    /// A medium-length textual summary of the place.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
} // struct PlaceEditorialSummary

// -----------------------------------------------------------------------------

impl std::str::FromStr for PlaceEditorialSummary {
    type Err = simd_json::Error;
    /// Parse a Google Maps Places API JSON response into a usable
    /// `PlaceEditorialSummary` struct.
    fn from_str(s: &str) -> Result<Self, simd_json::Error> {
        let mut bytes = s.to_string().into_bytes();
        simd_json::serde::from_slice(&mut bytes)
    } // fn from_str
} // impl FromStr
