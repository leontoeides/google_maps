use crate::places_new::AddressComponentType;
use icu_locale::Locale;

// -------------------------------------------------------------------------------------------------
//
/// The structured components that form the formatted address, if this information is available.
/// 
/// Address components provide detailed breakdown of an address into its constituent parts, allowing
/// for precise parsing and understanding of address structure across different countries and
/// addressing systems.
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
pub struct AddressComponent {
    /// The full text description or name of the address component.
    /// 
    /// For example, an address component for the country Australia may have a `long_text` of
    /// "Australia".
    #[serde(default)]
    #[getset(get = "pub")]
    pub long_text: Option<String>,

    /// An abbreviated textual name for the address component, if available.
    /// 
    /// For example, an address component for the country of Australia may have a `short_text` of
    /// "AU".
    #[serde(default)]
    #[getset(get = "pub")]
    pub short_text: Option<String>,

    /// An array indicating the type(s) of the address component.
    /// 
    /// Address components can have multiple types that describe their role in the address
    /// structure. See the address component types documentation for possible values.
    #[serde(default)]
    #[getset(get = "pub")]
    pub types: Vec<AddressComponentType>,

    /// The language used to format this component, in CLDR notation.
    /// 
    /// This indicates the language and locale used for the text representation of this address
    /// component.
    #[serde(
        rename = "languageCode",
        default,
        serialize_with = "crate::places_new::serde::serialize_optional_locale",
        deserialize_with = "crate::places_new::serde::deserialize_optional_locale"
    )]
    #[getset(get = "pub")]
    pub language: Option<Locale>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl AddressComponent {
    /// Gets the language code as a string.
    /// 
    /// This is useful when you need the locale identifier as a string for APIs or display purposes.
    #[must_use]
    pub fn language_code_str(&self) -> Option<String> {
        self.language.as_ref().map(Locale::to_string)
    }

    /// Returns whether this component has a long text representation.
    #[must_use]
    pub const fn has_long_text(&self) -> bool {
        self.long_text.is_some()
    }

    /// Returns whether this component has a short text representation.
    #[must_use]
    pub const fn has_short_text(&self) -> bool {
        self.short_text.is_some()
    }

    /// Returns whether this component has any type classifications.
    #[must_use]
    pub fn has_types(&self) -> bool {
        !self.types.is_empty()
    }

    /// Adds a type classification to this address component.
    /// 
    /// Address components can have multiple types that describe their role in the address
    /// structure.
    pub fn add_type(&mut self, component_type: impl Into<AddressComponentType>) {
        self.types.push(component_type.into());
    }
}