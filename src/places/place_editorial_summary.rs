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
    pub language: Option<Language>,

    /// A medium-length textual summary of the place.
    pub overview: Option<String>,
} // struct PlaceEditorialSummary

// -----------------------------------------------------------------------------

impl std::str::FromStr for PlaceEditorialSummary {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Places API JSON response into a usable
    /// `PlaceEditorialSummary` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    } // fn from_str
} // impl FromStr
