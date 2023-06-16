use crate::error::Error as GoogleMapsError;
use crate::roads::snap_to_roads::{
    request::Request as SnapToRoadsRequest,
    response::Response as SnapToRoadsResponse,
}; // crate::roads::snap_to_roads

// =============================================================================

impl<'a> SnapToRoadsRequest<'a> {

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

    pub async fn execute(
        &'a mut self
    ) -> Result<SnapToRoadsResponse, GoogleMapsError> {

        self.build().get().await

    } // fn

} // impl