impl crate::traits::Validatable for crate::geocoding::reverse::ReverseRequest<'_> {
    /// Ensures that the request is valid.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    ///
    /// # Errors
    ///
    /// * This trait implementation is required for operation but there are
    ///   currently no validations performed for this request type. This method
    ///   will always return `Ok`.
    fn validate(&self) -> Result<(), crate::Error> { Ok(()) }
} // impl