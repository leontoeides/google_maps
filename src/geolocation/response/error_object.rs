use crate::geolocation::response::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ErrorObject {
    pub errors: Vec<Error>,
    pub code: u16,
    pub message: String,
} // struct