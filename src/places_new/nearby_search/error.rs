#![allow(unused_assignments, reason = "these unused fields are useful in debug printing")]

use miette::SourceSpan;

// -------------------------------------------------------------------------------------------------
//
/// Errors that can occur when using the Google Maps Places API (New) Text Search service.
///
/// Provides detailed diagnostic information for debugging API client issues and server errors.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error, miette::Diagnostic)]
#[non_exhaustive]
pub enum Error {
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
        span: SourceSpan,
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
        span: SourceSpan,
    },

    /// Place type appears in both inclusion and exclusion lists.
    ///
    /// A place type cannot appear in both `included_types` and `excluded_types`. This creates a
    /// logical contradiction - you cannot both include and exclude the same type. Remove the type
    /// from one of the lists.
    #[error("Place type '{place_type}' appears in both included_types and excluded_types")]
    #[diagnostic(
        code(google_maps::nearby_search::conflicting_place_types),
        help("Remove '{place_type}' from either included_types or excluded_types")
    )]
    ConflictingPlaceTypes {
        /// The conflicting place type.
        place_type: String,
        #[source_code]
        debug: String,
        #[label("type appears in both lists")]
        span: SourceSpan,
    },

    /// Primary place type appears in both inclusion and exclusion lists.
    ///
    /// A place type cannot appear in both `included_primary_types` and `excluded_primary_types`.
    /// This creates a logical contradiction - you cannot both include and exclude the same primary
    /// type. Remove the type from one of the lists.
    #[error("Place type '{place_type}' appears in both included_primary_types and excluded_primary_types")]
    #[diagnostic(
        code(google_maps::nearby_search::conflicting_primary_place_types),
        help("Remove '{place_type}' from either included_primary_types or excluded_primary_types")
    )]
    ConflictingPrimaryPlaceTypes {
        /// The conflicting primary place type.
        place_type: String,
        #[source_code]
        debug: String,
        #[label("primary type appears in both lists")]
        span: SourceSpan,
    },

    /// Nearby Search does not support this location restriction type.
    ///
    /// The Nearby Search API only accepts circular location restrictions. Rectangle/viewport
    /// restrictions are not supported by this endpoint.
    #[error("Nearby Search only supports circular location restrictions")]
    UnsupportedLocationRestriction {
        /// The location restriction type that was provided.
        restriction_type: String,

        /// Debug representation for diagnostics.
        #[source_code]
        debug: String,

        /// Span highlighting the problematic value.
        #[label("this restriction type is not supported")]
        span: SourceSpan,
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