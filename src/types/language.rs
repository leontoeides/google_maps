//! Contains the `Language` enum and its associated traits. It is used to
//! specify a desired language for a response. _This is not a comprehensive list
//! of languages, it is a list of languages that Google Maps supports._

use crate::error::Error as GoogleMapsError;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------

/// Specifies the language in which to return results.
///
/// [Languages](https://developers.google.com/maps/faq#languagesupport)
/// ====================================================================
///
/// * See the [list of supported languages](https://developers.google.com/maps/faq#languagesupport).
///   Google often updates the supported languages, so this list may not be
///   exhaustive.
///
/// * If `language` is not supplied, the API attempts to use the preferred
///   language as specified in the `Accept-Language` header, or the native
///   language of the domain from which the request is sent.
///
/// * The API does its best to provide a street address that is readable for
///   both the user and locals. To achieve that goal, it returns street
///   addresses in the local language, transliterated to a script readable by
///   the user if necessary, observing the preferred language. All other
///   addresses are returned in the preferred language. Address components are
///   all returned in the same language, which is chosen from the first
///   component.
///
/// * If a name is not available in the preferred language, the API uses the
///   closest match.
///
/// * The preferred language has a small influence on the set of results that
///   the API chooses to return, and the order in which they are returned. The
///   geocoder interprets abbreviations differently depending on language, such
///   as the abbreviations for street types, or synonyms that may be valid in
///   one language but not in another. For example, _utca_ and _tér_ are
///   synonyms for street in Hungarian.
///
/// By default the API will attempt to load the most appropriate language based
/// on the users location or browser settings. Some APIs allow you to explicitly
/// set a language when you make a request. More information on how to set the
/// language is available in the documentation for each API:
///
/// * [Maps JavaScript API](https://developers.google.com/maps/documentation/javascript/localization)
/// * [Geocoding API](https://developers.google.com/maps/documentation/geocoding/intro#language-param)
/// * [Directions API](https://developers.google.com/maps/documentation/directions/intro#language-param)
/// * [Distance Matrix API](https://developers.google.com/maps/documentation/distance-matrix/intro#language-param)
/// * [Places API](https://developers.google.com/places/web-service/search#language-param)
///
/// Supported Languages:
/// Google often updates supported languages. This list may not be exhaustive
/// and is subject to change.
///
/// You can see what the map will look like in any of the languages listed above
/// in this [sample application](https://developers.google.com/maps/documentation/javascript/demos/localization/).

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
#[non_exhaustive]
pub enum Language {
    Afrikaans = 0,
    Albanian = 1,
    Amharic = 2,
    Arabic = 3,
    Armenian = 4,
    Azerbaijani = 5,
    Basque = 6,
    Belarusian = 7,
    Bengali = 8,
    Bosnian = 9,
    Bulgarian = 10,
    Burmese = 11,
    Catalan = 12,
    Chinese = 13,
    ChineseHongKong = 14,
    ChineseSimplified = 15,
    ChineseTaiwan = 16,
    ChineseTraditional = 17,
    Croatian = 18,
    Czech = 19,
    Danish = 20,
    Dutch = 21,
    #[default]
    English = 22,
    EnglishAustralian = 23,
    EnglishCanada = 24,
    EnglishGreatBritain = 25,
    EnglishUs = 26,
    Estonian = 27,
    Farsi = 28,
    Finnish = 29,
    Filipino = 30,
    French = 31,
    FrenchCanada = 32,
    Galician = 33,
    Georgian = 34,
    German = 35,
    Greek = 36,
    Gujarati = 37,
    Hebrew = 38,
    Hindi = 39,
    Hungarian = 40,
    Icelandic = 41,
    Indonesian = 42,
    Italian = 43,
    Japanese = 44,
    Kannada = 45,
    Kazakh = 46,
    Khmer = 47,
    Korean = 48,
    Kyrgyz = 49,
    Lao = 50,
    Latvian = 51,
    Lithuanian = 52,
    Macedonian = 53,
    Malay = 54,
    Malayalam = 55,
    Marathi = 56,
    Mongolian = 57,
    Nepali = 58,
    Norwegian = 59,
    Polish = 60,
    Portuguese = 61,
    PortugueseBrazil = 62,
    PortuguesePortugal = 63,
    Punjabi = 64,
    Romanian = 65,
    Russian = 66,
    Serbian = 67,
    Sinhalese = 68,
    Slovak = 69,
    Slovenian = 70,
    Spanish = 71,
    SpanishLatinAmerica = 72,
    Swahili = 73,
    Swedish = 74,
    Tamil = 75,
    Telugu = 76,
    Thai = 77,
    Turkish = 78,
    Ukrainian = 79,
    Urdu = 80,
    Uzbek = 81,
    Vietnamese = 82,
    Zulu = 83,
    /// If the language is not recognized by
    /// [serde](https://crates.io/crates/serde) when reading data from
    /// Google it will be assigned to this `Other` variant.
    ///
    /// As new types are added to Google Maps, they must also be added to this
    /// crate. However, in the meantime, the `Other` catch-all variant allows
    /// `serde` to read data from Google without producing an error until the
    /// new variant added to this `enum`.
    Other = 84,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for Language {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match Self::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string())),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl Serialize for Language {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Self> for Language {
    /// Converts a borrowed `&Language` enum into an owned `Language` enum by
    /// copying it.
    fn from(language: &Self) -> Self {
        *language
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Language> for &str {
    /// Converts a `Language` enum to a `String` that contains a
    /// [language](https://developers.google.com/maps/faq#languagesupport) code.
    fn from(language: &Language) -> Self {
        match language {
            Language::Afrikaans => "af",
            Language::Albanian => "sq",
            Language::Amharic => "am",
            Language::Arabic => "ar",
            Language::Armenian => "hy",
            Language::Azerbaijani => "az",
            Language::Basque => "eu",
            Language::Belarusian => "be",
            Language::Bengali => "bn",
            Language::Bosnian => "bs",
            Language::Bulgarian => "bg",
            Language::Burmese => "my",
            Language::Catalan => "ca",
            Language::Chinese => "zh",
            Language::ChineseHongKong => "zh-HK",
            Language::ChineseSimplified => "zh-CN",
            Language::ChineseTaiwan => "zh-TW",
            Language::ChineseTraditional => "zh-Hant",
            Language::Croatian => "hr",
            Language::Czech => "cs",
            Language::Danish => "da",
            Language::Dutch => "nl",
            Language::English => "en",
            Language::EnglishAustralian => "en-AU",
            Language::EnglishCanada => "en-CA",
            Language::EnglishGreatBritain => "en-GB",
            Language::EnglishUs => "en-US",
            Language::Estonian => "et",
            Language::Farsi => "fa",
            Language::Finnish => "fi",
            Language::Filipino => "fil",
            Language::French => "fr",
            Language::FrenchCanada => "fr-CA",
            Language::Galician => "gl",
            Language::Georgian => "ka",
            Language::German => "de",
            Language::Greek => "el",
            Language::Gujarati => "gu",
            Language::Hebrew => "iw",
            Language::Hindi => "hi",
            Language::Hungarian => "hu",
            Language::Icelandic => "is",
            Language::Indonesian => "id",
            Language::Italian => "it",
            Language::Japanese => "ja",
            Language::Kannada => "kn",
            Language::Kazakh => "kk",
            Language::Khmer => "km",
            Language::Korean => "ko",
            Language::Kyrgyz => "ky",
            Language::Lao => "lo",
            Language::Latvian => "lv",
            Language::Lithuanian => "lt",
            Language::Macedonian => "mk",
            Language::Malay => "ms",
            Language::Malayalam => "ml",
            Language::Marathi => "mr",
            Language::Mongolian => "mn",
            Language::Nepali => "ne",
            Language::Norwegian => "no",
            Language::Polish => "pl",
            Language::Portuguese => "pt",
            Language::PortugueseBrazil => "pr-BR",
            Language::PortuguesePortugal => "pt-PT",
            Language::Punjabi => "pa",
            Language::Romanian => "ro",
            Language::Russian => "ru",
            Language::Serbian => "sr",
            Language::Sinhalese => "si",
            Language::Slovak => "sk",
            Language::Slovenian => "sl",
            Language::Spanish => "es",
            Language::SpanishLatinAmerica => "es-419",
            Language::Swahili => "sw",
            Language::Swedish => "sv",
            Language::Tamil => "ta",
            Language::Telugu => "te",
            Language::Thai => "th",
            Language::Turkish => "tr",
            Language::Ukrainian => "uk",
            Language::Urdu => "ur",
            Language::Uzbek => "uz",
            Language::Vietnamese => "vi",
            Language::Zulu => "zu",
            Language::Other => "xx",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for Language {
    /// Converts a `Language` enum to a `String` that contains a
    /// [language](https://developers.google.com/maps/faq#languagesupport) code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Language> for String {
    /// Converts a `Language` enum to a `String` that contains a
    /// [language](https://developers.google.com/maps/faq#languagesupport) code.
    fn from(language: &Language) -> Self {
        std::convert::Into::<&str>::into(language).to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

static LANGUAGES_BY_CODE: phf::Map<&'static str, Language> = phf_map! {
    "af" => Language::Afrikaans,
    "sq" => Language::Albanian,
    "am" => Language::Amharic,
    "ar" => Language::Arabic,
    "hy" => Language::Armenian,
    "az" => Language::Azerbaijani,
    "eu" => Language::Basque,
    "be" => Language::Belarusian,
    "bn" => Language::Bengali,
    "bs" => Language::Bosnian,
    "bg" => Language::Bulgarian,
    "my" => Language::Burmese,
    "ca" => Language::Catalan,
    "zh" => Language::Chinese,
    "zh-CN" => Language::ChineseSimplified,
    "zh-Hant" => Language::ChineseTraditional,
    "zh-HK" => Language::ChineseHongKong,
    "zh-TW" => Language::ChineseTaiwan,
    "hr" => Language::Croatian,
    "cs" => Language::Czech,
    "da" => Language::Danish,
    "nl" => Language::Dutch,
    "en" => Language::English,
    "en-AU" => Language::EnglishAustralian,
    "en-CA" => Language::EnglishCanada,
    "en-GB" => Language::EnglishGreatBritain,
    "en-US" => Language::EnglishUs,
    "et" => Language::Estonian,
    "fa" => Language::Farsi,
    "fi" => Language::Finnish,
    "fil" => Language::Filipino,
    "fr" => Language::French,
    "fr-CA" => Language::FrenchCanada,
    "gl" => Language::Galician,
    "ka" => Language::Georgian,
    "de" => Language::German,
    "el" => Language::Greek,
    "gu" => Language::Gujarati,
    "iw" => Language::Hebrew,
    "hi" => Language::Hindi,
    "hu" => Language::Hungarian,
    "is" => Language::Icelandic,
    "id" => Language::Indonesian,
    "it" => Language::Italian,
    "ja" => Language::Japanese,
    "kn" => Language::Kannada,
    "kk" => Language::Kazakh,
    "km" => Language::Khmer,
    "ko" => Language::Korean,
    "ky" => Language::Kyrgyz,
    "lo" => Language::Lao,
    "lv" => Language::Latvian,
    "lt" => Language::Lithuanian,
    "mk" => Language::Macedonian,
    "ms" => Language::Malay,
    "ml" => Language::Malayalam,
    "mr" => Language::Marathi,
    "mn" => Language::Mongolian,
    "ne" => Language::Nepali,
    "no" => Language::Norwegian,
    "pl" => Language::Polish,
    "pt" => Language::Portuguese,
    "pr-BR" => Language::PortugueseBrazil,
    "pt-PT" => Language::PortuguesePortugal,
    "pa" => Language::Punjabi,
    "ro" => Language::Romanian,
    "ru" => Language::Russian,
    "sr" => Language::Serbian,
    "si" => Language::Sinhalese,
    "sk" => Language::Slovak,
    "sl" => Language::Slovenian,
    "es" => Language::Spanish,
    "es-419" => Language::SpanishLatinAmerica,
    "sw" => Language::Swahili,
    "sv" => Language::Swedish,
    "ta" => Language::Tamil,
    "te" => Language::Telugu,
    "th" => Language::Thai,
    "tr" => Language::Turkish,
    "uk" => Language::Ukrainian,
    "ur" => Language::Urdu,
    "uz" => Language::Uzbek,
    "vi" => Language::Vietnamese,
    "zu" => Language::Zulu,
    "xx" => Language::Other,
};

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for Language {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = GoogleMapsError;
    /// Gets a `Language` enum from a `String` that contains a supported
    /// [language](https://developers.google.com/maps/faq#languagesupport) code.
    fn try_from(language_code: &str) -> Result<Self, Self::Error> {
        Ok(LANGUAGES_BY_CODE
            .get(language_code)
            .copied()
            .unwrap_or(Self::Other))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for Language {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Err = GoogleMapsError;
    /// Gets a `Language` enum from a `String` that contains a supported
    /// [language](https://developers.google.com/maps/faq#languagesupport) code.
    fn from_str(language_code: &str) -> Result<Self, Self::Err> {
        Ok(LANGUAGES_BY_CODE
            .get(language_code)
            .copied()
            .unwrap_or(Self::Other))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl Language {
    /// Formats a `Language` enum into a string that is presentable to the
    /// end user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::Afrikaans => "Afrikaans",
            Self::Albanian => "Albanian",
            Self::Amharic => "Amharic",
            Self::Arabic => "Arabic",
            Self::Armenian => "Armenian",
            Self::Azerbaijani => "Azerbaijani",
            Self::Basque => "Basque",
            Self::Belarusian => "Belarusian",
            Self::Bengali => "Bengali",
            Self::Bosnian => "Bosnian",
            Self::Bulgarian => "Bulgarian",
            Self::Burmese => "Burmese",
            Self::Catalan => "Catalan",
            Self::Chinese => "Chinese",
            Self::ChineseHongKong => "Chinese (Hong Kong)",
            Self::ChineseSimplified => "Chinese (Simplified)",
            Self::ChineseTaiwan => "Chinese (Taiwan)",
            Self::ChineseTraditional => "Chinese (Traditional)",
            Self::Croatian => "Croatian",
            Self::Czech => "Czech",
            Self::Danish => "Danish",
            Self::Dutch => "Dutch",
            Self::English => "English",
            Self::EnglishAustralian => "English (Australian)",
            Self::EnglishCanada => "English (Canada)",
            Self::EnglishGreatBritain => "English (Great Britain)",
            Self::EnglishUs => "English (US)",
            Self::Estonian => "Estonian",
            Self::Farsi => "Farsi",
            Self::Finnish => "Finnish",
            Self::Filipino => "Filipino",
            Self::French => "French",
            Self::FrenchCanada => "French (Canada)",
            Self::Galician => "Galician",
            Self::Georgian => "Georgian",
            Self::German => "German",
            Self::Greek => "Greek",
            Self::Gujarati => "Gujarati",
            Self::Hebrew => "Hebrew",
            Self::Hindi => "Hindi",
            Self::Hungarian => "Hungarian",
            Self::Icelandic => "Icelandic",
            Self::Indonesian => "Indonesian",
            Self::Italian => "Italian",
            Self::Japanese => "Japanese",
            Self::Kannada => "Kannada",
            Self::Kazakh => "Kazakh",
            Self::Khmer => "Khmer",
            Self::Korean => "Korean",
            Self::Kyrgyz => "Kyrgyz",
            Self::Lao => "Lao",
            Self::Latvian => "Latvian",
            Self::Lithuanian => "Lithuanian",
            Self::Macedonian => "Macedonian",
            Self::Malay => "Malay",
            Self::Malayalam => "Malayalam",
            Self::Marathi => "Marathi",
            Self::Mongolian => "Mongolian",
            Self::Nepali => "Nepali",
            Self::Norwegian => "Norwegian",
            Self::Polish => "Polish",
            Self::Portuguese => "Portuguese",
            Self::PortugueseBrazil => "Portuguese (Brazil)",
            Self::PortuguesePortugal => "Portuguese (Portugal)",
            Self::Punjabi => "Punjabi",
            Self::Romanian => "Romanian",
            Self::Russian => "Russian",
            Self::Serbian => "Serbian",
            Self::Sinhalese => "Sinhalese",
            Self::Slovak => "Slovak",
            Self::Slovenian => "Slovenian",
            Self::Spanish => "Spanish",
            Self::SpanishLatinAmerica => "Spanish (Latin America)",
            Self::Swahili => "Swahili",
            Self::Swedish => "Swedish",
            Self::Tamil => "Tamil",
            Self::Telugu => "Telugu",
            Self::Thai => "Thai",
            Self::Turkish => "Turkish",
            Self::Ukrainian => "Ukrainian",
            Self::Urdu => "Urdu",
            Self::Uzbek => "Uzbek",
            Self::Vietnamese => "Vietnamese",
            Self::Zulu => "Zulu",
            Self::Other => "Other",
        } // match
    } // fn
} // impl
