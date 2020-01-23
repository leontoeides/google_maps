use crate::directions::{
    error::Error,
    request::Request,
    response::Response,
}; // use

impl Request {

    /// The earliest versions of this crate required you to call
    /// `.validate()?.build()?.get()?` at the end of your builder pattern. You
    /// may still wish to do this if you would like to omit validation. However,
    /// this `.execute()` method wraps all of these steps into a single method
    /// call for convenience.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn execute(&mut self) -> Result<Response, Error> {
        self.validate()?.build()?.get()
    } // fn

} // impl