//! Contains the `FormattableText` struct for place and query predictions.
//!
//! Formattable text combines display text with match ranges, allowing UI highlighting of portions
//! that matched the user's input query.

use crate::places_new::autocomplete::response::StringRange;

// -------------------------------------------------------------------------------------------------
//
/// Text representing a Place or query prediction with match highlighting.
///
/// Contains the full display text along with ranges indicating which portions matched the user's
/// input.
///
/// The text can be displayed as-is or formatted with the match ranges to highlight relevant
/// segments in the UI. This is commonly used in autocomplete results to show users which parts of
/// the suggestion correspond to their search query.
///
/// # Examples
///
/// ```rust,ignore
/// let formattable = FormattableText {
///     text: "Pizza Hut, San Francisco".to_string(),
///     matches: vec![StringRange::new(0, 5)], // "Pizza" matched
/// };
///
/// // Highlight the matching portion
/// for range in &formattable.matches {
///     if let Some(matched) = formattable.match_text(range) {
///         println!("Matched: {}", matched); // "Pizza"
///     }
/// }
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
    // other
    bon::Builder
)]
#[serde(rename_all = "camelCase")]
pub struct FormattableText {
    /// The full text that may be displayed or formatted.
    ///
    /// Contains the complete display string for the prediction, such as a place name with address
    /// or a query suggestion.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub text: String,

