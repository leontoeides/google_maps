//! Contains the autocomplete response structure for the Places API.
//!
//! The autocomplete response contains a list of suggestions ordered by relevance, mixing place and
//! query predictions based on the request parameters.

// Exports

mod string_range;
pub use crate::places_new::autocomplete::response::string_range::StringRange;

mod formattable_text;
pub use crate::places_new::autocomplete::response::formattable_text::FormattableText;

mod place_prediction;
pub use crate::places_new::autocomplete::response::place_prediction::PlacePrediction;

mod structured_format;
pub use crate::places_new::autocomplete::response::structured_format::StructuredFormat;

mod suggestion;
pub use crate::places_new::autocomplete::response::suggestion::Suggestion;

mod query_prediction;
pub use crate::places_new::autocomplete::response::query_prediction::QueryPrediction;

// -------------------------------------------------------------------------------------------------
//
/// Response from the Google Maps Places API (New) Autocomplete service.
///
/// Contains a list of suggestions ordered in descending order of relevance. Each suggestion can be
/// either a specific place prediction or a general query prediction, depending on the request
/// parameters. The response may be empty if no relevant suggestions were found for the input text.
///
/// # Examples
///
/// ```rust,ignore
/// let response: Response = /* ... from API ... */;
///
/// // Iterate over all suggestions
/// for suggestion in &response.suggestions {
///     match suggestion {
///         Suggestion::PlacePrediction(place) => {
///             println!("Place: {}", place.to_html("strong"));
///         }
///         Suggestion::QueryPrediction(query) => {
///             println!("Query: {}", query.to_html("em"));
///         }
///     }
/// }
///
/// // Filter for only place predictions
/// let places: Vec<_> = response.places().collect();
///
/// // Get formatted HTML for all suggestions
/// let html_list: Vec<String> = response.suggestions
///     .iter()
///     .map(|s| s.to_html("mark"))
///     .collect();
/// ```
///
/// # `Response` vs. `ResponseWithContext`
///
/// * `Response` - Just the API data. For simple queries or when request context isn't needed.
/// * `ResponseWithContext` - Includes original request. Preserves pagination tokens, session state
///   for follow-up requests.
#[derive(
    // std
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    // serde
    serde::Deserialize,
    serde::Serialize,
    // getset
    getset::Getters,
    getset::MutGetters,
    getset::Setters,
)]
pub struct Response {
    /// List of suggestions ordered by relevance.
    ///
    /// Contains place and/or query predictions sorted in descending order of relevance. May be
    /// empty if no suggestions were found. The order is determined by Google's relevance algorithms
    /// and should typically be preserved when displaying results to users.
    #[serde(default)]
    #[getset(get = "pub")]
    pub(crate) suggestions: Vec<Suggestion>,

    /// Error returned from the Google Maps API server.
    #[serde(default)]
    #[getset(get = "pub")]
    error: Option<crate::places_new::GoogleApiError>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Response {
    /// Creates a new `Response` with suggestions.
    ///
    /// Use this when constructing responses from API data or for testing.
    #[must_use]
    pub const fn new(suggestions: Vec<Suggestion>) -> Self {
        Self { suggestions, error: None }
    }

    /// Returns the number of suggestions in the response.
    ///
    /// The total count of both place and query predictions combined.
    #[must_use]
    pub fn len(&self) -> usize {
        self.suggestions.len()
    }

    /// Checks if the response contains no suggestions.
    ///
    /// Returns `true` if no place or query predictions were returned, indicating that no relevant
    /// results were found for the input.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.suggestions.is_empty()
    }

    /// Returns an iterator over only the place predictions.
    ///
    /// Filters the suggestions to include only specific places, excluding general query
    /// predictions.
    ///
    /// Use this when you only want to show navigable destinations to users.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let places: Vec<_> = response.places().collect();
    /// for place in places {
    ///     println!("Place ID: {}", place.place_id());
    /// }
    /// ```
    pub fn places(&self) -> impl Iterator<Item = &PlacePrediction> {
        self.suggestions.iter().filter_map(|s| s.as_place())
    }

    /// Returns an iterator over only the query predictions.
    ///
    /// Filters the suggestions to include only general search queries, excluding specific place
    /// predictions.
    ///
    /// Use this when you want to separate exploratory search suggestions from destination
    /// suggestions.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let queries: Vec<_> = response.queries().collect();
    /// for query in queries {
    ///     println!("Query: {}", query.query_string());
    /// }
    /// ```
    pub fn queries(&self) -> impl Iterator<Item = &QueryPrediction> {
        self.suggestions.iter().filter_map(|s| s.as_query())
    }

    /// Counts the number of place predictions in the response.
    ///
    /// Returns the count of specific place suggestions, excluding query predictions.
    #[must_use]
    pub fn place_count(&self) -> usize {
        self.suggestions.iter().filter(|s| s.is_place()).count()
    }

    /// Counts the number of query predictions in the response.
    ///
    /// Returns the count of general query suggestions, excluding place predictions.
    #[must_use]
    pub fn query_count(&self) -> usize {
        self.suggestions.iter().filter(|s| s.is_query()).count()
    }

    /// Formats all suggestions with HTML highlighting.
    ///
    /// Applies the same HTML tag to all suggestions (both places and queries) and returns them as a
    /// vector of formatted strings.
    ///
    /// Use this for simple HTML rendering where all suggestions should have the same style.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let html_suggestions = response.to_html_all("mark");
    /// for html in html_suggestions {
    ///     println!("<li>{}</li>", html);
    /// }
    /// ```
    #[must_use]
    pub fn to_html_all(&self, tag: &str) -> Vec<String> {
        self.suggestions
            .iter()
            .map(|suggestion| suggestion.to_html(tag))
            .collect()
    }

