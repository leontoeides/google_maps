//! Contains the `Currency` enum and its associated traits. It is used to
//! specify a currency. Included for use with the transit fares returned by
//! Google Maps Directions API.

use crate::directions::error::Error;
use serde::{Serialize, Deserialize};

/// A comprehensive list of currencies. At the moment this is used only for
/// Google Maps Transit Directions. The intent behind having _Serde_ convert
/// the currency code `String` to an `enum` is for efficient currency
/// conversions, information lookups, and manipulation in the future.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Currency {
    #[serde(alias = "AED")]
    UnitedArabEmiratesDirham,
    #[serde(alias = "AFN")]
    AfghanAfghani,
    #[serde(alias = "ALL")]
    AlbanianLek,
    #[serde(alias = "AMD")]
    ArmenianDram,
    #[serde(alias = "ANG")]
    NetherlandsAntilleanGuilder,
    #[serde(alias = "AOA")]
    AngolanKwanza,
    #[serde(alias = "ARS")]
    ArgentinePeso,
    #[serde(alias = "AUD")]
    AustralianDollar,
    #[serde(alias = "AWG")]
    ArubanFlorin,
    #[serde(alias = "AZN")]
    AzerbaijaniManat,
    #[serde(alias = "BAM")]
    BosniaAndHerzegovinaConvertibleMark,
    #[serde(alias = "BBD")]
    BarbadosDollar,
    #[serde(alias = "BDT")]
    BangladeshiTaka,
    #[serde(alias = "BGN")]
    BulgarianLev,
    #[serde(alias = "BHD")]
    BahrainiDinar,
    #[serde(alias = "BIF")]
    BurundianFranc,
    #[serde(alias = "BMD")]
    BermudianDollar,
    #[serde(alias = "BND")]
    BruneiDollar,
    #[serde(alias = "BOB")]
    Boliviano,
    #[serde(alias = "BOV")]
    BolivianMvdol,
    #[serde(alias = "BRL")]
    BrazilianReal,
    #[serde(alias = "BSD")]
    BahamianDollar,
    #[serde(alias = "BTN")]
    BhutaneseNgultrum,
    #[serde(alias = "BWP")]
    BotswanaPula,
    #[serde(alias = "BYN")]
    BelarusianRuble,
    #[serde(alias = "BZD")]
    BelizeDollar,
    #[serde(alias = "CAD")]
    CanadianDollar,
    #[serde(alias = "CDF")]
    CongoleseFranc,
    #[serde(alias = "CHE")]
    WirEuro,
    #[serde(alias = "CHF")]
    SwissFranc,
    #[serde(alias = "CHW")]
    WirFranc,
    #[serde(alias = "CLF")]
    UnidadDeFomento,
    #[serde(alias = "CLP")]
    ChileanPeso,
    #[serde(alias = "CNY")]
    RenminbiYuan,
    #[serde(alias = "COP")]
    ColombianPeso,
    #[serde(alias = "COU")]
    UnidadDeValorReal,
    #[serde(alias = "CRC")]
    CostaRicanColon,
    #[serde(alias = "CUC")]
    CubanConvertiblePeso,
    #[serde(alias = "CUP")]
    CubanPeso,
    #[serde(alias = "CVE")]
    CapeVerdeanEscudo,
    #[serde(alias = "CZK")]
    CzechKoruna,
    #[serde(alias = "DJF")]
    DjiboutianFranc,
    #[serde(alias = "DKK")]
    DanishKrone,
    #[serde(alias = "DOP")]
    DominicanPeso,
    #[serde(alias = "DZD")]
    AlgerianDinar,
    #[serde(alias = "EGP")]
    EgyptianPound,
    #[serde(alias = "ERN")]
    EritreanNakfa,
    #[serde(alias = "ETB")]
    EthiopianBirr,
    #[serde(alias = "EUR")]
    Euro,
    #[serde(alias = "FJD")]
    FijiDollar,
    #[serde(alias = "FKP")]
    FalklandIslandsPound,
    #[serde(alias = "GBP")]
    PoundSterling,
    #[serde(alias = "GEL")]
    GeorgianLari,
    #[serde(alias = "GHS")]
    GhanaianCedi,
    #[serde(alias = "GIP")]
    GibraltarPound,
    #[serde(alias = "GMD")]
    GambianDalasi,
    #[serde(alias = "GNF")]
    GuineanFranc,
    #[serde(alias = "GTQ")]
    GuatemalanQuetzal,
    #[serde(alias = "GYD")]
    GuyaneseDollar,
    #[serde(alias = "HKD")]
    HongKongDollar,
    #[serde(alias = "HNL")]
    HonduranLempira,
    #[serde(alias = "HRK")]
    CroatianKuna,
    #[serde(alias = "HTG")]
    HaitianGourde,
    #[serde(alias = "HUF")]
    HungarianForint,
    #[serde(alias = "IDR")]
    IndonesianRupiah,
    #[serde(alias = "ILS")]
    IsraeliNewShekel,
    #[serde(alias = "INR")]
    IndianRupee,
    #[serde(alias = "IQD")]
    IraqiDinar,
    #[serde(alias = "IRR")]
    IranianRial,
    #[serde(alias = "ISK")]
    IcelandicKrona,
    #[serde(alias = "JMD")]
    JamaicanDollar,
    #[serde(alias = "JOD")]
    JordanianDinar,
    #[serde(alias = "JPY")]
    JapaneseYen,
    #[serde(alias = "KES")]
    KenyanShilling,
    #[serde(alias = "KGS")]
    KyrgyzstaniSom,
    #[serde(alias = "KHR")]
    CambodianRiel,
    #[serde(alias = "KMF")]
    ComoroFranc,
    #[serde(alias = "KPW")]
    NorthKoreanWon,
    #[serde(alias = "KRW")]
    SouthKoreanWon,
    #[serde(alias = "KWD")]
    KuwaitiDinar,
    #[serde(alias = "KYD")]
    CaymanIslandsDollar,
    #[serde(alias = "KZT")]
    KazakhstaniTenge,
    #[serde(alias = "LAK")]
    LaoKip,
    #[serde(alias = "LBP")]
    LebanesePound,
    #[serde(alias = "LKR")]
    SriLankanRupee,
    #[serde(alias = "LRD")]
    LiberianDollar,
    #[serde(alias = "LSL")]
    LesothoLoti,
    #[serde(alias = "LYD")]
    LibyanDinar,
    #[serde(alias = "MAD")]
    MoroccanDirham,
    #[serde(alias = "MDL")]
    MoldovanLeu,
    #[serde(alias = "MGA")]
    MalagasyAriary,
    #[serde(alias = "MKD")]
    MacedonianDenar,
    #[serde(alias = "MMK")]
    MyanmarKyat,
    #[serde(alias = "MNT")]
    MongolianTogrog,
    #[serde(alias = "MOP")]
    MacanesePataca,
    #[serde(alias = "MRU")]
    MauritanianOuguiya,
    #[serde(alias = "MUR")]
    MauritianRupee,
    #[serde(alias = "MVR")]
    MaldivianRufiyaa,
    #[serde(alias = "MWK")]
    MalawianKwacha,
    #[serde(alias = "MXN")]
    MexicanPeso,
    #[serde(alias = "MXV")]
    MexicanUnidadDeInversion,
    #[serde(alias = "MYR")]
    MalaysianRinggit,
    #[serde(alias = "MZN")]
    MozambicanMetical,
    #[serde(alias = "NAD")]
    NamibianDollar,
    #[serde(alias = "NGN")]
    NigerianNaira,
    #[serde(alias = "NIO")]
    NicaraguanCordoba,
    #[serde(alias = "NOK")]
    NorwegianKrone,
    #[serde(alias = "NPR")]
    NepaleseRupee,
    #[serde(alias = "NZD")]
    NewZealandDollar,
    #[serde(alias = "OMR")]
    OmaniRial,
    #[serde(alias = "PAB")]
    PanamanianBalboa,
    #[serde(alias = "PEN")]
    PeruvianSol,
    #[serde(alias = "PGK")]
    PapuaNewGuineanKina,
    #[serde(alias = "PHP")]
    PhilippinePeso,
    #[serde(alias = "PKR")]
    PakistaniRupee,
    #[serde(alias = "PLN")]
    PolishZloty,
    #[serde(alias = "PYG")]
    ParaguayanGuarani,
    #[serde(alias = "QAR")]
    QatariRiyal,
    #[serde(alias = "RON")]
    RomanianLeu,
    #[serde(alias = "RSD")]
    SerbianDinar,
    #[serde(alias = "RUB")]
    RussianRuble,
    #[serde(alias = "RWF")]
    RwandanFranc,
    #[serde(alias = "SAR")]
    SaudiRiyal,
    #[serde(alias = "SBD")]
    SolomonIslandsDollar,
    #[serde(alias = "SCR")]
    SeychellesRupee,
    #[serde(alias = "SDG")]
    SudanesePound,
    #[serde(alias = "SHP")]
    SwedishKrona,
    #[serde(alias = "SLL")]
    SingaporeDollar,
    #[serde(alias = "SHP")]
    SaintHelenaPound,
    #[serde(alias = "SLL")]
    SierraLeoneanLeone,
    #[serde(alias = "SOS")]
    SomaliShilling,
    #[serde(alias = "SRD")]
    SurinameseDollar,
    #[serde(alias = "SSP")]
    SouthSudanesePound,
    #[serde(alias = "STN")]
    SaoTomeAndPrincipeDobra,
    #[serde(alias = "SVC")]
    SalvadoranColon,
    #[serde(alias = "SYP")]
    SyrianPound,
    #[serde(alias = "SZL")]
    SwaziLilangeni,
    #[serde(alias = "THB")]
    ThaiBaht,
    #[serde(alias = "TJS")]
    TajikistaniSomoni,
    #[serde(alias = "TMT")]
    TurkmenistanManat,
    #[serde(alias = "TND")]
    TunisianDinar,
    #[serde(alias = "TOP")]
    TonganPaanga,
    #[serde(alias = "TRY")]
    TurkishLira,
    #[serde(alias = "TTD")]
    TrinidadAndTobagoDollar,
    #[serde(alias = "TWD")]
    NewTaiwanDollar,
    #[serde(alias = "TZS")]
    TanzanianShilling,
    #[serde(alias = "UAH")]
    UkrainianHryvnia,
    #[serde(alias = "UGX")]
    UgandanShilling,
    #[serde(alias = "USD")]
    UnitedStatesDollar,
    #[serde(alias = "USN")]
    UnitedStatesDollarNextDay,
    #[serde(alias = "UYI")]
    UruguayPesoEnUnidadesIndexadas,
    #[serde(alias = "UYU")]
    UruguayanPeso,
    #[serde(alias = "UYW")]
    UnidadPrevisional,
    #[serde(alias = "UZS")]
    UzbekistanSom,
    #[serde(alias = "VES")]
    VenezuelanBolivarSoberano,
    #[serde(alias = "VND")]
    VietnameseDong,
    #[serde(alias = "VUV")]
    VanuatuVatu,
    #[serde(alias = "WST")]
    SamoanTala,
    #[serde(alias = "XAF")]
    CfaFrancBeac,
    #[serde(alias = "XAG")]
    Silver,
    #[serde(alias = "XAU")]
    Gold,
    #[serde(alias = "XBA")]
    EuropeanCompositeUnit,
    #[serde(alias = "XBB")]
    EuropeanMonetaryUnit,
    #[serde(alias = "XBC")]
    EuropeanUnitOfAccount9,
    #[serde(alias = "XBD")]
    EuropeanUnitOfAccount17,
    #[serde(alias = "XCD")]
    EastCaribbeanDollar,
    #[serde(alias = "XDR")]
    SpecialDrawingRights,
    #[serde(alias = "XOF")]
    CfaFrancBceao,
    #[serde(alias = "XPD")]
    Palladium,
    #[serde(alias = "XPF")]
    CfpFranc,
    #[serde(alias = "XPT")]
    Platinum,
    #[serde(alias = "XSU")]
    Sucre,
    #[serde(alias = "XTS")]
    CodeReservedForTesting,
    #[serde(alias = "XUA")]
    AdbUnitOfAccount,
    #[serde(alias = "XXX")]
    NoCurrency,
    #[serde(alias = "YER")]
    YemeniRial,
    #[serde(alias = "ZAR")]
    SouthAfricanRand,
    #[serde(alias = "ZMW")]
    ZambianKwacha,
    #[serde(alias = "ZWL")]
    ZimbabweanDollar,
} // enum

