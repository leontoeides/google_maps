use crate::distance_matrix::{
    error::Error,
    request::Request,
    response::Response,
}; // use

impl Request {

    /// Executes the query you've built.
    ///
    /// ## Description:
    ///
    /// You are not required to call the `.validate()?.build()?.get()?` chain
    /// at the end of your builder pattern. You may still wish to do so if you
    /// would like to manually control the method calls. For example, if you
    /// would like to omit validation. However, this `.execute()` method wraps
    /// all of these steps into a single method call for convenience.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn execute(&mut self) -> Result<Response, Error> {
        self.validate()?.build()?.get()
    } // fn

} // impl