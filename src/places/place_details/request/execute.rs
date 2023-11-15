use crate::error::Error as GoogleMapsError;
use crate::places::place_details::{
    request::Request as PlaceDetailsRequest, response::Response as PlaceDetailsResponse,
}; // crate::places::place_details

// =============================================================================

impl<'a> PlaceDetailsRequest<'a> {
    // -------------------------------------------------------------------------
    //
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

    pub async fn execute(&'a mut self) -> Result<PlaceDetailsResponse, GoogleMapsError> {
        self.build().get().await
    } // fn
} // impl
