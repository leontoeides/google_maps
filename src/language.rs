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

#[derive(Clone, Debug)]
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

impl From<&Language> for String {
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

impl From<String> for Language {
    /// Gets a `Language` enum from a `String` that contains a supported
    /// [language](https://developers.google.com/maps/faq#languagesupport) code.
    fn from(language: String) -> Language {
        match language.as_ref() {
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
            "zh-HK" => Language::ChineseHongKong,
            "zh-TW" => Language::ChineseTraditional,
            "hr" => Language::Croatian,
            "cs" => Language::Czech,
            "da" => Language::Danish,
            "nl" => Language::Dutch,
            "en" => Language::English,
            "en-AU" => Language::EnglishAustralian,
            "en-GB" => Language::EnglishGreatBritain,
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
            _ => panic!("'{}' is not a known language code. Tip: The language code must be in lowercase. For a list of supported languages see https://developers.google.com/maps/faq#languagesupport", language),
        } // match
    } // fn
} // impl