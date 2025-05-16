//! Transit directions return additional information

use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Provides information about the operator of the line.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TransitAgency {
    /// Contains the name of the transit agency.
    pub name: String,

    /// Contains the phone number of the transit agency.
    #[serde(default)]
    pub phone: Option<String>,

    /// Contains the URL for the transit agency.
    pub url: String,
} // struct
