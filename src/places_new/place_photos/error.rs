#![allow(unused_assignments, reason = "these unused fields are useful in debug printing")]

use miette::SourceSpan;

// -------------------------------------------------------------------------------------------------
//
/// Errors that can occur when using the Google Maps Places API (New) Place Photos service.
///
/// Provides detailed diagnostic information for debugging API client issues and server errors.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error, miette::Diagnostic)]
#[non_exhaustive]
pub enum Error {
    /// Place has no photos available.
    ///
    /// Attempted to create a photo request from a place that has no photos in its photos array.
    /// This typically happens when the place exists but hasn't had any photos contributed, or when
    /// the photos field wasn't included in the field mask when fetching the place.
    #[error("Place has no photos available")]
    #[diagnostic(
        code(google_maps::places::place_photos::no_photos),
        help("Ensure the place has photos and that 'photos' was included in the field mask")
    )]
    PlaceHasNoPhotos {
        /// The ID of the place that has no photos.
        #[source_code]
        place_id: Option<String>,

        #[label("This place has no associated photos")]
        span: SourceSpan,
    },

    /// Failed to download photo from URI.
    ///
    /// The HTTP request to fetch the image bytes failed, either due to network issues, the URI
    /// expiring, rate limiting, or server errors. The photo URI may be expired or rate-limited.
    #[error("Failed to download photo from {url}")]
    #[diagnostic(
        code(google_maps::places::place_photos::download_failed),
        help("The photo URI may have expired. Try fetching a fresh photo URI from a place query.")
    )]
    PhotoDownloadFailed {
        /// The HTTP status code returned, if any.
        status: u16,

        /// The URL that failed to download.
        #[source_code]
        url: String,

        #[label("HTTP request to this URI failed")]
        span: SourceSpan,
    },

    /// Missing photo dimension constraints.
    ///
    /// The photo request must specify at least one dimension constraint (`max_width_px` or
    /// `max_height_px`). This error occurs when both are omitted, making it impossible to determine
    /// what size image to fetch.
    #[error("Photo request missing required dimension constraints")]
    #[diagnostic(
        code(google_maps::places::place_photos::missing_dimensions),
        help("Specify at least one of max_width_px or max_height_px (range: 1-4800 pixels)")
    )]
    MissingPhotoDimensions {
        /// Debug representation showing the request state.
        #[source_code]
        debug: String,

        /// Span highlighting the missing parameters.
        #[label("Neither max_width_px nor max_height_px was specified")]
        span: SourceSpan,
    },

    /// Invalid photo width constraint.
    ///
    /// The `max_width_px` parameter must be between 1 and 4800 pixels (inclusive). Values outside
    /// this range are rejected by the Google Maps API.
    #[error("Invalid photo width constraint: {width} pixels")]
    #[diagnostic(
        code(google_maps::places::place_photos::invalid_width),
        help("max_width_px must be between 1 and 4800 pixels (inclusive)")
    )]
    InvalidPhotoWidth {
        /// The invalid width value that was provided.
        width: u32,

        /// Debug representation showing the invalid parameter.
        #[source_code]
        debug: String,

        /// Span highlighting the invalid width value.
        #[label("This width is outside the valid range of 1-4800")]
        span: SourceSpan,
    },

    /// Invalid photo height constraint.
    ///
    /// The `max_height_px` parameter must be between 1 and 4800 pixels (inclusive). Values outside
    /// this range are rejected by the Google Maps API.
    #[error("Invalid photo height constraint: {height} pixels")]
    #[diagnostic(
        code(google_maps::places::place_photos::invalid_height),
        help("max_height_px must be between 1 and 4800 pixels (inclusive)")
    )]
    InvalidPhotoHeight {
        /// The invalid height value that was provided.
        height: u32,

        /// Debug representation showing the invalid parameter.
        #[source_code]
        debug: String,

        /// Span highlighting the invalid height value.
        #[label("This height is outside the valid range of 1-4800")]
        span: SourceSpan,
    },

    /// Failed to decode image bytes.
    ///
    /// The image data could not be decoded, likely due to corrupted data, unsupported format, or
    /// incomplete download. This can occur when trying to convert image bytes to ASCII art or when
    /// processing the image data.
    #[error("Failed to decode image")]
    #[diagnostic(
        code(google_maps::places::place_photos::image_decode_failed),
        help("Ensure the image was downloaded completely and is in a supported format (JPEG, PNG, WebP)")
    )]
    ImageDecodeError {
        /// Description of the decode failure.
        message: String,
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
    /// If a “Places API (New) Text Search” error is permenant, retrying the same request will not
    /// change the outcome. Transient errors are typically from network or disk related.
    fn classify(&self) -> crate::ClassifiedError<'_, Self> {
        crate::ClassifiedError::Permanent(self)
    }
}