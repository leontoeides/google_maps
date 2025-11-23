//! Contains the `PlacePrediction` struct for autocomplete place suggestions.
//!
//! Place predictions represent specific locations or establishments (as opposed to general query
//! suggestions) that can be used for navigation or information lookup.

use crate::places_new::autocomplete::response::{FormattableText, StructuredFormat};
use crate::places_new::PlaceType;
use rust_decimal::{Decimal, prelude::FromPrimitive};

// -------------------------------------------------------------------------------------------------
//
/// Prediction results for a Place Autocomplete prediction.
///
/// Represents a specific place or establishment suggestion, such as a business, landmark, or
/// address.
///
/// Place predictions include resource identifiers that can be used with other Google Maps APIs
/// (like Place Details) to fetch complete information about the location.
///
/// Use these for autocomplete where users are searching for specific destinations rather than
/// general queries.
///
/// # Examples
///
/// ```rust,ignore
/// let prediction = PlacePrediction {
///     place: "places/ChIJN1t_tDeuEmsRUsoyG83frY4".to_string(),
///     place_id: "ChIJN1t_tDeuEmsRUsoyG83frY4".to_string(),
///     text: FormattableText {
///         text: "Pizza Hut, 123 Main St".to_string(),
///         matches: vec![StringRange::new(0, 5)],
///     },
///     structured_format: Some(StructuredFormat {
///         main_text: FormattableText::from("Pizza Hut"),
///         secondary_text: Some(FormattableText::from("123 Main St")),
///     }),
///     types: vec!["restaurant".to_string()],
///     distance_meters: Some(500),
/// };
///
/// // Use the place ID for Place Details lookup
/// let place_id = prediction.place_id();
///
/// // Display with HTML highlighting
/// let html = prediction.text().to_html("strong");
/// // "<strong>Pizza</strong> Hut, 123 Main St"
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
pub struct PlacePrediction {
    /// The resource name of the suggested Place.
    ///
    /// This name can be used in other APIs that accept Place resource names, following the format
    /// `places/{place_id}`. Use this for API calls that require the full resource name rather than
    /// just the place ID.
    pub place: String,

    /// The unique identifier of the suggested Place.
    ///
    /// This identifier can be used in other APIs that accept Place IDs, such as Place Details, to
    /// fetch complete information about the location. This is the most commonly used field for
    /// subsequent API calls.
    #[serde(rename = "placeId")]
    pub place_id: String,

    /// The human-readable name for the place.
    ///
    /// Contains the business name and address or other identifying information. For establishment
    /// results, this is usually the business name followed by the address. This text may be
    /// different from the display name shown in Place Details and may be in mixed languages if the
    /// request input or place name is in different languages.
    pub text: FormattableText,

    /// Structured breakdown of the place prediction.
    ///
    /// Separates the place name into main text (the name of the place) and secondary text
    /// (additional disambiguating features like city or region). The structured format may differ
    /// from `text` and is optimized for formatted display in UIs. May be `None` if structured
    /// formatting is not available.
    #[serde(rename = "structuredFormat", skip_serializing_if = "Option::is_none")]
    pub structured_format: Option<StructuredFormat>,

    /// Place types that apply to this location.
    ///
    /// A list of types from Google's Place Types taxonomy (Table A or Table B). Types categorize
    /// places and indicate shared characteristics. For example, a place might have types like
    /// `["restaurant", "food", "point_of_interest"]`.
    #[serde(default)]
    pub types: Vec<PlaceType>,

