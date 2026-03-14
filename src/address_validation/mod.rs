//! The [Address Validation](https://developers.google.com/maps/documentation/address-validation)
//! API validates an address and its components, standardize the address for
//! mailing, and determine the best known geocode for it.

// -----------------------------------------------------------------------------

pub mod error;

pub use crate::address_validation::error::Error;

// -----------------------------------------------------------------------------

pub mod provide_validation_feedback;

pub use crate::address_validation::provide_validation_feedback::{
    Request as ProvideValidationFeedbackRequest,
    RequestQuery as ProvideValidationFeedbackRequestQuery,
    Response as ProvideValidationFeedbackResponse,
    ValidationConclusion,
};

// -----------------------------------------------------------------------------

pub mod validate_address;

pub use crate::address_validation::validate_address::{
    Address,
    AddressComponent,
    AddressMetadata,
    ComponentName,
    ConfirmationLevel,
    Geocode,
    Granularity,
    LanguageOptions,
    PlusCode,
    PostalAddress,
    Request as ValidateAddressRequest,
    RequestQuery as ValidateAddressRequestQuery,
    Response as ValidateAddressResponse,
    UspsAddress,
    UspsData,
    ValidationResult,
    Verdict,
    Viewport,
};