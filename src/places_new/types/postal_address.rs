use icu_locale::Locale;
use rust_iso3166::CountryCode;

// -------------------------------------------------------------------------------------------------
//
/// Represents a postal address, such as for postal delivery or payments addresses.
/// 
/// With a postal address, a postal service can deliver items to a premise, P.O. box, or similar. A
/// ostal address is not intended to model geographical locations like roads, towns, or mountains.
///
/// In typical usage, an address would be created by user input or from importing existing data,
/// depending on the type of process.
///
/// ## Advice on address input or editing:
///
/// - Use an internationalization-ready address widget such as
///   [https://github.com/google/libaddressinput](https://github.com/google/libaddressinput).
///
/// - Users should not be presented with UI elements for input or editing of fields outside
///   countries where that field is used.
///
/// For more guidance on how to use this schema, see:
/// <https://support.google.com/business/answer/6397478>.
#[derive(
    //std
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    // getset
    getset::CopyGetters,
    getset::Getters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[serde(rename_all = "camelCase")]
pub struct PostalAddress {
    /// The schema revision of the `PostalAddress`. This must be set to 0, which is the latest
    /// revision.
    ///
    /// All new revisions must be backward compatible with old revisions.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub revision: Option<i32>,

    /// CLDR region code of the country/region of the address.
    ///
    /// This is never inferred and it is up to the user to ensure the value is correct.
    /// See <http://cldr.unicode.org/> and
    /// <http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html>
    /// for details. Example: "US" for United States.
    #[serde(
        serialize_with = "crate::places_new::serde::serialize_country_code",
        deserialize_with = "crate::places_new::serde::deserialize_country_code"
    )]
    #[getset(get_copy = "pub")]
    pub region_code: CountryCode,

    /// BCP-47 language code of the contents of this address (if known).
    ///
    /// This is often the UI language of the input form or is expected to match one of the languages
    /// used in the address' country/region, or their transliterated equivalents. This can affect
    /// formatting in certain countries, but is not critical to the correctness of the data and will
    /// never affect any validation or other non-formatting related operations.
    ///
    /// If this value is not known, it should be omitted (rather than specifying a possibly
    /// incorrect default).
    ///
    /// Examples: "zh-Hant", "ja", "ja-Latn", "en".
    #[serde(
        default,
        serialize_with = "crate::places_new::serde::serialize_optional_locale",
        deserialize_with = "crate::places_new::serde::deserialize_optional_locale"
    )]
    #[getset(get = "pub")]
    pub language_code: Option<Locale>,

    /// Postal code of the address.
    ///
    /// Not all countries use or require postal codes to be present, but where they are used, they
    /// may trigger additional validation with other parts of the address (for example, state or zip
    /// code validation in the United States).
    #[serde(default)]
    #[getset(get = "pub")]
    pub postal_code: Option<String>,

    /// Additional, country-specific, sorting code.
    ///
    /// This is not used in most regions. Where it is used, the value is either a string like
    /// "CEDEX", optionally followed by a number (for example, "CEDEX 7"), or just a number alone,
    /// representing the "sector code" (Jamaica), "delivery area indicator" (Malawi) or "post office
    /// indicator" (Côte d'Ivoire).
    #[serde(default)]
    #[getset(get = "pub")]
    pub sorting_code: Option<String>,

    /// Highest administrative subdivision which is used for postal addresses of a country or
    /// region.
    ///
    /// For example, this can be a state, a province, an oblast, or a prefecture. For Spain, this is
    /// the province and not the autonomous community (for example, "Barcelona" and not
    /// "Catalonia"). Many countries don't use an administrative area in postal addresses. For
    /// example, in Switzerland, this should be left unpopulated.
    #[serde(default)]
    #[getset(get = "pub")]
    pub administrative_area: Option<String>,

    /// Generally refers to the city or town portion of the address.
    ///
    /// Examples: US city, IT comune, UK post town. In regions of the world where localities are not
    /// well defined or do not fit into this structure well, leave `locality` empty and use
    /// `address_lines`.
    #[serde(default)]
    #[getset(get = "pub")]
    pub locality: Option<String>,

    /// Sublocality of the address.
    ///
    /// For example, this can be a neighborhood, borough, or district.
    #[serde(default)]
    #[getset(get = "pub")]
    pub sublocality: Option<String>,

    /// Unstructured address lines describing the lower levels of an address.
    ///
    /// Because values in `address_lines` do not have type information and may sometimes contain
    /// multiple values in a single field (e.g. "Austin, TX"), it is important that the line order
    /// is clear. The order of address lines should be "envelope order" for the country/region of
    /// the address. In places where this can vary (e.g. Japan), `address_validation` can be used to
    /// provide hints in the address.
    ///
    /// This format maintains compatibility with real-world addressing schemes which are sometimes
    /// flat and sometimes hierarchical.
    ///
    /// The minimum permitted structural representation of an address consists of a `region_code`
    /// with all remaining information placed in the `address_lines`. It is possible to format such
    /// an address for display, but additional components cannot be extracted.
    ///
    /// Creating an address only containing a `region_code` and `address_lines`, and then geocoding
    /// is the recommended way to handle completely unstructured addresses (as opposed to guessing
    /// which parts of the address should be localities or administrative areas).
    #[serde(default)]
    #[getset(get = "pub")]
    pub address_lines: Vec<String>,

    /// The recipient at the address.
    ///
    /// This field may, under certain circumstances, contain multiline information. For example, it
    /// might contain "care of" information.
    #[serde(default)]
    #[getset(get = "pub")]
    pub recipients: Vec<String>,

    /// The name of the organization at the address.
    #[serde(default)]
    #[getset(get = "pub")]
    pub organization: Option<String>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl PostalAddress {
    /// Creates a new `PostalAddress` with the minimum required field.
    ///
    /// Only `region_code` is required according to the specification.
    #[must_use]
    pub const fn new(region_code: CountryCode) -> Self {
        Self {
            revision: None,
            region_code,
            language_code: None,
            postal_code: None,
            sorting_code: None,
            administrative_area: None,
            locality: None,
            sublocality: None,
            address_lines: Vec::new(),
            recipients: Vec::new(),
            organization: None,
        }
    }

    /// Creates a new `PostalAddress` from a country code string.
    ///
    /// Returns an error if the country code is not a valid ISO 3166-1 alpha-2 code.
    #[must_use]
    pub fn from_country_code(region_code: &str) -> Option<Self> {
        rust_iso3166::from_alpha2(region_code).map(Self::new)
    }

    /// Returns whether this address has any address lines.
    ///
    /// Address lines are the unstructured components that describe lower levels of the address when
    /// structured fields are not sufficient.
    #[must_use]
    pub fn has_address_lines(&self) -> bool {
        !self.address_lines.is_empty()
    }

    /// Returns whether this address has any recipients specified.
    ///
    /// Recipients represent who should receive mail at this address.
    #[must_use]
    pub fn has_recipients(&self) -> bool {
        !self.recipients.is_empty()
    }

    /// Gets the country code as a 2-letter string.
    #[must_use]
    pub const fn get_region_code_str(&self) -> &str {
        self.region_code.alpha2
    }

    /// Gets the language code as a string if available.
    #[must_use]
    pub fn get_language_code_str(&self) -> Option<String> {
        self.language_code.as_ref().map(icu_locale::Locale::to_string)
    }
}