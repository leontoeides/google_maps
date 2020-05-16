//! Contains the `Language` enum and its associated traits. It is used to
//! specify a desired language for a response. _This is not a comprehensive list
//! of languages, it is a list of languages that Google Maps supports._

use crate::error::Error;
use serde::{Deserialize, Serialize};

/// Specifies the language in which to return results.
///
/// [Languages](https://developers.google.com/maps/faq#languagesupport)
/// ====================================================================
///
/// * See the [list of supported languages](https://developers.google.com/maps/faq#languagesupport).
/// Google often updates the supported languages, so this list may not be
/// exhaustive.
///
/// * If `language` is not supplied, the API attempts to use the preferred
/// language as specified in the `Accept-Language` header, or the native
/// language of the domain from which the request is sent.
///
/// * The API does its best to provide a street address that is readable for
/// both the user and locals. To achieve that goal, it returns street addresses
/// in the local language, transliterated to a script readable by the user if
/// necessary, observing the preferred language. All other addresses are
/// returned in the preferred language. Address components are all returned in
/// the same language, which is chosen from the first component.
///
/// * If a name is not available in the preferred language, the API uses the
/// closest match.
///
/// * The preferred language has a small influence on the set of results that
/// the API chooses to return, and the order in which they are returned. The
/// geocoder interprets abbreviations differently depending on language, such as
/// the abbreviations for street types, or synonyms that may be valid in one
/// language but not in another. For example, _utca_ and _tér_ are synonyms for
/// street in Hungarian.
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Language {
    Afrikaans,
    Albanian,
    Amharic,
    Arabic,
    Armenian,
    Azerbaijani,
    Basque,
    Belarusian,
    Bengali,
    Bosnian,
    Bulgarian,
    Burmese,
    Catalan,
    Chinese,
    ChineseSimplified,
    ChineseHongKong,
    ChineseTraditional,
    Croatian,
    Czech,
    Danish,
    Dutch,
    English,
    EnglishAustralian,
    EnglishGreatBritain,
    Estonian,
    Farsi,
    Finnish,
    Filipino,
    French,
    FrenchCanada,
    Galician,
    Georgian,
    German,
    Greek,
    Gujarati,
    Hebrew,
    Hindi,
    Hungarian,
    Icelandic,
    Indonesian,
    Italian,
    Japanese,
    Kannada,
    Kazakh,
    Khmer,
    Korean,
    Kyrgyz,
    Lao,
    Latvian,
    Lithuanian,
    Macedonian,
    Malay,
    Malayalam,
    Marathi,
    Mongolian,
    Nepali,
    Norwegian,
    Polish,
    Portuguese,
    PortugueseBrazil,
    PortuguesePortugal,
    Punjabi,
    Romanian,
    Russian,
    Serbian,
    Sinhalese,
    Slovak,
    Slovenian,
    Spanish,
    SpanishLatinAmerica,
    Swahili,
    Swedish,
    Tamil,
    Telugu,
    Thai,
    Turkish,
    Ukrainian,
    Urdu,
    Uzbek,
    Vietnamese,
    Zulu,
} // enum