    /// Formats all suggestions with structured HTML highlighting.
    ///
    /// Applies different HTML tags to main and secondary text for all suggestions.
    ///
    /// Use this when you want hierarchical formatting with emphasized main text and de-emphasized
    /// context.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let html_suggestions = response.to_html_structured_all("strong", "em");
    /// // Each item: "<strong>Pizza</strong> Hut, <em>123 Main</em> St"
    /// ```
    #[must_use]
    pub fn to_html_structured_all(&self, main_tag: &str, secondary_tag: &str) -> Vec<String> {
        self.suggestions
            .iter()
            .map(|suggestion| suggestion.to_html_structured(main_tag, secondary_tag))
            .collect()
    }

    /// Formats all suggestions with a custom formatter function.
    ///
    /// Applies the same custom formatting function to all suggestions (both places and queries).
    /// Use this for consistent formatting in any output format (ANSI, Markdown, custom markup,
    /// etc.).
    ///
    /// # Examples
    ///
    /// **ANSI terminal colors:**
    /// ```rust,ignore
    /// let formatted = response.format_with_all(|text, is_matched| {
    ///     if is_matched {
    ///         format!("\x1b[1;32m{}\x1b[0m", text)  // Bold green
    ///     } else {
    ///         text.to_string()
    ///     }
    /// });
    /// ```
    ///
    /// **Markdown:**
    /// ```rust,ignore
    /// let formatted = response.format_with_all(|text, is_matched| {
    ///     if is_matched {
    ///         format!("**{}**", text)
    ///     } else {
    ///         text.to_string()
    ///     }
    /// });
    /// ```
    #[must_use]
    pub fn format_with_all<F>(&self, mut formatter: F) -> Vec<String>
    where
        F: FnMut(&str, bool) -> String,
    {
        self.suggestions
            .iter()
            .map(|s| s.format_with(&mut formatter))
            .collect()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::ops::Deref for Response {
    type Target = Vec<Suggestion>;

    /// Dereferences to the underlying suggestions vector.
    ///
    /// Allows using the response directly as a `Vec<Suggestion>` for operations like iteration,
    /// indexing, and length checks.
    fn deref(&self) -> &Self::Target {
        &self.suggestions
    }
}

impl std::ops::DerefMut for Response {
    /// Mutably dereferences to the underlying suggestions vector.
    ///
    /// Allows mutable access to the suggestions list.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.suggestions
    }
}

impl std::ops::Index<usize> for Response {
    type Output = Suggestion;

    /// Indexes into the suggestions list.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn index(&self, index: usize) -> &Self::Output {
        &self.suggestions[index]
    }
}

impl std::ops::IndexMut<usize> for Response {
    /// Mutably indexes into the suggestions list.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.suggestions[index]
    }
}

impl AsRef<[Suggestion]> for Response {
    /// Returns the suggestions as a slice.
    ///
    /// Allows passing the response to functions expecting `&[Suggestion]`.
    fn as_ref(&self) -> &[Suggestion] {
        &self.suggestions
    }
}

impl AsMut<[Suggestion]> for Response {
    /// Returns the suggestions as a mutable slice.
    ///
    /// Allows passing the response to functions expecting `&mut [Suggestion]`.
    fn as_mut(&mut self) -> &mut [Suggestion] {
        &mut self.suggestions
    }
}

impl IntoIterator for Response {
    type Item = Suggestion;
    type IntoIter = std::vec::IntoIter<Suggestion>;

    /// Converts the response into an iterator over suggestions.
    ///
    /// Consumes the response and allows iteration over the owned suggestions.
    fn into_iter(self) -> Self::IntoIter {
        self.suggestions.into_iter()
    }
}

impl<'a> IntoIterator for &'a Response {
    type Item = &'a Suggestion;
    type IntoIter = std::slice::Iter<'a, Suggestion>;

    /// Creates an iterator over borrowed suggestions.
    ///
    /// Allows iteration over the suggestions without consuming the response.
    fn into_iter(self) -> Self::IntoIter {
        self.suggestions.iter()
    }
}

impl<'a> IntoIterator for &'a mut Response {
    type Item = &'a mut Suggestion;
    type IntoIter = std::slice::IterMut<'a, Suggestion>;

    /// Creates a mutable iterator over suggestions.
    ///
    /// Allows mutable iteration over the suggestions without consuming the response.
    fn into_iter(self) -> Self::IntoIter {
        self.suggestions.iter_mut()
    }
}

impl Response {
    /// Returns an iterator over the suggestions.
    ///
    /// This is equivalent to calling `response.suggestions.iter()` or `(&response).into_iter()`.
    pub fn iter(&self) -> std::slice::Iter<'_, Suggestion> {
        self.suggestions.iter()
    }

    /// Returns a mutable iterator over the suggestions.
    ///
    /// This is equivalent to calling `response.suggestions.iter_mut()`.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Suggestion> {
        self.suggestions.iter_mut()
    }
}

impl std::convert::From<Response> for Result<Response, crate::Error> {
    /// Converts a Google Maps API `Response` into a `Result<Response, Error>` by examining the
    /// `status` field inside of the response.
    ///
    /// If the status indicates a success, then an `Ok(response)` will be returned. If the status
    /// indicates an error, then an `Err(error)` will be returned.
    fn from(response: Response) -> Self {
        match response.error {
            Some(error) => Err(crate::places_new::Error::from(error).into()),
            None => Ok(response),
        }
    }
}