#![allow(clippy::ref_option, reason = "for the getset crate")]

use crate::places_new::FieldMask;
use crate::places_new::nearby_search::Response;
use crate::places_new::types::request::{LocationRestriction, RankPreference};
use icu_locale::Locale;
use reqwest::header::HeaderMap;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub included_types: Option<Vec<crate::places_new::PlaceType>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub excluded_types: Option<Vec<crate::places_new::PlaceType>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub included_primary_types: Option<Vec<crate::places_new::PlaceType>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub excluded_primary_types: Option<Vec<crate::places_new::PlaceType>>,

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

impl RequestWithClient<'_> {
    /// Converts to a serializable request by stripping the client reference.
    ///
    /// Creates a `Request` containing all query parameters but without the client reference, making
    /// it possible to serialize, store, or transmit. Use this when you need to persist request
    /// state for later execution.
    ///
    /// Note: Typically you don't need to call this directly, as `execute()` automatically returns a
    /// `Response` containing the serializable request.
    #[must_use]
    pub fn into_request(self) -> crate::places_new::nearby_search::Request {
        self.into()
    }

    /// Executes the text search request.
    ///
    /// Sends the configured request to the Google Maps API and returns both the response and a
    /// serializable copy of the request parameters in a `Response`.
    ///
    /// The returned context preserves all request state including the pagination token, enabling
    /// pagination continuation with `Client::next_nearby_search()`.
    ///
    /// This method is available on both the builder (via `.build().execute()` shorthand) and on
    /// `RequestWithClient` directly when constructing requests manually.
    pub async fn execute(self) -> Result<Response, crate::Error> {
        let response = self.client.post_request(&self).await?;
        Ok(response)
    }
}

impl<S: request_with_client_builder::State> RequestWithClientBuilder<'_, S> {
    /// Executes the text search request.
    ///
    /// Builds the request and sends it to the Google Maps API, returning the parsed text search
    /// response. This method both completes the builder and executes the HTTP request in one step.
    pub async fn execute(self) -> Result<Response, crate::Error>
    where
        S: request_with_client_builder::IsComplete,
    {
        let request = self.build();  // Build request
        let response = request.client.post_request(&request).await?;
        Ok(response)
    }
}

