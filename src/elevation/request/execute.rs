use crate::elevation::{
    error::Error,
    request::Request,
    response::Response,
}; // use

impl Request {

    /// # Arguments:
    ///
    /// This method accepts no arguments.

    pub fn execute(&mut self) -> Result<Response, Error> {
        self.validate()?.build()?.get()
    } // fn

} // impl