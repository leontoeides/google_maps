use crate::error::Error as GoogleMapsError;
use crate::places::place_search::text_search::{
    request::Request as TextSearchRequest, response::Response as TextSearchResponse,
}; // crate::places::place_search::text_search

// =============================================================================

impl<'a> TextSearchRequest<'a> {
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

    pub async fn execute(&'a mut self) -> Result<TextSearchResponse, GoogleMapsError> {
        self.build().get().await
    } // fn
} // impl