impl crate::client::Client {
    /// Searches for places within a geographic area.
    ///
    /// The Google Maps Places API Nearby Search (New) service finds places within a specified
    /// circular area, optionally filtered by place type.
    ///
    /// Unlike Text Search which uses text queries, Nearby Search focuses purely on location-based
    /// discovery - perfect for "what's nearby" functionality or finding specific types of places
    /// within a radius.
    ///
    /// Results can be ranked by distance (closest first) or popularity (most relevant first).
    /// This endpoint does not support pagination - the maximum 20 results per request is a hard
    /// limit.
    ///
    /// Use field masking to control which place data is returned and manage API costs, as you're
    /// charged based on the fields requested. Different field groups trigger different pricing
    /// SKUs (Pro, Enterprise, Enterprise + Atmosphere).
    ///
    /// # Location Restriction
    ///
    /// The location must be specified as a circle (center point + radius). You can provide this
    /// as:
    /// - A 3-tuple: `(latitude, longitude, radius_meters)`
    /// - A `Circle` struct
    /// - A `LocationRestriction::Circle` enum variant
    ///
    /// **Note**: Nearby Search only supports circular search areas. Rectangle/viewport
    /// restrictions will fail validation.
    ///
    /// # Examples
    ///
    /// Basic nearby search using tuple syntax:
    ///
    /// ```rust,no_run
    /// use google_maps::places_new::{Field, PlaceType};
    ///
    /// // Find restaurants within 5km of San Francisco
    /// let response = google_maps_client
    ///     .nearby_search((37.7749, -122.4194, 5000.0))?
    ///     .field_mask([
    ///         Field::PlacesDisplayName,
    ///         Field::PlacesFormattedAddress,
    ///         Field::PlacesRating,
    ///     ])
    ///     .included_primary_types(vec![PlaceType::Restaurant])
    ///     .execute()
    ///     .await?;
    ///
    /// for place in &response {
    ///     if let Some(name) = place.display_text() {
    ///         println!("Found: {}", name);
    ///     }
    /// }
    /// ```
    ///
    /// Rank by distance to find the closest places:
    ///
    /// ```rust,no_run
    /// use google_maps::places_new::{Field, RankPreference};
    ///
    /// let response = google_maps_client
    ///     .nearby_search((40.7128, -74.0060, 2000.0))?
    ///     .field_mask([Field::PlacesDisplayName, Field::PlacesLocation])
    ///     .rank_preference(RankPreference::Distance)
    ///     .execute()
    ///     .await?;
    ///
    /// println!("Closest {} places:", response.len());
    /// ```
    ///
    /// Filter by multiple types and exclude specific types:
    ///
    /// ```rust,no_run
    /// use google_maps::places_new::{Field, PlaceType};
    ///
    /// // Find schools but exclude primary schools
    /// let response = google_maps_client
    ///     .nearby_search((35.6762, 139.6503, 3000.0))?
    ///     .field_mask([Field::PlacesDisplayName, Field::PlacesPrimaryType])
    ///     .included_types(vec![PlaceType::School])
    ///     .excluded_types(vec![PlaceType::PrimarySchool])
    ///     .max_result_count(10)
    ///     .execute()
    ///     .await?;
    /// ```
    ///
    /// Using an explicit Circle for more control:
    ///
    /// ```rust,no_run
    /// use google_maps::places_new::{Circle, LatLng, Field};
    /// use rust_decimal_macros::dec;
    ///
    /// let center = LatLng::try_from_dec(dec!(51.5074), dec!(-0.1278))?;
    /// let circle = Circle::try_new(center, dec!(1000.0))?;
    ///
    /// let response = google_maps_client
    ///     .nearby_search(circle)?
    ///     .field_mask([Field::PlacesDisplayName])
    ///     .execute()
    ///     .await?;
    /// ```
    pub fn nearby_search<L>(
        &self,
        location_restriction: L,
    ) -> Result<
        RequestWithClientBuilder<
            '_,
            crate::places_new::nearby_search::request_with_client::request_with_client_builder::SetLocationRestriction<
                crate::places_new::nearby_search::request_with_client::request_with_client_builder::SetClient
            >
        >,
        crate::Error,
    >
    where
        L: TryInto<LocationRestriction>,
        L::Error: Into<crate::Error>,
    {
        let location_restriction = location_restriction
            .try_into()
            .map_err(Into::into)?;

        Ok(RequestWithClient::builder()
            .client(self)
            .location_restriction(location_restriction))
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
        "https://places.googleapis.com/v1/places:searchNearby"
    }

