//! Contains the `QueryPrediction` struct for autocomplete query suggestions.
//!
//! Query predictions represent search text suggestions (as opposed to specific places) that can be
//! used in search endpoints like Text Search.

use crate::places_new::autocomplete::response::{FormattableText, StructuredFormat};

// -------------------------------------------------------------------------------------------------
//
/// Prediction results for a Query Autocomplete prediction.
///
/// Represents a text query suggestion rather than a specific place. Query predictions are search
/// strings that could be used in a search endpoint (such as Text Search) to find relevant places or
/// information.
///
/// Use these to provide search-as-you-type suggestions where the user hasn't yet specified a
/// particular place but is exploring general search terms.
///
/// # Examples
///
/// ```rust,ignore
/// let prediction = QueryPrediction {
///     text: FormattableText {
///         text: "pizza restaurants near me".to_string(),
///         matches: vec![StringRange::new(0, 5)], // "pizza" matched
///     },
///     structured_format: Some(StructuredFormat {
///         main_text: FormattableText::from("pizza restaurants"),
///         secondary_text: Some(FormattableText::from("near me")),
///     }),
/// };
///
/// // Use the query for a search
/// println!("Search for: {}", prediction.text.text());
/// // "pizza restaurants near me"
///
/// // Display with HTML highlighting
/// let html = prediction.text().to_html("strong");
/// // "<strong>pizza</strong> restaurants near me"
/// ```
#[derive(
    // std
    Clone,
    Debug,
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
#[serde(rename_all = "camelCase")]
pub struct QueryPrediction {
    /// The predicted query text.
    ///
    /// This text does not represent a specific Place, but rather a text query that could be used in
    /// a search endpoint (for example, Text Search). May be in mixed languages if the request input
    /// or query is in different languages.
    ///
    /// Use this text directly as a search query when the user selects this prediction.
    #[getset(set = "pub", get_mut = "pub")]
    pub text: FormattableText,

