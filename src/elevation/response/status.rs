use serde::{Serialize, Deserialize};

/// Indicates the status of the response.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Status {

    /// Indicates that the request was malformed.
    #[serde(alias = "INVALID_REQUEST")]
    InvalidRequest,

    /// Indicates that the request was successful.
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

    /// Indicates the requestor has exceeded quota.
    #[serde(alias = "OVER_QUERY_LIMIT")]
    OverQueryLimit,

    /// Indicates that the API did not complete the request.
    #[serde(alias = "REQUEST_DENIED")]
    RequestDenied,

    /// Indicates an unknown error.
    #[serde(alias = "UNKNOWN_ERROR")]
    UnknownError,

} // struct