use crate::geocoding::{
    error::Error,
    response::Response,
    reverse::ReverseRequest,
}; // use

impl ReverseRequest {

    /// # Arguments:
    ///
    /// This method accepts no arguments.

    pub fn execute(&mut self) -> Result<Response, Error> {
        self.build().get()
    } // fn

} // impl