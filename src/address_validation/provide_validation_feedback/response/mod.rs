//! Resources (enums, structs) for processing the Address Validation API
//! _Provide Validation Feedback_ response from the Google Maps Platform. Look
//! in here for more information about the data returned from Google's server
//! and how to parse it with your program.

use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// The response to an provide validation feedback request.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, CopyGetters, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct Response {
} // struct Response

// -----------------------------------------------------------------------------

impl std::convert::From<Response> for Result<Response, crate::Error> {
    /// Converts a Google Maps API `Response` into a `Result<Response, Error>`
    /// by examining the `status` field inside of the response.
    ///
    /// If the status indicates a success, then an `Ok(response)` will be
    /// returned. If the status indicates an error, then an `Err(error)` will be
    /// returned.
    fn from(response: Response) -> Self {
        Ok(response)
    } // fn
} // impl