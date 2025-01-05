//! In the case of an error, a standard format error response body will be
//! returned and the HTTP status code will be set to an error status.

// -----------------------------------------------------------------------------

use crate::roads::status::Status;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// In the case of an error, a standard format error response body will be
/// returned and the HTTP status code will be set to an error status.
///
/// The response contains an object with a single error object with the
/// following keys:
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct ErrorResponse {
    /// This is the same as the
    /// [HTTP status](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status)
    /// of the response.
    pub code: u16,

    /// A short description of the error.
    pub message: String,

    /// A status code indicating the nature of the error.
    pub status: Status,
} // struct
