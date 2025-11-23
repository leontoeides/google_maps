#![allow(clippy::ref_option, reason = "for the getset crate")]

use crate::places_new::types::request::{LocationBias, LocationRestriction};
use crate::places_new::{LatLng, PlaceType};
use icu_locale::Locale;
use rust_iso3166::CountryCode;

// -------------------------------------------------------------------------------------------------
//
/// Response from the Google Maps Places API (New) Autocomplete service.
///
/// Autocomplete requests return predictions based on user input text and optional parameters
/// like location bias, restrictions, and language preferences. This is used to power
/// search-as-you-type functionality in applications.
///
/// Session tokens should be used to group queries into sessions for billing purposes.
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
    /// The text string on which to search.
    ///
    /// Required. The user's input text that will be matched against places and queries to generate
    /// predictions.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub input: String,

    /// Bias results to a specified location.
    ///
    /// Biases results toward the specified location without restricting them.
    ///
    /// At most one of `location_bias` or `location_restriction` should be set. If neither are set,
    /// the results will be biased by IP address, meaning the IP address will be mapped to an
    /// imprecise location and used as a biasing signal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub location_bias: Option<LocationBias>,

    /// Restrict results to a specified location.
    ///
    /// Restricts results to the specified location, excluding results outside.
    ///
    /// At most one of `location_bias` or `location_restriction` should be set. If neither are set,
    /// the results will be biased by IP address, meaning the IP address will be mapped to an
    /// imprecise location and used as a biasing signal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub location_restriction: Option<LocationRestriction>,

    /// Included primary Place types.
    ///
    /// Included primary Place type (for example, `restaurant` or `gas_station`) in [Place
    /// Types](https://developers.google.com/maps/documentation/places/web-service/place-types), or
    /// only (regions), or only (cities). A Place is only returned if its primary type is included
    /// in this list. Up to 5 values can be specified. If no types are specified, all Place types
    /// are returned.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub included_primary_types: Vec<PlaceType>,

    /// Only include results in the specified regions.
    ///
    /// Specified as up to 15 CLDR two-character region codes. An empty set will not restrict
    /// results. If both `location_restriction` and `included_regions` are set, results will be in
    /// the intersection of the two.
    #[serde(
        rename = "includedRegionCodes",
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "crate::places_new::serde::serialize_vec_country_code",
        deserialize_with = "crate::places_new::serde::deserialize_vec_country_code"
    )]
    #[builder(default)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub included_regions: Vec<CountryCode>,

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

    /// The origin point for calculating geodesic distance.
    ///
    /// The origin point from which to calculate geodesic distance to the destination (returned as
    /// `distance_meters`). If this value is omitted, geodesic distance will not be returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub origin: Option<LatLng>,

    /// Unicode character offset of input where user cursor is.
    ///
    /// A zero-based Unicode character offset of input indicating the cursor position in input. The
    /// cursor position may influence what predictions are returned.
    ///
    /// If empty, defaults to the length of input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub input_offset: Option<i32>,

    /// Whether to include query predictions in the response.
    ///
    /// If `true`, response includes both Place and query predictions. Otherwise only `Place`
    /// predictions are returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub include_query_predictions: Option<bool>,

    /// Session token for billing purposes.
    ///
    /// Session tokens are user-generated strings that track Autocomplete (New) calls as "sessions."
    /// Autocomplete (New) uses session tokens to group the query and place selection phases of a
    /// user autocomplete search into a discrete session for billing purposes. Session tokens are
    /// passed into Place Details (New) calls that follow Autocomplete (New) calls. For more
    /// information, see [Session
    /// tokens](https://developers.google.com/maps/documentation/places/web-service/place-session-tokens).
    ///
    /// A string which identifies an Autocomplete session for billing purposes. Must be a URL and
    /// filename safe base64 string with at most 36 ASCII characters in length. Otherwise an
    /// `INVALID_ARGUMENT` error is returned.
    ///
    /// The session begins when the user starts typing a query, and concludes when they select a
    /// place and a call to Place Details or Address Validation is made. Each session can have
    /// multiple queries, followed by one Place Details or Address Validation request. The
    /// credentials used for each request within a session must belong to the same Google Cloud
    /// Console project. Once a session has concluded, the token is no longer valid; your app must
    /// generate a fresh token for each session. If the `sessionToken` parameter is omitted, or if
    /// you reuse a session token, the session is charged as if no session token was provided (each
    /// request is billed separately).
    ///
    /// We recommend the following guidelines:
    ///
    /// * Use session tokens for all Place Autocomplete calls.
    ///
    /// * Generate a fresh token for each session. Using a version 4 UUID is recommended.
    ///
    /// * Ensure that the credentials used for all Place Autocomplete, Place Details, and Address
    ///   Validation requests within a session belong to the same Cloud Console project.
    ///
    /// * Be sure to pass a unique session token for each new session. Using the same token for more
    ///   than one session will result in each request being billed individually.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub session_token: Option<uuid::Uuid>,

    /// Include pure service area businesses.
    ///
    /// Include pure service area businesses if the field is set to true. Pure service area business
    /// is a business that visits or delivers to customers directly but does not serve customers at
    /// their business address.
    ///
    /// For example, businesses like cleaning services or plumbers. Those businesses do not have a
    /// physical address or location on Google Maps. Places will not return fields including
    /// `location`, `plusCode`, and other location related fields for these businesses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub include_pure_service_area_businesses: Option<bool>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Request {
    /// Upgrades to an executable request by adding a client reference.
    ///
    /// Combines this serializable request with a client reference, creating a `RequestWithClient`
    /// that can be executed.
    ///
    /// Use this to resume a stored or deserialized request.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Deserialize a stored request
    /// let request: Request = serde_json::from_str(&json)?;
    ///
    /// // Add client and execute
    /// let response = request
    ///     .with_client(&client)
    ///     .execute()
    ///     .await?;
    /// ```
    #[cfg(feature = "reqwest")]
    #[must_use]
    pub fn with_client(
        self,
        client: &crate::Client
    ) -> crate::places_new::autocomplete::RequestWithClient<'_> {
        crate::places_new::autocomplete::RequestWithClient {
            client,
            input: self.input,
            location_bias: self.location_bias,
            location_restriction: self.location_restriction,
            included_primary_types: self.included_primary_types,
            included_regions: self.included_regions,
            language: self.language,
            region: self.region,
            origin: self.origin,
            input_offset: self.input_offset,
            include_query_predictions: self.include_query_predictions,
            session_token: self.session_token,
            include_pure_service_area_businesses: self.include_pure_service_area_businesses,
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

#[cfg(feature = "reqwest")]
impl std::convert::From<crate::places_new::autocomplete::RequestWithClient<'_>> for Request {
    /// Converts a `RequestWithClient` to a serializable `Request`.
    ///
    /// Strips the client reference and returns only the query parameters, creating a fully
    /// serializable request that can be stored or transmitted.
    ///
    /// All configuration including session tokens and location preferences are preserved.
    fn from(request: crate::places_new::autocomplete::RequestWithClient) -> Self {
        Self {
            input: request.input,
            location_bias: request.location_bias,
            location_restriction: request.location_restriction,
            included_primary_types: request.included_primary_types,
            included_regions: request.included_regions,
            language: request.language,
            region: request.region,
            origin: request.origin,
            input_offset: request.input_offset,
            include_query_predictions: request.include_query_predictions,
            session_token: request.session_token,
            include_pure_service_area_businesses: request.include_pure_service_area_businesses,
        }
    }
}