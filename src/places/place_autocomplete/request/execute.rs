use crate::error::Error as GoogleMapsError;
use crate::places::place_autocomplete::{
    request::Request as PlaceAutocompleteRequest,
    response::Response as PlaceAutocompleteResponse,
}; // crate::places::place_autocomplete

// =============================================================================

impl<'a> PlaceAutocompleteRequest<'a> {

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
    ) -> Result<PlaceAutocompleteResponse, GoogleMapsError> {

        self.build().get().await

    } // fn

} // impl