impl std::convert::From<&Currency> for String {
    /// Converts a `Currency` enum to a `String` that contains an [ISO 4217
    /// currency code](https://en.wikipedia.org/wiki/ISO_4217).
    fn from(currency: &Currency) -> String {
        match currency {
            Currency::UnitedArabEmiratesDirham => String::from("AED"),
            Currency::AfghanAfghani => String::from("AFN"),
            Currency::AlbanianLek => String::from("ALL"),
            Currency::ArmenianDram => String::from("AMD"),
            Currency::NetherlandsAntilleanGuilder => String::from("ANG"),
            Currency::AngolanKwanza => String::from("AOA"),
            Currency::ArgentinePeso => String::from("ARS"),
            Currency::AustralianDollar => String::from("AUD"),
            Currency::ArubanFlorin => String::from("AWG"),
            Currency::AzerbaijaniManat => String::from("AZN"),
            Currency::BosniaAndHerzegovinaConvertibleMark => String::from("BAM"),
            Currency::BarbadosDollar => String::from("BBD"),
            Currency::BangladeshiTaka => String::from("BDT"),
            Currency::BulgarianLev => String::from("BGN"),
            Currency::BahrainiDinar => String::from("BHD"),
            Currency::BurundianFranc => String::from("BIF"),
            Currency::BermudianDollar => String::from("BMD"),
            Currency::BruneiDollar => String::from("BND"),
            Currency::Boliviano => String::from("BOB"),
            Currency::BolivianMvdol => String::from("BOV"),
            Currency::BrazilianReal => String::from("BRL"),
            Currency::BahamianDollar => String::from("BSD"),
            Currency::BhutaneseNgultrum => String::from("BTN"),
            Currency::BotswanaPula => String::from("BWP"),
            Currency::BelarusianRuble => String::from("BYN"),
            Currency::BelizeDollar => String::from("BZD"),
            Currency::CanadianDollar => String::from("CAD"),
            Currency::CongoleseFranc => String::from("CDF"),
            Currency::WirEuro => String::from("CHE"),
            Currency::SwissFranc => String::from("CHF"),
            Currency::WirFranc => String::from("CHW"),
            Currency::UnidadDeFomento => String::from("CLF"),
            Currency::ChileanPeso => String::from("CLP"),
            Currency::RenminbiYuan => String::from("CNY"),
            Currency::ColombianPeso => String::from("COP"),
            Currency::UnidadDeValorReal => String::from("COU"),
            Currency::CostaRicanColon => String::from("CRC"),
            Currency::CubanConvertiblePeso => String::from("CUC"),
            Currency::CubanPeso => String::from("CUP"),
            Currency::CapeVerdeanEscudo => String::from("CVE"),
            Currency::CzechKoruna => String::from("CZK"),
            Currency::DjiboutianFranc => String::from("DJF"),
            Currency::DanishKrone => String::from("DKK"),
            Currency::DominicanPeso => String::from("DOP"),
            Currency::AlgerianDinar => String::from("DZD"),
            Currency::EgyptianPound => String::from("EGP"),
            Currency::EritreanNakfa => String::from("ERN"),
            Currency::EthiopianBirr => String::from("ETB"),
            Currency::Euro => String::from("EUR"),
            Currency::FijiDollar => String::from("FJD"),
            Currency::FalklandIslandsPound => String::from("FKP"),
            Currency::PoundSterling => String::from("GBP"),
            Currency::GeorgianLari => String::from("GEL"),
            Currency::GhanaianCedi => String::from("GHS"),
            Currency::GibraltarPound => String::from("GIP"),
            Currency::GambianDalasi => String::from("GMD"),
            Currency::GuineanFranc => String::from("GNF"),
            Currency::GuatemalanQuetzal => String::from("GTQ"),
            Currency::GuyaneseDollar => String::from("GYD"),
            Currency::HongKongDollar => String::from("HKD"),
            Currency::HonduranLempira => String::from("HNL"),
            Currency::CroatianKuna => String::from("HRK"),
            Currency::HaitianGourde => String::from("HTG"),
            Currency::HungarianForint => String::from("HUF"),
            Currency::IndonesianRupiah => String::from("IDR"),
            Currency::IsraeliNewShekel => String::from("ILS"),
            Currency::IndianRupee => String::from("INR"),
            Currency::IraqiDinar => String::from("IQD"),
            Currency::IranianRial => String::from("IRR"),
            Currency::IcelandicKrona => String::from("ISK"),
            Currency::JamaicanDollar => String::from("JMD"),
            Currency::JordanianDinar => String::from("JOD"),
            Currency::JapaneseYen => String::from("JPY"),
            Currency::KenyanShilling => String::from("KES"),
            Currency::KyrgyzstaniSom => String::from("KGS"),
            Currency::CambodianRiel => String::from("KHR"),
            Currency::ComoroFranc => String::from("KMF"),
            Currency::NorthKoreanWon => String::from("KPW"),
            Currency::SouthKoreanWon => String::from("KRW"),
            Currency::KuwaitiDinar => String::from("KWD"),
            Currency::CaymanIslandsDollar => String::from("KYD"),
            Currency::KazakhstaniTenge => String::from("KZT"),
            Currency::LaoKip => String::from("LAK"),
            Currency::LebanesePound => String::from("LBP"),
            Currency::SriLankanRupee => String::from("LKR"),
            Currency::LiberianDollar => String::from("LRD"),
            Currency::LesothoLoti => String::from("LSL"),
            Currency::LibyanDinar => String::from("LYD"),
            Currency::MoroccanDirham => String::from("MAD"),
            Currency::MoldovanLeu => String::from("MDL"),
            Currency::MalagasyAriary => String::from("MGA"),
            Currency::MacedonianDenar => String::from("MKD"),
            Currency::MyanmarKyat => String::from("MMK"),
            Currency::MongolianTogrog => String::from("MNT"),
            Currency::MacanesePataca => String::from("MOP"),
            Currency::MauritanianOuguiya => String::from("MRU"),
            Currency::MauritianRupee => String::from("MUR"),
            Currency::MaldivianRufiyaa => String::from("MVR"),
            Currency::MalawianKwacha => String::from("MWK"),
            Currency::MexicanPeso => String::from("MXN"),
            Currency::MexicanUnidadDeInversion => String::from("MXV"),
            Currency::MalaysianRinggit => String::from("MYR"),
            Currency::MozambicanMetical => String::from("MZN"),
            Currency::NamibianDollar => String::from("NAD"),
            Currency::NigerianNaira => String::from("NGN"),
            Currency::NicaraguanCordoba => String::from("NIO"),
            Currency::NorwegianKrone => String::from("NOK"),
            Currency::NepaleseRupee => String::from("NPR"),
            Currency::NewZealandDollar => String::from("NZD"),
            Currency::OmaniRial => String::from("OMR"),
            Currency::PanamanianBalboa => String::from("PAB"),
            Currency::PeruvianSol => String::from("PEN"),
            Currency::PapuaNewGuineanKina => String::from("PGK"),
            Currency::PhilippinePeso => String::from("PHP"),
            Currency::PakistaniRupee => String::from("PKR"),
            Currency::PolishZloty => String::from("PLN"),
            Currency::ParaguayanGuarani => String::from("PYG"),
            Currency::QatariRiyal => String::from("QAR"),
            Currency::RomanianLeu => String::from("RON"),
            Currency::SerbianDinar => String::from("RSD"),
            Currency::RussianRuble => String::from("RUB"),
            Currency::RwandanFranc => String::from("RWF"),
            Currency::SaudiRiyal => String::from("SAR"),
            Currency::SolomonIslandsDollar => String::from("SBD"),
            Currency::SeychellesRupee => String::from("SCR"),
            Currency::SudanesePound => String::from("SDG"),
            Currency::SwedishKrona => String::from("SEK"),
            Currency::SingaporeDollar => String::from("SGD"),
            Currency::SaintHelenaPound => String::from("SHP"),
            Currency::SierraLeoneanLeone => String::from("SLL"),
            Currency::SomaliShilling => String::from("SOS"),
            Currency::SurinameseDollar => String::from("SRD"),
            Currency::SouthSudanesePound => String::from("SSP"),
            Currency::SaoTomeAndPrincipeDobra => String::from("STN"),
            Currency::SalvadoranColon => String::from("SVC"),
            Currency::SyrianPound => String::from("SYP"),
            Currency::SwaziLilangeni => String::from("SZL"),
            Currency::ThaiBaht => String::from("THB"),
            Currency::TajikistaniSomoni => String::from("TJS"),
            Currency::TurkmenistanManat => String::from("TMT"),
            Currency::TunisianDinar => String::from("TND"),
            Currency::TonganPaanga => String::from("TOP"),
            Currency::TurkishLira => String::from("TRY"),
            Currency::TrinidadAndTobagoDollar => String::from("TTD"),
            Currency::NewTaiwanDollar => String::from("TWD"),
            Currency::TanzanianShilling => String::from("TZS"),
            Currency::UkrainianHryvnia => String::from("UAH"),
            Currency::UgandanShilling => String::from("UGX"),
            Currency::UnitedStatesDollar => String::from("USD"),
            Currency::UnitedStatesDollarNextDay => String::from("USN"),
            Currency::UruguayPesoEnUnidadesIndexadas => String::from("UYI"),
            Currency::UruguayanPeso => String::from("UYU"),
            Currency::UnidadPrevisional => String::from("UYW"),
            Currency::UzbekistanSom => String::from("UZS"),
            Currency::VenezuelanBolivarSoberano => String::from("VES"),
            Currency::VietnameseDong => String::from("VND"),
            Currency::VanuatuVatu => String::from("VUV"),
            Currency::SamoanTala => String::from("WST"),
            Currency::CfaFrancBeac => String::from("XAF"),
            Currency::Silver => String::from("XAG"),
            Currency::Gold => String::from("XAU"),
            Currency::EuropeanCompositeUnit => String::from("XBA"),
            Currency::EuropeanMonetaryUnit => String::from("XBB"),
            Currency::EuropeanUnitOfAccount9 => String::from("XBC"),
            Currency::EuropeanUnitOfAccount17 => String::from("XBD"),
            Currency::EastCaribbeanDollar => String::from("XCD"),
            Currency::SpecialDrawingRights => String::from("XDR"),
            Currency::CfaFrancBceao => String::from("XOF"),
            Currency::Palladium => String::from("XPD"),
            Currency::CfpFranc => String::from("CFP franc"),
            Currency::Platinum => String::from("XPT"),
            Currency::Sucre => String::from("XSU"),
            Currency::CodeReservedForTesting => String::from("XTS"),
            Currency::AdbUnitOfAccount => String::from("XUA"),
            Currency::NoCurrency => String::from("XXX"),
            Currency::YemeniRial => String::from("YER"),
            Currency::SouthAfricanRand => String::from("ZAR"),
            Currency::ZambianKwacha => String::from("ZMW"),
            Currency::ZimbabweanDollar => String::from("ZWL"),
        } // match
    } // fn
} // impl

