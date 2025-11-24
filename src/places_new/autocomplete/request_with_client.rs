#![allow(clippy::ref_option, reason = "this is how getset works")]

use crate::places_new::autocomplete::ResponseWithContext;
use crate::places_new::LatLng;
use crate::places_new::types::request::{LocationBias, LocationRestriction, PlaceTypeSet};
use icu_locale::Locale;
use reqwest::header::HeaderMap;
use rust_iso3166::CountryCode;

// -------------------------------------------------------------------------------------------------
//
/// Request for the Google Maps Places API (New) Autocomplete service.
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
pub struct RequestWithClient<'c> {
    /// The Google Maps API client.
    ///
    /// The `Client` structure contains the application's API key and other user-definable settings
    /// such as "maximum retries," and most importantly the
    /// [reqwest](https://crates.io/crates/reqwest) client itself.
    #[serde(skip_deserializing, skip_serializing)]
    pub(crate) client: &'c crate::Client,

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
    #[serde(default, skip_serializing_if = "PlaceTypeSet::is_empty")]
    #[builder(default, into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub included_primary_types: PlaceTypeSet,

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

impl RequestWithClient<'_> {
    /// Converts to a serializable request by stripping the client reference.
    ///
    /// Creates a `Request` containing all query parameters but without the client reference, making
    /// it possible to serialize, store, or transmit. Use this when you need to persist request
    /// state for later execution.
    ///
    /// Note: Typically you don't need to call this directly, as `execute()` automatically returns a
    /// `ResponseWithContext` containing the serializable request.
    #[must_use]
    pub fn into_request(self) -> crate::places_new::autocomplete::Request {
        self.into()
    }

    /// Executes the autocomplete request.
    ///
    /// Sends the configured request to the Google Maps API and returns both the response and a
    /// serializable copy of the request parameters in a `ResponseWithContext`.
    ///
    /// The returned context preserves all request state including the session token, enabling
    /// session continuation with `Client::next_autocomplete()`.
    ///
    /// This method is available on both the builder (via `.build().execute()` shorthand) and on
    /// `RequestWithClient` directly when constructing requests manually.
    ///
    /// # Examples
    ///
    /// Using the builder pattern (most common):
    /// ```rust,ignore
    /// let ctx = client
    ///     .autocomplete("pizza")
    ///     .location_bias(LocationBias::circle(circle))
    ///     .execute()
    ///     .await?;
    /// ```
    ///
    /// Using `RequestWithClient` directly:
    /// ```rust,ignore
    /// let request = Request::default().with_client(&client);
    /// let ctx = request.execute().await?;
    /// ```
    ///
    /// Continue the session
    /// ```rust,ignore
    /// let ctx = client.next_autocomplete("pizza margherita", ctx).await?;
    /// ```
    pub async fn execute(self) -> Result<ResponseWithContext, crate::Error> {
        let response = self.client.post_request(&self).await?;
        Ok(ResponseWithContext { response, request: self.into() })
    }
}

impl<S: request_with_client_builder::State> RequestWithClientBuilder<'_, S> {
    /// Executes the autocomplete request.
    ///
    /// Builds the request and sends it to the Google Maps API, returning the parsed autocomplete
    /// response. This method both completes the builder and executes the HTTP request in one step.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let response = client
    ///     .autocomplete("pizza")
    ///     .location_bias(LocationBias::circle(circle))
    ///     .execute()
    ///     .await?;
    /// ```
    pub async fn execute(self) -> Result<ResponseWithContext, crate::Error>
    where
        S: request_with_client_builder::IsComplete,
    {
        let request = self.build();  // Build request
        let response = request.client.post_request(&request).await?;
        Ok(ResponseWithContext { response, request: request.into() })
    }
}

