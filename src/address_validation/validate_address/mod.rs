//! Validates an address.

pub mod request;
pub mod response;

// -----------------------------------------------------------------------------

pub use crate::address_validation::validate_address::{
    request::{
        LanguageOptions,
        PostalAddress,
        Request,
        RequestQuery,
    },
    response::{
        Address,
        AddressComponent,
        AddressMetadata,
        ComponentName,
        ConfirmationLevel,
        Geocode,
        Granularity,
        PlusCode,
        Response,
        UspsAddress,
        UspsData,
        ValidationResult,
        Verdict,
        Viewport,
    },
};