    /// List of string ranges identifying where the input matched.
    ///
    /// Unicode character offsets indicating which portions of `text` matched the user's input
    /// query.
    ///
    /// These ranges can be used to highlight or bold matching segments in the UI. May be empty if
    /// no specific matches are identified.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[getset(set = "pub", get_mut = "pub")]
    pub matches: Vec<StringRange>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl FormattableText {
    /// Creates a new `FormattableText` with text and match ranges.
    ///
    /// Use this when constructing formattable text from API responses or when manually creating
    /// formatted predictions with known match ranges.
    #[must_use]
    pub const fn new(text: String, matches: Vec<StringRange>) -> Self {
        Self { text, matches }
    }

    /// Creates a new `FormattableText` with text and no matches.
    ///
    /// Use this when you have display text but no specific match information, such as when creating
    /// fallback suggestions or when match highlighting is not available.
    #[must_use]
    pub const fn new_unmatched(text: String) -> Self {
        Self {
            text,
            matches: Vec::new(),
        }
    }

    /// Returns a reference to the match ranges.
    ///
    /// The list of ranges indicating which portions of the text matched the user's input query.
    #[must_use]
    pub fn matches(&self) -> &[StringRange] {
        &self.matches
    }

    /// Checks if there are any match ranges.
    ///
    /// Returns `true` if at least one match range exists, indicating that some portion of the text
    /// matched the user's input.
    #[must_use]
    pub fn has_matches(&self) -> bool {
        !self.matches.is_empty()
    }

    /// Extracts the matching text for a given range.
    ///
    /// Returns the substring of `text` corresponding to the provided range. Returns `None` if the
    /// range is invalid or outside the text bounds.
    ///
    /// Use this to extract matched portions for highlighting or analysis.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let formattable = FormattableText {
    ///     text: "Pizza Hut".to_string(),
    ///     matches: vec![StringRange::new(0, 5)],
    /// };
    ///
    /// if let Some(matched) = formattable.match_text(&formattable.matches[0]) {
    ///     println!("{}", matched); // "Pizza"
    /// }
    /// ```
    #[must_use]
    pub fn match_text(&self, range: StringRange) -> Option<&str> {
        self.text.get(range.range())
    }

    /// Returns all matched text segments as a vector of string slices.
    ///
    /// Extracts all portions of the text that matched the user's input based on the match ranges.
    /// Use this to collect all matching segments for display or processing. Skips any invalid
    /// ranges.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let formattable = FormattableText {
    ///     text: "Pizza Hut, San Francisco".to_string(),
    ///     matches: vec![
    ///         StringRange::new(0, 5),   // "Pizza"
    ///         StringRange::new(11, 14), // "San"
    ///     ],
    /// };
    ///
    /// let matched = formattable.all_matches();
    /// // matched = ["Pizza", "San"]
    /// ```
    #[must_use]
    pub fn all_matches(&self) -> Vec<&str> {
        self.matches
            .iter()
            .filter_map(|range| self.match_text(*range))
            .collect()
    }

    /// Returns the unmatched portions of the text.
    ///
    /// Extracts all segments that did not match the user's input by computing the gaps between
    /// match ranges. Use this for advanced formatting where you need to style matched and unmatched
    /// portions differently.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let formattable = FormattableText {
    ///     text: "Pizza Hut".to_string(),
    ///     matches: vec![StringRange::new(0, 5)],
    /// };
    ///
    /// let unmatched = formattable.unmatched_portions();
    /// // unmatched = [" Hut"]
    /// ```
    #[must_use]
    pub fn unmatched_portions(&self) -> Vec<&str> {
        let mut portions = Vec::new();
        let mut last_end = 0;

        for range in &self.matches {
            let r = range.range();

            if r.start > last_end {
                if let Some(unmatched) = self.text.get(last_end..r.start) {
                    portions.push(unmatched);
                }
            }

            last_end = r.end;
        }

        if last_end < self.text.len() {
            if let Some(unmatched) = self.text.get(last_end..) {
                portions.push(unmatched);
            }
        }

        portions
    }

    /// Formats the text with HTML tags around matched portions.
    ///
    /// Wraps each matched segment in the specified HTML tag for highlighting in web UIs.
    ///
    /// Use this to generate HTML where matched portions are visually emphasized. Common tags
    /// include `b` (bold), `strong` (strong emphasis), `mark` (highlight), `em` (italic), or `code`
    /// (monospace).
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let formattable = FormattableText {
    ///     text: "Pizza Hut, San Francisco".to_string(),
    ///     matches: vec![
    ///         StringRange::new(0, 5),   // "Pizza"
    ///         StringRange::new(11, 14), // "San"
    ///     ],
    /// };
    ///
    /// let html = formattable.to_html("strong");
    /// // "<strong>Pizza</strong> Hut, <strong>San</strong> Francisco"
    ///
    /// let highlighted = formattable.to_html("mark");
    /// // "<mark>Pizza</mark> Hut, <mark>San</mark> Francisco"
    /// ```
    #[must_use]
    pub fn to_html(&self, tag: &str) -> String {
        let mut result = String::with_capacity(self.text.len() + self.matches.len() * 20);
        let mut last_end = 0;

        for range in &self.matches {
            let r = range.range();

            // Add unmatched text before this match
            if r.start > last_end {
                if let Some(unmatched) = self.text.get(last_end..r.start) {
                    result.push_str(unmatched);
                }
            }

            // Add matched text wrapped in tags
            if let Some(matched) = self.text.get(r.clone()) {
                result.push('<');
                result.push_str(tag);
                result.push('>');
                result.push_str(matched);
                result.push_str("</");
                result.push_str(tag);
                result.push('>');
            }

            last_end = r.end;
        }

        // Add any remaining unmatched text
        if last_end < self.text.len() {
            if let Some(remaining) = self.text.get(last_end..) {
                result.push_str(remaining);
            }
        }

        result
    }

    /// Formats the text with a custom HTML tag and CSS class.
    ///
    /// Wraps each matched segment in the specified HTML tag with a CSS class attribute for more
    /// flexible styling. Use this when you need custom CSS styling of matched portions.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let formattable = FormattableText {
    ///     text: "Pizza Hut".to_string(),
    ///     matches: vec![StringRange::new(0, 5)],
    /// };
    ///
    /// let html = formattable.to_html_with_class("span", "highlight");
    /// // "<span class=\"highlight\">Pizza</span> Hut"
    /// ```
    #[must_use]
    pub fn to_html_with_class(&self, tag: &str, class: &str) -> String {
        let mut result = String::with_capacity(self.text.len() + self.matches.len() * 30);
        let mut last_end = 0;

        for range in &self.matches {
            let r = range.range();

            // Add unmatched text before this match
            if r.start > last_end {
                if let Some(unmatched) = self.text.get(last_end..r.start) {
                    result.push_str(unmatched);
                }
            }

            // Add matched text wrapped in tags with class
            if let Some(matched) = self.text.get(r.clone()) {
                result.push('<');
                result.push_str(tag);
                result.push_str(" class=\"");
                result.push_str(class);
                result.push_str("\">");
                result.push_str(matched);
                result.push_str("</");
                result.push_str(tag);
                result.push('>');
            }

            last_end = r.end;
        }

        // Add any remaining unmatched text
        if last_end < self.text.len() {
            if let Some(remaining) = self.text.get(last_end..) {
                result.push_str(remaining);
            }
        }

        result
    }

    /// Formats the text with a custom formatter function.
    ///
    /// Applies a custom formatting function to each text segment, distinguishing between matched
    /// and unmatched portions.
    ///
    /// This provides maximum flexibility for output formats beyond HTML, such as ANSI terminal
    /// codes, Markdown, or custom markup languages.
    ///
    /// The formatter function receives two arguments:
    /// * `text` · The text segment to format
    /// * `is_matched` · `true` if this segment matched the user's input, `false` otherwise
    ///
    /// # Examples
    ///
    /// **HTML (equivalent to `to_html("b")`):**
    /// ```rust,ignore
    /// let formatted = formattable.format_with(|text, is_matched| {
    ///     if is_matched {
    ///         format!("<b>{}</b>", text)
    ///     } else {
    ///         text.to_string()
    ///     }
    /// });
    /// // "<b>Pizza</b> Hut"
    /// ```
    ///
    /// **ANSI terminal colors (bold red for matches):**
    /// ```rust,ignore
    /// let formatted = formattable.format_with(|text, is_matched| {
    ///     if is_matched {
    ///         format!("\x1b[1;31m{}\x1b[0m", text)  // Bold red
    ///     } else {
    ///         text.to_string()
    ///     }
    /// });
    /// // "\x1b[1;31mPizza\x1b[0m Hut"
    /// ```
    ///
    /// **Markdown (bold for matches):**
    /// ```rust,ignore
    /// let formatted = formattable.format_with(|text, is_matched| {
    ///     if is_matched {
    ///         format!("**{}**", text)
    ///     } else {
    ///         text.to_string()
    ///     }
    /// });
    /// // "**Pizza** Hut"
    /// ```
    ///
    /// **Custom markers:**
    /// ```rust,ignore
    /// let formatted = formattable.format_with(|text, is_matched| {
    ///     if is_matched {
    ///         format!("<<{}>>", text)
    ///     } else {
    ///         text.to_string()
    ///     }
    /// });
    /// // "<<Pizza>> Hut"
    /// ```
    #[must_use]
    pub fn format_with<F>(&self, mut formatter: F) -> String
    where
        F: FnMut(&str, bool) -> String,
    {
        let mut result = String::with_capacity(self.text.len() + self.matches.len() * 20);
        let mut last_end = 0;

        for range in &self.matches {
            let r = range.range();

            // Add unmatched text before this match
            if r.start > last_end {
                if let Some(unmatched) = self.text.get(last_end..r.start) {
                    result.push_str(&formatter(unmatched, false));
                }
            }

            // Add matched text with formatting
            if let Some(matched) = self.text.get(r.clone()) {
                result.push_str(&formatter(matched, true));
            }

            last_end = r.end;
        }

        // Add any remaining unmatched text
        if last_end < self.text.len() {
            if let Some(remaining) = self.text.get(last_end..) {
                result.push_str(&formatter(remaining, false));
            }
        }

        result
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::fmt::Display for FormattableText {
    /// Formats as the plain text without match highlighting.
    ///
    /// Returns just the text content for machine-readable output or contexts where highlighting is
    /// not needed.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl From<String> for FormattableText {
    /// Converts a `String` to `FormattableText` with no matches.
    ///
    /// Creates formattable text from a plain string with an empty match list.
    ///
    /// Use this for convenience when you only have text without match information.
    fn from(text: String) -> Self {
        Self::new_unmatched(text)
    }
}

impl From<&str> for FormattableText {
    /// Converts a `&str` to `FormattableText` with no matches.
    ///
    /// Creates formattable text from a string slice with an empty match list.
    ///
    /// Use this for convenience when you only have text without match information.
    fn from(text: &str) -> Self {
        Self::new_unmatched(text.to_string())
    }
}