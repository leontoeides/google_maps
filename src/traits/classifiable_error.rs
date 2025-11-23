use crate::ClassifiedError;

// -----------------------------------------------------------------------------
//
/// This trait is used to classify types of errors and failures for different
/// error types and different status types.
///
/// This classification will, in turn, be used to decide whether the request
/// should be retried or not.
pub trait ClassifiableError<'a, E> {
    /// Wraps an error, or a status, in a `ClassifiedError` enum. This wrapper
    /// indicates the type of failure.
    ///
    /// This classification will, in turn, be used to decide whether the request
    /// should be retried or not.
     fn classify(&self) -> ClassifiedError<'_, E>;
}

#[cfg(feature = "reqwest")]
impl ClassifiableError<'_, Self> for reqwest::Error {
    /// Classifies a [reqwest](https://crates.io/crates/reqwest) error as a
    /// `Transient` error or `Permanent` error.
    ///
    /// This classification will, in turn, be used to decide whether the HTTP
    /// request should be retried or not.
    fn classify(&self) -> ClassifiedError<'_, Self> {
        // re: `without_url` - Strip the related url from this error because it
        // contains the API key which is sensitive information.
        if self.is_connect() ||self.is_timeout() {
            ClassifiedError::Transient(self)
        } else {
            ClassifiedError::Permanent(self)
        }
    }
}

// -----------------------------------------------------------------------------

#[cfg(feature = "reqwest")]
impl ClassifiableError<'_, Self> for reqwest::StatusCode {
    /// Classifies a [reqwest](https://crates.io/crates/reqwest) HTTP status
    /// code as `None` (no error), a `Transient` error, or a `Permanent` error.
    ///
    /// This classification will, in turn, be used to decide whether the HTTP
    /// request should be retried or not.
    fn classify(&self) -> ClassifiedError<'_, Self> {
        if self.is_success() {
            // Success responses should not be retried:
            ClassifiedError::None(self)
        } else if self.is_server_error() || *self == 429 {
            // We got a response from the server but it was not OK. Only HTTP
            // "500 Server Errors", and HTTP "429 Too Many Requests" are
            // eligible for retries.
            ClassifiedError::Transient(self)
        } else {
            // Any other type of error is considered permanent, and should not
            // be re-tried:
            ClassifiedError::Permanent(self)
        }
    }
}

// -----------------------------------------------------------------------------

impl ClassifiableError<'_, Self> for serde_json::Error {
    /// Classifies a [serde_json](https://crates.io/crates/serde_json) error as
    /// a `Transient` error or `Permanent` error.
    ///
    /// This classification will, in turn, be used to decide whether the HTTP
    /// request should be retried or not.
    fn classify(&self) -> ClassifiedError<'_, Self> {
        ClassifiedError::Permanent(self)
    }
}