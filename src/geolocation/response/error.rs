use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Error {
    pub domain: String,
    pub reason: String,
    pub message: String,
} // struct