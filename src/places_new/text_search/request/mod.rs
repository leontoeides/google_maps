#![allow(clippy::ref_option, reason = "for the getset crate")]

// Exports

mod ev_options;
pub use crate::places_new::text_search::request::ev_options::EvOptions;

// Imports

use crate::places_new::types::request::{LocationBias, LocationRestriction, RankPreference};
use crate::places_new::{FieldMask, PriceLevel};
use icu_locale::Locale;
use rust_iso3166::CountryCode;

// -------------------------------------------------------------------------------------------------
//
/// Request for the Google Maps Places API (New) Text Search service.
///
/// Used to search for places based on a text query string like "pizza in New York" or "shoe stores
/// near Ottawa". The API returns matching places with the data fields specified in the field mask.
/// At minimum, you must provide a text query and field mask to make a valid request.
///
/// # `Request` vs. `RequestWithClient`
///
/// * `Request` - Serializable, no client reference. For caching, storage, transmission.
/// * `RequestWithClient` - Contains client reference, executable. For immediate use.
///
/// You can convert between these types using `with_client()` or `into()`.
#[derive(
    //std
    Clone,
    Debug,
    // serde
    serde::Serialize,
    // getset
    getset::Getters,
    getset::CopyGetters,
    getset::MutGetters,
    getset::Setters,
    // other
    bon::Builder
)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// Fields to include in the response.
    ///
    /// Specifies which place data to return. This directly impacts API costs since different fields
    /// trigger different SKU charges. Use specific fields rather than `FieldMask::all()` to
    /// optimize costs.
    ///
    /// Field masking is a good design practice to ensure that you don't request unnecessary data,
    /// which helps to avoid unnecessary processing time and billing charges.
    ///
    /// While the `FieldMask::all()` is fine to use in development, Google discourages the use of
    /// the wildcard response field mask in production because of the large amount of data that can
    /// be returned.
    ///
    /// > ℹ️ Further guidance for using `places.iconMaskBaseUri` and `places.iconBackgroundColor`
    /// > can be found in [Place
    /// > Icons](https://developers.google.com/maps/documentation/places/web-service/icons) section.
    #[serde(skip)]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub field_mask: FieldMask,

    /// The text string to search for.
    ///
    /// Can be a place name ("Eiffel Tower"), category ("restaurants"), address ("123 Main St"), or
    /// combination with location ("pizza in New York"). The API ranks results by perceived
    /// relevance to this query.
    ///
    /// Text Search (New) is **not** intended for ambiguous queries, including the following:
    ///
    /// | Query type                                                                                                 | Example                                                       |
    /// |------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------|
    /// | Too many concepts or constraints, such as the names of multiple places, roads, or cities in a single query | "Market Street San Francisco San Jose Airport"                |
    /// | Postal address elements not represented on Google Maps                                                     | "C/O John Smith 123 Main Street", "P.O. Box 13 San Francisco" |
    /// | Names of businesses, chains, or categories combined with locations where these entities are not available  | "Tesco near Dallas, Texas"                                    |
    /// | Ambiguous queries with multiple interpretations                                                            | "Charger drop-off"                                            |
    /// | Historical names no longer in use                                                                          | "Middlesex United Kingdom"                                    |
    /// | Non-geospatial elements or intent                                                                          | "How many boats are in Ventura Harbor?"                       |
    /// | Unofficial or vanity names                                                                                 | "The Jenga", "The Helter Skelter"                             |
    /// | Latitude and longitude coordinates                                                                         | "37.422131,-122.084801"                                       |
    ///
    /// > ℹ️ For best results when searching on a phone number, include the country code followed by
    /// > a space, and set the `region` parameter to correspond to the country code. Phone number
    /// > formats vary by country and the API attempts to return a result for these different
    /// > formats.
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub text_query: String,

    /// Restrict results to places of this type from
    /// [Table A](https://developers.google.com/maps/documentation/places/web-service/place-types#table-a).
    ///
    /// Only one type may be specified. Type filtering applies automatically for categorical queries
    /// but may not apply to specific addresses. Set `strict_type_filtering` to `true` to force
    /// filtering on all queries.
    ///
    /// > ℹ️ This parameter does not apply to hotel queries or geopolitical queries (for example,
    /// > queries related to administrative areas, localities, postal codes, school districts, or
    /// > countries).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub included_type: Option<crate::places_new::PlaceType>,

    /// Include businesses that visit/deliver but have no physical location.
    ///
    /// When `true`, includes service area businesses like mobile cleaners or food trucks in
    /// results. When `false` or unset, only returns businesses with physical addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub include_pure_service_area_businesses: Option<bool>,

    /// Language for results.
    ///
    /// Uses a BCP-47 language code. Defaults to `en` if not specified. Street addresses appear in
    /// the local language (transliterated if needed), while other text uses this preferred language
    /// when available.
    ///
    /// # Notes
    ///
    /// * See the [list of supported languages](https://developers.google.com/maps/faq#languagesupport).
    ///   Google often updates the supported languages, so this list may not be exhaustive.
    ///
    /// * If `language` is not supplied, the API defaults to `en`. If you specify an invalid
    ///   language code, the API returns an `INVALID_ARGUMENT` error.
    ///
    /// * The API does its best to provide a street address that is readable for both the user and
    ///   locals. To achieve that goal, it returns street addresses in the local language,
    ///   transliterated to a script readable by the user if necessary, observing the preferred
    ///   language. All other addresses are returned in the preferred language. Address components
    ///   are all returned in the same language, which is chosen from the first component.
    ///
    /// * If a name is not available in the preferred language, the API uses the closest match.
    ///
    /// * The preferred language has a small influence on the set of results that the API chooses to
    ///   return, and the order in which they are returned. The geocoder interprets abbreviations
    ///   differently depending on language, such as the abbreviations for street types, or synonyms
    ///   that may be valid in one language but not in another.
    #[serde(
        rename = "languageCode",
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::places_new::serde::serialize_optional_locale",
        deserialize_with = "crate::places_new::serde::deserialize_optional_locale"
    )]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub language: Option<Locale>,

    /// Biases search results toward a specified geographic region.
    ///
    /// Location bias influences the ranking of results to favor places within or near the specified
    /// area, but does not exclude results outside that area.
    ///
    /// Use this when you want to prioritize local results while still allowing relevant distant
    /// results to appear.
    ///
    /// > ℹ️ If you omit both `location_bias` and `location_restriction`, then the API uses IP
    /// > biasing by default. With IP biasing, the API uses the device's IP address to bias the
    /// > results.
    ///
    /// > ℹ️ The `location_bias` parameter can be overridden if the `text_query` contains an
    /// > explicit location such as `Market in Barcelona`. In this case, `location_bias` is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub location_bias: Option<LocationBias>,

    /// Restrict search results to a specified geographic region.
    ///
    /// Location restriction hard-limits results to only include places within the specified area,
    /// completely excluding results outside that area.
    ///
    /// Use this when you need strict geographic boundaries, such as limiting searches to a specific
    /// city or service area.
    ///
    /// > ℹ️ If you omit both `location_bias` and `location_restriction`, then the API uses IP
    /// > biasing by default. With IP biasing, the API uses the device's IP address to bias the
    /// > results.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub location_restriction: Option<LocationRestriction>,

    /// EV charging requirements for filtering results.
    ///
    /// Filters places by available EV charging connector types and minimum charging rates. Only
    /// places meeting the criteria are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub ev_options: Option<EvOptions>,

    /// Minimum average rating (0.0-5.0).
    ///
    /// Only returns places with ratings greater than or equal to this value. Values should be in
    /// 0.5 increments (0, 0.5, 1.0, etc.) and are rounded up to the nearest 0.5 internally.
    ///
    /// > ℹ️ This parameter does not apply to hotel queries or geopolitical queries (for example,
    /// > queries related to administrative areas, localities, postal codes, school districts, or
    /// > countries).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub min_rating: Option<f64>,

    /// Only return places currently open.
    ///
    /// * When `true`, filters to places open at query time.
    ///
    /// * When `false` or unset, returns all places regardless of open status. Places without hours
    ///   data are included when this is `false`.
    ///
    /// > ℹ️ This parameter does not apply to hotel queries or geopolitical queries (for example,
    /// > queries related to administrative areas, localities, postal codes, school districts, or
    /// > countries).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub open_now: Option<bool>,

    /// Number of results per page (1-20).
    ///
    /// Limits results returned in a single response. Use `page_token` with the `nextPageToken` from
    /// responses to retrieve additional pages.
    ///
    /// > ℹ️ If `page_size` is 0 or unspecified, the API will return 20 results per page by default.
    /// > If `page_size` is greater than 20, the API will return no more than 20 results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub page_size: Option<i32>,

    /// Token for accessing the next page of results.
    ///
    /// Obtained from the `nextPageToken` field in a previous response. Pass this to retrieve the
    /// next page when results span multiple pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub page_token: Option<String>,

    /// Filter by price levels.
    ///
    /// Only returns places at the specified price levels.
    ///
    /// > ℹ️ Only applies to place types that support pricing:
    /// > [Food and Drink](https://developers.google.com/maps/documentation/places/web-service/place-types#food-and-drink),
    /// > [Services](https://developers.google.com/maps/documentation/places/web-service/place-types#services),
    /// > and [Shopping](https://developers.google.com/maps/documentation/places/web-service/place-types#shopping).
    /// > Non-supported types are excluded from results when this filter is set.
    ///
    /// > ℹ️ This parameter does not apply to hotel queries or geopolitical queries (for example,
    /// > queries related to administrative areas, localities, postal codes, school districts, or
    /// > countries).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub price_levels: Option<Vec<PriceLevel>>,

    /// How to rank results in the response.
    ///
    /// Specifies the ranking method for results:
    ///
    /// * `Popularity` (default) - Ranks results based on their popularity.
    ///
    /// * `Distance` - Ranks results in ascending order by their distance from the specified
    ///   location.
    ///
    /// If this parameter is omitted, results are ranked by popularity.
    ///
    /// > ℹ️ This parameter does not apply to hotel queries or geopolitical queries (for example,
    /// > queries related to administrative areas, localities, postal codes, school districts, or
    /// > countries).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub rank_preference: Option<RankPreference>,

    /// Region for formatting the response.
    ///
    /// Affects how addresses are formatted (e.g., country code omitted if it matches) and can bias
    /// results based on applicable law.
    ///
    /// The region used to format the response, specified as a [two-character CLDR
    /// code](https://www.unicode.org/cldr/charts/latest/supplemental/territory_language_information.html)
    /// value. This parameter can also have a bias effect on the search results. There is no default
    /// value.
    ///
    /// If the country name of the `formatted_address` field in the response matches the `region`,
    /// the country code is omitted from `formatted_address`. This parameter has no effect on
    /// `adr_format_address`, which always includes the country name when available, or on
    /// `short_formatted_address`, which never includes it.
    ///
    /// Most CLDR codes are identical to ISO 3166-1 codes, with some notable exceptions. For
    /// example, the United Kingdom's ccTLD is "uk" (.co.uk) while its ISO 3166-1 code is "gb"
    /// (technically for the entity of "The United Kingdom of Great Britain and Northern Ireland").
    /// The parameter can affect results based on applicable law.
    #[serde(
        rename = "regionCode",
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::places_new::serde::serialize_optional_country_code",
        deserialize_with = "crate::places_new::serde::deserialize_optional_country_code"
    )]
    pub region: Option<CountryCode>,

    /// Enforce strict type filtering.
    ///
    /// * When `true` with `included_type` set, only returns places exactly matching the specified
    ///   type.
    ///
    /// * When `false` (default), may return places not matching the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub strict_type_filtering: Option<bool>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Request {
    /// Creates a new text search request with the minimum required fields.
    ///
    /// You must provide a text query (what to search for) and a field mask (which data fields to
    /// return). Additional parameters can be set using the builder methods or by directly modifying
    /// the struct fields.
    pub fn new(text_query: impl Into<String>, field_mask: impl Into<FieldMask>) -> Self {
        Self {
            text_query: text_query.into(),
            field_mask: field_mask.into(),
            included_type: None,
            include_pure_service_area_businesses: None,
            language: None,
            location_bias: None,
            location_restriction: None,
            ev_options: None,
            min_rating: None,
            open_now: None,
            page_size: None,
            page_token: None,
            price_levels: None,
            rank_preference: None,
            region: None,
            strict_type_filtering: None,
        }
    }

    /// Upgrades to an executable request by adding a client reference.
    ///
    /// Combines this serializable request with a client reference, creating a `RequestWithClient`
    /// that can be executed.
    ///
    /// Use this to resume a stored or deserialized request.
    #[cfg(feature = "reqwest")]
    #[must_use]
    pub fn with_client(
        self,
        client: &crate::Client
    ) -> crate::places_new::text_search::RequestWithClient<'_> {
        crate::places_new::text_search::RequestWithClient {
            client,
            text_query: self.text_query,
            field_mask: self.field_mask,
            included_type: self.included_type,
            include_pure_service_area_businesses: self.include_pure_service_area_businesses,
            language: self.language,
            location_bias: self.location_bias,
            location_restriction: self.location_restriction,
            ev_options: self.ev_options,
            min_rating: self.min_rating,
            open_now: self.open_now,
            page_size: self.page_size,
            page_token: self.page_token,
            price_levels: self.price_levels,
            rank_preference: self.rank_preference,
            region: self.region,
            strict_type_filtering: self.strict_type_filtering,
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

#[cfg(feature = "reqwest")]
impl std::convert::From<crate::places_new::text_search::RequestWithClient<'_>> for Request {
    /// Converts a `RequestWithClient` to a serializable `Request`.
    ///
    /// Strips the client reference and returns only the query parameters, creating a fully
    /// serializable request that can be stored or transmitted.
    fn from(request: crate::places_new::text_search::RequestWithClient) -> Self {
        Self {
            text_query: request.text_query,
            field_mask: request.field_mask,
            included_type: request.included_type,
            include_pure_service_area_businesses: request.include_pure_service_area_businesses,
            language: request.language,
            location_bias: request.location_bias,
            location_restriction: request.location_restriction,
            ev_options: request.ev_options,
            min_rating: request.min_rating,
            open_now: request.open_now,
            page_size: request.page_size,
            page_token: request.page_token,
            price_levels: request.price_levels,
            rank_preference: request.rank_preference,
            region: request.region,
            strict_type_filtering: request.strict_type_filtering,
        }
    }
}