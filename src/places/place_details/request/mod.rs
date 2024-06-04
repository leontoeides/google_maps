//! **Look in this module for documentation on building your _Places API_
//! _Place Autocomplete_ query**. In particular, look at the _Request_ struct
//! for examples of the builder pattern. This module contains the tools (enums,
//! structs, methods) for building your Google Maps Platform request.

mod build;
#[cfg(feature = "enable-reqwest")]
mod execute;
#[cfg(feature = "enable-reqwest")]
mod get;
mod new;
mod query_url;
mod with_fields;
mod with_language;
mod with_no_review_translations;
mod with_region;
mod with_reviews_sort;
mod with_sessiontoken;

// -----------------------------------------------------------------------------

use crate::places::place_details::{Field, SortOrder};
use crate::{client::GoogleMapsClient, types::Language, types::Region};

// -----------------------------------------------------------------------------

/// **Look at this `Request` struct for documentation on how to build your
/// _Place Autocomplete_ query**. The methods implemented for this struct are
/// what's used to build your request.

#[derive(Debug)]
pub struct Request<'a> {
    // Required parameters:
    // --------------------
    /// This structure contains the application's API key and other
    /// user-definable settings such as "maximum retries."
    client: &'a GoogleMapsClient,

    /// A textual identifier that uniquely identifies a place, returned from a
    /// [Place Search](https://developers.google.com/maps/documentation/places/web-service/search).
    /// For more information about place IDs, see the
    /// [place ID overview](https://developers.google.com/maps/documentation/places/web-service/place-id).
    place_id: String,

    // Optional parameters:
    // --------------------
    /// Use the fields parameter to specify a comma-separated list of place data
    /// types to return. For example: `fields=formatted_address,name,geometry`.
    /// Use a forward slash when specifying compound values. For example:
    /// `opening_hours/open_now`.
    ///
    /// Fields are divided into three billing categories: Basic, Contact, and
    /// Atmosphere. Basic fields are billed at base rate, and incur no
    /// additional charges. Contact and Atmosphere fields are billed at a higher
    /// rate. See the [pricing sheet](https://cloud.google.com/maps-platform/pricing/sheet/)
    /// for more information. Attributions, `html_attributions`, are always
    /// returned with every call, regardless of whether the field has been
    /// requested.
    ///
    /// * Caution: Place Search requests and Place Details requests do not
    ///   return the same fields. Place Search requests return a subset of the
    ///   fields that are returned by Place Details requests. If the field you
    ///   want is not returned by Place Search, you can use Place Search to get
    ///   a `place_id`, then use that Place ID to make a Place Details request.
    ///   For more information on the fields that are unavailable in a Place
    ///   Search request, see
    ///   [Places API fields support](https://developers.google.com/maps/documentation/places/web-service/place-data-fields#places-api-fields-support).
    fields: Vec<Field>,

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

    /// The region code, specified as a [ccTLD ("top-level
    /// domain")](https://en.wikipedia.org/wiki/List_of_Internet_top-level_domains#Country_code_top-level_domains)
    /// two-character value. Most ccTLD codes are identical to ISO 3166-1 codes,
    /// with some notable exceptions. For example, the United Kingdom's ccTLD is
    /// "uk" (.co.uk) while its ISO 3166-1 code is "gb" (technically for the
    /// entity of "The United Kingdom of Great Britain and Northern Ireland").
    region: Option<Region>,

    /// Specify `reviews_no_translations=true` to disable translation of
    /// reviews; specify `reviews_no_translations=false` to enable translation
    /// of reviews. Reviews are returned in their original language.
    ///
    /// If omitted, or passed with no value, translation of reviews is enabled.
    /// If the `language` parameter was specified in the request, use the
    /// specified language as the preferred language for translation. If
    /// `language` is omitted, the API attempts to use the `Accept-Language`
    /// header as the preferred language.
    reviews_no_translations: Option<bool>,

    /// The sorting method to use when returning reviews. Can be set to
    /// `most_relevant` (default) or newest.
    ///
    /// * For `most_relevant` (default), reviews are sorted by relevance; the
    ///   service will bias the results to return reviews originally written in
    ///   the preferred language.
    ///
    /// * For `newest`, reviews are sorted in chronological order; the preferred
    ///   language does not affect the sort order.
    ///
    /// Google recommends that you display how the reviews are being sorted to
    /// the end user.
    reviews_sort: Option<SortOrder>,

    /// A random string which identifies an autocomplete
    /// [session](https://developers.google.com/maps/documentation/places/web-service/details#session_tokens)
    /// for billing purposes.
    ///
    /// The session begins when the user starts typing a query, and concludes
    /// when they select a place and a call to Place Details is made. Each
    /// session can have multiple queries, followed by one place selection. The
    /// API key(s) used for each request within a session must belong to the
    /// same Google Cloud Console project. Once a session has concluded, the
    /// token is no longer valid; your app must generate a fresh token for each
    /// session. If the `sessiontoken` parameter is omitted, or if you reuse a
    /// session token, the session is charged as if no session token was
    /// provided (each request is billed separately).
    ///
    /// We recommend the following guidelines:
    ///
    /// * Use session tokens for all autocomplete sessions.
    ///
    /// * Generate a fresh token for each session. Using a version 4 UUID is
    ///   recommended.
    ///
    /// * Ensure that the API key(s) used for all Place Autocomplete and Place
    ///   Details requests within a session belong to the same Cloud Console
    ///   project.
    ///
    /// * Be sure to pass a unique session token for each new session. Using the
    ///   same token for more than one session will result in each request being
    ///   billed individually.
    sessiontoken: Option<String>,

    // Internal use only:
    // ------------------
    /// Query string that is to be submitted to the Google Cloud Maps Platform.
    query: Option<String>,
} // struct
