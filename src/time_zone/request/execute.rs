use crate::error::Error as GoogleMapsError;
use crate::time_zone::{
    request::Request as TimeZoneRequest, response::Response as TimeZoneResponse,
}; // crate::time_zone

// =============================================================================

impl<'a> TimeZoneRequest<'a> {
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

    pub async fn execute(&'a mut self) -> Result<TimeZoneResponse, GoogleMapsError> {
        self.build().get().await
    } // fn
} // impl
