//! Roads API error types and error messages.

// -----------------------------------------------------------------------------
//
/// An error produced by a Google Maps Roads API request.
#[derive(Clone, Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    // -------------------------------------------------------------------------
    // Client-side errors:
    // -------------------------------------------------------------------------

    // Parsing errors:

    /// Invalid status code.
    ///
    /// Valid codes are `INVALID_ARGUMENT`, `PERMISSION_DENIED`, `NOT_FOUND` and
    /// `RESOURCE_EXHAUSTED`
    #[error("invalid status: `{0}`")]
    #[diagnostic(
        code(google_maps::roads::parse::invalid_status_code),
        url("https://developers.google.com/maps/documentation/roads/errors#errors"),
        help("valid codes are `INVALID_ARGUMENT`, `PERMISSION_DENIED`, \
            `NOT_FOUND` and `RESOURCE_EXHAUSTED`")
    )]
    InvalidStatusCode(String),

    // -------------------------------------------------------------------------
    // Server-side errors (statuses):
    // -------------------------------------------------------------------------

    /// Invalid argument.
    ///
    /// 1. Your API key is not valid or was not included in the request. Please
    ///    ensure that you've included the entire key, and that you've enabled
    ///    the API for this key.
    ///
    /// 2. Your request contained invalid arguments. The most likely causes of
    ///    this error are:
    ///
    ///     * A problem with your path parameter.
    ///
    ///     * Please ensure you have at least 1, and fewer than 100 points. Each
    ///       point should be a pair of numbers separated by a comma, such as:
    ///       `48.409114,-123.369158`. Points should be separated by a pipe:
    ///       `|`.
    ///
    ///     * Your request included an invalid `placeId`.
    ///
    ///     * Your request included both `placeId`s and a `path`. Only one of
    ///       these parameters may be specified for each request. This error
    ///       will not be returned if a `placeId` is passed for a road which no
    ///       longer exists, or for a place which is not a road.
    #[error("invalid argument: {0}")]
    #[diagnostic(
        code(google_maps::roads::status::invalid_argument),
        url("https://developers.google.com/maps/documentation/roads/errors#errors"),
        help("your API key is not valid or was not included in the request or \
            your request contained invalid arguments")
    )]
    InvalidArgument(String),

    /// Permission denied indicates that the request was denied for one or more
    /// of the following reasons:
    ///
    /// * The API key is missing or invalid.
    ///
    /// * Billing has not been enabled on your account.
    ///
    /// * A self-imposed usage cap has been exceeded.
    ///
    /// * The provided method of payment is no longer valid (for example, a
    ///   credit card has expired).
    ///
    /// In order to use Google Maps Platform products, billing must be enabled
    /// on your account, and all requests must include a valid API key. To fix
    /// this, take the following steps:
    ///
    /// * [Get an API key](https://developers.google.com/maps/documentation/roads/errors?hl=en#new-key)
    ///
    /// * [Enable billing](https://console.cloud.google.com/project/_/billing/enable)
    ///   on your account.
    ///
    /// * [Adjust your usage cap](https://developers.google.com/maps/documentation/roads/errors?hl=en#usage-cap)
    ///   to increase your daily limit (if applicable).
    #[error("permission denied: {0}")]
    #[diagnostic(
        code(google_maps::roads::status::permission_denied),
        url("https://developers.google.com/maps/documentation/roads/errors#errors"),
        help("either the API key is missing or invalid, billing has not been \
            enabled on your account, a self-imposed usage cap has been \
            exceeded, or the provided method of payment is no longer valid \
            (for example, a credit card has expired)")
    )]
    PermissionDenied(String),

    /// Not found. Ensure that you are sending requests to
    /// `https://roads.googleapis.com/` and not `http://roads.googleapis.com/`.
    #[error("not found: {0}")]
    #[diagnostic(
        code(google_maps::roads::status::not_found),
        url("https://developers.google.com/maps/documentation/roads/errors#errors"),
        help("ensure that you are sending requests to \
            `https://roads.googleapis.com/` and not \
            `http://roads.googleapis.com/`")
    )]
    NotFound(String),

    /// Resource exhaused. You have exceeded the request limit that you
    /// configured in the Google Cloud Platform Console. This limit is typically
    /// set as requests per day, requests per 100 seconds, and requests per 100
    /// seconds per user. This limit should be configured to prevent a single or
    /// small group of users from exhausting your daily quota, while still
    /// allowing reasonable access to all users. See Capping API Usage to
    /// configure these limits.
    #[error("resource exhausted: {0}")]
    #[diagnostic(
        code(google_maps::roads::status::permission_denied),
        url("https://cloud.google.com/apis/docs/capping-api-usage"),
        help("you have exceeded the request limit that you configured in the \
            Google Cloud Platform Console")
    )]
    ResourceExhausted(String),
} // enum Error

// -----------------------------------------------------------------------------

use crate::ClassifiedError;

impl crate::traits::ClassifiableError<'_, Self> for Error {
    /// Classifies an API error as a `Transient` error or `Permanent` error.
    ///
    /// This classification will, in turn, be used to decide whether the HTTP
    /// request should be retried or not.
    fn classify(&self) -> ClassifiedError<'_, Self> {
        ClassifiedError::Permanent(self)
    } // fn
} // impl