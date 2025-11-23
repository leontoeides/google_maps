use reqwest::header::HeaderMap;

// -------------------------------------------------------------------------------------------------

impl crate::traits::RequestHeaders for crate::places::place_search::nearby_search::Request<'_> {
    /// Returns a map of HTTP header names to values.
    ///
    /// These headers will be added to the HTTP request alongside the standard headers like
    /// `Content-Type`.
    ///
    /// This API end-point does not use headers so it returns an empty hash map.
    fn request_headers(&self) -> HeaderMap {
        HeaderMap::default()  // No additional, extra or custom headers
    }

    /// Returns whether the `X-Goog-Api-Key` header should be set for this request.
    fn send_x_goog_api_key() -> bool {
        false
    }
}