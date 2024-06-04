//! **Look in this module for documentation on building your _Querys API_
//! _Query Autocomplete_ query**. In particular, look at the _Request_ struct
//! for examples of the builder pattern. This module contains the tools (enums,
//! structs, methods) for building your Google Maps Platform request.

mod build;
#[cfg(feature = "enable-reqwest")]
mod execute;
#[cfg(feature = "enable-reqwest")]
mod get;
mod new;
mod query_url;
mod with_language;
mod with_location;
mod with_offset;

// -----------------------------------------------------------------------------

use crate::{client::GoogleMapsClient, types::Language, types::LatLng};

// -----------------------------------------------------------------------------
//
/// **Look at this `Request` struct for documentation on how to build your
/// _Query Autocomplete_ query**. The methods implemented for this struct are
/// what's used to build your request.

#[derive(Debug)]
pub struct Request<'a> {
    // Required parameters:
    // --------------------
    /// This structure contains the application's API key and other
    /// user-definable settings such as "maximum retries."
    client: &'a GoogleMapsClient,

    /// The text string on which to search. The Query Autocomplete service will
    /// return candidate matches based on this string and order results based on
    /// their perceived relevance.
    input: String,

    // Optional parameters:
    // --------------------
    /// The language in which to return results.
    ///
    /// * See the list of supported languages. Google often updates the
    ///   supported languages, so this list may not be exhaustive.
    ///
    /// * If `language` is not supplied, the API attempts to use the preferred
    ///   language as specified in the `Accept-Language` header.
    ///
    /// * The API does its best to provide a street address that is readable for
    ///   both the user and locals. To achieve that goal, it returns street
    ///   addresses in the local language, transliterated to a script readable
    ///   by the user if necessary, observing the preferred language. All other
    ///   addresses are returned in the preferred language. Address components
    ///   are all returned in the same language, which is chosen from the first
    ///   component.
    ///
    /// * If a name is not available in the preferred language, the API uses the
    ///   closest match.
    ///
    /// * The preferred language has a small influence on the set of results
    ///   that the API chooses to return, and the order in which they are
    ///   returned. The geocoder interprets abbreviations differently depending
    ///   on language, such as the abbreviations for street types, or synonyms
    ///   that may be valid in one language but not in another. For example,
    ///   _utca_ and _tér_ are synonyms for street in Hungarian.
    language: Option<Language>,

    /// The point around which to retrieve place information.
    ///
    /// * When using the Text Search API, the 'location' parameter may be
    ///   overriden if the 'query' contains an explicit location such as
    ///   'Market in Barcelona'.
    location: Option<LatLng>,

    /// The position, in the input term, of the last character that the service
    /// uses to match predictions. For example, if the input is `Google` and the
    /// offset is 3, the service will match on `Goo`. The string determined by
    /// the offset is matched against the first word in the input term only.
    /// For example, if the input term is `Google abc` and the offset is 3, the
    /// service will attempt to match against `Goo abc`. If no offset is
    /// supplied, the service will use the whole term. The offset should
    /// generally be set to the position of the text caret.
    offset: Option<u8>,

    /// Defines the distance (in meters) within which to return place results.
    /// You may bias results to a specified circle by passing a location and a
    /// radius parameter. Doing so instructs the Places service to prefer
    /// showing results within that circle; results outside of the defined area
    /// may still be displayed.
    ///
    /// The radius will automatically be clamped to a maximum value depending on
    /// the type of search and other parameters.
    ///
    /// * Autocomplete: 50,000 meters
    /// * Nearby Search:
    ///     * with `keyword` or `name`: 50,000 meters
    ///     * without `keyword` or `name`
    ///         * Up to 50,000 meters, adjusted dynamically based on area
    ///           density, independent of `rankby` parameter.
    ///         * When using `rankby=distance`, the radius parameter will not be
    ///           accepted, and will result in an `INVALID_REQUEST`.
    /// * Query Autocomplete: 50,000 meters
    /// * Text Search: 50,000 meters
    radius: Option<u32>,

    // Internal use only:
    // ------------------
    /// Query string that is to be submitted to the Google Cloud Maps Platform.
    query: Option<String>,
} // struct
