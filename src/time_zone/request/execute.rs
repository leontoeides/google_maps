use crate::time_zone::{
    error::Error,
    request::Request,
    response::Response,
}; // use

impl Request {

    /// # Arguments:
    ///
    /// This method accepts no arguments.

    pub fn execute(&mut self) -> Result<Response, Error> {
        self.build().get()
    } // fn

} // impl