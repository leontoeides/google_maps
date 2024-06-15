use crate::geolocation::response::error::Error;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct ErrorObject {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Error>,
    pub code: u16,
    pub message: String,
} // struct