impl std::convert::From<&Language> for String {
    /// Converts a `Language` enum to a `String` that contains a
    /// [language](https://developers.google.com/maps/faq#languagesupport) code.
    fn from(language: &Language) -> String {
        match language {
            Language::Afrikaans => String::from("af"),
            Language::Albanian => String::from("sq"),
            Language::Amharic => String::from("am"),
            Language::Arabic => String::from("ar"),
            Language::Armenian => String::from("hy"),
            Language::Azerbaijani => String::from("az"),
            Language::Basque => String::from("eu"),
            Language::Belarusian => String::from("be"),
            Language::Bengali => String::from("bn"),
            Language::Bosnian => String::from("bs"),
            Language::Bulgarian => String::from("bg"),
            Language::Burmese => String::from("my"),
            Language::Catalan => String::from("ca"),
            Language::Chinese => String::from("zh"),
            Language::ChineseSimplified => String::from("zh-CN"),
            Language::ChineseHongKong => String::from("zh-HK"),
            Language::ChineseTraditional => String::from("zh-TW"),
            Language::Croatian => String::from("hr"),
            Language::Czech => String::from("cs"),
            Language::Danish => String::from("da"),
            Language::Dutch => String::from("nl"),
            Language::English => String::from("en"),
            Language::EnglishAustralian => String::from("en-AU"),
            Language::EnglishGreatBritain => String::from("en-GB"),
            Language::Estonian => String::from("et"),
            Language::Farsi => String::from("fa"),
            Language::Finnish => String::from("fi"),
            Language::Filipino => String::from("fil"),
            Language::French => String::from("fr"),
            Language::FrenchCanada => String::from("fr-CA"),
            Language::Galician => String::from("gl"),
            Language::Georgian => String::from("ka"),
            Language::German => String::from("de"),
            Language::Greek => String::from("el"),
            Language::Gujarati => String::from("gu"),
            Language::Hebrew => String::from("iw"),
            Language::Hindi => String::from("hi"),
            Language::Hungarian => String::from("hu"),
            Language::Icelandic => String::from("is"),
            Language::Indonesian => String::from("id"),
            Language::Italian => String::from("it"),
            Language::Japanese => String::from("ja"),
            Language::Kannada => String::from("kn"),
            Language::Kazakh => String::from("kk"),
            Language::Khmer => String::from("km"),
            Language::Korean => String::from("ko"),
            Language::Kyrgyz => String::from("ky"),
            Language::Lao => String::from("lo"),
            Language::Latvian => String::from("lv"),
            Language::Lithuanian => String::from("lt"),
            Language::Macedonian => String::from("mk"),
            Language::Malay => String::from("ms"),
            Language::Malayalam => String::from("ml"),
            Language::Marathi => String::from("mr"),
            Language::Mongolian => String::from("mn"),
            Language::Nepali => String::from("ne"),
            Language::Norwegian => String::from("no"),
            Language::Polish => String::from("pl"),
            Language::Portuguese => String::from("pt"),
            Language::PortugueseBrazil => String::from("pr-BR"),
            Language::PortuguesePortugal => String::from("pt-PT"),
            Language::Punjabi => String::from("pa"),
            Language::Romanian => String::from("ro"),
            Language::Russian => String::from("ru"),
            Language::Serbian => String::from("sr"),
            Language::Sinhalese => String::from("si"),
            Language::Slovak => String::from("sk"),
            Language::Slovenian => String::from("sl"),
            Language::Spanish => String::from("es"),
            Language::SpanishLatinAmerica => String::from("es-419"),
            Language::Swahili => String::from("sw"),
            Language::Swedish => String::from("sv"),
            Language::Tamil => String::from("ta"),
            Language::Telugu => String::from("te"),
            Language::Thai => String::from("th"),
            Language::Turkish => String::from("tr"),
            Language::Ukrainian => String::from("uk"),
            Language::Urdu => String::from("ur"),
            Language::Uzbek => String::from("uz"),
            Language::Vietnamese => String::from("vi"),
            Language::Zulu => String::from("zu"),
        } // match
    } // fn
} // impl

