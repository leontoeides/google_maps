// -------------------------------------------------------------------------------------------------
//
/// Errors that can occur when using the Google Maps Places API (New) Autocomplete service.
///
/// Provides detailed diagnostic information for debugging API client issues and server errors.
#[derive(Clone, Debug, PartialEq, thiserror::Error, miette::Diagnostic)]
#[non_exhaustive]
pub enum Error {
    /// Both location bias and location restriction were set.
    ///
    /// Occurs when attempting to create an autocomplete request with both `location_bias` and
    /// `location_restriction` set.
    ///
    /// Google Maps API requires that at most one of these fields is specified per request.
    #[diagnostic(
        code(google_maps::places_new::mutually_exclusive_location_fields),
        severity(Error),
        help(
            "use either location_bias OR location_restriction, not both. \
            bias influences ranking while restriction hard-limits results"
        )
    )]
    #[error("cannot set both location bias and location restriction")]
    MutuallyExclusiveLocationFields {
        #[source_code]
        debug: String,
        #[label("both fields were set, but only one is allowed")]
        span: (usize, usize),
    },

    /// Decimal arithmetic or parsing operation failed.
    ///
    /// Wraps errors from the `rust_decimal` crate when performing decimal arithmetic operations or
    /// parsing decimal strings.
    #[diagnostic(
        code(google_maps::places::decimal_error),
        severity(Error),
        help("check that decimal values are within valid range and properly formatted")
    )]
    #[error("decimal operation error: {source}")]
    Decimal {
        #[from]
        #[source]
        source: rust_decimal::Error,
    },

    /// Unknown error from the Google Maps server.
    ///
    /// Indicates a server-side error that couldn't be categorized. The request may succeed if
    /// retried after a short delay.
    #[diagnostic(
        code(google_maps::places_new::unknown_error),
        severity(Error),
        help("the request may succeed if you try again after a brief delay")
    )]
    #[error("unknown server error")]
    UnknownError,

    /// The error type for errors that can never happen.
    ///
    /// This error is guaranteed to be unreachable.
    #[error("an error that can never happen")]
    StdConvertInfallible {
        #[from]
        #[source]
        source: std::convert::Infallible,
    },
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::convert::From<Error> for crate::Error {
    fn from(error: Error) -> Self {
        Self::PlacesNew { source: error.into() }
    }
}

impl crate::traits::ClassifiableError<'_, Self> for Error {
    /// If a “Places API (New) Autcomplete” error is permenant, retrying the same request will not
    /// change the outcome. Transient errors are typically from network or disk related.
    fn classify(&self) -> crate::ClassifiedError<'_, Self> {
        if self == &Self::UnknownError {
            crate::ClassifiedError::Transient(self)
        } else {
            crate::ClassifiedError::Permanent(self)
        }
    }
}