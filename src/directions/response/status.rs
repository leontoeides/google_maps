use serde::{Serialize, Deserialize};

/// The `status` field within the Directions response object contains the status
/// of the request, and may contain debugging information to help you track down
/// why the Directions service failed.

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Status {

    /// Indicates that the provided request was invalid. Common causes of this
    /// status include an invalid parameter or parameter value.
    #[serde(alias = "INVALID_REQUEST")]
    InvalidRequest,

    /// Indicates the requested route is too long and cannot be processed. This
    /// error occurs when more complex directions are returned. Try reducing the
    /// number of waypoints, turns, or instructions.
    #[serde(alias = "MAX_ROUTE_LENGTH_EXCEEDED")]
    MaxRouteLengthExceeded,

    /// Indicates that too many `waypoints` were provided in the request. For
    /// applications using the Directions API as a web service, or the
    /// [directions service in the Maps JavaScript API](https://developers.google.com/maps/documentation/javascript/directions),
    /// the maximum allowed number of `waypoints` is 25, plus the origin and
    /// destination.
    #[serde(alias = "MAX_WAYPOINTS_EXCEEDED")]
    MaxWaypointsExceeded,

    /// Indicates at least one of the locations specified in the request's
    /// origin, destination, or waypoints could not be geocoded.
    #[serde(alias = "NOT_FOUND")]
    NotFound,

    /// Indicates the response contains a valid `result`.
    #[serde(alias = "OK")]
    Ok,

    /// Indicates any of the following:
    /// * The API key is missing or invalid.
    /// * Billing has not been enabled on your account.
    /// * A self-imposed usage cap has been exceeded.
    /// * The provided method of payment is no longer valid (for example, a
    /// credit card has expired).
    ///
    /// See the [Maps
    /// FAQ](https://developers.google.com/maps/faq#over-limit-key-error) to
    /// learn how to fix this.
    #[serde(alias = "OVER_DAILY_LIMIT")]
    OverDailyLimit,

    /// Indicates the service has received too many requests from your
    /// application within the allowed time period.
    #[serde(alias = "OVER_QUERY_LIMIT")]
    OverQueryLimit,

    /// Indicates that the service denied use of the directions service by your
    /// application.
    #[serde(alias = "REQUEST_DENIED")]
    RequestDenied,

    /// Indicates a directions request could not be processed due to a server
    /// error. The request may succeed if you try again.
    #[serde(alias = "UNKNOWN_ERROR")]
    UnknownError,

    /// Indicates no route could be found between the origin and destination.
    #[serde(alias = "ZERO_RESULTS")]
    ZeroResults,

} // struct