use serde::{Deserialize, Serialize};

// The mobile radio type.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum RadioType {
    /// Code-Division Multiple Access
    #[serde(alias = "cdma")]
    Cdma,
    /// Global System for Mobile communications
    #[serde(alias = "gsm")]
    Gsm,
    /// Long-Term Evolution
    #[serde(alias = "lte")]
    Lte,
    /// Wideband Code-Division Multiple Access
    #[serde(alias = "wcdma")]
    Wcdma,
} // enum