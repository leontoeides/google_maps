//! Contains the `TransitCurrency` enum and its associated traits. It is used to
//! specify a currency. Included for use with the transit fares returned by
//! Google Maps Directions API.

use crate::directions::error::Error;
use phf::phf_map;
use serde::{Deserialize, Serialize, Deserializer};

// -----------------------------------------------------------------------------

/// A comprehensive list of currencies. At the moment this is used only for
/// Google Maps Transit Directions. The intent behind having _Serde_ convert
/// the currency code `String` to an `enum` is for efficient currency
/// conversions, information lookups, and manipulation in the future.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TransitCurrency {
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

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for TransitCurrency {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match TransitCurrency::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string()))
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&TransitCurrency> for String {
    /// Converts a `TransitCurrency` enum to a `String` that contains an [ISO
    /// 4217 currency code](https://en.wikipedia.org/wiki/ISO_4217).
    fn from(currency: &TransitCurrency) -> String {
        match currency {
            TransitCurrency::UnitedArabEmiratesDirham => String::from("AED"),
            TransitCurrency::AfghanAfghani => String::from("AFN"),
            TransitCurrency::AlbanianLek => String::from("ALL"),
            TransitCurrency::ArmenianDram => String::from("AMD"),
            TransitCurrency::NetherlandsAntilleanGuilder => String::from("ANG"),
            TransitCurrency::AngolanKwanza => String::from("AOA"),
            TransitCurrency::ArgentinePeso => String::from("ARS"),
            TransitCurrency::AustralianDollar => String::from("AUD"),
            TransitCurrency::ArubanFlorin => String::from("AWG"),
            TransitCurrency::AzerbaijaniManat => String::from("AZN"),
            TransitCurrency::BosniaAndHerzegovinaConvertibleMark => String::from("BAM"),
            TransitCurrency::BarbadosDollar => String::from("BBD"),
            TransitCurrency::BangladeshiTaka => String::from("BDT"),
            TransitCurrency::BulgarianLev => String::from("BGN"),
            TransitCurrency::BahrainiDinar => String::from("BHD"),
            TransitCurrency::BurundianFranc => String::from("BIF"),
            TransitCurrency::BermudianDollar => String::from("BMD"),
            TransitCurrency::BruneiDollar => String::from("BND"),
            TransitCurrency::Boliviano => String::from("BOB"),
            TransitCurrency::BolivianMvdol => String::from("BOV"),
            TransitCurrency::BrazilianReal => String::from("BRL"),
            TransitCurrency::BahamianDollar => String::from("BSD"),
            TransitCurrency::BhutaneseNgultrum => String::from("BTN"),
            TransitCurrency::BotswanaPula => String::from("BWP"),
            TransitCurrency::BelarusianRuble => String::from("BYN"),
            TransitCurrency::BelizeDollar => String::from("BZD"),
            TransitCurrency::CanadianDollar => String::from("CAD"),
            TransitCurrency::CongoleseFranc => String::from("CDF"),
            TransitCurrency::WirEuro => String::from("CHE"),
            TransitCurrency::SwissFranc => String::from("CHF"),
            TransitCurrency::WirFranc => String::from("CHW"),
            TransitCurrency::UnidadDeFomento => String::from("CLF"),
            TransitCurrency::ChileanPeso => String::from("CLP"),
            TransitCurrency::RenminbiYuan => String::from("CNY"),
            TransitCurrency::ColombianPeso => String::from("COP"),
            TransitCurrency::UnidadDeValorReal => String::from("COU"),
            TransitCurrency::CostaRicanColon => String::from("CRC"),
            TransitCurrency::CubanConvertiblePeso => String::from("CUC"),
            TransitCurrency::CubanPeso => String::from("CUP"),
            TransitCurrency::CapeVerdeanEscudo => String::from("CVE"),
            TransitCurrency::CzechKoruna => String::from("CZK"),
            TransitCurrency::DjiboutianFranc => String::from("DJF"),
            TransitCurrency::DanishKrone => String::from("DKK"),
            TransitCurrency::DominicanPeso => String::from("DOP"),
            TransitCurrency::AlgerianDinar => String::from("DZD"),
            TransitCurrency::EgyptianPound => String::from("EGP"),
            TransitCurrency::EritreanNakfa => String::from("ERN"),
            TransitCurrency::EthiopianBirr => String::from("ETB"),
            TransitCurrency::Euro => String::from("EUR"),
            TransitCurrency::FijiDollar => String::from("FJD"),
            TransitCurrency::FalklandIslandsPound => String::from("FKP"),
            TransitCurrency::PoundSterling => String::from("GBP"),
            TransitCurrency::GeorgianLari => String::from("GEL"),
            TransitCurrency::GhanaianCedi => String::from("GHS"),
            TransitCurrency::GibraltarPound => String::from("GIP"),
            TransitCurrency::GambianDalasi => String::from("GMD"),
            TransitCurrency::GuineanFranc => String::from("GNF"),
            TransitCurrency::GuatemalanQuetzal => String::from("GTQ"),
            TransitCurrency::GuyaneseDollar => String::from("GYD"),
            TransitCurrency::HongKongDollar => String::from("HKD"),
            TransitCurrency::HonduranLempira => String::from("HNL"),
            TransitCurrency::CroatianKuna => String::from("HRK"),
            TransitCurrency::HaitianGourde => String::from("HTG"),
            TransitCurrency::HungarianForint => String::from("HUF"),
            TransitCurrency::IndonesianRupiah => String::from("IDR"),
            TransitCurrency::IsraeliNewShekel => String::from("ILS"),
            TransitCurrency::IndianRupee => String::from("INR"),
            TransitCurrency::IraqiDinar => String::from("IQD"),
            TransitCurrency::IranianRial => String::from("IRR"),
            TransitCurrency::IcelandicKrona => String::from("ISK"),
            TransitCurrency::JamaicanDollar => String::from("JMD"),
            TransitCurrency::JordanianDinar => String::from("JOD"),
            TransitCurrency::JapaneseYen => String::from("JPY"),
            TransitCurrency::KenyanShilling => String::from("KES"),
            TransitCurrency::KyrgyzstaniSom => String::from("KGS"),
            TransitCurrency::CambodianRiel => String::from("KHR"),
            TransitCurrency::ComoroFranc => String::from("KMF"),
            TransitCurrency::NorthKoreanWon => String::from("KPW"),
            TransitCurrency::SouthKoreanWon => String::from("KRW"),
            TransitCurrency::KuwaitiDinar => String::from("KWD"),
            TransitCurrency::CaymanIslandsDollar => String::from("KYD"),
            TransitCurrency::KazakhstaniTenge => String::from("KZT"),
            TransitCurrency::LaoKip => String::from("LAK"),
            TransitCurrency::LebanesePound => String::from("LBP"),
            TransitCurrency::SriLankanRupee => String::from("LKR"),
            TransitCurrency::LiberianDollar => String::from("LRD"),
            TransitCurrency::LesothoLoti => String::from("LSL"),
            TransitCurrency::LibyanDinar => String::from("LYD"),
            TransitCurrency::MoroccanDirham => String::from("MAD"),
            TransitCurrency::MoldovanLeu => String::from("MDL"),
            TransitCurrency::MalagasyAriary => String::from("MGA"),
            TransitCurrency::MacedonianDenar => String::from("MKD"),
            TransitCurrency::MyanmarKyat => String::from("MMK"),
            TransitCurrency::MongolianTogrog => String::from("MNT"),
            TransitCurrency::MacanesePataca => String::from("MOP"),
            TransitCurrency::MauritanianOuguiya => String::from("MRU"),
            TransitCurrency::MauritianRupee => String::from("MUR"),
            TransitCurrency::MaldivianRufiyaa => String::from("MVR"),
            TransitCurrency::MalawianKwacha => String::from("MWK"),
            TransitCurrency::MexicanPeso => String::from("MXN"),
            TransitCurrency::MexicanUnidadDeInversion => String::from("MXV"),
            TransitCurrency::MalaysianRinggit => String::from("MYR"),
            TransitCurrency::MozambicanMetical => String::from("MZN"),
            TransitCurrency::NamibianDollar => String::from("NAD"),
            TransitCurrency::NigerianNaira => String::from("NGN"),
            TransitCurrency::NicaraguanCordoba => String::from("NIO"),
            TransitCurrency::NorwegianKrone => String::from("NOK"),
            TransitCurrency::NepaleseRupee => String::from("NPR"),
            TransitCurrency::NewZealandDollar => String::from("NZD"),
            TransitCurrency::OmaniRial => String::from("OMR"),
            TransitCurrency::PanamanianBalboa => String::from("PAB"),
            TransitCurrency::PeruvianSol => String::from("PEN"),
            TransitCurrency::PapuaNewGuineanKina => String::from("PGK"),
            TransitCurrency::PhilippinePeso => String::from("PHP"),
            TransitCurrency::PakistaniRupee => String::from("PKR"),
            TransitCurrency::PolishZloty => String::from("PLN"),
            TransitCurrency::ParaguayanGuarani => String::from("PYG"),
            TransitCurrency::QatariRiyal => String::from("QAR"),
            TransitCurrency::RomanianLeu => String::from("RON"),
            TransitCurrency::SerbianDinar => String::from("RSD"),
            TransitCurrency::RussianRuble => String::from("RUB"),
            TransitCurrency::RwandanFranc => String::from("RWF"),
            TransitCurrency::SaudiRiyal => String::from("SAR"),
            TransitCurrency::SolomonIslandsDollar => String::from("SBD"),
            TransitCurrency::SeychellesRupee => String::from("SCR"),
            TransitCurrency::SudanesePound => String::from("SDG"),
            TransitCurrency::SwedishKrona => String::from("SEK"),
            TransitCurrency::SingaporeDollar => String::from("SGD"),
            TransitCurrency::SaintHelenaPound => String::from("SHP"),
            TransitCurrency::SierraLeoneanLeone => String::from("SLL"),
            TransitCurrency::SomaliShilling => String::from("SOS"),
            TransitCurrency::SurinameseDollar => String::from("SRD"),
            TransitCurrency::SouthSudanesePound => String::from("SSP"),
            TransitCurrency::SaoTomeAndPrincipeDobra => String::from("STN"),
            TransitCurrency::SalvadoranColon => String::from("SVC"),
            TransitCurrency::SyrianPound => String::from("SYP"),
            TransitCurrency::SwaziLilangeni => String::from("SZL"),
            TransitCurrency::ThaiBaht => String::from("THB"),
            TransitCurrency::TajikistaniSomoni => String::from("TJS"),
            TransitCurrency::TurkmenistanManat => String::from("TMT"),
            TransitCurrency::TunisianDinar => String::from("TND"),
            TransitCurrency::TonganPaanga => String::from("TOP"),
            TransitCurrency::TurkishLira => String::from("TRY"),
            TransitCurrency::TrinidadAndTobagoDollar => String::from("TTD"),
            TransitCurrency::NewTaiwanDollar => String::from("TWD"),
            TransitCurrency::TanzanianShilling => String::from("TZS"),
            TransitCurrency::UkrainianHryvnia => String::from("UAH"),
            TransitCurrency::UgandanShilling => String::from("UGX"),
            TransitCurrency::UnitedStatesDollar => String::from("USD"),
            TransitCurrency::UnitedStatesDollarNextDay => String::from("USN"),
            TransitCurrency::UruguayPesoEnUnidadesIndexadas => String::from("UYI"),
            TransitCurrency::UruguayanPeso => String::from("UYU"),
            TransitCurrency::UnidadPrevisional => String::from("UYW"),
            TransitCurrency::UzbekistanSom => String::from("UZS"),
            TransitCurrency::VenezuelanBolivarSoberano => String::from("VES"),
            TransitCurrency::VietnameseDong => String::from("VND"),
            TransitCurrency::VanuatuVatu => String::from("VUV"),
            TransitCurrency::SamoanTala => String::from("WST"),
            TransitCurrency::CfaFrancBeac => String::from("XAF"),
            TransitCurrency::Silver => String::from("XAG"),
            TransitCurrency::Gold => String::from("XAU"),
            TransitCurrency::EuropeanCompositeUnit => String::from("XBA"),
            TransitCurrency::EuropeanMonetaryUnit => String::from("XBB"),
            TransitCurrency::EuropeanUnitOfAccount9 => String::from("XBC"),
            TransitCurrency::EuropeanUnitOfAccount17 => String::from("XBD"),
            TransitCurrency::EastCaribbeanDollar => String::from("XCD"),
            TransitCurrency::SpecialDrawingRights => String::from("XDR"),
            TransitCurrency::CfaFrancBceao => String::from("XOF"),
            TransitCurrency::Palladium => String::from("XPD"),
            TransitCurrency::CfpFranc => String::from("CFP franc"),
            TransitCurrency::Platinum => String::from("XPT"),
            TransitCurrency::Sucre => String::from("XSU"),
            TransitCurrency::CodeReservedForTesting => String::from("XTS"),
            TransitCurrency::AdbUnitOfAccount => String::from("XUA"),
            TransitCurrency::NoCurrency => String::from("XXX"),
            TransitCurrency::YemeniRial => String::from("YER"),
            TransitCurrency::SouthAfricanRand => String::from("ZAR"),
            TransitCurrency::ZambianKwacha => String::from("ZMW"),
            TransitCurrency::ZimbabweanDollar => String::from("ZWL"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static TRANSIT_CURRENCIES_BY_CODE: phf::Map<&'static str, TransitCurrency> = phf_map! {
    "AED" => TransitCurrency::UnitedArabEmiratesDirham,
    "AFN" => TransitCurrency::AfghanAfghani,
    "ALL" => TransitCurrency::AlbanianLek,
    "AMD" => TransitCurrency::ArmenianDram,
    "ANG" => TransitCurrency::NetherlandsAntilleanGuilder,
    "AOA" => TransitCurrency::AngolanKwanza,
    "ARS" => TransitCurrency::ArgentinePeso,
    "AUD" => TransitCurrency::AustralianDollar,
    "AWG" => TransitCurrency::ArubanFlorin,
    "AZN" => TransitCurrency::AzerbaijaniManat,
    "BAM" => TransitCurrency::BosniaAndHerzegovinaConvertibleMark,
    "BBD" => TransitCurrency::BarbadosDollar,
    "BDT" => TransitCurrency::BangladeshiTaka,
    "BGN" => TransitCurrency::BulgarianLev,
    "BHD" => TransitCurrency::BahrainiDinar,
    "BIF" => TransitCurrency::BurundianFranc,
    "BMD" => TransitCurrency::BermudianDollar,
    "BND" => TransitCurrency::BruneiDollar,
    "BOB" => TransitCurrency::Boliviano,
    "BOV" => TransitCurrency::BolivianMvdol,
    "BRL" => TransitCurrency::BrazilianReal,
    "BSD" => TransitCurrency::BahamianDollar,
    "BTN" => TransitCurrency::BhutaneseNgultrum,
    "BWP" => TransitCurrency::BotswanaPula,
    "BYN" => TransitCurrency::BelarusianRuble,
    "BZD" => TransitCurrency::BelizeDollar,
    "CAD" => TransitCurrency::CanadianDollar,
    "CDF" => TransitCurrency::CongoleseFranc,
    "CHE" => TransitCurrency::WirEuro,
    "CHF" => TransitCurrency::SwissFranc,
    "CHW" => TransitCurrency::WirFranc,
    "CLF" => TransitCurrency::UnidadDeFomento,
    "CLP" => TransitCurrency::ChileanPeso,
    "CNY" => TransitCurrency::RenminbiYuan,
    "COP" => TransitCurrency::ColombianPeso,
    "COU" => TransitCurrency::UnidadDeValorReal,
    "CRC" => TransitCurrency::CostaRicanColon,
    "CUC" => TransitCurrency::CubanConvertiblePeso,
    "CUP" => TransitCurrency::CubanPeso,
    "CVE" => TransitCurrency::CapeVerdeanEscudo,
    "CZK" => TransitCurrency::CzechKoruna,
    "DJF" => TransitCurrency::DjiboutianFranc,
    "DKK" => TransitCurrency::DanishKrone,
    "DOP" => TransitCurrency::DominicanPeso,
    "DZD" => TransitCurrency::AlgerianDinar,
    "EGP" => TransitCurrency::EgyptianPound,
    "ERN" => TransitCurrency::EritreanNakfa,
    "ETB" => TransitCurrency::EthiopianBirr,
    "EUR" => TransitCurrency::Euro,
    "FJD" => TransitCurrency::FijiDollar,
    "FKP" => TransitCurrency::FalklandIslandsPound,
    "GBP" => TransitCurrency::PoundSterling,
    "GEL" => TransitCurrency::GeorgianLari,
    "GHS" => TransitCurrency::GhanaianCedi,
    "GIP" => TransitCurrency::GibraltarPound,
    "GMD" => TransitCurrency::GambianDalasi,
    "GNF" => TransitCurrency::GuineanFranc,
    "GTQ" => TransitCurrency::GuatemalanQuetzal,
    "GYD" => TransitCurrency::GuyaneseDollar,
    "HKD" => TransitCurrency::HongKongDollar,
    "HNL" => TransitCurrency::HonduranLempira,
    "HRK" => TransitCurrency::CroatianKuna,
    "HTG" => TransitCurrency::HaitianGourde,
    "HUF" => TransitCurrency::HungarianForint,
    "IDR" => TransitCurrency::IndonesianRupiah,
    "ILS" => TransitCurrency::IsraeliNewShekel,
    "INR" => TransitCurrency::IndianRupee,
    "IQD" => TransitCurrency::IraqiDinar,
    "IRR" => TransitCurrency::IranianRial,
    "ISK" => TransitCurrency::IcelandicKrona,
    "JMD" => TransitCurrency::JamaicanDollar,
    "JOD" => TransitCurrency::JordanianDinar,
    "JPY" => TransitCurrency::JapaneseYen,
    "KES" => TransitCurrency::KenyanShilling,
    "KGS" => TransitCurrency::KyrgyzstaniSom,
    "KHR" => TransitCurrency::CambodianRiel,
    "KMF" => TransitCurrency::ComoroFranc,
    "KPW" => TransitCurrency::NorthKoreanWon,
    "KRW" => TransitCurrency::SouthKoreanWon,
    "KWD" => TransitCurrency::KuwaitiDinar,
    "KYD" => TransitCurrency::CaymanIslandsDollar,
    "KZT" => TransitCurrency::KazakhstaniTenge,
    "LAK" => TransitCurrency::LaoKip,
    "LBP" => TransitCurrency::LebanesePound,
    "LKR" => TransitCurrency::SriLankanRupee,
    "LRD" => TransitCurrency::LiberianDollar,
    "LSL" => TransitCurrency::LesothoLoti,
    "LYD" => TransitCurrency::LibyanDinar,
    "MAD" => TransitCurrency::MoroccanDirham,
    "MDL" => TransitCurrency::MoldovanLeu,
    "MGA" => TransitCurrency::MalagasyAriary,
    "MKD" => TransitCurrency::MacedonianDenar,
    "MMK" => TransitCurrency::MyanmarKyat,
    "MNT" => TransitCurrency::MongolianTogrog,
    "MOP" => TransitCurrency::MacanesePataca,
    "MRU" => TransitCurrency::MauritanianOuguiya,
    "MUR" => TransitCurrency::MauritianRupee,
    "MVR" => TransitCurrency::MaldivianRufiyaa,
    "MWK" => TransitCurrency::MalawianKwacha,
    "MXN" => TransitCurrency::MexicanPeso,
    "MXV" => TransitCurrency::MexicanUnidadDeInversion,
    "MYR" => TransitCurrency::MalaysianRinggit,
    "MZN" => TransitCurrency::MozambicanMetical,
    "NAD" => TransitCurrency::NamibianDollar,
    "NGN" => TransitCurrency::NigerianNaira,
    "NIO" => TransitCurrency::NicaraguanCordoba,
    "NOK" => TransitCurrency::NorwegianKrone,
    "NPR" => TransitCurrency::NepaleseRupee,
    "NZD" => TransitCurrency::NewZealandDollar,
    "OMR" => TransitCurrency::OmaniRial,
    "PAB" => TransitCurrency::PanamanianBalboa,
    "PEN" => TransitCurrency::PeruvianSol,
    "PGK" => TransitCurrency::PapuaNewGuineanKina,
    "PHP" => TransitCurrency::PhilippinePeso,
    "PKR" => TransitCurrency::PakistaniRupee,
    "PLN" => TransitCurrency::PolishZloty,
    "PYG" => TransitCurrency::ParaguayanGuarani,
    "QAR" => TransitCurrency::QatariRiyal,
    "RON" => TransitCurrency::RomanianLeu,
    "RSD" => TransitCurrency::SerbianDinar,
    "RUB" => TransitCurrency::RussianRuble,
    "RWF" => TransitCurrency::RwandanFranc,
    "SAR" => TransitCurrency::SaudiRiyal,
    "SBD" => TransitCurrency::SolomonIslandsDollar,
    "SCR" => TransitCurrency::SeychellesRupee,
    "SDG" => TransitCurrency::SudanesePound,
    "SEK" => TransitCurrency::SwedishKrona,
    "SGD" => TransitCurrency::SingaporeDollar,
    "SHP" => TransitCurrency::SaintHelenaPound,
    "SLL" => TransitCurrency::SierraLeoneanLeone,
    "SOS" => TransitCurrency::SomaliShilling,
    "SRD" => TransitCurrency::SurinameseDollar,
    "SSP" => TransitCurrency::SouthSudanesePound,
    "STN" => TransitCurrency::SaoTomeAndPrincipeDobra,
    "SVC" => TransitCurrency::SalvadoranColon,
    "SYP" => TransitCurrency::SyrianPound,
    "SZL" => TransitCurrency::SwaziLilangeni,
    "THB" => TransitCurrency::ThaiBaht,
    "TJS" => TransitCurrency::TajikistaniSomoni,
    "TMT" => TransitCurrency::TurkmenistanManat,
    "TND" => TransitCurrency::TunisianDinar,
    "TOP" => TransitCurrency::TonganPaanga,
    "TRY" => TransitCurrency::TurkishLira,
    "TTD" => TransitCurrency::TrinidadAndTobagoDollar,
    "TWD" => TransitCurrency::NewTaiwanDollar,
    "TZS" => TransitCurrency::TanzanianShilling,
    "UAH" => TransitCurrency::UkrainianHryvnia,
    "UGX" => TransitCurrency::UgandanShilling,
    "USD" => TransitCurrency::UnitedStatesDollar,
    "USN" => TransitCurrency::UnitedStatesDollarNextDay,
    "UYI" => TransitCurrency::UruguayPesoEnUnidadesIndexadas,
    "UYU" => TransitCurrency::UruguayanPeso,
    "UYW" => TransitCurrency::UnidadPrevisional,
    "UZS" => TransitCurrency::UzbekistanSom,
    "VES" => TransitCurrency::VenezuelanBolivarSoberano,
    "VND" => TransitCurrency::VietnameseDong,
    "VUV" => TransitCurrency::VanuatuVatu,
    "WST" => TransitCurrency::SamoanTala,
    "XAF" => TransitCurrency::CfaFrancBeac,
    "XAG" => TransitCurrency::Silver,
    "XAU" => TransitCurrency::Gold,
    "XBA" => TransitCurrency::EuropeanCompositeUnit,
    "XBB" => TransitCurrency::EuropeanMonetaryUnit,
    "XBC" => TransitCurrency::EuropeanUnitOfAccount9,
    "XBD" => TransitCurrency::EuropeanUnitOfAccount17,
    "XCD" => TransitCurrency::EastCaribbeanDollar,
    "XDR" => TransitCurrency::SpecialDrawingRights,
    "XOF" => TransitCurrency::CfaFrancBceao,
    "XPD" => TransitCurrency::Palladium,
    "XPF" => TransitCurrency::CfpFranc,
    "XPT" => TransitCurrency::Platinum,
    "XSU" => TransitCurrency::Sucre,
    "XTS" => TransitCurrency::CodeReservedForTesting,
    "XUA" => TransitCurrency::AdbUnitOfAccount,
    "XXX" => TransitCurrency::NoCurrency,
    "YER" => TransitCurrency::YemeniRial,
    "ZAR" => TransitCurrency::SouthAfricanRand,
    "ZMW" => TransitCurrency::ZambianKwacha,
    "ZWL" => TransitCurrency::ZimbabweanDollar,
};

impl std::convert::TryFrom<&str> for TransitCurrency {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;
    /// Gets a `TransitCurrency` enum from a `String` that contains a supported
    /// [ISO 4217 currency code](https://en.wikipedia.org/wiki/ISO_4217).
    fn try_from(currency_code: &str) -> Result<TransitCurrency, Error> {
        TRANSIT_CURRENCIES_BY_CODE
            .get(currency_code)
            .cloned()
            .ok_or_else(|| Error::InvalidCurrencyCode(currency_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for TransitCurrency {
    /// Returns a reasonable default variant for the `TransitCurrency` enum
    /// type:
    fn default() -> Self {
        TransitCurrency::UnitedStatesDollar
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for TransitCurrency {
    /// Formats a `TransitCurrency` enum into a string that is presentable to
    /// the end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TransitCurrency::UnitedArabEmiratesDirham => write!(f, "United Arab Emirates dirham"),
            TransitCurrency::AfghanAfghani => write!(f, "Afghan afghani"),
            TransitCurrency::AlbanianLek => write!(f, "Albanian lek"),
            TransitCurrency::ArmenianDram => write!(f, "Armenian dram"),
            TransitCurrency::NetherlandsAntilleanGuilder => write!(f, "Netherlands Antillean guilder"),
            TransitCurrency::AngolanKwanza => write!(f, "Angolan kwanza"),
            TransitCurrency::ArgentinePeso => write!(f, "Argentine peso"),
            TransitCurrency::AustralianDollar => write!(f, "Australian dollar"),
            TransitCurrency::ArubanFlorin => write!(f, "Aruban florin"),
            TransitCurrency::AzerbaijaniManat => write!(f, "Azerbaijani manat"),
            TransitCurrency::BosniaAndHerzegovinaConvertibleMark => write!(f, "Bosnia and Herzegovina convertible mark"),
            TransitCurrency::BarbadosDollar => write!(f, "Barbados dollar"),
            TransitCurrency::BangladeshiTaka => write!(f, "Bangladeshi taka"),
            TransitCurrency::BulgarianLev => write!(f, "Bulgarian lev"),
            TransitCurrency::BahrainiDinar => write!(f, "Bahraini dinar"),
            TransitCurrency::BurundianFranc => write!(f, "Burundian franc"),
            TransitCurrency::BermudianDollar => write!(f, "Bermudian dollar"),
            TransitCurrency::BruneiDollar => write!(f, "Brunei dollar"),
            TransitCurrency::Boliviano => write!(f, "Boliviano"),
            TransitCurrency::BolivianMvdol => write!(f, "Bolivian Mvdol"),
            TransitCurrency::BrazilianReal => write!(f, "Brazilian real"),
            TransitCurrency::BahamianDollar => write!(f, "Bahamian dollar"),
            TransitCurrency::BhutaneseNgultrum => write!(f, "Bhutanese ngultrum"),
            TransitCurrency::BotswanaPula => write!(f, "Botswana pula"),
            TransitCurrency::BelarusianRuble => write!(f, "Belarusian ruble"),
            TransitCurrency::BelizeDollar => write!(f, "Belize dollar"),
            TransitCurrency::CanadianDollar => write!(f, "Canadian dollar"),
            TransitCurrency::CongoleseFranc => write!(f, "Congolese franc"),
            TransitCurrency::WirEuro => write!(f, "WIR Euro"),
            TransitCurrency::SwissFranc => write!(f, "Swiss franc"),
            TransitCurrency::WirFranc => write!(f, "WIR Franc"),
            TransitCurrency::UnidadDeFomento => write!(f, "Unidad de Fomento"),
            TransitCurrency::ChileanPeso => write!(f, "Chilean peso"),
            TransitCurrency::RenminbiYuan => write!(f, "Renminbi yuan"),
            TransitCurrency::ColombianPeso => write!(f, "Colombian peso"),
            TransitCurrency::UnidadDeValorReal => write!(f, "Unidad de Valor Real"),
            TransitCurrency::CostaRicanColon => write!(f, "Costa Rican colon"),
            TransitCurrency::CubanConvertiblePeso => write!(f, "Cuban convertible peso"),
            TransitCurrency::CubanPeso => write!(f, "Cuban peso"),
            TransitCurrency::CapeVerdeanEscudo => write!(f, "Cape Verdean escudo"),
            TransitCurrency::CzechKoruna => write!(f, "Czech koruna"),
            TransitCurrency::DjiboutianFranc => write!(f, "Djiboutian franc"),
            TransitCurrency::DanishKrone => write!(f, "Danish krone"),
            TransitCurrency::DominicanPeso => write!(f, "Dominican peso"),
            TransitCurrency::AlgerianDinar => write!(f, "Algerian dinar"),
            TransitCurrency::EgyptianPound => write!(f, "Egyptian pound"),
            TransitCurrency::EritreanNakfa => write!(f, "Eritrean nakfa"),
            TransitCurrency::EthiopianBirr => write!(f, "Ethiopian birr"),
            TransitCurrency::Euro => write!(f, "Euro"),
            TransitCurrency::FijiDollar => write!(f, "Fiji dollar"),
            TransitCurrency::FalklandIslandsPound => write!(f, "Falkland Islands pound"),
            TransitCurrency::PoundSterling => write!(f, "Pound sterling"),
            TransitCurrency::GeorgianLari => write!(f, "Georgian lari"),
            TransitCurrency::GhanaianCedi => write!(f, "Ghanaian cedi"),
            TransitCurrency::GibraltarPound => write!(f, "Gibraltar pound"),
            TransitCurrency::GambianDalasi => write!(f, "Gambian dalasi"),
            TransitCurrency::GuineanFranc => write!(f, "Guinean franc"),
            TransitCurrency::GuatemalanQuetzal => write!(f, "Guatemalan quetzal"),
            TransitCurrency::GuyaneseDollar => write!(f, "Guyanese dollar"),
            TransitCurrency::HongKongDollar => write!(f, "Hong Kong dollar"),
            TransitCurrency::HonduranLempira => write!(f, "Honduran lempira"),
            TransitCurrency::CroatianKuna => write!(f, "Croatian kuna"),
            TransitCurrency::HaitianGourde => write!(f, "Haitian gourde"),
            TransitCurrency::HungarianForint => write!(f, "Hungarian forint"),
            TransitCurrency::IndonesianRupiah => write!(f, "Indonesian rupiah"),
            TransitCurrency::IsraeliNewShekel => write!(f, "Israeli new shekel"),
            TransitCurrency::IndianRupee => write!(f, "Indian rupee"),
            TransitCurrency::IraqiDinar => write!(f, "Iraqi dinar"),
            TransitCurrency::IranianRial => write!(f, "Iranian rial"),
            TransitCurrency::IcelandicKrona => write!(f, "Icelandic króna"),
            TransitCurrency::JamaicanDollar => write!(f, "Jamaican dollar"),
            TransitCurrency::JordanianDinar => write!(f, "Jordanian dinar"),
            TransitCurrency::JapaneseYen => write!(f, "Japanese yen"),
            TransitCurrency::KenyanShilling => write!(f, "Kenyan shilling"),
            TransitCurrency::KyrgyzstaniSom => write!(f, "Kyrgyzstani som"),
            TransitCurrency::CambodianRiel => write!(f, "Cambodian riel"),
            TransitCurrency::ComoroFranc => write!(f, "Comoro franc"),
            TransitCurrency::NorthKoreanWon => write!(f, "North Korean won"),
            TransitCurrency::SouthKoreanWon => write!(f, "South Korean won"),
            TransitCurrency::KuwaitiDinar => write!(f, "Kuwaiti dinar"),
            TransitCurrency::CaymanIslandsDollar => write!(f, "Cayman Islands dollar"),
            TransitCurrency::KazakhstaniTenge => write!(f, "Kazakhstani tenge"),
            TransitCurrency::LaoKip => write!(f, "Lao kip"),
            TransitCurrency::LebanesePound => write!(f, "Lebanese pound"),
            TransitCurrency::SriLankanRupee => write!(f, "Sri Lankan rupee"),
            TransitCurrency::LiberianDollar => write!(f, "Liberian dollar"),
            TransitCurrency::LesothoLoti => write!(f, "Lesotho loti"),
            TransitCurrency::LibyanDinar => write!(f, "Libyan dinar"),
            TransitCurrency::MoroccanDirham => write!(f, "Moroccan dirham"),
            TransitCurrency::MoldovanLeu => write!(f, "Moldovan leu"),
            TransitCurrency::MalagasyAriary => write!(f, "Malagasy ariary"),
            TransitCurrency::MacedonianDenar => write!(f, "Macedonian denar"),
            TransitCurrency::MyanmarKyat => write!(f, "Myanmar kyat"),
            TransitCurrency::MongolianTogrog => write!(f, "Mongolian tögrög"),
            TransitCurrency::MacanesePataca => write!(f, "Macanese pataca"),
            TransitCurrency::MauritanianOuguiya => write!(f, "Mauritanian ouguiya"),
            TransitCurrency::MauritianRupee => write!(f, "Mauritian rupee"),
            TransitCurrency::MaldivianRufiyaa => write!(f, "Maldivian rufiyaa"),
            TransitCurrency::MalawianKwacha => write!(f, "Malawian kwacha"),
            TransitCurrency::MexicanPeso => write!(f, "Mexican peso"),
            TransitCurrency::MexicanUnidadDeInversion => write!(f, "Mexican Unidad de Inversion"),
            TransitCurrency::MalaysianRinggit => write!(f, "Malaysian ringgit"),
            TransitCurrency::MozambicanMetical => write!(f, "Mozambican metical"),
            TransitCurrency::NamibianDollar => write!(f, "Namibian dollar"),
            TransitCurrency::NigerianNaira => write!(f, "Nigerian naira"),
            TransitCurrency::NicaraguanCordoba => write!(f, "Nicaraguan córdoba"),
            TransitCurrency::NorwegianKrone => write!(f, "Norwegian krone"),
            TransitCurrency::NepaleseRupee => write!(f, "Nepalese rupee"),
            TransitCurrency::NewZealandDollar => write!(f, "New Zealand dollar"),
            TransitCurrency::OmaniRial => write!(f, "Omani rial"),
            TransitCurrency::PanamanianBalboa => write!(f, "Panamanian balboa"),
            TransitCurrency::PeruvianSol => write!(f, "Peruvian sol"),
            TransitCurrency::PapuaNewGuineanKina => write!(f, "Papua New Guinean kina"),
            TransitCurrency::PhilippinePeso => write!(f, "Philippine peso"),
            TransitCurrency::PakistaniRupee => write!(f, "Pakistani rupee"),
            TransitCurrency::PolishZloty => write!(f, "Polish złoty"),
            TransitCurrency::ParaguayanGuarani => write!(f, "Paraguayan guaraní"),
            TransitCurrency::QatariRiyal => write!(f, "Qatari riyal"),
            TransitCurrency::RomanianLeu => write!(f, "Romanian leu"),
            TransitCurrency::SerbianDinar => write!(f, "Serbian dinar"),
            TransitCurrency::RussianRuble => write!(f, "Russian ruble"),
            TransitCurrency::RwandanFranc => write!(f, "Rwandan franc"),
            TransitCurrency::SaudiRiyal => write!(f, "Saudi riyal"),
            TransitCurrency::SolomonIslandsDollar => write!(f, "Solomon Islands dollar"),
            TransitCurrency::SeychellesRupee => write!(f, "Seychelles rupee"),
            TransitCurrency::SudanesePound => write!(f, "Sudanese pound"),
            TransitCurrency::SwedishKrona => write!(f, "Swedish krona"),
            TransitCurrency::SingaporeDollar => write!(f, "Singapore dollar"),
            TransitCurrency::SaintHelenaPound => write!(f, "Saint Helena pound"),
            TransitCurrency::SierraLeoneanLeone => write!(f, "Sierra Leonean leone"),
            TransitCurrency::SomaliShilling => write!(f, "Somali shilling"),
            TransitCurrency::SurinameseDollar => write!(f, "Surinamese dollar"),
            TransitCurrency::SouthSudanesePound => write!(f, "South Sudanese pound"),
            TransitCurrency::SaoTomeAndPrincipeDobra => write!(f, "São Tomé and Príncipe dobra"),
            TransitCurrency::SalvadoranColon => write!(f, "Salvadoran colón"),
            TransitCurrency::SyrianPound => write!(f, "Syrian pound"),
            TransitCurrency::SwaziLilangeni => write!(f, "Swazi lilangeni"),
            TransitCurrency::ThaiBaht => write!(f, "Thai baht"),
            TransitCurrency::TajikistaniSomoni => write!(f, "Tajikistani somoni"),
            TransitCurrency::TurkmenistanManat => write!(f, "Turkmenistan manat"),
            TransitCurrency::TunisianDinar => write!(f, "Tunisian dinar"),
            TransitCurrency::TonganPaanga => write!(f, "Tongan paʻanga"),
            TransitCurrency::TurkishLira => write!(f, "Turkish lira"),
            TransitCurrency::TrinidadAndTobagoDollar => write!(f, "Trinidad and Tobago dollar"),
            TransitCurrency::NewTaiwanDollar => write!(f, "New Taiwan dollar"),
            TransitCurrency::TanzanianShilling => write!(f, "Tanzanian shilling"),
            TransitCurrency::UkrainianHryvnia => write!(f, "Ukrainian hryvnia"),
            TransitCurrency::UgandanShilling => write!(f, "Ugandan shilling"),
            TransitCurrency::UnitedStatesDollar => write!(f, "United States dollar"),
            TransitCurrency::UnitedStatesDollarNextDay => write!(f, "United States dollar next day"),
            TransitCurrency::UruguayPesoEnUnidadesIndexadas => write!(f, "Uruguay Peso en Unidades Indexadas"),
            TransitCurrency::UruguayanPeso => write!(f, "Uruguayan peso"),
            TransitCurrency::UnidadPrevisional => write!(f, "Unidad previsional"),
            TransitCurrency::UzbekistanSom => write!(f, "Uzbekistan som"),
            TransitCurrency::VenezuelanBolivarSoberano => write!(f, "Venezuelan bolívar soberano"),
            TransitCurrency::VietnameseDong => write!(f, "Vietnamese đồng"),
            TransitCurrency::VanuatuVatu => write!(f, "Vanuatu vatu"),
            TransitCurrency::SamoanTala => write!(f, "Samoan tala"),
            TransitCurrency::CfaFrancBeac => write!(f, "CFA franc BEAC"),
            TransitCurrency::Silver => write!(f, "Silver"),
            TransitCurrency::Gold => write!(f, "Gold"),
            TransitCurrency::EuropeanCompositeUnit => write!(f, "European Composite Unit"),
            TransitCurrency::EuropeanMonetaryUnit => write!(f, "European Monetary Unit"),
            TransitCurrency::EuropeanUnitOfAccount9 => write!(f, "European Unit of Account 9"),
            TransitCurrency::EuropeanUnitOfAccount17 => write!(f, "European Unit of Account 17"),
            TransitCurrency::EastCaribbeanDollar => write!(f, "East Caribbean dollar"),
            TransitCurrency::SpecialDrawingRights => write!(f, "Special drawing rights"),
            TransitCurrency::CfaFrancBceao => write!(f, "CFA franc BCEAO"),
            TransitCurrency::Palladium => write!(f, "Palladium"),
            TransitCurrency::CfpFranc => write!(f, "CFP franc"),
            TransitCurrency::Platinum => write!(f, "Platinum"),
            TransitCurrency::Sucre => write!(f, "SUCRE"),
            TransitCurrency::CodeReservedForTesting => write!(f, "Code reserved for testing"),
            TransitCurrency::AdbUnitOfAccount => write!(f, "ADB Unit of Account"),
            TransitCurrency::NoCurrency => write!(f, "No currency"),
            TransitCurrency::YemeniRial => write!(f, "Yemeni rial"),
            TransitCurrency::SouthAfricanRand => write!(f, "South African rand"),
            TransitCurrency::ZambianKwacha => write!(f, "Zambian kwacha"),
            TransitCurrency::ZimbabweanDollar => write!(f, "Zimbabwean dollar"),
        } // match
    } // fn
} // impl