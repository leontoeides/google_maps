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

    /// Forward geocoding requests (address to latlng) must specify an `address`
    /// or at least one `component`.
    #[error("forward geocoding requests must specify an `address` or at least \
        one `component`")]
    #[diagnostic(
        code(google_maps::geocoding::validate::either_positional_or_sampled_path),
        url(docsrs),
        help("ensure that the `with_address` and/or `with_component` methods \
            are being used to build the request")
    )]
    AddressOrComponentsRequired,

    // Parsing errors:

    /// Invalid status code.
    ///
    /// Valid codes are `OK`, `ZERO_RESULT`, `OVER_DAILY_LIMIT`,
    /// `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, `INVALID_REQUEST`, and
    /// `UNKNOWN_ERROR`.
    #[error("invalid status: `{0}`")]
    #[diagnostic(
        code(google_maps::geocoding::parse::invalid_status_code),
        url("https://developers.google.com/maps/documentation/geocoding/requests-geocoding#StatusCodes"),
        help("valid codes are `OK`, `ZERO_RESULT`, `OVER_DAILY_LIMIT`, \
            `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, `INVALID_REQUEST`, and \
            `UNKNOWN_ERROR`.")
    )]
    InvalidStatusCode(String),

    // -------------------------------------------------------------------------
    // Server-side errors (statuses):
    // -------------------------------------------------------------------------

    /// Zero results.
    ///
    /// The geocode was successful but returned no results. This may occur if
    /// the geocoder was passed a non-existent `address`.
    #[error("zero results")]
    #[diagnostic(
        code(google_maps::geocoding::status::zero_results),
        url("https://developers.google.com/maps/documentation/geocoding/requests-geocoding#StatusCodes"),
        help("indicates that the geocode was successful but returned no \
            results. This may occur if the geocoder was passed a non-existent \
            address")
    )]
    ZeroResults,

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
        code(google_maps::geocoding::status::over_daily_limit),
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
        code(google_maps::geocoding::status::over_query_limit),
        url("https://developers.google.com/maps/faq#over-limit-key-error"),
        help("either you have exceeded the QPS limits, billing has not been \
            enabled on your account, a self-imposed usage cap has been \
            exceeded or the provided method of payment is no longer valid")
    )]
    OverQueryLimit,

    /// Request denied by the server.
    #[error("request denied")]
    #[diagnostic(
        code(google_maps::geocoding::status::request_denied),
        url("https://developers.google.com/maps/documentation/geocoding/requests-geocoding#StatusCodes"),
        help("indicates that your request was denied")
    )]
    RequestDenied,

    /// Invalid request.
    ///
    /// The query (`address`, `components` or `latlng`) is missing.
    #[error("invalid request")]
    #[diagnostic(
        code(google_maps::geocoding::status::invalid_request),
        url("https://developers.google.com/maps/documentation/geocoding/requests-geocoding#StatusCodes"),
        help("generally indicates that the query (address, components or \
            latlng) is missing")
    )]
    InvalidRequest,

    /// Unknown error from the server.
    #[error("unknown error")]
    #[diagnostic(
        code(google_maps::geocoding::status::unknown_error),
        url("https://developers.google.com/maps/documentation/geocoding/requests-geocoding#StatusCodes"),
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