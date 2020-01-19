use serde::{Serialize, Deserialize};

/// Indicates the status code resulting from the geocoding operation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GeocoderStatus {
    /// Indicates that no errors occurred; the address was successfully parsed
    /// and at least one geocode was returned.
    #[serde(alias = "OK")]
    Ok,
    /// Indicates that the geocode was successful but returned no results. This
    /// may occur if the geocoder was passed a non-existent `address`.
    #[serde(alias = "ZERO_RESULTS")]
    ZeroResults,
} // struct