    fn output_format() -> std::option::Option<&'static str> {
        None // No need to specify the output format, this end-point always returns JSON.
    }

    #[cfg(feature = "reqwest")]
    fn title() -> &'static str {
        "Places API (New) Nearby Search"
    }

    #[cfg(feature = "reqwest")]
    fn apis() -> &'static [Api] {
        &[Api::All, Api::PlacesNew, Api::NearbySearch]
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
        let field_mask = self.field_mask().to_string();
        let mut headers = HeaderMap::new();
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
    /// Validates the nearby search request parameters.
    ///
    /// Checks that the combination of parameters makes sense and will be accepted by the Google
    /// Maps Places API. This does not validate individual parameter values (like coordinate
    /// ranges), only the logical consistency of the request.
    ///
    /// ## Validation Rules
    ///
    /// - `field_mask` cannot be empty when using `FieldMask::Specific`
    /// - All place types in `included_types`, `excluded_types`, `included_primary_types`, and
    ///   `excluded_primary_types` must be Table A types (Table B types cannot be used for
    ///   filtering)
    /// - A type cannot appear in both `included_types` and `excluded_types`
    /// - A type cannot appear in both `included_primary_types` and `excluded_primary_types`
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Field mask is `FieldMask::Specific` with an empty vector
    /// - Any place type in the filter lists is a Table B place type
    /// - A place type appears in both inclusion and exclusion lists
    /// - Location restriction is a Rectangle (only Circle is supported)
    fn validate(&self) -> Result<(), crate::Error> {
        // Check that field mask is not empty
        if let crate::places_new::FieldMask::Specific(fields) = &self.field_mask {
            if fields.is_empty() {
                let debug = "field_mask: FieldMask::Specific(vec![])".to_string();
                let span = (0, debug.len());
                return Err(crate::places_new::nearby_search::Error::EmptyFieldMask {
                    debug,
                    span: span.into(),
                }
                .into());
            }
        }

        // Helper to validate that all types in a vec are Table A
        let validate_table_a = |types: &[crate::places_new::PlaceType], field_name: &str| {
            for place_type in types {
                if place_type.is_table_b() {
                    let debug = format!("{field_name}: vec![..., PlaceType::{place_type}, ...]");
                    let span = (0, debug.len());
                    return Err::<(), crate::Error>(crate::places_new::nearby_search::Error::InvalidPlaceTypeForFilter {
                        place_type: place_type.to_string(),
                        debug,
                        span: span.into(),
                    }
                    .into());
                }
            }
            Ok(())
        };

        // Nearby Search only supports Circle, not Rectangle
        if let LocationRestriction::Rectangle(viewport) = &self.location_restriction {
            let debug = format!(
                "location_restriction: Rectangle(Viewport {{ low: {}, high: {} }})",
                viewport.low, viewport.high
            );
            let span = (0, debug.len());

            return Err(crate::places_new::nearby_search::Error::UnsupportedLocationRestriction {
                restriction_type: "Rectangle".to_string(),
                debug,
                span: span.into(),
            }.into());
        }

        // Validate all type filter fields contain only Table A types
        if let Some(types) = &self.included_types {
            validate_table_a(types, "included_types")?;
        }

        if let Some(types) = &self.excluded_types {
            validate_table_a(types, "excluded_types")?;
        }

        if let Some(types) = &self.included_primary_types {
            validate_table_a(types, "included_primary_types")?;
        }

        if let Some(types) = &self.excluded_primary_types {
            validate_table_a(types, "excluded_primary_types")?;
        }

        // Check for conflicts between included_types and excluded_types
        if let (Some(included), Some(excluded)) = (&self.included_types, &self.excluded_types) {
            for place_type in included {
                if excluded.contains(place_type) {
                    let debug = format!(
                        "included_types: vec![..., PlaceType::{place_type}, ...], \
                         excluded_types: vec![..., PlaceType::{place_type}, ...]"
                    );
                    let span = (0, debug.len());
                    return Err(
                        crate::places_new::nearby_search::Error::ConflictingPlaceTypes {
                            place_type: place_type.to_string(),
                            debug,
                            span: span.into(),
                        }
                        .into(),
                    );
                }
            }
        }

        // Check for conflicts between included_primary_types and excluded_primary_types
        if let (Some(included), Some(excluded)) =
            (&self.included_primary_types, &self.excluded_primary_types)
        {
            for place_type in included {
                if excluded.contains(place_type) {
                    let debug = format!(
                        "included_primary_types: vec![..., PlaceType::{place_type}, ...], \
                         excluded_primary_types: vec![..., PlaceType::{place_type}, ...]"
                    );
                    let span = (0, debug.len());
                    return Err(
                        crate::places_new::nearby_search::Error::ConflictingPrimaryPlaceTypes {
                            place_type: place_type.to_string(),
                            debug,
                            span: span.into(),
                        }
                        .into(),
                    );
                }
            }
        }

        Ok(())
    }
}