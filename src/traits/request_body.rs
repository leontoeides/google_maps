// -----------------------------------------------------------------------------
//
/// Provides a generic way of converting a request `struct` (such as a
/// `crate::address_validation::validate_address::RequestBody`) into an HTTP 
/// request body. Presumably the request body is in JSON format, but that's up 
/// to the the implementation and the end-point.
pub trait RequestBody {
    /// Converts a request `struct` (presumably a request type such as
    /// `crate::address_validation::validate_address::Request`) into an HTTP 
    /// request body. Presumably the request body is in JSON format, but that's 
    /// up to the the implementation and the end-point.
    ///
    /// # Errors
    ///
    /// * This can fail if the request `struct` fails validation. For example,
    ///   parameters in the request conflict with one another, or the request
    ///   parameters are set in a way that's incompatible.
    ///
    ///   For example, Google Maps Address Validation API can only accept 280
    ///   characters of text in the `PostalAddress` struct. Having more 
    ///   characters than this will cause a validation failure.
    fn request_body(&self) -> Result<String, crate::Error>;
} // trait RequestBody