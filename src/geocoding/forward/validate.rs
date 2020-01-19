use crate::geocoding::{
    error::Error,
    request::Request,
};

impl Request {

    /// Ensures the built query is valid. This function checks the combination
    /// of parameters to ensure that they make sense together and that Google
    /// Maps Geocoding API will accept them - i.e. it will not allow both a
    /// Positional Request and a Sampled Path Request in the same query. This
    /// function does not check parameter values for validity - i.e. it will not
    /// ensure Polylines or Latitudes/Longitudes are valid and well-formed.

    pub fn validate(&mut self) -> Result<&mut Request, Error> {
        // If a positional request has been set...
        if self.address == None && self.components == None {
            Err(Error::AddressOrComponentsRequired),
        } // if
        // Indicated that the request passed validation.
        self.validated = true;
        // Return modified Request struct to caller.
        Ok(self)
    } // fn

} // impl