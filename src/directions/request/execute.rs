use crate::directions::{error::Error, request::Request, response::Response};

impl<'a> Request<'a> {

    /// Executes the query you've built.
    ///
    /// ## Description:
    ///
    /// My adventures in Rust became messy so I had to make this method. It
    /// wraps the `.validate()?.build()?.get()?` chain needed at the end of the
    /// builder pattern.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn execute(&'a mut self) -> Result<Response, Error> {
        self.validate()?.build()?.get()
    } // fn

} // impl