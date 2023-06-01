use crate::error::Error as GoogleMapsError;
use crate::distance_matrix::{
    request::Request as DistanceMatrixRequest,
    response::Response as DistanceMatrixResponse,
}; // crate::distance_matrix
use miette::Result;

// =============================================================================

impl<'a> DistanceMatrixRequest<'a> {

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
    ) -> Result<DistanceMatrixResponse, GoogleMapsError> {

        self.validate()?.build()?.get().await

    } // fn

} // impl