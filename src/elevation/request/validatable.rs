use crate::elevation::Error;

// -----------------------------------------------------------------------------

impl crate::traits::Validatable for crate::elevation::Request<'_> {
    /// Ensures that the request is valid.
    ///
    /// This function checks the combination of parameters to ensure that they
    /// make sense together and that Google Maps API will accept them.
    ///
    /// For example, Google Maps will not allow both a Positional Request and a
    /// Sampled Path Request in the same query.
    ///
    /// This function does not check parameter values for validity - i.e. it
    /// will not ensure Polylines or Latitudes/Longitudes are valid and
    /// well-formed.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    ///
    /// # Errors
    ///
    /// * This will fail if the request `struct` fails validation. For example,
    ///   parameters in the request conflict with one another, or the request
    ///   parameters are set in a way that's incompatible.
    ///
    ///   For example, Google Maps Directions API cannot calculate alternative
    ///   routes if waypoints have been set. This will cause a validation
    ///   failure.
    fn validate(&self) -> Result<(), crate::Error> {
        // If a "for positional request" has been set...
        if let Some(_locations) = &self.locations {
            // ...a "for sampled path request" cannot be set.
            if let Some(_path) = &self.path {
                return Err(Error::EitherPositionalOrSampledPath)?;
            } // if
        } // if

        // If we made it to the bottom, all tests have passed. Return `Ok` to
        // caller:
        Ok(())
    } // fn
} // impl
