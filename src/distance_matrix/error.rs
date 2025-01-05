//! Distance Matrix API error types and error messages.

// -----------------------------------------------------------------------------
//
/// An error produced by a Google Maps Distance Matrix API request.
#[derive(Clone, Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    // -------------------------------------------------------------------------
    // Client-side errors:
    // -------------------------------------------------------------------------

    // Validation errors:

    /// An arrival time may only be specified in Transit travel mode.
    #[error("`with_arrival_time` method may only be used when \
        `with_travel_mode` is set to `TravelMode::Transit`. The travel mode has
        been set to `{0}` and the arrival time has been set to `{1}`")]
    #[diagnostic(
        code(google_maps::distance_matrix::validate::arrival_time_is_for_transit_only),
        url(docsrs),
        help("try again either with a travel mode of `TravelMode::Transit` or \
            no arrival time")
    )]
    ArrivalTimeIsForTransitOnly(String, String),

    /// Either departure time or arrival time may be specified, but not both.
    #[error("`with_departure_time` method cannot be used when \
        `with_arrival_time` has been set. The arrival time has been set to \
        `{0}` and the departure time has been set to `{1}`")]
    #[diagnostic(
        code(google_maps::distance_matrix::validate::either_departure_time_or_arrival_time),
        url(docsrs),
        help("try again either with no arrival time or no departure time")
    )]
    EitherDepartureTimeOrArrivalTime(String, String),

    /// Transit mode may only be specified in Transit travel mode.
    #[error("`with_transit_modes` method may only be used when \
        `with_travel_mode` is set to `TravelMode::Transit`. The travel mode \
        has been set to `{0}` and the transit modes have been set to `{1}`")]
    #[diagnostic(
        code(google_maps::distance_matrix::validate::transit_mode_is_for_transit_only),
        url(docsrs),
        help("try again either with a travel mode of `TravelMode::Transit` or no transit modes")
    )]
    TransitModeIsForTransitOnly(String, String),

    /// Transit routing preference may only be specified in Transit travel mode.
    #[error("`with_transit_route_preference` method may only be used when \
        `with_travel_mode` is set to `TravelMode::Transit`. The travel mode \
        has been set to `{0}` and the transit route preference has been set to \
        `{1}`")]
    #[diagnostic(
        code(google_maps::distance_matrix::validate::transit_mode_is_for_transit_only),
        url(docsrs),
        help("try again either with a travel mode of `TravelMode::Transit` or no transit route preference")
    )]
    TransitRoutePreferenceIsForTransitOnly(String, String),

    // Parse errors:

    /// Invalid element status code.
    ///
    /// Valid codes are `OK`, `NOT_FOUND`, `ZERO_RESULTS`, and
    /// `MAX_ROUTE_LENGTH_EXCEEDED`.
    #[error("invalid element status: `{0}`")]
    #[diagnostic(
        code(google_maps::distance_matrix::parse::invalid_element_status_code),
        url("https://developers.google.com/maps/documentation/distance-matrix/distance-matrix#DistanceMatrixStatus"),
        help("valid codes are `OK`, `NOT_FOUND`, `ZERO_RESULTS`, and \
            `MAX_ROUTE_LENGTH_EXCEEDED`")
    )]
    InvalidElementStatusCode(String),

    /// Invalid status code.
    ///
    /// Valid codes are `INVALID_REQUEST`, `MAX_ELEMENTS_EXCEEDED`,
    /// `MAX_DIMENSIONS_EXCEEDED`, `OVER_QUERY_LIMIT`, `OVER_QUERY_LIMIT`,
    /// `REQUEST_DENIED`, and `UNKNOWN_ERROR`.
    #[error("invalid status: `{0}`")]
    #[diagnostic(
        code(google_maps::distance_matrix::parse::invalid_status_code),
        url("https://developers.google.com/maps/documentation/directions/get-directions#DirectionsStatus"),
        help("valid codes are `INVALID_REQUEST`, `MAX_ELEMENTS_EXCEEDED`, \
            `MAX_DIMENSIONS_EXCEEDED`, `OVER_QUERY_LIMIT`, `OVER_QUERY_LIMIT`, \
            `REQUEST_DENIED`, and `UNKNOWN_ERROR`")
    )]
    InvalidStatusCode(String),

    // -------------------------------------------------------------------------
    // Server-side errors (statuses):
    // -------------------------------------------------------------------------

    /// Invalid request indicates that the provided request was invalid. Common
    /// causes of this status include an invalid parameter or parameter value.
    #[error("invalid request")]
    #[diagnostic(
        code(google_maps::distance_matrix::status::invalid_request),
        url("https://developers.google.com/maps/documentation/distance-matrix/distance-matrix#DistanceMatrixStatus"),
        help("indicates that the provided request was invalid. Common causes \
            of this status include an invalid parameter or parameter value")
    )]
    InvalidRequest,

    /// Maximum elements exceeded indicates that the product of origins and
    /// destinations exceeds the per-query limit.
    #[error("maximum elements exceeded")]
    #[diagnostic(
        code(google_maps::distance_matrix::status::max_elements_exceeded),
        url("https://developers.google.com/maps/documentation/distance-matrix/distance-matrix#DistanceMatrixStatus"),
        help("indicates that the product of origins and destinations exceeds \
            the per-query limit")
    )]
    MaxElementsExceeded,

    /// Maximum dimensions exceeded indicates that the number of origins or
    /// destinations exceeds the per-query limit.
    #[error("maximum dimensions exceeded")]
    #[diagnostic(
        code(google_maps::distance_matrix::status::max_dimensions_exceeded),
        url("https://developers.google.com/maps/documentation/distance-matrix/distance-matrix#DistanceMatrixStatus"),
        help("indicates that the number of origins or destinations exceeds the \
            per-query limit")
    )]
    MaxDimensionsExceeded,

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
        code(google_maps::distance_matrix::status::over_daily_limit),
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
        code(google_maps::distance_matrix::status::over_query_limit),
        url("https://developers.google.com/maps/faq#over-limit-key-error"),
        help("either you have exceeded the QPS limits, billing has not been \
            enabled on your account, a self-imposed usage cap has been \
            exceeded or the provided method of payment is no longer valid")
    )]
    OverQueryLimit,

    /// Request denied by the server.
    #[error("request denied")]
    #[diagnostic(
        code(google_maps::distance_matrix::status::request_denied),
        url("https://developers.google.com/maps/documentation/directions/get-directions#DirectionsStatus"),
        help("indicates that the service denied use of the Distance Matrix \
            service by your application")
    )]
    RequestDenied,

    /// Unknown error from the server.
    #[error("unknown error")]
    #[diagnostic(
        code(google_maps::distance_matrix::status::unknown_error),
        url("https://developers.google.com/maps/documentation/distance-matrix/distance-matrix#DistanceMatrixStatus"),
        help("indicates a Distance Matrix request could not be processed due \
            to a server error. The request may succeed if you try again.")
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