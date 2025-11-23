#[cfg(feature = "places-new-ascii-art")]
mod ascii_art;

pub mod photo_image;
pub use crate::places_new::place_photos::image::response::photo_image::PhotoImage;

// -------------------------------------------------------------------------------------------------
//
/// Response from the Google Maps Places API (New) Place Photos service.
///
/// The response format depends on the `skip_http_redirect` parameter in the request. When false
/// (default), the response contains the actual image bytes. When true, the response contains a URI
/// to the image that you can fetch separately.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Response(pub(crate) Vec<u8>);

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::ops::Deref for Response {
    type Target = Vec<u8>;

    /// Dereferences to the underlying bytes.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Response {
    /// Mutably dereferences to the underlying bytes.
    ///
    /// Allows mutable access to the places list.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<Vec<u8>> for Response {
    fn as_ref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl AsMut<Vec<u8>> for Response {
    fn as_mut(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}

impl std::convert::From<Response> for Result<Response, crate::Error> {
    /// Converts a Google Maps API `Response` into a `Result<Response, Error>` by examining the
    /// `status` field inside of the response.
    ///
    /// If the status indicates a success, then an `Ok(response)` will be returned. If the status
    /// indicates an error, then an `Err(error)` will be returned.
    fn from(response: Response) -> Self {
        Ok(response)
    }
}