use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error {
    pub domain: String,
    pub reason: String,
    pub message: String,
} // struct