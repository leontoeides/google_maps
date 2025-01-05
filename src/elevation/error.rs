//! Elevation API error types and error messages.

// -----------------------------------------------------------------------------
//
/// An error produced by a Google Maps Elevation API request.
#[derive(Clone, Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    // -------------------------------------------------------------------------
    // Client-side errors:
    // -------------------------------------------------------------------------

    // Validation errors:

    /// A `sampled_path_request` method cannot be used when `postional_request`
    /// has been set.
    #[error("a `for_sampled_path_request` method cannot be used when \
        `for_postional_request` has been set")]
    #[diagnostic(
        code(google_maps::elevation::validate::either_positional_or_sampled_path),
        url(docsrs),
        help("try again with only a positional request or only a sampled path \
            request")
    )]
    EitherPositionalOrSampledPath,

    // Parsing errors:

    /// Invalid status code.
    ///
    /// Valid codes are `OK`, `DATA_NOT_AVAILABLE`, `INVALID_REQUEST`,
    /// `OVER_DAILY_LIMIT`, `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, and
    /// `UNKNOWN_ERROR`.
    #[error("invalid status: `{0}`")]
    #[diagnostic(
        code(google_maps::elevation::parse::invalid_status_code),
        url("https://developers.google.com/maps/documentation/elevation/requests-elevation#ElevationStatus"),
        help("valid codes are `OK`, `DATA_NOT_AVAILABLE`, `INVALID_REQUEST`, \
            `OVER_DAILY_LIMIT`, `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, and \
            `UNKNOWN_ERROR`")
    )]
    InvalidStatusCode(String),

    // -------------------------------------------------------------------------
    // Server-side errors (statuses):
    // -------------------------------------------------------------------------

    /// Data not available.
    ///
    /// No available data for the input locations.
    #[error("data not available")]
    #[diagnostic(
        code(google_maps::elevation::status::data_not_available),
        url("https://developers.google.com/maps/documentation/elevation/requests-elevation#ElevationStatus"),
        help("indicates that there's no available data for the input location")
    )]
    DataNotAvailable,

    /// Invalid request.
    ///
    /// The API request was malformed
    #[error("invalid request")]
    #[diagnostic(
        code(google_maps::elevation::status::invalid_request),
        url("https://developers.google.com/maps/documentation/elevation/requests-elevation#ElevationStatus"),
        help("indicates the API request was malformed")
    )]
    InvalidRequest,

    /// Over daily limit indicates that the request was denied for one or more
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
    #[error("over daily limit")]
    #[diagnostic(
        code(google_maps::elevation::status::over_daily_limit),
        url("https://developers.google.com/maps/faq#over-limit-key-error"),
        help("either the API key is missing or invalid, billing has not been \
            enabled on your account, a self-imposed usage cap has been \
            exceeded, or the provided method of payment is no longer valid \
            (for example, a credit card has expired)")
    )]
    OverDailyLimit,

    /// Overy query limit indicates any of the following:
    ///
    /// * You have exceeded the QPS limits.
    ///
    /// * Billing has not been enabled on your account.
    ///
    /// * The monthly $200 credit, or a self-imposed usage cap, has been
    ///   exceeded.
    ///
    /// * The provided method of payment is no longer valid (for example, a
    ///   credit card has expired).
    ///
    /// See the [Maps FAQ](https://developers.google.com/maps/faq#over-limit-key-error)
    /// for more information about how to resolve this error.
    #[error("over query limit")]
    #[diagnostic(
        code(google_maps::elevation::status::over_query_limit),
        url("https://developers.google.com/maps/faq#over-limit-key-error"),
        help("either you have exceeded the QPS limits, billing has not been \
            enabled on your account, a self-imposed usage cap has been \
            exceeded or the provided method of payment is no longer valid")
    )]
    OverQueryLimit,

    /// Request denied by the server.
    #[error("request denied")]
    #[diagnostic(
        code(google_maps::elevation::status::request_denied),
        url("https://developers.google.com/maps/documentation/elevation/requests-elevation#ElevationStatus"),
        help("indicates that your request was denied")
    )]
    RequestDenied,

    /// Unknown error from the server.
    #[error("unknown error")]
    #[diagnostic(
        code(google_maps::elevation::status::unknown_error),
        url("https://developers.google.com/maps/documentation/elevation/requests-elevation#ElevationStatus"),
        help("indicates that the request could not be processed due to a \
            server error. The request may succeed if you try again")
    )]
    UnknownError,
} // enum Error

// -----------------------------------------------------------------------------

use crate::ClassifiedError;

impl crate::traits::ClassifiableError<'_, Self> for Error {
    /// Classifies an API error as a `Transient` error or `Permanent` error.
    ///
    /// This classification will, in turn, be used to decide whether the HTTP
    /// request should be retried or not.
    fn classify(&self) -> ClassifiedError<'_, Self> {
        match self {
            Self::UnknownError => ClassifiedError::Transient(self),
            _ => ClassifiedError::Permanent(self),
        } // match
    } // fn
} // impl