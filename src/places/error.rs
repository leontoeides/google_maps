//! Places API Place Autocomplete service error types and error messages.

// -----------------------------------------------------------------------------
//
/// An error produced by a Google Maps Places API Place Autocomplete request.
#[derive(Clone, Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    // -------------------------------------------------------------------------
    // Client-side errors:
    // -------------------------------------------------------------------------

    // Parsing errors:

    /// Invalid [autocomplete type](https://developers.google.com/maps/documentation/places/web-service/autocomplete#types).
    ///
    /// Valid types are `geocode`, `address`, `establishment`, `(regions)`, and
    /// `(cities)`.
    #[error("invalid autocomplete type: `{0}`")]
    #[diagnostic(
        code(google_maps::places::parse::invalid_autocomplete_type),
        url("https://developers.google.com/maps/documentation/places/web-service/autocomplete#types"),
        help("valid types are `geocode`, `address`, `establishment`, \
            `(regions)`, and `(cities)`")
    )]
    InvalidAutocompleteType(String),

    /// Invalid [business status](https://developers.google.com/maps/documentation/places/web-service/search-find-place#Place-business_status) code.
    ///
    /// Valid codes are: `OPERATIONAL`, `CLOSED_TEMPORARILY`, and
    /// `CLOSED_PERMANENTLY`.
    #[error("invalid business status: `{0}`")]
    #[diagnostic(
        code(google_maps::places::parse::invalid_business_status_code),
        url("https://developers.google.com/maps/documentation/places/web-service/search-find-place#Place-business_status"),
        help("valid codes are: `OPERATIONAL`, `CLOSED_TEMPORARILY`, and \
            `CLOSED_PERMANENTLY`")
    )]
    InvalidBusinessStatusCode(String),

    /// Invalid [secondary hours type](<https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#secondaryhourstype>.
    ///
    /// Valid codes are `DRIVE_THROUGH`, `HAPPY_HOUR`, `DELIVERY`, `TAKEOUT`,
    /// `KITCHEN`, `BREAKFAST`, `LUNCH`, `DINNER`, `BRUNCH`, `PICKUP`, and
    /// `SENIOR_HOURS`.
    #[error("invalid secondary hours type: `{0}`")]
    #[diagnostic(
        code(google_maps::places::parse::invalid_secondary_hours_type),
        url("https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#secondaryhourstype"),
        help("valid codes are: `DRIVE_THROUGH`, `HAPPY_HOUR`, `DELIVERY`, \
            `TAKEOUT`, `KITCHEN`, `BREAKFAST`, `LUNCH`, `DINNER`, `BRUNCH`, \
            `PICKUP`, and `SENIOR_HOURS`")
    )]
    InvalidSecondaryHoursType(String),

    /// Invalid [field](https://developers.google.com/maps/documentation/places/web-service/details#fields) code.
    ///
    /// See `https://developers.google.com/maps/documentation/places/web-service/details#fields`
    /// for a list of valid fields.
    #[error("invalid field code: `{0}`")]
    #[diagnostic(
        code(google_maps::places::parse::invalid_field_code),
        url("https://developers.google.com/maps/documentation/places/web-service/details#fields"),
    )]
    InvalidFieldCode(String),

    /// Invalid [rank-by](https://developers.google.com/maps/documentation/places/web-service/search-nearby#rankby) code.
    ///
    /// Valid codes are: `prominence` and `distance`.
    #[error("invalid rank-by code: `{0}`")]
    #[diagnostic(
        code(google_maps::places::parse::invalid_rank_by_code),
        url("https://developers.google.com/maps/documentation/places/web-service/search-nearby#rankby"),
        help("valid codes are: `prominence` and `distance`")
    )]
    InvalidRankByCode(String),

    /// Invalid [sort](https://developers.google.com/maps/documentation/places/web-service/details#reviews_sort) order code.
    ///
    /// Valid codes are: `most_relevant` and `newest`.
    #[error("invalid sort-order code: `{0}`")]
    #[diagnostic(
        code(google_maps::places::parse::invalid_sort_order_code),
        url("https://developers.google.com/maps/documentation/places/web-service/details#reviews_sort"),
        help("valid codes are: `most_relevant` and `newest`")
    )]
    InvalidSortOrderCode(String),

    /// Invalid status code.
    ///
    /// Valid codes are `OK`, `ZERO_RESULTS`, `INVALID_REQUEST`,
    /// `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, and `UNKNOWN_ERROR`.
    #[error("invalid status: `{0}`")]
    #[diagnostic(
        code(google_maps::places::parse::invalid_status_code),
        url(docsrs),
        help("valid codes are `OK`, `ZERO_RESULTS`, `INVALID_REQUEST`, \
            `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, and `UNKNOWN_ERROR`")
    )]
    InvalidStatusCode(String),

    // -------------------------------------------------------------------------
    // Server-side errors (statuses):
    // -------------------------------------------------------------------------

    /// Zero results indicates that the search was successful but returned no
    /// results.
    #[error("zero results")]
    #[diagnostic(
        code(google_maps::places::status::zero_results),
        url(docsrs),
        help("indicates that the search was successful but returned no \
            results")
    )]
    ZeroResults,

    /// Invalid request indicates the API request was malformed, generally due
    /// to a missing required query parameter.
    #[error("invalid request")]
    #[diagnostic(
        code(google_maps::places::status::invalid_request),
        url(docsrs),
        help("indicates the API request was malformed, generally due to a \
            missing required query parameter")
    )]
    InvalidRequest,

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
        code(google_maps::places::status::over_query_limit),
        url("https://developers.google.com/maps/faq#over-limit-key-error"),
        help("either you have exceeded the QPS limits, billing has not been \
            enabled on your account, a self-imposed usage cap has been \
            exceeded or the provided method of payment is no longer valid")
    )]
    OverQueryLimit,

    /// Request denied indicates that your request was denied, generally
    /// because:
    ///
    /// * The request is missing an API key.
    ///
    /// * The `key` parameter is invalid.
    #[error("request denied")]
    #[diagnostic(
        code(google_maps::places::status::request_denied),
        url(docsrs),
        help("indicates that your request was denied, generally because the \
            request is missing an API key, or the `key` parameter is invalid")
    )]
    RequestDenied,

    /// Unknown error from the server.
    #[error("unknown error")]
    #[diagnostic(
        code(google_maps::places::status::unknown_error),
        url(docsrs),
        help("indicates that the request could not be processed due to a \
            server error. The request may succeed if you try again")
    )]
    UnknownError,

    /// Not found indicates that that the referenced location was not found in
    /// the places database.
    #[error("not found")]
    #[diagnostic(
        code(google_maps::places::status::not_found),
        url(docsrs),
        help("indicates that that the referenced location was not found in the \
            places database")
    )]
    NotFound,
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