impl crate::client::Client {
    /// The Places API **Place Autocomplete** service returns place predictions.
    ///
    /// Autocomplete (New) is a web service that returns place predictions and query predictions in
    /// response to an HTTP request. In the request, specify a text search string and geographic
    /// bounds that controls the search area.
    ///
    /// Autocomplete (New) can match on full words and substrings of the input, resolving place
    /// names, addresses, and plus codes. Applications can therefore send queries as the user types,
    /// to provide on-the-fly place and query predictions.
    ///
    /// The response from Autocomplete (New) can contain two types of predictions:
    ///
    /// - **Place predictions**: Places, such as businesses, addresses and points of interest, based
    ///   on the specified input text string and search area. Place predictions are returned by
    ///   default.
    ///
    /// - **Query predictions**: Query strings matching the input text string and search area. Query
    ///   predictions are not returned by default. Use the `.include_query_predictions(true)`
    ///   request parameter to add query predictions to the response.
    ///
    /// For example, you call Autocomplete (New) using as input a string that contains a partial
    /// user input, "Sicilian piz", with the search area limited to San Francisco, CA. The response
    /// then contains a list of place predictions that match the search string and search area, such
    /// as the restaurant named "Sicilian Pizza Kitchen", along with details about the place.
    ///
    /// The returned place predictions are designed to be presented to the user to aid them in
    /// selecting the intended place. You can make a Place Details (New) request to get more
    /// information about any of the returned place predictions.
    ///
    /// The response can also contain a list of query predictions that match the search string and
    /// search area, such as "Sicilian Pizza & Pasta". Each query prediction in the response
    /// includes the text field containing a recommended text search string. Use that string as an
    /// input to Text Search (New) to perform a more detailed search.
    ///
    /// The APIs Explorer lets you make live requests so that you can get familiar with the API and
    /// the API options: <https://developers.google.com/maps/documentation/places/web-service/place-autocomplete#try_it>
    ///
    /// ## Arguments
    ///
    /// * `input` · The text string on which to search.
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use google_maps::places_new::PlaceType;
    ///
    /// let google_maps_client = google_maps::Client::try_new("YOUR_API_KEY_HERE")?;
    ///
    /// // Start a new autocomplete session
    /// let response = google_maps_client
    ///     .autocomplete("pizza")
    ///     .included_primary_types(vec![PlaceType::Restaurant])
    ///     .execute()
    ///     .await?;
    ///
    /// // Display suggestions with HTML highlighting
    /// for suggestion in &response.suggestions {
    ///     println!("{}", suggestion.to_html("mark"));
    /// }
    ///
    /// // Output:
    /// // <mark>Pizza</mark> By The Bay, Marine Drive, Churchgate, Mumbai, Maharashtra, India
    /// // <mark>Pizza</mark> 4P's Indiranagar, 12th Main Road, HAL 2nd Stage, Bengaluru, Karnataka, India
    /// // <mark>Pizza</mark> Culture Napoletana, Edmonton Trail, Calgary, AB, Canada
    ///
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "places-new-autocomplete")]
    pub fn autocomplete(
        &self,
        input: impl Into<String>
    ) -> RequestWithClientBuilder<
        '_,
        crate::places_new::autocomplete::request_with_client::request_with_client_builder::SetInput<
            crate::places_new::autocomplete::request_with_client::request_with_client_builder::SetClient
        >
    > {
        RequestWithClient::builder().client(self).input(input.into())
    }

    /// Continue a Place Autocomplete session with new input.
    ///
    /// Reuses the session token and all other parameters from the previous response, only updating
    /// the input text. This maintains session continuity for Google's billing model and ensures
    /// consistent relevance scoring across the autocomplete interaction.
    ///
    /// This method immediately executes the request with the preserved parameters. If you need to
    /// modify parameters before the next request, update the `ResponseWithContext` first:
    ///
    /// ## Arguments
    ///
    /// * `input` · The new text string on which to search.
    /// * `previous` · The previous response context containing the session state to continue.
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// use google_maps::places_new::PlaceType;
    ///
    /// let google_maps_client = google_maps::Client::try_new("YOUR_API_KEY_HERE")?;
    ///
    /// // Initial autocomplete request
    /// let response = google_maps_client
    ///     .autocomplete("pizza")
    ///     .included_primary_types(vec![PlaceType::Restaurant])
    ///     .execute()
    ///     .await?;
    ///
    /// for suggestion in &response.suggestions {
    ///     println!("{}", suggestion.to_html("mark"));
    /// }
    ///
    /// // Output:
    /// // <mark>Pizza</mark> By The Bay, Marine Drive, Churchgate, Mumbai, Maharashtra, India
    /// // <mark>Pizza</mark> 4P's Indiranagar, 12th Main Road, HAL 2nd Stage, Bengaluru, Karnataka, India
    ///
    /// // Continue the session as the user types more
    /// let response = google_maps_client
    ///     .next_autocomplete("pizza sicilian", response)
    ///     .await?;
    ///
    /// for suggestion in &response.suggestions {
    ///     println!("{}", suggestion.to_html("b"));
    /// }
    ///
    /// // Output:
    /// // <b>Pizza Sicilian</b>a, Rue Sully Prudhomme, Châtillon, France
    /// // <b>Pizza Sicilian</b>a, 6a Avenida, Puerto Barrios, Guatemala
    /// // <b>Pizza Sicilian</b>a 762, Chiquimula, Guatemala
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn next_autocomplete(
        &self,
        input: impl Into<String>,
        previous: ResponseWithContext,
    ) -> Result<ResponseWithContext, crate::Error> {
        let mut request_with_client = previous.request.with_client(self);
        request_with_client.set_input(input.into());
        request_with_client.execute().await
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
impl crate::traits::EndPoint for &RequestWithClient<'_> {
    fn service_url() -> &'static str {
        "https://places.googleapis.com/v1/places:autocomplete"
    }

    fn output_format() -> std::option::Option<&'static str> {
        None // No need to specify the output format, this end-point always returns JSON.
    }

    #[cfg(feature = "reqwest")]
    fn title() -> &'static str {
        "Places API (New) Autocomplete"
    }

    #[cfg(feature = "reqwest")]
    fn apis() -> &'static [Api] {
        &[Api::All, Api::PlacesNew, Api::Autocomplete]
    }
}

