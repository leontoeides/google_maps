//! Contains the `Suggestion` enum for autocomplete results.
//!
//! Suggestions can be either specific places or general query suggestions, allowing autocomplete
//! responses to mix both types of results.

use crate::places_new::autocomplete::response::{PlacePrediction, QueryPrediction};

// -------------------------------------------------------------------------------------------------
//
/// An Autocomplete suggestion result.
///
/// Represents a single suggestion from the autocomplete API, which can be either a specific place
/// (like "Pizza Hut, 123 Main St") or a general query suggestion (like "pizza restaurants near
/// me"). The API may return a mix of both types, allowing users to either navigate to a specific
/// location or explore a broader search query.
///
/// # Examples
///
/// ```rust,ignore
/// match suggestion {
///     Suggestion::PlacePrediction(place) => {
///         println!("Place: {}", place.text().text());
///         // Use place.place_id() for Place Details lookup
///     }
///     Suggestion::QueryPrediction(query) => {
///         println!("Query: {}", query.query_string());
///         // Use query text for search endpoint
///     }
/// }
///
/// // Format with HTML regardless of type
/// let html = match &suggestion {
///     Suggestion::PlacePrediction(p) => p.to_html("strong"),
///     Suggestion::QueryPrediction(q) => q.to_html("strong"),
/// };
/// ```
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Suggestion {
    /// A prediction for a specific Place.
    ///
    /// Represents a concrete location or establishment that the user can navigate to or get details
    /// about. Place predictions include identifiers that can be used with other Google Maps APIs.
    PlacePrediction(PlacePrediction),

    /// A prediction for a general query.
    ///
    /// Represents a search text suggestion that can be used in search endpoints to find relevant
    /// places. Query predictions are exploratory and don't represent specific locations.
    QueryPrediction(QueryPrediction),
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Suggestion {
    /// Checks if this suggestion is a place prediction.
    ///
    /// Returns `true` if the suggestion represents a specific place rather than a general query.
    #[must_use]
    pub const fn is_place(&self) -> bool {
        matches!(self, Self::PlacePrediction(_))
    }

    /// Checks if this suggestion is a query prediction.
    ///
    /// Returns `true` if the suggestion represents a general search query rather than a specific
    /// place.
    #[must_use]
    pub const fn is_query(&self) -> bool {
        matches!(self, Self::QueryPrediction(_))
    }

    /// Returns a reference to the place prediction if this is a place.
    ///
    /// Use this to access place-specific information like place IDs, types, and distance
    /// measurements. Returns `None` if this is a query prediction.
    #[must_use]
    pub const fn as_place(&self) -> Option<&PlacePrediction> {
        match self {
            Self::PlacePrediction(place) => Some(place),
            Self::QueryPrediction(_) => None,
        }
    }

    /// Returns a reference to the query prediction if this is a query.
    ///
    /// Use this to access query text for search operations. Returns `None` if this is a place
    /// prediction.
    #[must_use]
    pub const fn as_query(&self) -> Option<&QueryPrediction> {
        match self {
            Self::PlacePrediction(_) => None,
            Self::QueryPrediction(query) => Some(query),
        }
    }

    /// Returns the display text for this suggestion.
    ///
    /// Extracts the text content regardless of whether this is a place or query prediction. Use
    /// this when you need the text without caring about the suggestion type.
    #[must_use]
    pub fn text(&self) -> &str {
        match self {
            Self::PlacePrediction(place) => place.text().text(),
            Self::QueryPrediction(query) => query.text().text(),
        }
    }

    /// Formats the suggestion text with HTML highlighting.
    ///
    /// Applies HTML tags to matched portions regardless of suggestion type. Use this for simple
    /// HTML formatting without needing to match on the enum variant.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let html = suggestion.to_html("mark");
    /// // "<mark>Pizza</mark> Hut" (for either place or query)
    /// ```
    #[must_use]
    pub fn to_html(&self, tag: &str) -> String {
        match self {
            Self::PlacePrediction(place) => place.to_html(tag),
            Self::QueryPrediction(query) => query.to_html(tag),
        }
    }

    /// Formats the suggestion with structured HTML if available.
    ///
    /// Applies different HTML tags to main and secondary text portions if structured format is
    /// available. Otherwise, falls back to highlighting the full text with the main tag. Works for
    /// both place and query predictions.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let html = suggestion.to_html_structured("strong", "em");
    /// // "<strong>Pizza</strong> Hut, <em>123 Main</em> St"
    /// ```
    #[must_use]
    pub fn to_html_structured(&self, main_tag: &str, secondary_tag: &str) -> String {
        match self {
            Self::PlacePrediction(place) => place.to_html_structured(main_tag, secondary_tag),
            Self::QueryPrediction(query) => query.to_html_structured(main_tag, secondary_tag),
        }
    }

    /// Returns the Place ID associated with the suggestion. If it's a place prediction this will
    /// return `Some`, if it's a query prediction it will return `None`.
    #[must_use]
    pub fn place_id(&self) -> Option<&str> {
        match self {
            Self::PlacePrediction(place) => Some(place.place_id.as_str()),
            Self::QueryPrediction(_query) => None,
        }
    }

    /// Formats the suggestion with a custom formatter function.
    ///
    /// Applies a custom formatting function to the text, distinguishing between matched and
    /// unmatched portions. Works for both place and query predictions, providing maximum
    /// flexibility for any output format (ANSI, Markdown, etc.).
    ///
    /// # Examples
    ///
    /// **ANSI terminal colors:**
    /// ```rust,ignore
    /// let formatted = suggestion.format_with(|text, is_matched| {
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
    /// let formatted = suggestion.format_with(|text, is_matched| {
    ///     if is_matched {
    ///         format!("**{}**", text)
    ///     } else {
    ///         text.to_string()
    ///     }
    /// });
    /// ```
    #[must_use]
    pub fn format_with<F>(&self, formatter: F) -> String
    where
        F: FnMut(&str, bool) -> String,
    {
        match self {
            Self::PlacePrediction(place) => place.format_with(formatter),
            Self::QueryPrediction(query) => query.format_with(formatter),
        }
    }

    /// Formats with custom formatters using structured format if available.
    ///
    /// Applies different formatting functions to main and secondary text if structured format
    /// exists. Works for both place and query predictions. Falls back to the main formatter only if
    /// structured format is not available.
    ///
    /// # Examples
    ///
    /// **ANSI terminal colors:**
    /// ```rust,ignore
    /// let formatted = suggestion.format_with_structured(
    ///     |text, is_matched| {
    ///         if is_matched {
    ///             format!("\x1b[1;33m{}\x1b[0m", text)  // Bold yellow
    ///         } else {
    ///             text.to_string()
    ///         }
    ///     },
    ///     |text, is_matched| {
    ///         if is_matched {
    ///             format!("\x1b[36m{}\x1b[0m", text)  // Cyan
    ///         } else {
    ///             text.to_string()
    ///         }
    ///     }
    /// );
    /// ```
    #[must_use]
    pub fn format_with_structured<F, G>(&self, main_formatter: F, secondary_formatter: G) -> String
    where
        F: FnMut(&str, bool) -> String,
        G: FnMut(&str, bool) -> String,
    {
        match self {
            Self::PlacePrediction(place) => {
                place.format_with_structured(main_formatter, secondary_formatter)
            }
            Self::QueryPrediction(query) => {
                query.format_with_structured(main_formatter, secondary_formatter)
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::fmt::Display for Suggestion {
    /// Formats as the plain suggestion text.
    ///
    /// Returns the text content without formatting, regardless of whether this is a place or query
    /// prediction.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl From<PlacePrediction> for Suggestion {
    /// Converts a `PlacePrediction` to a `Suggestion`.
    ///
    /// Wraps a place prediction in the suggestion enum. Use this for convenience when constructing
    /// suggestion lists.
    fn from(place: PlacePrediction) -> Self {
        Self::PlacePrediction(place)
    }
}

impl From<QueryPrediction> for Suggestion {
    /// Converts a `QueryPrediction` to a `Suggestion`.
    ///
    /// Wraps a query prediction in the suggestion enum. Use this for convenience when constructing
    /// suggestion lists.
    fn from(query: QueryPrediction) -> Self {
        Self::QueryPrediction(query)
    }
}