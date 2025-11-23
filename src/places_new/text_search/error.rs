// -------------------------------------------------------------------------------------------------
//
/// Errors that can occur when using the Google Maps Places API (New) Text Search service.
///
/// Provides detailed diagnostic information for debugging API client issues and server errors.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error, miette::Diagnostic)]
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
            "use either \"location_bias\" OR \"location_restriction\", not both. \
            Bias influences ranking while restriction hard-limits results"
        )
    )]
    #[error("cannot set both location bias and location restriction")]
    MutuallyExclusiveLocationFields {
        #[source_code]
        debug: String,
        #[label("both fields were set, but only one is allowed")]
        span: (usize, usize),
    },

    /// A Table B place type was used for filtering.
    ///
    /// Occurs when attempting to filter results using a place type from Table B. The Google Maps
    /// API only accepts Table A place types for the `includedType` parameter. Table B types are
    /// only returned in responses and cannot be used as filters.
    #[diagnostic(
        code(google_maps::places_new::invalid_place_type_for_filter),
        severity(Error),
        help(
            "Use a Table A place type for filtering. Table B types like \"geocode\" or \
            \"political\" can only appear in responses, not in filters. See \
            https://developers.google.com/maps/documentation/places/web-service/place-types"
        )
    )]
    #[error("place type {place_type:?} is from Table B and cannot be used for filtering")]
    InvalidPlaceTypeForFilter {
        /// The place type that was incorrectly used for filtering.
        place_type: String,
        #[source_code]
        debug: String,
        #[label("Table B place type cannot be used in includedType parameter")]
        span: (usize, usize),
    },

    /// Field mask was empty.
    ///
    /// Occurs when attempting to make an API request with an empty field mask. The Google Maps API
    /// requires at least one field to be specified in the response field mask, otherwise it returns
    /// an error.
    #[diagnostic(
        code(google_maps::places_new::empty_field_mask),
        severity(Error),
        help(
            "Specify at least one field in the field mask. Use FieldMask::All for all fields, \
            or FieldMask::Specific with fields like Field::PlacesDisplayName, \
            Field::PlacesLocation, etc."
        )
    )]
    #[error("field mask cannot be empty, specify at least one field to return")]
    EmptyFieldMask {
        #[source_code]
        debug: String,
        #[label("field mask was specified but contains no fields")]
        span: (usize, usize),
    },

    /// Attempted to fetch the next page when no next page token was available.
    ///
    /// Occurs when calling `next_text_search()` on a response that represents the last page of
    /// results. Check `response.next_page_token.is_some()` before requesting the next page.
    #[diagnostic(
        code(google_maps::places_new::no_next_page_available),
        severity(Error),
        help(
            "check if response.next_page_token.is_some() before calling next_text_search(). \
            The previous response was the last page of results"
        )
    )]
    #[error("no next page available - the previous response was the last page of results")]
    NoNextPageAvailable {
        #[source_code]
        debug: String,
        #[label("attempted to get next page but no token was available")]
        span: (usize, usize),
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