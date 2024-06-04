//! **Look in this module for documentation on building your _Places API_
//! _Text Search_ query**. In particular, look at the _Request_ struct
//! for examples of the builder pattern. This module contains the tools (enums,
//! structs, methods) for building your Google Maps Platform request.

mod build;
#[cfg(feature = "enable-reqwest")]
mod execute;
#[cfg(feature = "enable-reqwest")]
mod get;
mod is_open_now;
mod new;
mod query_url;
mod with_language;
mod with_location;
mod with_max_price;
mod with_min_price;
mod with_pagetoken;
mod with_region;
mod with_type;

// -----------------------------------------------------------------------------

use crate::{
    client::GoogleMapsClient, types::Language, types::LatLng, types::PlaceType, types::Region,
};

// -----------------------------------------------------------------------------

/// **Look at this `Request` struct for documentation on how to build your
/// _Text Search_ query**. The methods implemented for this struct are
/// what's used to build your request.

#[derive(Debug)]
pub struct Request<'a> {
    // Required parameters:
    // --------------------
    /// This structure contains the application's API key and other
    /// user-definable settings such as "maximum retries."
    client: &'a GoogleMapsClient,

    /// The text string on which to search, for example: "restaurant" or "123
    /// Main Street". This must a place name, address, or category of
    /// establishments. Any other types of input can generate errors and are not
    /// guaranteed to return valid results. The Google Places service will
    /// return candidate matches based on this string and order the results
    /// based on their perceived relevance.
    input: String,

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
    radius: u32,

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

    /// Restricts results to only those places within the specified range. Valid
    /// values range between 0 (most affordable) to 4 (most expensive),
    /// inclusive. The exact amount indicated by a specific value will vary from
    /// region to region.
    maxprice: Option<u8>,

    /// Restricts results to only those places within the specified range. Valid
    /// values range between 0 (most affordable) to 4 (most expensive),
    /// inclusive. The exact amount indicated by a specific value will vary from
    /// region to region.
    minprice: Option<u8>,

    /// Returns only those places that are open for business at the time the
    /// query is sent. Places that do not specify opening hours in the Google
    /// Places database will not be returned if you include this parameter in
    /// your query.
    opennow: Option<bool>,

    /// Returns up to 20 results from a previously run search. Setting a
    /// pagetoken parameter will execute a search with the same parameters used
    /// previously — all parameters other than pagetoken will be ignored.
    pagetoken: Option<String>,

    /// The region code, specified as a [ccTLD ("top-level
    /// domain")](https://en.wikipedia.org/wiki/List_of_Internet_top-level_domains#Country_code_top-level_domains)
    /// two-character value. Most ccTLD codes are identical to ISO 3166-1 codes,
    /// with some notable exceptions. For example, the United Kingdom's ccTLD is
    /// "uk" (.co.uk) while its ISO 3166-1 code is "gb" (technically for the
    /// entity of "The United Kingdom of Great Britain and Northern Ireland").
    region: Option<Region>,

    /// Restricts the results to places matching the specified type. Only one
    /// type may be specified. If more than one type is provided, all types
    /// following the first entry are ignored.
    ///
    /// `type=hospital|pharmacy|doctor` becomes `type=hospital`
    /// `type=hospital,pharmacy,doctor` is ignored entirely
    ///
    /// See the list of [supported types](https://developers.google.com/maps/documentation/places/web-service/supported_types).
    ///
    /// * Note: Adding both `keyword` and `type` with the same value
    ///   (`keyword=cafe&type=cafe` or `keyword=parking&type=parking`) can yield
    ///   `ZERO_RESULTS`.
    place_type: Option<PlaceType>,

    // Internal use only:
    // ------------------
    /// Query string that is to be submitted to the Google Cloud Maps Platform.
    query: Option<String>,
} // struct
