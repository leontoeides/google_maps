#![allow(clippy::ref_option, reason = "for the getset crate")]

use crate::places_new::text_search::{EvOptions, ResponseWithContext};
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
pub struct RequestWithClient<'c> {
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
    /// * For categorical queries like "Restaurants in NYC", defaults to `Relevance`.
    /// * For non-categorical queries like "Mountain View, CA", leaving unset is recommended.
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
    pub fn into_request(self) -> crate::places_new::text_search::Request {
        self.into()
    }

    /// Executes the text search request.
    ///
    /// Sends the configured request to the Google Maps API and returns both the response and a
    /// serializable copy of the request parameters in a `ResponseWithContext`.
    ///
    /// The returned context preserves all request state including the pagination token, enabling
    /// pagination continuation with `Client::next_text_search()`.
    ///
    /// This method is available on both the builder (via `.build().execute()` shorthand) and on
    /// `RequestWithClient` directly when constructing requests manually.
    pub async fn execute(self) -> Result<ResponseWithContext, crate::Error> {
        let response = self.client.post_request(&self).await?;
        Ok(ResponseWithContext { response, request: self.into() })
    }
}

impl<S: request_with_client_builder::State> RequestWithClientBuilder<'_, S> {
    /// Executes the text search request.
    ///
    /// Builds the request and sends it to the Google Maps API, returning the parsed text search
    /// response. This method both completes the builder and executes the HTTP request in one step.
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
    /// Searches for places based on a text query string.
    ///
    /// The Google Maps Places API Text Search (New) service finds places matching a natural
    /// language query like "pizza in New York", "shoe stores near Ottawa", or specific addresses
    /// like "123 Main Street". This is the most flexible search method, accepting free-form text
    /// that can include business names, place types, addresses, or combinations of location and
    /// query terms.
    ///
    /// Text search supports optional location biasing to prioritize results near a specific area,
    /// though the query itself can override this with explicit location references. Results are
    /// ranked by relevance to the query text, considering factors like prominence, distance (when
    /// location context exists), and how well the place matches the search terms.
    ///
    /// Use field masking to control which place data is returned and manage API costs, as you're
    /// charged based on the fields requested. The response may include a `next_page_token` for
    /// pagination when more than 20 results are available.
    ///
    /// # Examples
    ///
    /// Basic text search with specific fields:
    ///
    /// ```rust,no_run
    /// use google_maps::places_new::Field;
    ///
    /// let response = google_maps_client
    ///     .text_search("Spicy Vegetarian Food in Nairobi Kenya")
    ///     .field_mask([
    ///         Field::PlacesId,
    ///         Field::PlacesName,
    ///         Field::PlacesDisplayName,
    ///         Field::PlacesFormattedAddress,
    ///         Field::PlacesGoogleMapsUri,
    ///         Field::NextPageToken,
    ///     ])
    ///     .execute()
    ///     .await?;
    ///
    /// for place in &response.places {
    ///     if let Some(name) = place.display_text() {
    ///         println!("Found: {}", name);
    ///     }
    /// }
    /// ```
    ///
    /// Search with location biasing:
    ///
    /// ```rust,no_run
    /// use google_maps::LatLng;
    /// use google_maps::places_new::{Field, LocationBias};
    ///
    /// let response = google_maps_client
    ///     .text_search("coffee shops")
    ///     .location_bias(LocationBias::circle(
    ///         LatLng::try_from_dec(40.7128, -74.0060)?,
    ///         2000.0, // 2km radius
    ///     ))
    ///     .field_mask([Field::PlacesDisplayName, Field::PlacesLocation])
    ///     .execute()
    ///     .await?;
    /// ```
    ///
    /// Handling pagination:
    ///
    /// ```rust,no_run
    /// use google_maps::places_new::Field;
    ///
    /// // First page
    /// let response = google_maps_client
    ///     .text_search("restaurants in Tokyo")
    ///     .field_mask([
    ///         Field::PlacesDisplayName,
    ///         Field::PlacesRating,
    ///         Field::NextPageToken,
    ///     ])
    ///     .execute()
    ///     .await?;
    ///
    /// println!("Page 1: {} results", response.places.len());
    ///
    /// // Get next page if available
    /// if response.has_next_page() {
    ///     let next_response = google_maps_client
    ///         .next_text_search(response)
    ///         .await?;
    ///     println!("Page 2: {} results", next_response.places.len());
    /// }
    /// ```
    pub fn text_search(
        &self,
        text_query: impl Into<String>
    ) -> RequestWithClientBuilder<
        '_,
        crate::places_new::text_search::request_with_client::request_with_client_builder::SetTextQuery<
            crate::places_new::text_search::request_with_client::request_with_client_builder::SetClient
        >
    > {
        RequestWithClient::builder().client(self).text_query(text_query.into())
    }

    /// Retrieves the next page of results from a previous text search.
    ///
    /// When a text search returns more than 20 results, Google provides a `next_page_token` in the
    /// response to retrieve additional results. This method handles pagination by extracting the
    /// token from a previous response and executing a new request with the same search parameters.
    ///
    /// The next page request automatically preserves all original search parameters (query, field
    /// mask, location bias, restrictions, etc.) from the initial search. Each page can contain up
    /// to 20 places, and you can continue calling this method as long as the response includes
    /// another `next_page_token`.
    ///
    /// Note that page tokens expire after a short time (typically a few minutes), so pagination
    /// should be done in a timely manner. The total number of pages available may be limited by
    /// Google's API even when more results theoretically exist.
    ///
    /// # Examples
    ///
    /// Collecting all available pages:
    ///
    /// ```rust,no_run
    /// use google_maps::places_new::Field;
    ///
    /// let mut all_places = Vec::new();
    ///
    /// // Get first page
    /// let mut response = google_maps_client
    ///     .text_search("hotels in Paris")
    ///     .field_mask([
    ///         Field::PlacesDisplayName,
    ///         Field::PlacesRating,
    ///         Field::NextPageToken,
    ///     ])
    ///     .execute()
    ///     .await?;
    ///
    /// all_places.extend(response.places.clone());
    ///
    /// // Get remaining pages
    /// while response.has_next_page() {
    ///     response = google_maps_client.next_text_search(response).await?;
    ///     all_places.extend(response.places.clone());
    /// }
    ///
    /// println!("Found {} total places", all_places.len());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The page token has expired
    /// - Network connectivity issues occur
    /// - The API returns an error response
    /// - The previous response didn't contain a `next_page_token`
    pub async fn next_text_search(
        &self,
        previous: ResponseWithContext,
    ) -> Result<ResponseWithContext, crate::Error> {
        let next_page_token = previous.response.next_page_token
            .ok_or_else(|| {
                let debug = "previous.response.next_page_token: None".to_string();
                let span = (0, debug.len());
                crate::places_new::text_search::Error::NoNextPageAvailable {
                    debug,
                    span,
                }
            })?;

        let mut request_with_client = previous.request.with_client(self);
        request_with_client.set_page_token(Some(next_page_token));
        request_with_client.execute().await
    }

    /// Collects all remaining pages from a text search into a single vector.
    ///
    /// This is a convenience method that automatically handles pagination, collecting all places
    /// from the initial response and all subsequent pages into a single vector. Useful when you
    /// need the complete result set without manually managing the pagination loop.
    ///
    /// The method will continue fetching pages until no `next_page_token` is available or until
    /// Google's API stops returning more results. Each page is fetched sequentially to avoid
    /// overwhelming the API.
    ///
    /// Note that collecting all pages may take significant time for queries with many results and
    /// will consume API quota for each page fetched. Consider using manual pagination with
    /// `next_text_search()` if you need more control over the fetching process or want to process
    /// results as they arrive.
    ///
    /// # Examples
    ///
    /// Collecting all results at once:
    ///
    /// ```rust,no_run
    /// use google_maps::places_new::Field;
    ///
    /// // Get first page
    /// let response = google_maps_client
    ///     .text_search("coffee shops in Seattle")
    ///     .field_mask([
    ///         Field::PlacesDisplayName,
    ///         Field::PlacesLocation,
    ///         Field::NextPageToken,
    ///     ])
    ///     .execute()
    ///     .await?;
    ///
    /// // Collect all remaining pages
    /// let all_places = google_maps_client
    ///     .collect_all_text_search_pages(response)
    ///     .await?;
    ///
    /// println!("Found {} total places", all_places.len());
    /// ```
    ///
    /// Processing with progress updates:
    ///
    /// ```rust,no_run
    /// use google_maps::places_new::Field;
    ///
    /// let mut response = google_maps_client
    ///     .text_search("restaurants in Tokyo")
    ///     .field_mask([Field::PlacesDisplayName, Field::NextPageToken])
    ///     .execute()
    ///     .await?;
    ///
    /// let first_page_count = response.places.len();
    /// println!("Page 1: {} results", first_page_count);
    ///
    /// let all_places = google_maps_client
    ///     .collect_all_text_search_pages(response)
    ///     .await?;
    ///
    /// println!("Total: {} results across all pages", all_places.len());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Any page request fails due to network issues
    /// - The API returns an error response for any page
    /// - A page token expires during collection (pagination should be timely)
    pub async fn collect_all_text_search_pages(
        &self,
        mut initial_response: ResponseWithContext,
    ) -> Result<Vec<crate::places_new::Place>, crate::Error> {
        let mut all_places = initial_response.places().clone();

        while initial_response.has_next_page() {
            initial_response = self.next_text_search(initial_response).await?;
            all_places.extend(initial_response.places().clone());
        }

        Ok(all_places)
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
        "https://places.googleapis.com/v1/places:searchText"
    }

    fn output_format() -> std::option::Option<&'static str> {
        None // No need to specify the output format, this end-point always returns JSON.
    }

    #[cfg(feature = "reqwest")]
    fn title() -> &'static str {
        "Places API (New) Text Search"
    }

    #[cfg(feature = "reqwest")]
    fn apis() -> &'static [Api] {
        &[Api::All, Api::PlacesNew, Api::TextSearch]
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
impl crate::traits::Validatable for &RequestWithClient<'_> {
    /// Validates the text search request parameters.
    ///
    /// Checks that the combination of parameters makes sense and will be accepted by the Google
    /// Maps Places API. This does not validate individual parameter values (like coordinate
    /// ranges), only the logical consistency of the request.
    ///
    /// ## Validation Rules
    ///
    /// - `location_bias` and `location_restriction` cannot both be set
    /// - `field_mask` cannot be empty when using `FieldMask::Specific`
    /// - `included_type` must be a Table A place type, not Table B
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Both `location_bias` and `location_restriction` are set
    /// - Field mask is `FieldMask::Specific` with an empty vector
    /// - Included type is a Table B place type that cannot be used for filtering
    fn validate(&self) -> Result<(), crate::Error> {
        // Check that location_bias and location_restriction are mutually exclusive
        if self.location_bias.is_some() && self.location_restriction.is_some() {
            let debug = "location_bias: Some(...), location_restriction: Some(...)".to_string();
            let span = (0, debug.len());
            return Err(crate::places_new::text_search::Error::MutuallyExclusiveLocationFields {
                debug,
                span,
            }.into());
        }

        // Check that field mask is not empty
        if let crate::places_new::FieldMask::Specific(fields) = &self.field_mask {
            if fields.is_empty() {
                let debug = "field_mask: FieldMask::Specific(vec![])".to_string();
                let span = (0, debug.len());
                return Err(crate::places_new::text_search::Error::EmptyFieldMask {
                    debug,
                    span,
                }.into());
            }
        }

        // Check that included_type is a Table A type
        if let Some(place_type) = &self.included_type {
            if place_type.is_table_b() {
                let debug = format!("included_type: Some(PlaceType::{place_type})");
                let span = (0, debug.len());
                return Err(crate::places_new::text_search::Error::InvalidPlaceTypeForFilter {
                    place_type: place_type.to_string(),
                    debug,
                    span,
                }.into());
            }
        }

        Ok(())
    }
}