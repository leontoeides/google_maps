use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Error {
    pub domain: String,
    pub reason: String,
    pub message: String,
} // struct