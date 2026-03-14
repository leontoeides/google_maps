use crate::address_validation::Error;
use crate::address_validation::validate_address::request::postal_address::MAX_CHARS;

// -----------------------------------------------------------------------------

impl crate::traits::Validatable for crate::address_validation::validate_address::Request<'_> {
    /// Ensures that the request is valid.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    ///
    /// # Errors
    ///
    /// * This can fail if the request `struct` fails validation. For example,
    ///   parameters in the request conflict with one another, or the request
    ///   parameters are set in a way that's incompatible.
    ///
    ///   For example, Google Maps Address Validation API will refuse a
    ///   `PostalAddress` greater than 280 characters. This will cause a
    ///   validation failure.
    fn validate(&self) -> std::result::Result<(), crate::Error> {
        let len: usize = self.address.len();

        // Check if the `PostalAddress` exceeds the maximum number of
        // characters.
        if self.address.len() > MAX_CHARS {
            return Err(Error::PostalAddressTooManyChars(len))?;
        } // if

        // If we made it to the bottom, all tests have passed. Return `Ok` to
        // caller:
        Ok(())
    } // fn
} // impl