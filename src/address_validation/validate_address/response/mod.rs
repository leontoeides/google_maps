//! Resources (enums, structs) for processing the _Address Validation API_
//! response from the Google Maps Platform. Look in here for more information
//! about the data returned from Google's server and how to parse it with your
//! program.

pub(super) mod address;
pub(super) mod address_component;
pub(super) mod address_metadata;
pub(super) mod component_name;
pub(super) mod confirmation_level;
pub(super) mod geocode;
pub(super) mod granularity;
pub(super) mod plus_code;
pub(super) mod usps_address;
pub(super) mod usps_data;
pub(super) mod validation_result;
pub(super) mod verdict;
pub(super) mod viewport;

// -----------------------------------------------------------------------------

pub use crate::address_validation::validate_address::response::{
    address::Address,
    address_component::AddressComponent,
    address_metadata::AddressMetadata,
    component_name::ComponentName,
    confirmation_level::ConfirmationLevel,
    geocode::Geocode,
    granularity::Granularity,
    plus_code::PlusCode,
    usps_address::UspsAddress,
    usps_data::UspsData,
    validation_result::ValidationResult,
    verdict::Verdict,
    viewport::Viewport,
};

// -----------------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use getset::{CopyGetters, Getters, MutGetters, Setters};

// -----------------------------------------------------------------------------
//
/// The response to an address validation request.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, CopyGetters, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// The result of the address validation.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub result: ValidationResult,

    /// The UUID that identifies this response. If the address needs to be
    /// re-validated, this UUID must accompany the new request.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub response_id: String,
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