impl std::convert::TryFrom<String> for Currency {

    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;

    /// Gets a `Currency` enum from a `String` that contains a supported [ISO
    /// 4217 currency code](https://en.wikipedia.org/wiki/ISO_4217).
    fn try_from(currency: String) -> Result<Currency, Error> {
        match currency.as_ref() {
            "AED" => Ok(Currency::UnitedArabEmiratesDirham),
            "AFN" => Ok(Currency::AfghanAfghani),
            "ALL" => Ok(Currency::AlbanianLek),
            "AMD" => Ok(Currency::ArmenianDram),
            "ANG" => Ok(Currency::NetherlandsAntilleanGuilder),
            "AOA" => Ok(Currency::AngolanKwanza),
            "ARS" => Ok(Currency::ArgentinePeso),
            "AUD" => Ok(Currency::AustralianDollar),
            "AWG" => Ok(Currency::ArubanFlorin),
            "AZN" => Ok(Currency::AzerbaijaniManat),
            "BAM" => Ok(Currency::BosniaAndHerzegovinaConvertibleMark),
            "BBD" => Ok(Currency::BarbadosDollar),
            "BDT" => Ok(Currency::BangladeshiTaka),
            "BGN" => Ok(Currency::BulgarianLev),
            "BHD" => Ok(Currency::BahrainiDinar),
            "BIF" => Ok(Currency::BurundianFranc),
            "BMD" => Ok(Currency::BermudianDollar),
            "BND" => Ok(Currency::BruneiDollar),
            "BOB" => Ok(Currency::Boliviano),
            "BOV" => Ok(Currency::BolivianMvdol),
            "BRL" => Ok(Currency::BrazilianReal),
            "BSD" => Ok(Currency::BahamianDollar),
            "BTN" => Ok(Currency::BhutaneseNgultrum),
            "BWP" => Ok(Currency::BotswanaPula),
            "BYN" => Ok(Currency::BelarusianRuble),
            "BZD" => Ok(Currency::BelizeDollar),
            "CAD" => Ok(Currency::CanadianDollar),
            "CDF" => Ok(Currency::CongoleseFranc),
            "CHE" => Ok(Currency::WirEuro),
            "CHF" => Ok(Currency::SwissFranc),
            "CHW" => Ok(Currency::WirFranc),
            "CLF" => Ok(Currency::UnidadDeFomento),
            "CLP" => Ok(Currency::ChileanPeso),
            "CNY" => Ok(Currency::RenminbiYuan),
            "COP" => Ok(Currency::ColombianPeso),
            "COU" => Ok(Currency::UnidadDeValorReal),
            "CRC" => Ok(Currency::CostaRicanColon),
            "CUC" => Ok(Currency::CubanConvertiblePeso),
            "CUP" => Ok(Currency::CubanPeso),
            "CVE" => Ok(Currency::CapeVerdeanEscudo),
            "CZK" => Ok(Currency::CzechKoruna),
            "DJF" => Ok(Currency::DjiboutianFranc),
            "DKK" => Ok(Currency::DanishKrone),
            "DOP" => Ok(Currency::DominicanPeso),
            "DZD" => Ok(Currency::AlgerianDinar),
            "EGP" => Ok(Currency::EgyptianPound),
            "ERN" => Ok(Currency::EritreanNakfa),
            "ETB" => Ok(Currency::EthiopianBirr),
            "EUR" => Ok(Currency::Euro),
            "FJD" => Ok(Currency::FijiDollar),
            "FKP" => Ok(Currency::FalklandIslandsPound),
            "GBP" => Ok(Currency::PoundSterling),
            "GEL" => Ok(Currency::GeorgianLari),
            "GHS" => Ok(Currency::GhanaianCedi),
            "GIP" => Ok(Currency::GibraltarPound),
            "GMD" => Ok(Currency::GambianDalasi),
            "GNF" => Ok(Currency::GuineanFranc),
            "GTQ" => Ok(Currency::GuatemalanQuetzal),
            "GYD" => Ok(Currency::GuyaneseDollar),
            "HKD" => Ok(Currency::HongKongDollar),
            "HNL" => Ok(Currency::HonduranLempira),
            "HRK" => Ok(Currency::CroatianKuna),
            "HTG" => Ok(Currency::HaitianGourde),
            "HUF" => Ok(Currency::HungarianForint),
            "IDR" => Ok(Currency::IndonesianRupiah),
            "ILS" => Ok(Currency::IsraeliNewShekel),
            "INR" => Ok(Currency::IndianRupee),
            "IQD" => Ok(Currency::IraqiDinar),
            "IRR" => Ok(Currency::IranianRial),
            "ISK" => Ok(Currency::IcelandicKrona),
            "JMD" => Ok(Currency::JamaicanDollar),
            "JOD" => Ok(Currency::JordanianDinar),
            "JPY" => Ok(Currency::JapaneseYen),
            "KES" => Ok(Currency::KenyanShilling),
            "KGS" => Ok(Currency::KyrgyzstaniSom),
            "KHR" => Ok(Currency::CambodianRiel),
            "KMF" => Ok(Currency::ComoroFranc),
            "KPW" => Ok(Currency::NorthKoreanWon),
            "KRW" => Ok(Currency::SouthKoreanWon),
            "KWD" => Ok(Currency::KuwaitiDinar),
            "KYD" => Ok(Currency::CaymanIslandsDollar),
            "KZT" => Ok(Currency::KazakhstaniTenge),
            "LAK" => Ok(Currency::LaoKip),
            "LBP" => Ok(Currency::LebanesePound),
            "LKR" => Ok(Currency::SriLankanRupee),
            "LRD" => Ok(Currency::LiberianDollar),
            "LSL" => Ok(Currency::LesothoLoti),
            "LYD" => Ok(Currency::LibyanDinar),
            "MAD" => Ok(Currency::MoroccanDirham),
            "MDL" => Ok(Currency::MoldovanLeu),
            "MGA" => Ok(Currency::MalagasyAriary),
            "MKD" => Ok(Currency::MacedonianDenar),
            "MMK" => Ok(Currency::MyanmarKyat),
            "MNT" => Ok(Currency::MongolianTogrog),
            "MOP" => Ok(Currency::MacanesePataca),
            "MRU" => Ok(Currency::MauritanianOuguiya),
            "MUR" => Ok(Currency::MauritianRupee),
            "MVR" => Ok(Currency::MaldivianRufiyaa),
            "MWK" => Ok(Currency::MalawianKwacha),
            "MXN" => Ok(Currency::MexicanPeso),
            "MXV" => Ok(Currency::MexicanUnidadDeInversion),
            "MYR" => Ok(Currency::MalaysianRinggit),
            "MZN" => Ok(Currency::MozambicanMetical),
            "NAD" => Ok(Currency::NamibianDollar),
            "NGN" => Ok(Currency::NigerianNaira),
            "NIO" => Ok(Currency::NicaraguanCordoba),
            "NOK" => Ok(Currency::NorwegianKrone),
            "NPR" => Ok(Currency::NepaleseRupee),
            "NZD" => Ok(Currency::NewZealandDollar),
            "OMR" => Ok(Currency::OmaniRial),
            "PAB" => Ok(Currency::PanamanianBalboa),
            "PEN" => Ok(Currency::PeruvianSol),
            "PGK" => Ok(Currency::PapuaNewGuineanKina),
            "PHP" => Ok(Currency::PhilippinePeso),
            "PKR" => Ok(Currency::PakistaniRupee),
            "PLN" => Ok(Currency::PolishZloty),
            "PYG" => Ok(Currency::ParaguayanGuarani),
            "QAR" => Ok(Currency::QatariRiyal),
            "RON" => Ok(Currency::RomanianLeu),
            "RSD" => Ok(Currency::SerbianDinar),
            "RUB" => Ok(Currency::RussianRuble),
            "RWF" => Ok(Currency::RwandanFranc),
            "SAR" => Ok(Currency::SaudiRiyal),
            "SBD" => Ok(Currency::SolomonIslandsDollar),
            "SCR" => Ok(Currency::SeychellesRupee),
            "SDG" => Ok(Currency::SudanesePound),
            "SEK" => Ok(Currency::SwedishKrona),
            "SGD" => Ok(Currency::SingaporeDollar),
            "SHP" => Ok(Currency::SaintHelenaPound),
            "SLL" => Ok(Currency::SierraLeoneanLeone),
            "SOS" => Ok(Currency::SomaliShilling),
            "SRD" => Ok(Currency::SurinameseDollar),
            "SSP" => Ok(Currency::SouthSudanesePound),
            "STN" => Ok(Currency::SaoTomeAndPrincipeDobra),
            "SVC" => Ok(Currency::SalvadoranColon),
            "SYP" => Ok(Currency::SyrianPound),
            "SZL" => Ok(Currency::SwaziLilangeni),
            "THB" => Ok(Currency::ThaiBaht),
            "TJS" => Ok(Currency::TajikistaniSomoni),
            "TMT" => Ok(Currency::TurkmenistanManat),
            "TND" => Ok(Currency::TunisianDinar),
            "TOP" => Ok(Currency::TonganPaanga),
            "TRY" => Ok(Currency::TurkishLira),
            "TTD" => Ok(Currency::TrinidadAndTobagoDollar),
            "TWD" => Ok(Currency::NewTaiwanDollar),
            "TZS" => Ok(Currency::TanzanianShilling),
            "UAH" => Ok(Currency::UkrainianHryvnia),
            "UGX" => Ok(Currency::UgandanShilling),
            "USD" => Ok(Currency::UnitedStatesDollar),
            "USN" => Ok(Currency::UnitedStatesDollarNextDay),
            "UYI" => Ok(Currency::UruguayPesoEnUnidadesIndexadas),
            "UYU" => Ok(Currency::UruguayanPeso),
            "UYW" => Ok(Currency::UnidadPrevisional),
            "UZS" => Ok(Currency::UzbekistanSom),
            "VES" => Ok(Currency::VenezuelanBolivarSoberano),
            "VND" => Ok(Currency::VietnameseDong),
            "VUV" => Ok(Currency::VanuatuVatu),
            "WST" => Ok(Currency::SamoanTala),
            "XAF" => Ok(Currency::CfaFrancBeac),
            "XAG" => Ok(Currency::Silver),
            "XAU" => Ok(Currency::Gold),
            "XBA" => Ok(Currency::EuropeanCompositeUnit),
            "XBB" => Ok(Currency::EuropeanMonetaryUnit),
            "XBC" => Ok(Currency::EuropeanUnitOfAccount9),
            "XBD" => Ok(Currency::EuropeanUnitOfAccount17),
            "XCD" => Ok(Currency::EastCaribbeanDollar),
            "XDR" => Ok(Currency::SpecialDrawingRights),
            "XOF" => Ok(Currency::CfaFrancBceao),
            "XPD" => Ok(Currency::Palladium),
            "XPF" => Ok(Currency::CfpFranc),
            "XPT" => Ok(Currency::Platinum),
            "XSU" => Ok(Currency::Sucre),
            "XTS" => Ok(Currency::CodeReservedForTesting),
            "XUA" => Ok(Currency::AdbUnitOfAccount),
            "XXX" => Ok(Currency::NoCurrency),
            "YER" => Ok(Currency::YemeniRial),
            "ZAR" => Ok(Currency::SouthAfricanRand),
            "ZMW" => Ok(Currency::ZambianKwacha),
            "ZWL" => Ok(Currency::ZimbabweanDollar),
            _ => Err(Error::InvalidCurrencyCode(currency)),
        } // match
    } // fn
} // impl

