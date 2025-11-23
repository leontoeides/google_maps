// -----------------------------------------------------------------------------
//
/// Defines a Google Maps API HTTP end-point for a request.
///
/// This trait returns information needed to make connect HTTP `GET` requests to
/// their API end-point. It also includes some extra information for debugging
/// and rating-limiting.
///
/// ## Notes
///
/// * This trait should be implemented for the `Request` structure. When the
///   request structure is submitted, the end-point information will be looked
///   up from this trait.
pub trait EndPoint {
    /// URL for the HTTP end-point. For example,
    /// `https://maps.googleapis.com/maps/api/directions`. This string will be
    /// used to build the HTTP requests.
    fn service_url() -> &'static str;

    /// Google Maps accepts `xml` and `json` formats. Currently, this crate only
    /// supports the `json` format and this function should only return `json`
    /// for now.
    #[must_use]
    fn output_format() -> Option<&'static str> { Some("json") }

    /// Title of the API request end-point. For example `Directions API` or
    /// `Elevation API`. This title will be output in `tracing` messages.
    #[cfg(feature = "reqwest")]
    fn title() -> &'static str;

    /// Returns which APIs are being used. This is used for rate-limiting on an
    /// API-basis.
    #[cfg(feature = "reqwest")]
    fn apis() -> &'static [crate::request_rate::api::Api];
} // trait EndPoint