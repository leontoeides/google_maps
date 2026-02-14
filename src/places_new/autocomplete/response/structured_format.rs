//! Contains the `StructuredFormat` struct for place and query predictions.
//!
//! Structured format breaks down predictions into main text (the primary name) and secondary text
//! (additional context), enabling better UI layouts and formatting control.

use crate::places_new::autocomplete::response::FormattableText;

// -------------------------------------------------------------------------------------------------
//
/// A breakdown of a prediction into main text and secondary text.
///
/// Separates predictions into two components: main text (the primary identifier like a place name
/// or query) and secondary text (additional disambiguating context like a city or region).
///
/// This structure enables UI designs where the main text can be emphasized while secondary text
/// provides supporting context:
///
/// * For Place predictions, the main text typically contains the specific name of the place, while
///   secondary text includes the address or location details.
///
/// * For query predictions, the main text contains the query itself, while secondary text provides
///   additional context.
///
/// # Examples
///
/// ```rust,ignore
/// // Place prediction
/// let format = StructuredFormat {
///     main_text: FormattableText::from("Pizza Hut"),
///     secondary_text: Some(FormattableText::from("123 Main St, San Francisco")),
/// };
///
/// println!("{}", format.main_text.text());     // "Pizza Hut"
/// println!("{}", format.secondary_text.unwrap().text()); // "123 Main St, San Francisco"
///
/// // With HTML highlighting on main text
/// let highlighted = format.main_text().to_html("strong");
/// // "<strong>Pizza</strong> Hut"
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
pub struct StructuredFormat {
    /// The main text representing the name of the Place or query.
    ///
    /// Contains the primary identifier or display name:
    /// * For places, this is typically the business or location name.
    /// * For queries, this is the search text itself.
    ///
    /// This text usually has match highlighting and should be displayed more prominently in the UI.
    #[getset(set = "pub", get_mut = "pub")]
    pub main_text: FormattableText,

