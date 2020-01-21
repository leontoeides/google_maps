use crate::geocoding::{
    error::Error,
    forward::ForwardRequest,
    response::Response,
}; // use

impl ForwardRequest {

    /// # Arguments:
    ///
    /// This method accepts no arguments.

    pub fn execute(&mut self) -> Result<Response, Error> {
        self.validate()?.build()?.get()
    } // fn

} // impl