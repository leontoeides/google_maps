use crate::geocoding::{
    error::Error, forward::ForwardRequest, response::Response
}; // use crate

impl<'a> ForwardRequest<'a> {

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
        self.validate()?.build()?.get().await
    } // fn

} // impl