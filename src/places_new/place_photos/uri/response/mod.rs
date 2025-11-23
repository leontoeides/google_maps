#![allow(clippy::ref_option, reason = "for the getset crate")]

// Exports

pub mod photo_uri;
pub use crate::places_new::place_photos::uri::response::photo_uri::PhotoUri;

// Imports

use url::Url;

// -------------------------------------------------------------------------------------------------
//
/// Response from the Google Maps Places API (New) Place Photos service.
///
/// The response format depends on the `skip_http_redirect` parameter in the request. When false
/// (default), the response contains the actual image bytes. When true, the response contains a URI
/// to the image that you can fetch separately.
#[derive(
    // std
    Clone,
    Debug,
    // serde
    serde::Deserialize,
    serde::Serialize,
    // getset
    getset::Getters,
    getset::MutGetters,
    getset::Setters,
)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Identifier for this place photo.
    ///
    /// A reference representing this place photo which may be used to look up this place photo
    /// again, also called the API "resource" name. This name can be used with the Photo Media API
    /// to fetch the actual photo content at various sizes.
    #[getset(get = "pub")]
    pub(crate) name: String,

    /// A short-lived URI that can be used to render the photo.
    ///
    /// This is a Google-hosted URL (typically on `googleusercontent.com`) that directly serves the
    /// image file. The image format (JPEG, PNG, WebP) depends on what was originally uploaded and
    /// can be determined from the `Content-Type` header when fetching the image.
    ///
    /// **Important:** This URI is temporary and may expire or become rate-limited after some time.
    /// It's intended for immediate use - fetch and cache the image data itself, not this URI. The
    /// URI should not be stored long-term or hardcoded.
    ///
    /// Use this URI to:
    /// - Download the image bytes with an HTTP GET request
    /// - Pass to image loading libraries
    /// - Embed in `<img>` tags for web display (with appropriate caching)
    ///
    /// > ⚠️ The URI may have rate limiting or expiration. Always fetch the image promptly after
    /// > receiving this response.
    #[getset(get = "pub")]
    pub(crate) photo_uri: Url,

    /// Error returned from the Google Maps API server.
    ///
    /// When present, indicates the request failed. The photos list may be empty or incomplete when
    /// an error occurs.
    #[serde(default)]
    #[getset(get = "pub")]
    pub(crate) error: Option<crate::places_new::GoogleApiError>,
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::ops::Deref for Response {
    type Target = Url;

    /// Dereferences to the underlying URL.
    fn deref(&self) -> &Self::Target {
        &self.photo_uri
    }
}

impl std::ops::DerefMut for Response {
    /// Mutably dereferences to the underlying places vector.
    ///
    /// Allows mutable access to the places list.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.photo_uri
    }
}

impl AsRef<Url> for Response {
    /// Returns the places as a slice.
    ///
    /// Allows passing the response to functions expecting `&Url`.
    fn as_ref(&self) -> &Url {
        &self.photo_uri
    }
}

impl AsMut<Url> for Response {
    /// Returns the places as a mutable slice.
    ///
    /// Allows passing the response to functions expecting `&mut [Url]`.
    fn as_mut(&mut self) -> &mut Url {
        &mut self.photo_uri
    }
}

impl std::convert::From<Response> for Result<Response, crate::Error> {
    /// Converts a Google Maps API `Response` into a `Result<Response, Error>` by examining the
    /// `status` field inside of the response.
    ///
    /// If the status indicates a success, then an `Ok(response)` will be returned. If the status
    /// indicates an error, then an `Err(error)` will be returned.
    fn from(response: Response) -> Self {
        match response.error {
            Some(error) => Err(crate::places_new::Error::from(error).into()),
            None => Ok(response),
        }
    }
}