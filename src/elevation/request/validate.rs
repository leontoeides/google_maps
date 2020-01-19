use crate::elevation::{
    error::Error,
    request::Request,
}; // use

impl Request {

    /// Ensures the built query is valid. This function checks the combination
    /// of parameters to ensure that they make sense together and that Google
    /// Maps Directions API will accept them - i.e. it will not allow both a
    /// Positional Request and a Sampled Path Request in the same query. This
    /// function does not check parameter values for validity - i.e. it will not
    /// ensure Polylines or Latitudes/Longitudes are valid and well-formed.
    ///
    /// # Arguments:
    ///
    /// This method accepts no arguments.

    pub fn validate(&mut self) -> Result<&mut Request, Error> {
        // If a positional request has been set...
        if let Some(_locations) = &self.locations {
            // ...a sampled path request cannot be set.
            if let Some(_path) = &self.path { return Err(Error::EitherPositionalOrSampledPath) }
        } // if
        // Indicated that the request passed validation.
        self.validated = true;
        // Return modified Request struct to caller.
        Ok(self)
    } // fn

} // impl