// -------------------------------------------------------------------------------------------------
//
/// Additional HTTP headers required for this request.
///
/// Google's new Places API decided that query parameters weren't good enough and moved things like
/// the API key to HTTP headers instead. 🙄
pub trait RequestHeaders {
    /// Returns a map of HTTP header names to values.
    ///
    /// These headers will be added to the HTTP request alongside the standard headers like
    /// `Content-Type`. Defaults to an empty map for APIs that don't need custom headers.
    fn request_headers(&self) -> reqwest::header::HeaderMap;

    /// Returns whether the `X-Goog-Api-Key` header should be set for this request.
    fn send_x_goog_api_key() -> bool;
}