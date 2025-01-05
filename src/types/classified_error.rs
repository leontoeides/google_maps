use crate::traits::ClassifiableError;

// -----------------------------------------------------------------------------
//
/// Used to classify errors and API response statuses as `None`, `Transient` or
/// `Permanent`.
///
/// This classification will, in turn, be used to decide whether the request
/// should be re-tried using the [backon](https://crates.io/crates/backon) crate
/// or not.
#[derive(Debug)]
pub enum ClassifiedError<'a, E> {
    /// The passed type did not represent an error of any kind. Success
    /// responses should not be retried.
    None(&'a E),

    /// A potentially temporary error, such as a network glitch causing a
    /// connection issue. These errors are often suitable for retrying the
    /// request.
    Transient(&'a E),

    /// A likely permanent error, such as a request with invalid data. Retrying
    /// these errors are ineffective since the outcome shouldn't change.
    Permanent(&'a E),
} // enum ClassifiedError

// -----------------------------------------------------------------------------

impl<E> ClassifiedError<'_, E> {
    /// Returns whether the error has been classified as `None`.
    ///
    /// This is used for types that may, or may not, represent an error
    /// condition. In the event that the type represents a success or `Ok`
    /// state, `ClassifiedError::None` can be used.
    ///
    /// # Notes
    ///
    /// * `ClassifiedError` is used to determine whether an API request should
    ///   be retried or not. If `is_none` is `true`, everything is good and the
    ///   API call should _not_ be retried.
    #[must_use] pub const fn is_none(&self) -> bool {
        matches!(self, Self::None(_))
    } // fn

    /// Returns whether the error has been classified as `Transient`.
    ///
    /// A transient error is a potentially temporary error, such as a network
    /// glitch causing a connection issue. These errors are often suitable for
    /// retrying the request.
    ///
    /// # Notes
    ///
    /// * `ClassifiedError` is used to determine whether an API request should
    ///   be retried or not. If `is_transient` is `true`, the error result might
    ///   change on subsequent API calls. A retry is worth a shot.
    #[must_use] pub const fn is_transient(&self) -> bool {
        matches!(self, Self::Transient(_))
    } // fn

    /// Returns whether the error has been classified as `Permanent`.
    ///
    /// A likely permanent error, such as a request with invalid data. Retrying
    /// these errors are ineffective since the outcome shouldn't change.
    ///
    /// # Notes
    ///
    /// * `ClassifiedError` is used to determine whether an API request should
    ///   be retried or not. If `is_permanent` is `true`, the error result is
    ///   not expected to change on subsequent API calls. The API call should
    ///   _not_ be retried.
    #[must_use] pub const fn is_permanent(&self) -> bool {
        matches!(self, Self::Permanent(_))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl<'a, E: ClassifiableError<'a, E>> ClassifiedError<'a, E> {
    /// Wraps an error, or a status, in a `ClassifiedError` enum. This wrapper
    /// indicates the type of failure.
    ///
    /// This classification will, in turn, be used to decide whether the request
    /// should be retried or not.
    pub fn new(error: &'a E) -> Self { error.into() }
} // impl

// -----------------------------------------------------------------------------

impl<'a, E: ClassifiableError<'a, E>> std::convert::From<&'a E> for ClassifiedError<'a, E> {
    /// Wraps an error, or a status, in a `ClassifiedError` enum. This wrapper
    /// indicates the type of failure.
    ///
    /// This classification will, in turn, be used to decide whether the request
    /// should be retried or not.
    fn from(error: &'a E) -> Self { error.classify() }
} // impl

// -----------------------------------------------------------------------------

impl<E: std::fmt::Display> std::fmt::Display for ClassifiedError<'_, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassifiedError::None(response) => write!(f, "success: {response}"),
            ClassifiedError::Transient(error) => write!(f, "transient error: {error}"),
            ClassifiedError::Permanent(error) => write!(f, "permanent error: {error}"),
        } // match
    } // fn
} // impl