#[cfg(feature = "reqwest")]
impl crate::traits::RequestBody for &RequestWithClient<'_> {
    /// Converts the `RequestWithClient` struct into JSON for submission to Google Maps.
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
impl crate::traits::QueryString for &RequestWithClient<'_> {
    /// Builds the URL query string for the HTTP request.
    ///
    /// The Places (New) API uses the HTTP body for most request data, so the query string only
    /// contains the API key for authentication.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    fn query_string(&self) -> String {
        String::new()
    }
}

#[cfg(feature = "reqwest")]
impl crate::traits::RequestHeaders for &RequestWithClient<'_> {
    /// Returns a map of HTTP header names to values.
    ///
    /// These headers will be added to the HTTP request alongside the standard headers like
    /// `X-Goog-Api-Key`.
    fn request_headers(&self) -> HeaderMap {
        HeaderMap::default()
    }

    /// Returns whether the `X-Goog-Api-Key` header should be set for this request.
    fn send_x_goog_api_key() -> bool {
        true
    }
}

#[cfg(feature = "reqwest")]
impl crate::traits::Validatable for &RequestWithClient<'_> {
    /// Validates the autocomplete request parameters.
    ///
    /// Checks that the combination of parameters makes sense and will be accepted by the Google
    /// Maps Places API. This does not validate individual parameter values (like coordinate
    /// ranges), only the logical consistency of the request.
    ///
    /// ## Validation Rules
    ///
    /// - `location_bias` and `location_restriction` cannot both be set
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Both `location_bias` and `location_restriction` are set
    fn validate(&self) -> Result<(), crate::Error> {
        // Check that location_bias and location_restriction are mutually exclusive
        if self.location_bias.is_some() && self.location_restriction.is_some() {
            let debug = "location_bias: Some(...), location_restriction: Some(...)".to_string();
            let span = (0, debug.len());

            return Err(crate::places_new::autocomplete::Error::MutuallyExclusiveLocationFields {
                debug,
                span,
	        }.into());
        }

        Ok(())
    }
}