impl std::convert::TryFrom<&str> for Language {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Gets a `Language` enum from a `String` that contains a supported
    /// [language](https://developers.google.com/maps/faq#languagesupport) code.
    fn try_from(language: &str) -> Result<Language, Error> {
        match language {
            "af" => Ok(Language::Afrikaans),
            "sq" => Ok(Language::Albanian),
            "am" => Ok(Language::Amharic),
            "ar" => Ok(Language::Arabic),
            "hy" => Ok(Language::Armenian),
            "az" => Ok(Language::Azerbaijani),
            "eu" => Ok(Language::Basque),
            "be" => Ok(Language::Belarusian),
            "bn" => Ok(Language::Bengali),
            "bs" => Ok(Language::Bosnian),
            "bg" => Ok(Language::Bulgarian),
            "my" => Ok(Language::Burmese),
            "ca" => Ok(Language::Catalan),
            "zh" => Ok(Language::Chinese),
            "zh-CN" => Ok(Language::ChineseSimplified),
            "zh-HK" => Ok(Language::ChineseHongKong),
            "zh-TW" => Ok(Language::ChineseTraditional),
            "hr" => Ok(Language::Croatian),
            "cs" => Ok(Language::Czech),
            "da" => Ok(Language::Danish),
            "nl" => Ok(Language::Dutch),
            "en" => Ok(Language::English),
            "en-AU" => Ok(Language::EnglishAustralian),
            "en-GB" => Ok(Language::EnglishGreatBritain),
            "et" => Ok(Language::Estonian),
            "fa" => Ok(Language::Farsi),
            "fi" => Ok(Language::Finnish),
            "fil" => Ok(Language::Filipino),
            "fr" => Ok(Language::French),
            "fr-CA" => Ok(Language::FrenchCanada),
            "gl" => Ok(Language::Galician),
            "ka" => Ok(Language::Georgian),
            "de" => Ok(Language::German),
            "el" => Ok(Language::Greek),
            "gu" => Ok(Language::Gujarati),
            "iw" => Ok(Language::Hebrew),
            "hi" => Ok(Language::Hindi),
            "hu" => Ok(Language::Hungarian),
            "is" => Ok(Language::Icelandic),
            "id" => Ok(Language::Indonesian),
            "it" => Ok(Language::Italian),
            "ja" => Ok(Language::Japanese),
            "kn" => Ok(Language::Kannada),
            "kk" => Ok(Language::Kazakh),
            "km" => Ok(Language::Khmer),
            "ko" => Ok(Language::Korean),
            "ky" => Ok(Language::Kyrgyz),
            "lo" => Ok(Language::Lao),
            "lv" => Ok(Language::Latvian),
            "lt" => Ok(Language::Lithuanian),
            "mk" => Ok(Language::Macedonian),
            "ms" => Ok(Language::Malay),
            "ml" => Ok(Language::Malayalam),
            "mr" => Ok(Language::Marathi),
            "mn" => Ok(Language::Mongolian),
            "ne" => Ok(Language::Nepali),
            "no" => Ok(Language::Norwegian),
            "pl" => Ok(Language::Polish),
            "pt" => Ok(Language::Portuguese),
            "pr-BR" => Ok(Language::PortugueseBrazil),
            "pt-PT" => Ok(Language::PortuguesePortugal),
            "pa" => Ok(Language::Punjabi),
            "ro" => Ok(Language::Romanian),
            "ru" => Ok(Language::Russian),
            "sr" => Ok(Language::Serbian),
            "si" => Ok(Language::Sinhalese),
            "sk" => Ok(Language::Slovak),
            "sl" => Ok(Language::Slovenian),
            "es" => Ok(Language::Spanish),
            "es-419" => Ok(Language::SpanishLatinAmerica),
            "sw" => Ok(Language::Swahili),
            "sv" => Ok(Language::Swedish),
            "ta" => Ok(Language::Tamil),
            "te" => Ok(Language::Telugu),
            "th" => Ok(Language::Thai),
            "tr" => Ok(Language::Turkish),
            "uk" => Ok(Language::Ukrainian),
            "ur" => Ok(Language::Urdu),
            "uz" => Ok(Language::Uzbek),
            "vi" => Ok(Language::Vietnamese),
            "zu" => Ok(Language::Zulu),
            _ => Err(Error::InvalidLanguageCode(language.to_string())),
        } // match
    } // fn
} // impl

impl std::default::Default for Language {
    /// Returns a reasonable default variant for the `Language` enum type.
    fn default() -> Self {
        Language::English
    } // fn
} // impl