    /// Structured breakdown of the query prediction.
    ///
    /// A breakdown of the query into main text containing the query and secondary text containing
    /// additional disambiguating features (such as a city or region). May be `None` if structured
    /// formatting is not available.
    ///
    /// Use this to display the query with visual separation between the main query and contextual
    /// information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[getset(set = "pub", get_mut = "pub")]
    pub structured_format: Option<StructuredFormat>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl QueryPrediction {
    /// Creates a new `QueryPrediction` with text and optional structured format.
    ///
    /// Use this when constructing query predictions from API responses or when manually creating
    /// predictions with known query text.
    #[must_use]
    pub const fn new(text: FormattableText, structured_format: Option<StructuredFormat>) -> Self {
        Self {
            text,
            structured_format,
        }
    }

    /// Creates a new `QueryPrediction` with only query text.
    ///
    /// Use this when you have query text but no structured format breakdown, such as for simple
    /// query suggestions without additional context.
    #[must_use]
    pub const fn new_unstructured(text: FormattableText) -> Self {
        Self {
            text,
            structured_format: None,
        }
    }

    /// Returns a reference to the query text.
    ///
    /// The full predicted query string that can be used directly in search endpoints.
    ///
    /// Use the returned `FormattableText` to access match ranges and apply HTML highlighting.
    /// For example: `prediction.text().to_html("mark")` to highlight matched portions.
    #[must_use]
    pub const fn text(&self) -> &FormattableText {
        &self.text
    }

    /// Returns a reference to the structured format if present.
    ///
    /// The breakdown of the query into main and secondary text components.
    ///
    /// Use this to display the query with visual hierarchy where the main query is emphasized and
    /// context is de-emphasized.
    #[must_use]
    pub const fn structured_format(&self) -> Option<&StructuredFormat> {
        self.structured_format.as_ref()
    }

    /// Checks if structured format is available.
    ///
    /// Returns `true` if the query has a structured breakdown into main and secondary text
    /// components.
    #[must_use]
    pub const fn has_structured_format(&self) -> bool {
        self.structured_format.is_some()
    }

    /// Returns the plain query text string.
    ///
    /// Extracts the full query text without formatting or match highlighting.
    ///
    /// Use this when you need the raw query string for search operations or logging.
    #[must_use]
    pub fn query_string(&self) -> &str {
        self.text.text()
    }

    /// Formats the query text with HTML highlighting.
    ///
    /// Convenience method that wraps `text().to_html(tag)`. Use this to quickly generate
    /// HTML-highlighted query text for UI display.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let html = prediction.to_html("mark");
    /// // "<mark>pizza</mark> restaurants near me"
    /// ```
    #[must_use]
    pub fn to_html(&self, tag: &str) -> String {
        self.text.to_html(tag)
    }

    /// Formats the query with HTML using structured format if available.
    ///
    /// If structured format is present, applies different HTML tags to the main query and
    /// secondary context. Otherwise, falls back to highlighting the full query text with the main
    /// tag.
    ///
    /// Use this to create formatted output where the main query and context have different visual
    /// emphasis.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // With structured format
    /// let html = prediction.to_html_structured("strong", "em");
    /// // "<strong>pizza</strong> restaurants, <em>near</em> me"
    ///
    /// // Without structured format (fallback)
    /// let html = prediction.to_html_structured("strong", "em");
    /// // "<strong>pizza</strong> restaurants near me"
    /// ```
    #[must_use]
    pub fn to_html_structured(&self, main_tag: &str, secondary_tag: &str) -> String {
        self.structured_format
            .as_ref()
            .map_or_else(
                || self.text.to_html(main_tag),
                |format| format.combined_html(main_tag, secondary_tag)
            )
    }

    /// Formats the query text with a custom formatter function.
    ///
    /// Applies a custom formatting function to the query text, distinguishing between matched and
    /// unmatched portions. This provides maximum flexibility for output formats beyond HTML, such
    /// as ANSI terminal codes, Markdown, or custom markup languages.
    ///
    /// # Examples
    ///
    /// **ANSI terminal colors (bold magenta for matches):**
    /// ```rust,ignore
    /// let formatted = prediction.format_with(|text, is_matched| {
    ///     if is_matched {
    ///         format!("\x1b[1;35m{}\x1b[0m", text)  // Bold magenta
    ///     } else {
    ///         text.to_string()
    ///     }
    /// });
    /// // "\x1b[1;35mpizza\x1b[0m restaurants near me"
    /// ```
    ///
    /// **Markdown:**
    /// ```rust,ignore
    /// let formatted = prediction.format_with(|text, is_matched| {
    ///     if is_matched {
    ///         format!("**{}**", text)
    ///     } else {
    ///         text.to_string()
    ///     }
    /// });
    /// // "**pizza** restaurants near me"
    /// ```
    ///
    /// **Custom markers:**
    /// ```rust,ignore
    /// let formatted = prediction.format_with(|text, is_matched| {
    ///     if is_matched {
    ///         format!("{{{}}}}", text)
    ///     } else {
    ///         text.to_string()
    ///     }
    /// });
    /// // "{pizza} restaurants near me"
    /// ```
    #[must_use]
    pub fn format_with<F>(&self, formatter: F) -> String
    where
        F: FnMut(&str, bool) -> String,
    {
        self.text.format_with(formatter)
    }

    /// Formats the query with custom formatters using structured format if available.
    ///
    /// If structured format is present, applies different formatting functions to the main query
    /// and secondary context, then combines them with a comma separator. Otherwise, falls back to
    /// formatting the full query text with the main formatter.
    ///
    /// Use this to create formatted output where main and secondary text have different visual
    /// styles in any output format (ANSI, Markdown, custom markup, etc.).
    ///
    /// # Examples
    ///
    /// **ANSI terminal colors (bold yellow main, dim white secondary):**
    /// ```rust,ignore
    /// let formatted = prediction.format_with_structured(
    ///     |text, is_matched| {
    ///         if is_matched {
    ///             format!("\x1b[1;33m{}\x1b[0m", text)  // Bold yellow
    ///         } else {
    ///             text.to_string()
    ///         }
    ///     },
    ///     |text, is_matched| {
    ///         if is_matched {
    ///             format!("\x1b[2;37m{}\x1b[0m", text)  // Dim white
    ///         } else {
    ///             text.to_string()
    ///         }
    ///     }
    /// );
    /// // "\x1b[1;33mpizza\x1b[0m restaurants, \x1b[2;37mnear\x1b[0m me"
    /// ```
    ///
    /// **Markdown (bold main, italic secondary):**
    /// ```rust,ignore
    /// let formatted = prediction.format_with_structured(
    ///     |text, is_matched| {
    ///         if is_matched {
    ///             format!("**{}**", text)
    ///         } else {
    ///             text.to_string()
    ///         }
    ///     },
    ///     |text, is_matched| {
    ///         if is_matched {
    ///             format!("*{}*", text)
    ///         } else {
    ///             text.to_string()
    ///         }
    ///     }
    /// );
    /// // "**pizza** restaurants, *near* me"
    /// ```
    ///
    /// **Fallback when no structured format (uses main formatter only):**
    /// ```rust,ignore
    /// let formatted = prediction.format_with_structured(
    ///     |text, is_matched| {
    ///         if is_matched { format!("**{}**", text) } else { text.to_string() }
    ///     },
    ///     |text, is_matched| {
    ///         if is_matched { format!("*{}*", text) } else { text.to_string() }
    ///     }
    /// );
    /// // "**pizza** restaurants near me"  (no secondary formatter applied)
    /// ```
    #[must_use]
    pub fn format_with_structured<F, G>(&self, main_formatter: F, secondary_formatter: G) -> String
    where
        F: FnMut(&str, bool) -> String,
        G: FnMut(&str, bool) -> String,
    {
        match &self.structured_format {
            Some(format) => format.combined_format_with(main_formatter, secondary_formatter),
            None => self.text.format_with(main_formatter),
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::fmt::Display for QueryPrediction {
    /// Formats as the plain query text.
    ///
    /// Returns the full query string for machine-readable output without HTML formatting.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl From<String> for QueryPrediction {
    /// Converts a `String` to an unstructured `QueryPrediction`.
    ///
    /// Creates a query prediction from a plain string with no match highlighting or structured
    /// format.
    ///
    /// Use this for convenience when you only have query text without additional metadata.
    fn from(text: String) -> Self {
        Self::new_unstructured(FormattableText::from(text))
    }
}

impl From<&str> for QueryPrediction {
    /// Converts a `&str` to an unstructured `QueryPrediction`.
    ///
    /// Creates a query prediction from a string slice with no match highlighting or structured
    /// format.
    ///
    /// Use this for convenience when you only have query text without additional metadata.
    fn from(text: &str) -> Self {
        Self::new_unstructured(FormattableText::from(text))
    }
}

impl From<FormattableText> for QueryPrediction {
    /// Converts a `FormattableText` to an unstructured `QueryPrediction`.
    ///
    /// Creates a query prediction from formattable text with match highlighting but no structured
    /// format.
    ///
    /// Use this when you have formatted text but no breakdown into main and secondary components.
    fn from(text: FormattableText) -> Self {
        Self::new_unstructured(text)
    }
}