    /// Distance from the origin point in meters.
    ///
    /// The geodesic distance from the `origin` point specified in the request to this predicted
    /// place. Only present if an origin was specified in the autocomplete request. Use this to help
    /// users choose nearby results or to sort predictions by proximity.
    #[serde(rename = "distanceMeters", skip_serializing_if = "Option::is_none")]
    pub distance_meters: Option<i32>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl PlacePrediction {
    /// Creates a new `PlacePrediction` with all fields.
    ///
    /// Use this when constructing place predictions from API responses with complete information
    /// including distance measurements.
    #[must_use]
    pub const fn new(
        place: String,
        place_id: String,
        text: FormattableText,
        structured_format: Option<StructuredFormat>,
        types: Vec<PlaceType>,
        distance_meters: Option<i32>,
    ) -> Self {
        Self {
            place,
            place_id,
            text,
            structured_format,
            types,
            distance_meters,
        }
    }

    /// Returns a reference to the place resource name.
    ///
    /// The full resource name in the format `places/{place_id}`. Use this for API calls that
    /// require resource names.
    #[must_use]
    pub fn place(&self) -> &str {
        &self.place
    }

    /// Returns a reference to the place ID.
    ///
    /// The unique identifier for this place. Use this for Place Details lookups or other APIs that
    /// accept place IDs.
    #[must_use]
    pub fn place_id(&self) -> &str {
        &self.place_id
    }

    /// Returns a reference to the text description.
    ///
    /// The human-readable name and identifying information. Use the returned `FormattableText` to
    /// access match ranges and apply HTML highlighting.
    ///
    /// For example: `prediction.text().to_html("mark")` to highlight matched portions.
    #[must_use]
    pub const fn text(&self) -> &FormattableText {
        &self.text
    }

    /// Returns a reference to the structured format if present.
    ///
    /// The breakdown into main text (place name) and secondary text (location context).
    ///
    /// Use this for UI layouts where the place name should be emphasized separately from the
    /// address or location details.
    #[must_use]
    pub const fn structured_format(&self) -> Option<&StructuredFormat> {
        self.structured_format.as_ref()
    }

    /// Returns a reference to the place types.
    ///
    /// The list of type identifiers from Google's Place Types taxonomy.
    ///
    /// Use these to filter, categorize, or display icons for different place categories.
    #[must_use]
    pub fn types(&self) -> &[PlaceType] {
        &self.types
    }

    /// Returns the distance from origin in meters if available.
    ///
    /// The geodesic distance from the origin point to this place. Only present if an origin was
    /// specified in the request.
    #[must_use]
    pub const fn distance_meters(&self) -> Option<i32> {
        self.distance_meters
    }

    /// Returns the distance as a `Decimal` if available.
    ///
    /// Converts the distance to `Decimal` for precise calculations or formatting. Returns `None` if
    /// no distance is available.
    #[must_use]
    pub fn distance_decimal(&self) -> Option<Decimal> {
        self.distance_meters
            .and_then(Decimal::from_i32)
    }

    /// Checks if structured format is available.
    ///
    /// Returns `true` if the prediction includes a structured breakdown.
    #[must_use]
    pub const fn has_structured_format(&self) -> bool {
        self.structured_format.is_some()
    }

    /// Checks if distance information is available.
    ///
    /// Returns `true` if the prediction includes distance from origin.
    #[must_use]
    pub const fn has_distance(&self) -> bool {
        self.distance_meters.is_some()
    }

    /// Formats the place text with HTML highlighting.
    ///
    /// Convenience method that wraps `text().to_html(tag)`.
    ///
    /// Use this to quickly generate HTML-highlighted place text for UI display.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let html = prediction.to_html("strong");
    /// // "<strong>Pizza</strong> Hut, 123 Main St"
    /// ```
    #[must_use]
    pub fn to_html(&self, tag: &str) -> String {
        self.text.to_html(tag)
    }

    /// Formats the place with HTML using structured format if available.
    ///
    /// If structured format is present, applies different HTML tags to the place name and location
    /// context. Otherwise, falls back to highlighting the full text with the main tag.
    ///
    /// Use this to create formatted output where the place name and location have different visual
    /// emphasis.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // With structured format
    /// let html = prediction.to_html_structured("strong", "em");
    /// // "<strong>Pizza</strong> Hut, <em>123 Main</em> St"
    ///
    /// // Without structured format (fallback)
    /// let html = prediction.to_html_structured("strong", "em");
    /// // "<strong>Pizza</strong> Hut, 123 Main St"
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

    /// Formats the place text with a custom formatter function.
    ///
    /// Applies a custom formatting function to the place text, distinguishing between matched and
    /// unmatched portions. This provides maximum flexibility for output formats beyond HTML, such
    /// as ANSI terminal codes, Markdown, or custom markup languages.
    ///
    /// # Examples
    ///
    /// **ANSI terminal colors (bold blue for matches):**
    /// ```rust,ignore
    /// let formatted = prediction.format_with(|text, is_matched| {
    ///     if is_matched {
    ///         format!("\x1b[1;34m{}\x1b[0m", text)  // Bold blue
    ///     } else {
    ///         text.to_string()
    ///     }
    /// });
    /// // "\x1b[1;34mPizza\x1b[0m Hut, 123 Main St"
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
    /// // "**Pizza** Hut, 123 Main St"
    /// ```
    #[must_use]
    pub fn format_with<F>(&self, formatter: F) -> String
    where
        F: FnMut(&str, bool) -> String,
    {
        self.text.format_with(formatter)
    }

    /// Formats the place with custom formatters using structured format if available.
    ///
    /// If structured format is present, applies different formatting functions to the place name
    /// and location context, then combines them with a comma separator. Otherwise, falls back to
    /// formatting the full text with the main formatter.
    ///
    /// Use this to create formatted output where name and location have different visual styles in
    /// any output format (ANSI, Markdown, custom markup, etc.)
    ///
    /// # Examples
    ///
    /// **ANSI terminal colors (bold blue main, cyan secondary):**
    /// ```rust,ignore
    /// let formatted = prediction.format_with_structured(
    ///     |text, is_matched| {
    ///         if is_matched {
    ///             format!("\x1b[1;34m{}\x1b[0m", text)  // Bold blue
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
    /// // "\x1b[1;34mPizza\x1b[0m Hut, \x1b[36m123 Main\x1b[0m St"
    /// ```
    ///
    /// **Markdown (bold main, italic secondary):**
    /// ```rust,ignore
    /// let formatted = prediction.format_with_structured(
    ///     |text, is_matched| {
    ///         if is_matched { format!("**{}**", text) } else { text.to_string() }
    ///     },
    ///     |text, is_matched| {
    ///         if is_matched { format!("*{}*", text) } else { text.to_string() }
    ///     }
    /// );
    /// // "**Pizza** Hut, *123 Main* St"
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

impl std::fmt::Display for PlacePrediction {
    /// Formats as the plain place text.
    ///
    /// Returns the full place description for machine-readable output without HTML formatting.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}