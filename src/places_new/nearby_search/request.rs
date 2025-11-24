#![allow(clippy::ref_option, reason = "for the getset crate")]

use crate::places_new::FieldMask;
use crate::places_new::types::request::{LocationRestriction, PlaceTypeSet, RankPreference};
use icu_locale::Locale;
use rust_iso3166::CountryCode;

// -------------------------------------------------------------------------------------------------
//
/// Request for the Google Maps Places API (New) Nearby Search service.
///
/// Used to search for places within a specified geographic area, optionally filtered by place
/// type.
///
/// Unlike Text Search which uses text queries, Nearby Search focuses purely on location and
/// type-based discovery. Results can be ranked by distance or popularity. At minimum, you must
/// provide a location restriction and field mask to make a valid request.
///
/// # Understanding Type Filtering
///
/// Nearby Search offers two ways to filter by place type:
///
/// * **`included_types` / `excluded_types`** - Filter on ANY type in a place's type list (broad
///   search). A hotel with a restaurant inside would match `included_types: ["restaurant"]`.
///
/// * **`included_primary_types` / `excluded_primary_types`** - Filter on what the business
///   PRIMARILY is (precise search). That same hotel would NOT match `included_primary_types:
///   ["restaurant"]` because it's primarily a "lodging" business.
///
/// Use primary type filtering when you want businesses where this IS the main category, not just
/// an amenity.
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

    /// Restrict search results to a specified geographic region.
    ///
    /// Location restriction hard-limits results to only include places within the specified area,
    /// completely excluding results outside that area.
    ///
    /// Use this when you need strict geographic boundaries, such as limiting searches to a specific
    /// city or service area.
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub location_restriction: LocationRestriction,

    /// Place types to include in results from
    /// [Table A](https://developers.google.com/maps/documentation/places/web-service/place-types#table-a).
    ///
    /// Matches places that have any of the specified types **anywhere in their type list**. This
    /// casts a wide net since each place can have multiple types. Up to 50 types can be specified.
    ///
    /// Every place in Google's database has multiple types - for example, a seafood restaurant
    /// might have types `["seafood_restaurant", "restaurant", "food", "point_of_interest",
    /// "establishment"]`. This filter matches if **any** of those types appear in your list.
    ///
    /// # Example
    ///
    /// Searching with `included_types: ["restaurant"]` will match:
    /// - A seafood restaurant (has "restaurant" in its types)
    /// - A hotel with a restaurant inside (has "restaurant" in its types)
    /// - Any place tagged with "restaurant" regardless of what it primarily is
    ///
    /// If this parameter is omitted, places of all types are returned.
    ///
    /// > ℹ️ If both `included_types` and `included_primary_types` are set, results must satisfy at
    /// > least one condition from each list.
    #[serde(default, skip_serializing_if = "PlaceTypeSet::is_empty")]
    #[builder(default, into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub included_types: PlaceTypeSet,

    /// Place types to exclude from results from
    /// [Table A](https://developers.google.com/maps/documentation/places/web-service/place-types#table-a).
    ///
    /// Filters out places that have any of the specified types **anywhere in their type list**. Up
    /// to 50 types can be specified. A place is excluded if it has any type in this list.
    ///
    /// # Example
    ///
    /// `{"includedTypes": ["school"], "excludedTypes": ["primary_school"]}` returns places with
    /// "school" in their types but excludes any place that has "`primary_school`" in their types.
    ///
    /// > ⚠️ If a type appears in both `included_types` and `excluded_types`, the API returns an
    /// > `INVALID_REQUEST` error.
    #[serde(default, skip_serializing_if = "PlaceTypeSet::is_empty")]
    #[builder(default, into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub excluded_types: PlaceTypeSet,

    /// Primary place types to include in results from
    /// [Table A](https://developers.google.com/maps/documentation/places/web-service/place-types#table-a).
    ///
    /// Matches places where **the business's primary category** is one of the specified types.
    /// This is a more precise filter than `included_types` since each place has exactly one
    /// primary type that represents what the business mainly is. Up to 50 types can be specified.
    ///
    /// Every place in Google's database is classified with one primary type. For example, a
    /// seafood restaurant has `primary_type: "seafood_restaurant"` even though it also has types
    /// like `["seafood_restaurant", "restaurant", "food", ...]`. A hotel with a restaurant inside
    /// has `primary_type: "lodging"` even though "restaurant" appears in its types list.
    ///
    /// # Example
    ///
    /// Searching with `included_primary_types: ["restaurant"]` will match:
    /// - Places primarily categorized as "restaurant"
    /// - May also match specialized subtypes like "`chinese_restaurant`" (Google treats general
    ///   types as matching their specializations)
    ///
    /// But will NOT match:
    /// - A hotel that happens to have a restaurant inside (its primary type is "lodging")
    ///
    /// # When to Use
    ///
    /// Use `included_primary_types` when you want places where this IS the main business, not just
    /// an amenity or secondary feature.
    ///
    /// > ℹ️ If both `included_types` and `included_primary_types` are set, results must satisfy at
    /// > least one condition from each list.
    #[serde(default, skip_serializing_if = "PlaceTypeSet::is_empty")]
    #[builder(default, into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub included_primary_types: PlaceTypeSet,

    /// Primary place types to exclude from results from
    /// [Table A](https://developers.google.com/maps/documentation/places/web-service/place-types#table-a).
    ///
    /// Filters out places where **the business's primary category** is one of the specified types.
    /// Up to 50 types can be specified.
    ///
    /// # Example
    ///
    /// `{"includedTypes": ["restaurant"], "excludedPrimaryTypes": ["steak_house"]}` returns places
    /// that offer restaurant services (has "restaurant" somewhere in their types list) but
    /// excludes places whose main business is a steak house (primary type is "`steak_house`").
    ///
    /// This would include:
    /// - Seafood restaurants that serve some steak dishes (primary type: "`seafood_restaurant`")
    /// - Italian restaurants (primary type: "`italian_restaurant`")
    ///
    /// But exclude:
    /// - Steakhouses (primary type: "`steak_house`")
    ///
    /// > ⚠️ If a type appears in both `included_primary_types` and `excluded_primary_types`, the
    /// > API returns an `INVALID_ARGUMENT` error.
    #[serde(default, skip_serializing_if = "PlaceTypeSet::is_empty")]
    #[builder(default, into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub excluded_primary_types: PlaceTypeSet,

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

    /// Maximum number of place results to return (1-20).
    ///
    /// Specifies the maximum number of places to return in the response. Must be between 1 and 20
    /// inclusive, with a default of 20 if not specified.
    ///
    /// > ℹ️ Unlike Text Search, Nearby Search does not support pagination. The maximum 20 results
    /// > is a hard limit per request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub max_result_count: Option<i32>,

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
    /// The region code used to format the response, specified as a [two-character CLDR
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
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Request {
    /// Creates a new nearby search request with the minimum required fields.
    ///
    /// You must provide a location restriction (where to search) and a field mask (which data
    /// fields to return). Additional parameters like type filters and ranking preferences can be
    /// set using the builder methods or by directly modifying the struct fields.
    pub fn new(
        location_restriction: impl Into<LocationRestriction>,
        field_mask: impl Into<FieldMask>,
    ) -> Self {
        Self {
            location_restriction: location_restriction.into(),
            field_mask: field_mask.into(),
            included_types: PlaceTypeSet::default(),
            excluded_types: PlaceTypeSet::default(),
            included_primary_types: PlaceTypeSet::default(),
            excluded_primary_types: PlaceTypeSet::default(),
            language: None,
            max_result_count: None,
            rank_preference: None,
            region: None,
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
    ) -> crate::places_new::nearby_search::RequestWithClient<'_> {
        crate::places_new::nearby_search::RequestWithClient {
            client,
            location_restriction: self.location_restriction,
            field_mask: self.field_mask,
            included_types: self.included_types,
            excluded_types: self.excluded_types,
            included_primary_types: self.included_primary_types,
            excluded_primary_types: self.excluded_primary_types,
            language: self.language,
            max_result_count: self.max_result_count,
            rank_preference: self.rank_preference,
            region: self.region,
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

#[cfg(feature = "reqwest")]
impl std::convert::From<crate::places_new::nearby_search::RequestWithClient<'_>> for Request {
    /// Converts a `RequestWithClient` to a serializable `Request`.
    ///
    /// Strips the client reference and returns only the query parameters, creating a fully
    /// serializable request that can be stored or transmitted.
    fn from(request: crate::places_new::nearby_search::RequestWithClient) -> Self {
        Self {
            location_restriction: request.location_restriction,
            field_mask: request.field_mask,
            included_types: request.included_types,
            excluded_types: request.excluded_types,
            included_primary_types: request.included_primary_types,
            excluded_primary_types: request.excluded_primary_types,
            language: request.language,
            max_result_count: request.max_result_count,
            rank_preference: request.rank_preference,
            region: request.region,
        }
    }
}