impl std::default::Default for Currency {
    /// Returns a reasonable default variant for the `Currency` enum type.
    fn default() -> Self {
        Currency::UnitedStatesDollar
    } // fn
} // impl

impl std::fmt::Display for Currency {
    /// Formats a `Currency` enum into a string that is presentable to the end
    /// user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Currency::UnitedArabEmiratesDirham => write!(f, "United Arab Emirates dirham"),
            Currency::AfghanAfghani => write!(f, "Afghan afghani"),
            Currency::AlbanianLek => write!(f, "Albanian lek"),
            Currency::ArmenianDram => write!(f, "Armenian dram"),
            Currency::NetherlandsAntilleanGuilder => write!(f, "Netherlands Antillean guilder"),
            Currency::AngolanKwanza => write!(f, "Angolan kwanza"),
            Currency::ArgentinePeso => write!(f, "Argentine peso"),
            Currency::AustralianDollar => write!(f, "Australian dollar"),
            Currency::ArubanFlorin => write!(f, "Aruban florin"),
            Currency::AzerbaijaniManat => write!(f, "Azerbaijani manat"),
            Currency::BosniaAndHerzegovinaConvertibleMark => write!(f, "Bosnia and Herzegovina convertible mark"),
            Currency::BarbadosDollar => write!(f, "Barbados dollar"),
            Currency::BangladeshiTaka => write!(f, "Bangladeshi taka"),
            Currency::BulgarianLev => write!(f, "Bulgarian lev"),
            Currency::BahrainiDinar => write!(f, "Bahraini dinar"),
            Currency::BurundianFranc => write!(f, "Burundian franc"),
            Currency::BermudianDollar => write!(f, "Bermudian dollar"),
            Currency::BruneiDollar => write!(f, "Brunei dollar"),
            Currency::Boliviano => write!(f, "Boliviano"),
            Currency::BolivianMvdol => write!(f, "Bolivian Mvdol"),
            Currency::BrazilianReal => write!(f, "Brazilian real"),
            Currency::BahamianDollar => write!(f, "Bahamian dollar"),
            Currency::BhutaneseNgultrum => write!(f, "Bhutanese ngultrum"),
            Currency::BotswanaPula => write!(f, "Botswana pula"),
            Currency::BelarusianRuble => write!(f, "Belarusian ruble"),
            Currency::BelizeDollar => write!(f, "Belize dollar"),
            Currency::CanadianDollar => write!(f, "Canadian dollar"),
            Currency::CongoleseFranc => write!(f, "Congolese franc"),
            Currency::WirEuro => write!(f, "WIR Euro"),
            Currency::SwissFranc => write!(f, "Swiss franc"),
            Currency::WirFranc => write!(f, "WIR Franc"),
            Currency::UnidadDeFomento => write!(f, "Unidad de Fomento"),
            Currency::ChileanPeso => write!(f, "Chilean peso"),
            Currency::RenminbiYuan => write!(f, "Renminbi yuan"),
            Currency::ColombianPeso => write!(f, "Colombian peso"),
            Currency::UnidadDeValorReal => write!(f, "Unidad de Valor Real"),
            Currency::CostaRicanColon => write!(f, "Costa Rican colon"),
            Currency::CubanConvertiblePeso => write!(f, "Cuban convertible peso"),
            Currency::CubanPeso => write!(f, "Cuban peso"),
            Currency::CapeVerdeanEscudo => write!(f, "Cape Verdean escudo"),
            Currency::CzechKoruna => write!(f, "Czech koruna"),
            Currency::DjiboutianFranc => write!(f, "Djiboutian franc"),
            Currency::DanishKrone => write!(f, "Danish krone"),
            Currency::DominicanPeso => write!(f, "Dominican peso"),
            Currency::AlgerianDinar => write!(f, "Algerian dinar"),
            Currency::EgyptianPound => write!(f, "Egyptian pound"),
            Currency::EritreanNakfa => write!(f, "Eritrean nakfa"),
            Currency::EthiopianBirr => write!(f, "Ethiopian birr"),
            Currency::Euro => write!(f, "Euro"),
            Currency::FijiDollar => write!(f, "Fiji dollar"),
            Currency::FalklandIslandsPound => write!(f, "Falkland Islands pound"),
            Currency::PoundSterling => write!(f, "Pound sterling"),
            Currency::GeorgianLari => write!(f, "Georgian lari"),
            Currency::GhanaianCedi => write!(f, "Ghanaian cedi"),
            Currency::GibraltarPound => write!(f, "Gibraltar pound"),
            Currency::GambianDalasi => write!(f, "Gambian dalasi"),
            Currency::GuineanFranc => write!(f, "Guinean franc"),
            Currency::GuatemalanQuetzal => write!(f, "Guatemalan quetzal"),
            Currency::GuyaneseDollar => write!(f, "Guyanese dollar"),
            Currency::HongKongDollar => write!(f, "Hong Kong dollar"),
            Currency::HonduranLempira => write!(f, "Honduran lempira"),
            Currency::CroatianKuna => write!(f, "Croatian kuna"),
            Currency::HaitianGourde => write!(f, "Haitian gourde"),
            Currency::HungarianForint => write!(f, "Hungarian forint"),
            Currency::IndonesianRupiah => write!(f, "Indonesian rupiah"),
            Currency::IsraeliNewShekel => write!(f, "Israeli new shekel"),
            Currency::IndianRupee => write!(f, "Indian rupee"),
            Currency::IraqiDinar => write!(f, "Iraqi dinar"),
            Currency::IranianRial => write!(f, "Iranian rial"),
            Currency::IcelandicKrona => write!(f, "Icelandic króna"),
            Currency::JamaicanDollar => write!(f, "Jamaican dollar"),
            Currency::JordanianDinar => write!(f, "Jordanian dinar"),
            Currency::JapaneseYen => write!(f, "Japanese yen"),
            Currency::KenyanShilling => write!(f, "Kenyan shilling"),
            Currency::KyrgyzstaniSom => write!(f, "Kyrgyzstani som"),
            Currency::CambodianRiel => write!(f, "Cambodian riel"),
            Currency::ComoroFranc => write!(f, "Comoro franc"),
            Currency::NorthKoreanWon => write!(f, "North Korean won"),
            Currency::SouthKoreanWon => write!(f, "South Korean won"),
            Currency::KuwaitiDinar => write!(f, "Kuwaiti dinar"),
            Currency::CaymanIslandsDollar => write!(f, "Cayman Islands dollar"),
            Currency::KazakhstaniTenge => write!(f, "Kazakhstani tenge"),
            Currency::LaoKip => write!(f, "Lao kip"),
            Currency::LebanesePound => write!(f, "Lebanese pound"),
            Currency::SriLankanRupee => write!(f, "Sri Lankan rupee"),
            Currency::LiberianDollar => write!(f, "Liberian dollar"),
            Currency::LesothoLoti => write!(f, "Lesotho loti"),
            Currency::LibyanDinar => write!(f, "Libyan dinar"),
            Currency::MoroccanDirham => write!(f, "Moroccan dirham"),
            Currency::MoldovanLeu => write!(f, "Moldovan leu"),
            Currency::MalagasyAriary => write!(f, "Malagasy ariary"),
            Currency::MacedonianDenar => write!(f, "Macedonian denar"),
            Currency::MyanmarKyat => write!(f, "Myanmar kyat"),
            Currency::MongolianTogrog => write!(f, "Mongolian tögrög"),
            Currency::MacanesePataca => write!(f, "Macanese pataca"),
            Currency::MauritanianOuguiya => write!(f, "Mauritanian ouguiya"),
            Currency::MauritianRupee => write!(f, "Mauritian rupee"),
            Currency::MaldivianRufiyaa => write!(f, "Maldivian rufiyaa"),
            Currency::MalawianKwacha => write!(f, "Malawian kwacha"),
            Currency::MexicanPeso => write!(f, "Mexican peso"),
            Currency::MexicanUnidadDeInversion => write!(f, "Mexican Unidad de Inversion"),
            Currency::MalaysianRinggit => write!(f, "Malaysian ringgit"),
            Currency::MozambicanMetical => write!(f, "Mozambican metical"),
            Currency::NamibianDollar => write!(f, "Namibian dollar"),
            Currency::NigerianNaira => write!(f, "Nigerian naira"),
            Currency::NicaraguanCordoba => write!(f, "Nicaraguan córdoba"),
            Currency::NorwegianKrone => write!(f, "Norwegian krone"),
            Currency::NepaleseRupee => write!(f, "Nepalese rupee"),
            Currency::NewZealandDollar => write!(f, "New Zealand dollar"),
            Currency::OmaniRial => write!(f, "Omani rial"),
            Currency::PanamanianBalboa => write!(f, "Panamanian balboa"),
            Currency::PeruvianSol => write!(f, "Peruvian sol"),
            Currency::PapuaNewGuineanKina => write!(f, "Papua New Guinean kina"),
            Currency::PhilippinePeso => write!(f, "Philippine peso"),
            Currency::PakistaniRupee => write!(f, "Pakistani rupee"),
            Currency::PolishZloty => write!(f, "Polish złoty"),
            Currency::ParaguayanGuarani => write!(f, "Paraguayan guaraní"),
            Currency::QatariRiyal => write!(f, "Qatari riyal"),
            Currency::RomanianLeu => write!(f, "Romanian leu"),
            Currency::SerbianDinar => write!(f, "Serbian dinar"),
            Currency::RussianRuble => write!(f, "Russian ruble"),
            Currency::RwandanFranc => write!(f, "Rwandan franc"),
            Currency::SaudiRiyal => write!(f, "Saudi riyal"),
            Currency::SolomonIslandsDollar => write!(f, "Solomon Islands dollar"),
            Currency::SeychellesRupee => write!(f, "Seychelles rupee"),
            Currency::SudanesePound => write!(f, "Sudanese pound"),
            Currency::SwedishKrona => write!(f, "Swedish krona"),
            Currency::SingaporeDollar => write!(f, "Singapore dollar"),
            Currency::SaintHelenaPound => write!(f, "Saint Helena pound"),
            Currency::SierraLeoneanLeone => write!(f, "Sierra Leonean leone"),
            Currency::SomaliShilling => write!(f, "Somali shilling"),
            Currency::SurinameseDollar => write!(f, "Surinamese dollar"),
            Currency::SouthSudanesePound => write!(f, "South Sudanese pound"),
            Currency::SaoTomeAndPrincipeDobra => write!(f, "São Tomé and Príncipe dobra"),
            Currency::SalvadoranColon => write!(f, "Salvadoran colón"),
            Currency::SyrianPound => write!(f, "Syrian pound"),
            Currency::SwaziLilangeni => write!(f, "Swazi lilangeni"),
            Currency::ThaiBaht => write!(f, "Thai baht"),
            Currency::TajikistaniSomoni => write!(f, "Tajikistani somoni"),
            Currency::TurkmenistanManat => write!(f, "Turkmenistan manat"),
            Currency::TunisianDinar => write!(f, "Tunisian dinar"),
            Currency::TonganPaanga => write!(f, "Tongan paʻanga"),
            Currency::TurkishLira => write!(f, "Turkish lira"),
            Currency::TrinidadAndTobagoDollar => write!(f, "Trinidad and Tobago dollar"),
            Currency::NewTaiwanDollar => write!(f, "New Taiwan dollar"),
            Currency::TanzanianShilling => write!(f, "Tanzanian shilling"),
            Currency::UkrainianHryvnia => write!(f, "Ukrainian hryvnia"),
            Currency::UgandanShilling => write!(f, "Ugandan shilling"),
            Currency::UnitedStatesDollar => write!(f, "United States dollar"),
            Currency::UnitedStatesDollarNextDay => write!(f, "United States dollar next day"),
            Currency::UruguayPesoEnUnidadesIndexadas => write!(f, "Uruguay Peso en Unidades Indexadas"),
            Currency::UruguayanPeso => write!(f, "Uruguayan peso"),
            Currency::UnidadPrevisional => write!(f, "Unidad previsional"),
            Currency::UzbekistanSom => write!(f, "Uzbekistan som"),
            Currency::VenezuelanBolivarSoberano => write!(f, "Venezuelan bolívar soberano"),
            Currency::VietnameseDong => write!(f, "Vietnamese đồng"),
            Currency::VanuatuVatu => write!(f, "Vanuatu vatu"),
            Currency::SamoanTala => write!(f, "Samoan tala"),
            Currency::CfaFrancBeac => write!(f, "CFA franc BEAC"),
            Currency::Silver => write!(f, "Silver"),
            Currency::Gold => write!(f, "Gold"),
            Currency::EuropeanCompositeUnit => write!(f, "European Composite Unit"),
            Currency::EuropeanMonetaryUnit => write!(f, "European Monetary Unit"),
            Currency::EuropeanUnitOfAccount9 => write!(f, "European Unit of Account 9"),
            Currency::EuropeanUnitOfAccount17 => write!(f, "European Unit of Account 17"),
            Currency::EastCaribbeanDollar => write!(f, "East Caribbean dollar"),
            Currency::SpecialDrawingRights => write!(f, "Special drawing rights"),
            Currency::CfaFrancBceao => write!(f, "CFA franc BCEAO"),
            Currency::Palladium => write!(f, "Palladium"),
            Currency::CfpFranc => write!(f, "CFP franc"),
            Currency::Platinum => write!(f, "Platinum"),
            Currency::Sucre => write!(f, "SUCRE"),
            Currency::CodeReservedForTesting => write!(f, "Code reserved for testing"),
            Currency::AdbUnitOfAccount => write!(f, "ADB Unit of Account"),
            Currency::NoCurrency => write!(f, "No currency"),
            Currency::YemeniRial => write!(f, "Yemeni rial"),
            Currency::SouthAfricanRand => write!(f, "South African rand"),
            Currency::ZambianKwacha => write!(f, "Zambian kwacha"),
            Currency::ZimbabweanDollar => write!(f, "Zimbabwean dollar"),
        } // match
    } // fn
} // impl