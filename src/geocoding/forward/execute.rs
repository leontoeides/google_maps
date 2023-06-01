use crate::error::Error as GoogleMapsError;
use crate::geocoding::{
    forward::ForwardRequest as GeocodingForwardRequest,
    response::Response as GeocodingResponse,
}; // crate::geocoding
use miette::Result;

// =============================================================================

impl<'a> GeocodingForwardRequest<'a> {

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
    ) -> Result<GeocodingResponse, GoogleMapsError> {

        self.validate()?.build()?.get().await

    } // fn

} // impl