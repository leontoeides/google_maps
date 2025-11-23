#![allow(clippy::ref_option, reason = "for the getset crate")]

use crate::places_new::FieldMask;
use icu_locale::Locale;
use rust_iso3166::CountryCode;

// -------------------------------------------------------------------------------------------------
//
/// Request for the Google Maps Places API (New) Place Details service.
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
pub struct Request<'c> {
    /// The Google Maps API client.
    ///
    /// The `Client` structure contains the application's API key and other user-definable settings
    /// such as "maximum retries," and most importantly the
    /// [reqwest](https://crates.io/crates/reqwest) client itself.
    #[serde(skip_deserializing, skip_serializing)]
    pub(crate) client: &'c crate::Client,

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

    /// Once you have a [place ID](https://developers.google.com/maps/documentation/places/web-service/place-id),
    /// you can request more details about a particular establishment or point of interest by
    /// initiating a Place Details (New) request.
    ///
    /// A Place Details (New) request returns more comprehensive information about the indicated
    /// place such as its complete address, phone number, user rating and reviews.
    ///
    /// There are many ways to obtain a place ID. You can use:
    ///
    /// * Text Search (New) or Nearby Search (New)
    /// * Geocoding API
    /// * Routes API
    /// * Address Validation API
    /// * Autocomplete (New)
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub place_id: String,

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
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

#[cfg(feature = "reqwest")]
impl Request<'_> {
    pub async fn execute(self) -> Result<crate::places_new::place_details::Response, crate::Error> {
        let response = self.client.get_request(&self).await?;
        Ok(response)
    }
}

#[cfg(feature = "reqwest")]
impl<S: request_builder::State> RequestBuilder<'_, S> {
    /// Executes the text search request.
    ///
    /// Builds the request and sends it to the Google Maps API, returning the parsed text search
    /// response. This method both completes the builder and executes the HTTP request in one step.
    pub async fn execute(self) -> Result<crate::places_new::place_details::Response, crate::Error>
    where
        S: request_builder::IsComplete,
    {
        let request = self.build();  // Build request
        let response = request.client.get_request(&request).await?;
        Ok(response)
    }
}

impl crate::client::Client {
    /// Once you have a [place ID](https://developers.google.com/maps/documentation/places/web-service/place-id),
    /// you can request more details about a particular establishment or point of interest by
    /// initiating a Place Details (New) request.
    ///
    /// A Place Details (New) request returns more comprehensive information about the indicated
    /// place such as its complete address, phone number, user rating and reviews.
    ///
    /// There are many ways to obtain a place ID. You can use:
    ///
    /// * Text Search (New) or Nearby Search (New)
    /// * Geocoding API
    /// * Routes API
    /// * Address Validation API
    /// * Autocomplete (New)
    pub fn place_details(
        &self,
        place_id: impl Into<String>,
    ) -> Result<
        RequestBuilder<
            '_,
            crate::places_new::place_details::request::request_builder::SetPlaceId<
                crate::places_new::place_details::request::request_builder::SetClient
            >
        >,
        crate::Error,
    > {
        let place_id: String = place_id.into();

        Ok(Request::builder()
            .client(self)
            .place_id(place_id))
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

#[cfg(feature = "reqwest")]
use crate::request_rate::api::Api;

/// Defines the Google Maps Places API HTTP endpoint for requests.
///
/// This trait returns information needed to make HTTP `POST` requests to the Places API endpoint.
/// It includes service URL, debugging info, and rate-limiting configuration.
impl crate::traits::EndPoint for &Request<'_> {
    fn service_url() -> &'static str {
        ""
    }

    fn output_format() -> std::option::Option<&'static str> {
        None // No need to specify the output format, this end-point always returns JSON.
    }

    #[cfg(feature = "reqwest")]
    fn title() -> &'static str {
        "Places API (New) Place Details"
    }

    #[cfg(feature = "reqwest")]
    fn apis() -> &'static [Api] {
        &[Api::All, Api::PlacesNew, Api::PlaceDetails]
    }
}

#[cfg(feature = "reqwest")]
impl crate::traits::RequestBody for &Request<'_> {
    /// Converts the `Request` struct into JSON for submission to Google Maps.
    ///
    /// Serializes the request body fields into a JSON object for the HTTP POST request body.
    ///
    /// # Errors
    ///
    /// Returns an error if JSON serialization fails.
    fn request_body(&self) -> Result<String, crate::Error> {
        Ok(serde_json::to_string(self)?)
    }
}

#[cfg(feature = "reqwest")]
impl crate::traits::QueryString for &Request<'_> {
    /// Builds the URL query string for the HTTP request.
    ///
    /// The Places (New) API uses the HTTP body for most request data, so the query string only
    /// contains the API key for authentication.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    /// Builds the URL query string for the HTTP request.
    ///
    /// Constructs query parameters for dimension constraints and redirect behavior. At least one
    /// of `maxWidthPx` or `maxHeightPx` must be specified.
    ///
    /// # Example Query String
    ///
    /// ```text
    /// maxWidthPx=800&maxHeightPx=600&skipHttpRedirect=true
    /// ```
    fn query_string(&self) -> String {
        format!(
            "https://places.googleapis.com/v1/places/{place_id}",
            place_id = self.place_id()
        )
    }
}

#[cfg(feature = "reqwest")]
impl crate::traits::RequestHeaders for &Request<'_> {
    /// Returns a map of HTTP header names to values.
    ///
    /// These headers will be added to the HTTP request alongside the standard headers like
    /// `X-Goog-Api-Key`.
    fn request_headers(&self) -> reqwest::header::HeaderMap {
        let field_mask = self.field_mask().to_string();
        let mut headers = reqwest::header::HeaderMap::new();
        match reqwest::header::HeaderValue::from_str(field_mask.as_str()) {
            Ok(header_value) => { headers.insert("X-Goog-FieldMask", header_value); },
            Err(error) => tracing::error!("error building request headers: {error}"),
        }
        headers
    }

    /// Returns whether the `X-Goog-Api-Key` header should be set for this request.
    fn send_x_goog_api_key() -> bool {
        true
    }
}

#[cfg(feature = "reqwest")]
impl crate::traits::Validatable for &Request<'_> {
    /// Validates the nearby search request parameters.
    fn validate(&self) -> Result<(), crate::Error> {
        Ok(())
    }
}