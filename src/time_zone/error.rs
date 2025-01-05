//! Time Zone API error types and error messages.

// -----------------------------------------------------------------------------
//
/// An error produced by a Google Maps Time Zone API request.
#[derive(Clone, Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    // -------------------------------------------------------------------------
    // Client-side errors:
    // -------------------------------------------------------------------------

    // Parsing errors:

    /// Invalid status code.
    ///
    /// Valid codes are `OK`, `INVALID_REQUEST`, `OVER_DAILY_LIMIT`,
    /// `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, `UNKNOWN_ERROR` and
    /// `ZERO_RESULTS`.
    #[error("invalid status: `{0}`")]
    #[diagnostic(
        code(google_maps::time_zone::parse::invalid_status_code),
        url("https://developers.google.com/maps/documentation/timezone/requests-timezone#TimeZoneStatus"),
        help("valid codes are `OK`, `INVALID_REQUEST`, `OVER_DAILY_LIMIT`, \
            `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, `UNKNOWN_ERROR` and \
            `ZERO_RESULTS`")
    )]
    InvalidStatusCode(String),

    // -------------------------------------------------------------------------
    // Server-side errors (statuses):
    // -------------------------------------------------------------------------

    /// Invalid request. The request was malformed.
    #[error("invalid request")]
    #[diagnostic(
        code(google_maps::time_zone::status::invalid_request),
        url("https://developers.google.com/maps/documentation/timezone/requests-timezone#TimeZoneStatus"),
        help("indicates that the request was malformed")
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
        code(google_maps::time_zone::status::over_daily_limit),
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
        code(google_maps::time_zone::status::over_query_limit),
        url("https://developers.google.com/maps/faq#over-limit-key-error"),
        help("either you have exceeded the QPS limits, billing has not been \
            enabled on your account, a self-imposed usage cap has been \
            exceeded or the provided method of payment is no longer valid")
    )]
    OverQueryLimit,

    /// Request denied by the server.
    #[error("request denied")]
    #[diagnostic(
        code(google_maps::time_zone::status::request_denied),
        url("https://developers.google.com/maps/documentation/timezone/requests-timezone#TimeZoneStatus"),
        help("indicates that your request was denied")
    )]
    RequestDenied,

    /// Unknown error from the server.
    #[error("unknown error")]
    #[diagnostic(
        code(google_maps::time_zone::status::unknown_error),
        url("https://developers.google.com/maps/documentation/timezone/requests-timezone#TimeZoneStatus"),
        help("indicates that the request could not be processed due to a \
            server error. The request may succeed if you try again")
    )]
    UnknownError,

    /// Zero results indicates that no time zone data could be found for the
    /// specified position or time. Confirm that the request is for a location
    /// on land, and not over water.
    #[error("zero results")]
    #[diagnostic(
        code(google_maps::time_zone::status::zero_results),
        url("https://developers.google.com/maps/documentation/timezone/requests-timezone#TimeZoneStatus"),
        help("indicates that no time zone data could be found for the \
            specified position or time. Confirm that the request is for a \
            location on land, and not over water")
    )]
    ZeroResults,
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