use icu_locale::Locale;

// -------------------------------------------------------------------------------------------------
//
/// Localized variant of a text in a particular language.
///
/// This struct represents text content that has been localized to a specific language, providing
/// both the text content and its associated language code for proper internationalization support.
#[derive(
    //std
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    // getset
    getset::Getters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedText {
    /// Localized string in the language corresponding to
    /// [languageCode](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#LocalizedText.FIELDS.language_code).
    ///
    /// This field contains the actual text content that has been translated or localized for the
    /// specified language.
    #[getset(get = "pub")]
    pub text: String,

    /// The text's BCP-47 language code, such as "en-US" or "sr-Latn".
    ///
    /// For more information about BCP-47 language codes, see the
    /// [Unicode Locale Data Markup Language (LDML) specification](http://www.unicode.org/reports/tr35/#Unicode_locale_identifier).
    /// This code indicates the language and potentially region/script variant used for the
    /// localized text.
    #[serde(
        alias = "languageCode",
        serialize_with = "crate::places_new::serde::serialize_locale",
        deserialize_with = "crate::places_new::serde::deserialize_locale"
    )]
    #[getset(get = "pub")]
    pub language: Locale,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl LocalizedText {
    /// Creates a new `LocalizedText` with the given text and language code.
    ///
    /// Returns an error if the language code is not a valid BCP-47 locale identifier.
    pub fn new(
        text: impl Into<String>,
        language_code: impl AsRef<str>
    ) -> Result<Self, icu_locale::ParseError> {
        let locale = Locale::try_from_str(language_code.as_ref())?;

        Ok(Self {
            text: text.into(),
            language: locale,
        })
    }

    /// Gets the language code as a `String` (as opposed to `Locale`).
    ///
    /// This is useful when you need the locale identifier as a string for APIs or display purposes.
    #[must_use]
    pub fn language_str(&self) -> String {
        self.language.to_string()
    }

    /// Returns whether this `LocalizedText` is in the specified language or not.
    #[must_use]
    pub fn is_language(&self, language: &Locale) -> bool {
        &self.language == language
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::fmt::Display for LocalizedText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}