    /// Additional disambiguating features for context.
    ///
    /// Contains secondary information like city, region, address, or other context that helps
    /// identify or refine the prediction.
    ///
    /// May be `None` if no additional context is available.
    ///
    /// Display this text with less emphasis than the main text, such as in a smaller font or muted
    /// color.
    #[getset(set = "pub", get_mut = "pub")]
    pub secondary_text: Option<FormattableText>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl StructuredFormat {
    /// Creates a new `StructuredFormat` with main and optional secondary text.
    ///
    /// Use this when constructing structured formats from API responses or when manually creating
    /// formatted predictions.
    #[must_use]
    pub const fn new(main_text: FormattableText, secondary_text: Option<FormattableText>) -> Self {
        Self {
            main_text,
            secondary_text,
        }
    }

    /// Creates a new `StructuredFormat` with only main text.
    ///
    /// Use this when you have a primary name or query but no additional context information.
    #[must_use]
    pub const fn new_main_only(main_text: FormattableText) -> Self {
        Self {
            main_text,
            secondary_text: None,
        }
    }

    /// Returns a reference to the main text.
    ///
    /// The primary identifier or name that should be displayed prominently. Use the returned
    /// `FormattableText` to access match ranges and apply HTML highlighting.
    ///
    /// For example: `format.main_text().to_html("mark")` to highlight matched portions.
    #[must_use]
    pub const fn main_text(&self) -> &FormattableText {
        &self.main_text
    }

    /// Returns a reference to the secondary text if present.
    ///
    /// Additional context or disambiguating information.
    ///
    /// Use the returned `FormattableText` to access match ranges and apply HTML highlighting.
    ///
    /// For example: `format.secondary_text().map(|t| t.to_html("em"))` to emphasize matched
    /// portions.
    #[must_use]
    pub const fn secondary_text(&self) -> Option<&FormattableText> {
        self.secondary_text.as_ref()
    }

    /// Checks if secondary text is present.
    ///
    /// Returns `true` if additional context information is available.
    #[must_use]
    pub const fn has_secondary_text(&self) -> bool {
        self.secondary_text.is_some()
    }

    /// Returns a combined display string with main and secondary text.
    ///
    /// Formats as `main_text`, `secondary_text` if both are present, or just `main_text` if
    /// secondary text is absent.
    ///
    /// Use this for simple text display without HTML formatting. For formatted output, use the
    /// individual getters with `to_html()` methods.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let format = StructuredFormat {
    ///     main_text: FormattableText::from("Pizza Hut"),
    ///     secondary_text: Some(FormattableText::from("San Francisco")),
    /// };
    ///
    /// println!("{}", format.combined_text());
    /// // "Pizza Hut, San Francisco"
    /// ```
    #[must_use]
    pub fn combined_text(&self) -> String {
        self.secondary_text
            .as_ref()
            .map_or_else(
                || self.main_text.text().clone(),
                |secondary| format!(
                    "{main}, {secondary}",
                    main = self.main_text.text(),
                    secondary = secondary.text()
                )
            )
    }

    /// Formats the main text with HTML highlighting.
    ///
    /// Convenience method that wraps `main_text().to_html(tag)`. Use this to quickly generate
    /// HTML-highlighted main text for UI display.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let html = format.main_html("strong");
    /// // "<strong>Pizza</strong> Hut"
    /// ```
    #[must_use]
    pub fn main_html(&self, tag: &str) -> String {
        self.main_text.to_html(tag)
    }

    /// Formats the secondary text with HTML highlighting if present.
    ///
    /// Convenience method that wraps `secondary_text().map(|t| t.to_html(tag))`.
    ///
    /// Use this to quickly generate HTML-highlighted secondary text for UI display. Returns
    /// `None` if no secondary text is available.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// if let Some(html) = format.secondary_html("em") {
    ///     println!("{}", html);
    ///     // "<em>San</em> Francisco"
    /// }
    /// ```
    #[must_use]
    pub fn secondary_html(&self, tag: &str) -> Option<String> {
        self.secondary_text.as_ref().map(|text| text.to_html(tag))
    }

    /// Formats the combined text with separate HTML tags for main and secondary.
    ///
    /// Applies different HTML tags to main and secondary text portions, then combines them with a
    /// comma separator. Use this to create formatted output where main and secondary text have
    /// different visual emphasis.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let html = format.combined_html("strong", "em");
    /// // "<strong>Pizza</strong> Hut, <em>San</em> Francisco"
    /// ```
    #[must_use]
    pub fn combined_html(&self, main_tag: &str, secondary_tag: &str) -> String {
        self.secondary_text
            .as_ref()
            .map_or_else(
                || self.main_text.to_html(main_tag),
                |secondary| format!(
                    "{main}, {secondary}",
                    main = self.main_text.to_html(main_tag),
                    secondary = secondary.to_html(secondary_tag)
                ))
    }

    /// Formats the main text with a custom formatter function.
    ///
    /// Applies a custom formatting function to the main text, distinguishing between matched and
    /// unmatched portions.
    ///
    /// This provides maximum flexibility for output formats beyond HTML, such as ANSI terminal
    /// codes, Markdown, or custom markup languages.
    ///
    /// # Examples
    ///
    /// **ANSI terminal colors (bold green for matches):**
    /// ```rust,ignore
    /// let formatted = format.main_format_with(|text, is_matched| {
    ///     if is_matched {
    ///         format!("\x1b[1;32m{}\x1b[0m", text)  // Bold green
    ///     } else {
    ///         text.to_string()
    ///     }
    /// });
    /// // "\x1b[1;32mPizza\x1b[0m Hut"
    /// ```
    ///
    /// **Markdown:**
    /// ```rust,ignore
    /// let formatted = format.main_format_with(|text, is_matched| {
    ///     if is_matched {
    ///         format!("**{}**", text)
    ///     } else {
    ///         text.to_string()
    ///     }
    /// });
    /// // "**Pizza** Hut"
    /// ```
    #[must_use]
    pub fn main_format_with<F>(&self, formatter: F) -> String
    where
        F: FnMut(&str, bool) -> String,
    {
        self.main_text.format_with(formatter)
    }

    /// Formats the secondary text with a custom formatter function if present.
    ///
    /// Applies a custom formatting function to the secondary text, distinguishing between matched
    /// and unmatched portions.
    ///
    /// Returns `None` if no secondary text is available. Use this for flexible formatting of
    /// contextual information in any output format.
    ///
    /// # Examples
    ///
    /// **ANSI terminal colors (italic cyan for matches):**
    /// ```rust,ignore
    /// if let Some(formatted) = format.secondary_format_with(|text, is_matched| {
    ///     if is_matched {
    ///         format!("\x1b[3;36m{}\x1b[0m", text)  // Italic cyan
    ///     } else {
    ///         text.to_string()
    ///     }
    /// }) {
    ///     println!("{}", formatted);
    ///     // "\x1b[3;36mSan\x1b[0m Francisco"
    /// }
    /// ```
    ///
    /// **Markdown:**
    /// ```rust,ignore
    /// if let Some(formatted) = format.secondary_format_with(|text, is_matched| {
    ///     if is_matched {
    ///         format!("*{}*", text)
    ///     } else {
    ///         text.to_string()
    ///     }
    /// }) {
    ///     println!("{}", formatted);
    ///     // "*San* Francisco"
    /// }
    /// ```
    #[must_use]
    pub fn secondary_format_with<F>(&self, formatter: F) -> Option<String>
    where
        F: FnMut(&str, bool) -> String,
    {
        self.secondary_text
            .as_ref()
            .map(|text| text.format_with(formatter))
    }

    /// Formats the combined text with separate custom formatters for main and secondary.
    ///
    /// Applies different formatting functions to main and secondary text portions, then combines
    /// them with a comma separator.
    ///
    /// Use this to create formatted output where main and secondary text have different visual
    /// styles in any output format (ANSI, Markdown, custom markup, etc.).
    ///
    /// # Examples
    ///
    /// **ANSI terminal colors (bold green main, italic cyan secondary):**
    /// ```rust,ignore
    /// let formatted = format.combined_format_with(
    ///     |text, is_matched| {
    ///         if is_matched {
    ///             format!("\x1b[1;32m{}\x1b[0m", text)  // Bold green
    ///         } else {
    ///             text.to_string()
    ///         }
    ///     },
    ///     |text, is_matched| {
    ///         if is_matched {
    ///             format!("\x1b[3;36m{}\x1b[0m", text)  // Italic cyan
    ///         } else {
    ///             text.to_string()
    ///         }
    ///     }
    /// );
    /// // "\x1b[1;32mPizza\x1b[0m Hut, \x1b[3;36mSan\x1b[0m Francisco"
    /// ```
    ///
    /// **Markdown (bold main, italic secondary):**
    /// ```rust,ignore
    /// let formatted = format.combined_format_with(
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
    /// // "**Pizza** Hut, *San* Francisco"
    /// ```
    ///
    /// **Custom markers (main: <<>>, secondary: [[]]):**
    /// ```rust,ignore
    /// let formatted = format.combined_format_with(
    ///     |text, is_matched| {
    ///         if is_matched {
    ///             format!("<<{}>>", text)
    ///         } else {
    ///             text.to_string()
    ///         }
    ///     },
    ///     |text, is_matched| {
    ///         if is_matched {
    ///             format!("[[{}]]", text)
    ///         } else {
    ///             text.to_string()
    ///         }
    ///     }
    /// );
    /// // "<<Pizza>> Hut, [[San]] Francisco"
    /// ```
    #[must_use]
    #[allow(clippy::similar_names)]
    pub fn combined_format_with<F, G>(&self, mut main_formatter: F, secondary_formatter: G) -> String
    where
        F: FnMut(&str, bool) -> String,
        G: FnMut(&str, bool) -> String,
    {
        let main_formatted = self.main_text.format_with(&mut main_formatter);
        match &self.secondary_text {
            Some(secondary) => {
                format!(
                    "{main}, {secondary}",
                    main = main_formatted,
                    secondary = secondary.format_with(secondary_formatter)
                )
            }
            None => main_formatted,
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::fmt::Display for StructuredFormat {
    /// Formats as the combined plain text.
    ///
    /// Returns `main_text`, `secondary_text` for machine-readable output without HTML formatting.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.combined_text())
    }
}