impl std::fmt::Display for Language {
    /// Formats a `Language` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Language::Afrikaans => write!(f, "Afrikaans"),
            Language::Albanian => write!(f, "Albanian"),
            Language::Amharic => write!(f, "Amharic"),
            Language::Arabic => write!(f, "Arabic"),
            Language::Armenian => write!(f, "Armenian"),
            Language::Azerbaijani => write!(f, "Azerbaijani"),
            Language::Basque => write!(f, "Basque"),
            Language::Belarusian => write!(f, "Belarusian"),
            Language::Bengali => write!(f, "Bengali"),
            Language::Bosnian => write!(f, "Bosnian"),
            Language::Bulgarian => write!(f, "Bulgarian"),
            Language::Burmese => write!(f, "Burmese"),
            Language::Catalan => write!(f, "Catalan"),
            Language::Chinese => write!(f, "Chinese"),
            Language::ChineseSimplified => write!(f, "Chinese (Simplified)"),
            Language::ChineseHongKong => write!(f, "Chinese (Hong Kong)"),
            Language::ChineseTraditional => write!(f, "Chinese (Traditional)"),
            Language::Croatian => write!(f, "Croatian"),
            Language::Czech => write!(f, "Czech"),
            Language::Danish => write!(f, "Danish"),
            Language::Dutch => write!(f, "Dutch"),
            Language::English => write!(f, "English"),
            Language::EnglishAustralian => write!(f, "English (Australian)"),
            Language::EnglishGreatBritain => write!(f, "English (Great Britain)"),
            Language::Estonian => write!(f, "Estonian"),
            Language::Farsi => write!(f, "Farsi"),
            Language::Finnish => write!(f, "Finnish"),
            Language::Filipino => write!(f, "Filipino"),
            Language::French => write!(f, "French"),
            Language::FrenchCanada => write!(f, "French (Canada)"),
            Language::Galician => write!(f, "Galician"),
            Language::Georgian => write!(f, "Georgian"),
            Language::German => write!(f, "German"),
            Language::Greek => write!(f, "Greek"),
            Language::Gujarati => write!(f, "Gujarati"),
            Language::Hebrew => write!(f, "Hebrew"),
            Language::Hindi => write!(f, "Hindi"),
            Language::Hungarian => write!(f, "Hungarian"),
            Language::Icelandic => write!(f, "Icelandic"),
            Language::Indonesian => write!(f, "Indonesian"),
            Language::Italian => write!(f, "Italian"),
            Language::Japanese => write!(f, "Japanese"),
            Language::Kannada => write!(f, "Kannada"),
            Language::Kazakh => write!(f, "Kazakh"),
            Language::Khmer => write!(f, "Khmer"),
            Language::Korean => write!(f, "Korean"),
            Language::Kyrgyz => write!(f, "Kyrgyz"),
            Language::Lao => write!(f, "Lao"),
            Language::Latvian => write!(f, "Latvian"),
            Language::Lithuanian => write!(f, "Lithuanian"),
            Language::Macedonian => write!(f, "Macedonian"),
            Language::Malay => write!(f, "Malay"),
            Language::Malayalam => write!(f, "Malayalam"),
            Language::Marathi => write!(f, "Marathi"),
            Language::Mongolian => write!(f, "Mongolian"),
            Language::Nepali => write!(f, "Nepali"),
            Language::Norwegian => write!(f, "Norwegian"),
            Language::Polish => write!(f, "Polish"),
            Language::Portuguese => write!(f, "Portuguese"),
            Language::PortugueseBrazil => write!(f, "Portuguese (Brazil)"),
            Language::PortuguesePortugal => write!(f, "Portuguese (Portugal)"),
            Language::Punjabi => write!(f, "Punjabi"),
            Language::Romanian => write!(f, "Romanian"),
            Language::Russian => write!(f, "Russian"),
            Language::Serbian => write!(f, "Serbian"),
            Language::Sinhalese => write!(f, "Sinhalese"),
            Language::Slovak => write!(f, "Slovak"),
            Language::Slovenian => write!(f, "Slovenian"),
            Language::Spanish => write!(f, "Spanish"),
            Language::SpanishLatinAmerica => write!(f, "Spanish (Latin America)"),
            Language::Swahili => write!(f, "Swahili"),
            Language::Swedish => write!(f, "Swedish"),
            Language::Tamil => write!(f, "Tamil"),
            Language::Telugu => write!(f, "Telugu"),
            Language::Thai => write!(f, "Thai"),
            Language::Turkish => write!(f, "Turkish"),
            Language::Ukrainian => write!(f, "Ukrainian"),
            Language::Urdu => write!(f, "Urdu"),
            Language::Uzbek => write!(f, "Uzbek"),
            Language::Vietnamese => write!(f, "Vietnamese"),
            Language::Zulu => write!(f, "Zulu"),
        } // match
    } // fn
} // impl