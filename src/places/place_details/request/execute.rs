use crate::places::error::Error;
use crate::places::place_details::{
    request::Request,
    response::Response
}; // crate::places::place_details

// -----------------------------------------------------------------------------

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

    pub async fn execute(&'a mut self) -> Result<Response, Error> {
        self.build().get().await
    } // fn

} // impl