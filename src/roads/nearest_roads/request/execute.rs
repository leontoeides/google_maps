use crate::error::Error as GoogleMapsError;
use crate::roads::nearest_roads::{
    request::Request as NearestRoadsRequest, response::Response as NearestRoadsResponse,
}; // crate::roads::nearest_roads

// =============================================================================

impl<'a> NearestRoadsRequest<'a> {
    // -------------------------------------------------------------------------
    //
    /// Executes the query you've built.
    ///
    /// ## Description
    ///
    /// My adventures in Rust became messy so I had to make this method. It
    /// wraps the `.validate()?.build()?.get()?` chain needed at the end of the
    /// builder pattern.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.

    pub async fn execute(&'a mut self) -> Result<NearestRoadsResponse, GoogleMapsError> {
        self.build().get().await
    } // fn
} // impl
