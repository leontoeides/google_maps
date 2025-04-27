//! Address Validation API error types and error messages.

use crate::address_validation::validate_address::request::postal_address::MAX_CHARS;

// -----------------------------------------------------------------------------
//
/// An error produced by a Google Maps Address Validation API request.
#[derive(Clone, Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    // -------------------------------------------------------------------------
    // Client-side errors:
    // -------------------------------------------------------------------------

    // Validation errors:

    /// Too many characters in postal address.
    ///
    /// The total length of the fields in this input must not exceed 280
    /// characters.
    #[error(
        "postal address contains too many chars:
        counted {0} chars, max {MAX_CHARS} chars."
    )]
    #[diagnostic(
        code(google_maps::address_validation::validate::too_many_chars),
        url("https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#request-body"),
        help("shorten the postal address")
    )]
    PostalAddressTooManyChars(usize),
} // enum Error

// -----------------------------------------------------------------------------

use crate::ClassifiedError;

impl crate::traits::ClassifiableError<'_, Self> for Error {
    /// Classifies an API error as a `Transient` error or `Permanent` error.
    ///
    /// This classification will, in turn, be used to decide whether the HTTP
    /// request should be retried or not.
    fn classify(&self) -> ClassifiedError<'_, Self> {
        ClassifiedError::Permanent(self)
    } // fn
} // impl