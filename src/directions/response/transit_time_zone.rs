//! Contains the `TransitTimeZone` enum and its associated traits. It is used to
//! work with the times returned by transit directions from the Google Maps
//! Directions API.

use crate::directions::error::Error;
use serde::{Serialize, Deserialize};

/// A comprehensive list of time zones. At the moment this is used only for
/// Google Maps Transit Directions. The intent behind having _Serde_ convert
/// the time zone `String` to an `enum` is for efficient time zone conversions,
/// information lookups, and manipulation in the future.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum TransitTimeZone {
    #[serde(alias = "Africa/Abidjan")]
    AfricaAbidjan,
    #[serde(alias = "Africa/Accra")]
    AfricaAccra,
    #[serde(alias = "Africa/Algiers")]
    AfricaAlgiers,
    #[serde(alias = "Africa/Bissau")]
    AfricaBissau,
    #[serde(alias = "Africa/Cairo")]
    AfricaCairo,
    #[serde(alias = "Africa/Casablanca")]
    AfricaCasablanca,
    #[serde(alias = "Africa/Ceuta")]
    AfricaCeuta,
    #[serde(alias = "Africa/El_Aaiun")]
    AfricaElAaiun,
    #[serde(alias = "Africa/Johannesburg")]
    AfricaJohannesburg,
    #[serde(alias = "Africa/Juba")]
    AfricaJuba,
    #[serde(alias = "Africa/Khartoum")]
    AfricaKhartoum,
    #[serde(alias = "Africa/Lagos")]
    AfricaLagos,
    #[serde(alias = "Africa/Maputo")]
    AfricaMaputo,
    #[serde(alias = "Africa/Monrovia")]
    AfricaMonrovia,
    #[serde(alias = "Africa/Nairobi")]
    AfricaNairobi,
    #[serde(alias = "Africa/Ndjamena")]
    AfricaNdjamena,
    #[serde(alias = "Africa/Sao_Tome")]
    AfricaSaoTome,
    #[serde(alias = "Africa/Tripoli")]
    AfricaTripoli,
    #[serde(alias = "Africa/Tunis")]
    AfricaTunis,
    #[serde(alias = "Africa/Windhoek")]
    AfricaWindhoek,
    #[serde(alias = "America/Adak")]
    AmericaAdak,
    #[serde(alias = "America/Anchorage")]
    AmericaAnchorage,
    #[serde(alias = "America/Araguaina")]
    AmericaAraguaina,
    #[serde(alias = "America/Argentina/Buenos_Aires")]
    AmericaArgentinaBuenosAires,
    #[serde(alias = "America/Argentina/Catamarca")]
    AmericaArgentinaCatamarca,
    #[serde(alias = "America/Argentina/Cordoba")]
    AmericaArgentinaCordoba,
    #[serde(alias = "America/Argentina/Jujuy")]
    AmericaArgentinaJujuy,
    #[serde(alias = "America/Argentina/La_Rioja")]
    AmericaArgentinaLaRioja,
    #[serde(alias = "America/Argentina/Mendoza")]
    AmericaArgentinaMendoza,
    #[serde(alias = "America/Argentina/Rio_Gallegos")]
    AmericaArgentinaRioGallegos,
    #[serde(alias = "AmAmerica/Argentina/Salta")]
    AmericaArgentinaSalta,
    #[serde(alias = "America/Argentina/San_Juan")]
    AmericaArgentinaSanJuan,
    #[serde(alias = "America/Argentina/San_Luis")]
    AmericaArgentinaSanLuis,
    #[serde(alias = "America/Argentina/Tucuman")]
    AmericaArgentinaTucuman,
    #[serde(alias = "America/Argentina/Ushuaia")]
    AmericaArgentinaUshuaia,
    #[serde(alias = "America/Asuncion")]
    AmericaAsuncion,
    #[serde(alias = "America/Atikokan")]
    AmericaAtikokan,
    #[serde(alias = "America/Bahia")]
    AmericaBahia,
    #[serde(alias = "America/Bahia_Banderas")]
    AmericaBahiaBanderas,
    #[serde(alias = "America/Barbados")]
    AmericaBarbados,
    #[serde(alias = "America/Belem")]
    AmericaBelem,
    #[serde(alias = "America/Belize")]
    AmericaBelize,
    #[serde(alias = "America/Blanc-Sablon")]
    AmericaBlancSablon,
    #[serde(alias = "America/Boa_Vista")]
    AmericaBoaVista,
    #[serde(alias = "America/Bogota")]
    AmericaBogota,
    #[serde(alias = "America/Boise")]
    AmericaBoise,
    #[serde(alias = "America/Cambridge_Bay")]
    AmericaCambridgeBay,
    #[serde(alias = "America/Campo_Grande")]
    AmericaCampoGrande,
    #[serde(alias = "America/Cancun")]
    AmericaCancun,
    #[serde(alias = "America/Caracas")]
    AmericaCaracas,
    #[serde(alias = "America/Cayenne")]
    AmericaCayenne,
    #[serde(alias = "America/Chicago")]
    AmericaChicago,
    #[serde(alias = "America/Chihuahua")]
    AmericaChihuahua,
    #[serde(alias = "America/Costa_Rica")]
    AmericaCostaRica,
    #[serde(alias = "America/Creston")]
    AmericaCreston,
    #[serde(alias = "America/Cuiaba")]
    AmericaCuiaba,
    #[serde(alias = "America/Curacao")]
    AmericaCuracao,
    #[serde(alias = "America/Danmarkshavn")]
    AmericaDanmarkshavn,
    #[serde(alias = "America/Dawson")]
    AmericaDawson,
    #[serde(alias = "America/Dawson_Creek")]
    AmericaDawsonCreek,
    #[serde(alias = "America/Denver")]
    AmericaDenver,
    #[serde(alias = "America/Detroit")]
    AmericaDetroit,
    #[serde(alias = "America/Edmonton")]
    AmericaEdmonton,
    #[serde(alias = "America/Eirunepe")]
    AmericaEirunepe,
    #[serde(alias = "America/El_Salvador")]
    AmericaElSalvador,
    #[serde(alias = "America/Fort_Nelson")]
    AmericaFortNelson,
    #[serde(alias = "America/Fortaleza")]
    AmericaFortaleza,
    #[serde(alias = "America/Glace_Bay")]
    AmericaGlaceBay,
    #[serde(alias = "America/Godthab")]
    AmericaGodthab,
    #[serde(alias = "America/Goose_Bay")]
    AmericaGooseBay,
    #[serde(alias = "America/Grand_Turk")]
    AmericaGrandTurk,
    #[serde(alias = "America/Guatemala")]
    AmericaGuatemala,
    #[serde(alias = "America/Guayaquil")]
    AmericaGuayaquil,
    #[serde(alias = "America/Guyana")]
    AmericaGuyana,
    #[serde(alias = "America/Halifax")]
    AmericaHalifax,
    #[serde(alias = "America/Havana")]
    AmericaHavana,
    #[serde(alias = "America/Hermosillo")]
    AmericaHermosillo,
    #[serde(alias = "America/Indiana/Indianapolis")]
    AmericaIndianaIndianapolis,
    #[serde(alias = "America/Indiana/Knox")]
    AmericaIndianaKnox,
    #[serde(alias = "America/Indiana/Marengo")]
    AmericaIndianaMarengo,
    #[serde(alias = "America/Indiana/Petersburg")]
    AmericaIndianaPetersburg,
    #[serde(alias = "America/Indiana/Tell_City")]
    AmericaIndianaTellCity,
    #[serde(alias = "America/Indiana/Vevay")]
    AmericaIndianaVevay,
    #[serde(alias = "America/Indiana/Vincennes")]
    AmericaIndianaVincennes,
    #[serde(alias = "America/Indiana/Winamac")]
    AmericaIndianaWinamac,
    #[serde(alias = "America/Inuvik")]
    AmericaInuvik,
    #[serde(alias = "America/Iqaluit")]
    AmericaIqaluit,
    #[serde(alias = "America/Jamaica")]
    AmericaJamaica,
    #[serde(alias = "America/Juneau")]
    AmericaJuneau,
    #[serde(alias = "America/Kentucky/Louisville")]
    AmericaKentuckyLouisville,
    #[serde(alias = "America/Kentucky/Monticello")]
    AmericaKentuckyMonticello,
    #[serde(alias = "America/La_Paz")]
    AmericaLaPaz,
    #[serde(alias = "America/Lima")]
    AmericaLima,
    #[serde(alias = "America/Los_Angeles")]
    AmericaLosAngeles,
    #[serde(alias = "America/Maceio")]
    AmericaMaceio,
    #[serde(alias = "America/Managua")]
    AmericaManagua,
    #[serde(alias = "America/Manaus")]
    AmericaManaus,
    #[serde(alias = "America/Martinique")]
    AmericaMartinique,
    #[serde(alias = "America/Matamoros")]
    AmericaMatamoros,
    #[serde(alias = "America/Mazatlan")]
    AmericaMazatlan,
    #[serde(alias = "America/Menominee")]
    AmericaMenominee,
    #[serde(alias = "America/Merida")]
    AmericaMerida,
    #[serde(alias = "America/Metlakatla")]
    AmericaMetlakatla,
    #[serde(alias = "America/Mexico_City")]
    AmericaMexicoCity,
    #[serde(alias = "America/Miquelon")]
    AmericaMiquelon,
    #[serde(alias = "America/Moncton")]
    AmericaMoncton,
    #[serde(alias = "America/Monterrey")]
    AmericaMonterrey,
    #[serde(alias = "America/Montevideo")]
    AmericaMontevideo,
    #[serde(alias = "America/Nassau")]
    AmericaNassau,
    #[serde(alias = "America/New_York")]
    AmericaNewYork,
    #[serde(alias = "America/Nipigon")]
    AmericaNipigon,
    #[serde(alias = "America/Nome")]
    AmericaNome,
    #[serde(alias = "America/Noronha")]
    AmericaNoronha,
    #[serde(alias = "America/North_Dakota/Beulah")]
    AmericaNorthDakotaBeulah,
    #[serde(alias = "America/North_Dakota/Center")]
    AmericaNorthDakotaCenter,
    #[serde(alias = "America/North_Dakota/New_Salem")]
    AmericaNorthDakotaNewSalem,
    #[serde(alias = "America/Ojinaga")]
    AmericaOjinaga,
    #[serde(alias = "America/Panama")]
    AmericaPanama,
    #[serde(alias = "America/Pangnirtung")]
    AmericaPangnirtung,
    #[serde(alias = "America/Paramaribo")]
    AmericaParamaribo,
    #[serde(alias = "America/Phoenix")]
    AmericaPhoenix,
    #[serde(alias = "America/Port-au-Prince")]
    AmericaPortAuPrince,
    #[serde(alias = "America/Port_of_Spain")]
    AmericaPortOfSpain,
    #[serde(alias = "America/Porto_Velho")]
    AmericaPortoVelho,
    #[serde(alias = "America/Puerto_Rico")]
    AmericaPuertoRico,
    #[serde(alias = "America/Punta_Arenas")]
    AmericaPuntaArenas,
    #[serde(alias = "America/Rainy_River")]
    AmericaRainyRiver,
    #[serde(alias = "America/Rankin_Inlet")]
    AmericaRankinInlet,
    #[serde(alias = "America/Recife")]
    AmericaRecife,
    #[serde(alias = "America/Regina")]
    AmericaRegina,
    #[serde(alias = "America/Resolute")]
    AmericaResolute,
    #[serde(alias = "America/Rio_Branco")]
    AmericaRioBranco,
    #[serde(alias = "America/Santarem")]
    AmericaSantarem,
    #[serde(alias = "America/Santiago")]
    AmericaSantiago,
    #[serde(alias = "America/Santo_Domingo")]
    AmericaSantoDomingo,
    #[serde(alias = "America/Sao_Paulo")]
    AmericaSaoPaulo,
    #[serde(alias = "America/Scoresbysund")]
    AmericaScoresbysund,
    #[serde(alias = "America/Sitka")]
    AmericaSitka,
    #[serde(alias = "America/St_Johns")]
    AmericaStJohns,
    #[serde(alias = "America/Swift_Current")]
    AmericaSwiftCurrent,
    #[serde(alias = "America/Tegucigalpa")]
    AmericaTegucigalpa,
    #[serde(alias = "America/Thule")]
    AmericaThule,
    #[serde(alias = "America/Thunder_Bay")]
    AmericaThunderBay,
    #[serde(alias = "America/Tijuana")]
    AmericaTijuana,
    #[serde(alias = "America/Toronto")]
    AmericaToronto,
    #[serde(alias = "America/Vancouver")]
    AmericaVancouver,
    #[serde(alias = "America/Whitehorse")]
    AmericaWhitehorse,
    #[serde(alias = "America/Winnipeg")]
    AmericaWinnipeg,
    #[serde(alias = "America/Yakutat")]
    AmericaYakutat,
    #[serde(alias = "America/Yellowknife")]
    AmericaYellowknife,
    #[serde(alias = "Antarctica/Casey")]
    AntarcticaCasey,
    #[serde(alias = "Antarctica/Davis")]
    AntarcticaDavis,
    #[serde(alias = "Antarctica/DumontDUrville")]
    AntarcticaDumontDUrville,
    #[serde(alias = "Antarctica/Macquarie")]
    AntarcticaMacquarie,
    #[serde(alias = "Antarctica/Mawson")]
    AntarcticaMawson,
    #[serde(alias = "Antarctica/Palmer")]
    AntarcticaPalmer,
    #[serde(alias = "Antarctica/Rothera")]
    AntarcticaRothera,
    #[serde(alias = "Antarctica/Syowa")]
    AntarcticaSyowa,
    #[serde(alias = "Antarctica/Troll")]
    AntarcticaTroll,
    #[serde(alias = "Antarctica/Vostok")]
    AntarcticaVostok,
    #[serde(alias = "Asia/Almaty")]
    AsiaAlmaty,
    #[serde(alias = "Asia/Amman")]
    AsiaAmman,
    #[serde(alias = "Asia/Anadyr")]
    AsiaAnadyr,
    #[serde(alias = "Asia/Aqtau")]
    AsiaAqtau,
    #[serde(alias = "Asia/Aqtobe")]
    AsiaAqtobe,
    #[serde(alias = "Asia/Ashgabat")]
    AsiaAshgabat,
    #[serde(alias = "Asia/Atyrau")]
    AsiaAtyrau,
    #[serde(alias = "Asia/Baghdad")]
    AsiaBaghdad,
    #[serde(alias = "Asia/Baku")]
    AsiaBaku,
    #[serde(alias = "Asia/Bangkok")]
    AsiaBangkok,
    #[serde(alias = "Asia/Barnaul")]
    AsiaBarnaul,
    #[serde(alias = "Asia/Beirut")]
    AsiaBeirut,
    #[serde(alias = "Asia/Bishkek")]
    AsiaBishkek,
    #[serde(alias = "Asia/Brunei")]
    AsiaBrunei,
    #[serde(alias = "Asia/Chita")]
    AsiaChita,
    #[serde(alias = "Asia/Choibalsan")]
    AsiaChoibalsan,
    #[serde(alias = "Asia/Colombo")]
    AsiaColombo,
    #[serde(alias = "Asia/Damascus")]
    AsiaDamascus,
    #[serde(alias = "Asia/Dhaka")]
    AsiaDhaka,
    #[serde(alias = "Asia/Dili")]
    AsiaDili,
    #[serde(alias = "Asia/Dubai")]
    AsiaDubai,
    #[serde(alias = "Asia/Dushanbe")]
    AsiaDushanbe,
    #[serde(alias = "Asia/Famagusta")]
    AsiaFamagusta,
    #[serde(alias = "Asia/Gaza")]
    AsiaGaza,
    #[serde(alias = "Asia/Hebron")]
    AsiaHebron,
    #[serde(alias = "Asia/Ho_Chi_Minh")]
    AsiaHoChiMinh,
    #[serde(alias = "Asia/Hong_Kong")]
    AsiaHongKong,
    #[serde(alias = "Asia/Hovd")]
    AsiaHovd,
    #[serde(alias = "Asia/Irkutsk")]
    AsiaIrkutsk,
    #[serde(alias = "Asia/Jakarta")]
    AsiaJakarta,
    #[serde(alias = "Asia/Jayapura")]
    AsiaJayapura,
    #[serde(alias = "Asia/Jerusalem")]
    AsiaJerusalem,
    #[serde(alias = "Asia/Kabul")]
    AsiaKabul,
    #[serde(alias = "Asia/Kamchatka")]
    AsiaKamchatka,
    #[serde(alias = "Asia/Karachi")]
    AsiaKarachi,
    #[serde(alias = "Asia/Kathmandu")]
    AsiaKathmandu,
    #[serde(alias = "Asia/Khandyga")]
    AsiaKhandyga,
    #[serde(alias = "Asia/Kolkata")]
    AsiaKolkata,
    #[serde(alias = "Asia/Krasnoyarsk")]
    AsiaKrasnoyarsk,
    #[serde(alias = "Asia/Kuala_Lumpur")]
    AsiaKualaLumpur,
    #[serde(alias = "Asia/Kuching")]
    AsiaKuching,
    #[serde(alias = "Asia/Macau")]
    AsiaMacau,
    #[serde(alias = "Asia/Magadan")]
    AsiaMagadan,
    #[serde(alias = "Asia/Makassar")]
    AsiaMakassar,
    #[serde(alias = "Asia/Manila")]
    AsiaManila,
    #[serde(alias = "Asia/Nicosia")]
    AsiaNicosia,
    #[serde(alias = "Asia/Novokuznetsk")]
    AsiaNovokuznetsk,
    #[serde(alias = "Asia/Novosibirsk")]
    AsiaNovosibirsk,
    #[serde(alias = "Asia/Omsk")]
    AsiaOmsk,
    #[serde(alias = "Asia/Oral")]
    AsiaOral,
    #[serde(alias = "Asia/Pontianak")]
    AsiaPontianak,
    #[serde(alias = "Asia/Pyongyang")]
    AsiaPyongyang,
    #[serde(alias = "Asia/Qatar")]
    AsiaQatar,
    #[serde(alias = "Asia/Qostanay")]
    AsiaQostanay,
    #[serde(alias = "Asia/Qyzylorda")]
    AsiaQyzylorda,
    #[serde(alias = "Asia/Riyadh")]
    AsiaRiyadh,
    #[serde(alias = "Asia/Sakhalin")]
    AsiaSakhalin,
    #[serde(alias = "Asia/Samarkand")]
    AsiaSamarkand,
    #[serde(alias = "Asia/Seoul")]
    AsiaSeoul,
    #[serde(alias = "Asia/Shanghai")]
    AsiaShanghai,
    #[serde(alias = "Asia/Singapore")]
    AsiaSingapore,
    #[serde(alias = "Asia/Srednekolymsk")]
    AsiaSrednekolymsk,
    #[serde(alias = "Asia/Taipei")]
    AsiaTaipei,
    #[serde(alias = "Asia/Tashkent")]
    AsiaTashkent,
    #[serde(alias = "Asia/Tbilisi")]
    AsiaTbilisi,
    #[serde(alias = "Asia/Tehran")]
    AsiaTehran,
    #[serde(alias = "Asia/Thimphu")]
    AsiaThimphu,
    #[serde(alias = "Asia/Tokyo")]
    AsiaTokyo,
    #[serde(alias = "Asia/Tomsk")]
    AsiaTomsk,
    #[serde(alias = "Asia/Ulaanbaatar")]
    AsiaUlaanbaatar,
    #[serde(alias = "Asia/Urumqi")]
    AsiaUrumqi,
    #[serde(alias = "Asia/Ust-Nera")]
    AsiaUstNera,
    #[serde(alias = "Asia/Vladivostok")]
    AsiaVladivostok,
    #[serde(alias = "Asia/Yakutsk")]
    AsiaYakutsk,
    #[serde(alias = "Asia/Yangon")]
    AsiaYangon,
    #[serde(alias = "Asia/Yekaterinburg")]
    AsiaYekaterinburg,
    #[serde(alias = "Asia/Yerevan")]
    AsiaYerevan,
    #[serde(alias = "Atlantic/Azores")]
    AtlanticAzores,
    #[serde(alias = "Atlantic/Bermuda")]
    AtlanticBermuda,
    #[serde(alias = "Atlantic/Canary")]
    AtlanticCanary,
    #[serde(alias = "Atlantic/Cape_Verde")]
    AtlanticCapeVerde,
    #[serde(alias = "Atlantic/Faroe")]
    AtlanticFaroe,
    #[serde(alias = "Atlantic/Madeira")]
    AtlanticMadeira,
    #[serde(alias = "Atlantic/Reykjavik")]
    AtlanticReykjavik,
    #[serde(alias = "Atlantic/South_Georgia")]
    AtlanticSouthGeorgia,
    #[serde(alias = "Atlantic/Stanley")]
    AtlanticStanley,
    #[serde(alias = "Australia/Adelaide")]
    AustraliaAdelaide,
    #[serde(alias = "Australia/Brisbane")]
    AustraliaBrisbane,
    #[serde(alias = "Australia/Broken_Hill")]
    AustraliaBrokenHill,
    #[serde(alias = "Australia/Currie")]
    AustraliaCurrie,
    #[serde(alias = "Australia/Darwin")]
    AustraliaDarwin,
    #[serde(alias = "Australia/Eucla")]
    AustraliaEucla,
    #[serde(alias = "Australia/Hobart")]
    AustraliaHobart,
    #[serde(alias = "Australia/Lindeman")]
    AustraliaLindeman,
    #[serde(alias = "Australia/Lord_Howe")]
    AustraliaLordHowe,
    #[serde(alias = "Australia/Melbourne")]
    AustraliaMelbourne,
    #[serde(alias = "Australia/Perth")]
    AustraliaPerth,
    #[serde(alias = "Australia/Sydney")]
    AustraliaSydney,
    #[serde(alias = "Europe/Amsterdam")]
    EuropeAmsterdam,
    #[serde(alias = "Europe/Andorra")]
    EuropeAndorra,
    #[serde(alias = "Europe/Astrakhan")]
    EuropeAstrakhan,
    #[serde(alias = "Europe/Athens")]
    EuropeAthens,
    #[serde(alias = "Europe/Belgrade")]
    EuropeBelgrade,
    #[serde(alias = "Europe/Berlin")]
    EuropeBerlin,
    #[serde(alias = "Europe/Brussels")]
    EuropeBrussels,
    #[serde(alias = "Europe/Bucharest")]
    EuropeBucharest,
    #[serde(alias = "Europe/Budapest")]
    EuropeBudapest,
    #[serde(alias = "Europe/Chisinau")]
    EuropeChisinau,
    #[serde(alias = "Europe/Copenhagen")]
    EuropeCopenhagen,
    #[serde(alias = "Europe/Dublin")]
    EuropeDublin,
    #[serde(alias = "Europe/Gibraltar")]
    EuropeGibraltar,
    #[serde(alias = "Europe/Helsinki")]
    EuropeHelsinki,
    #[serde(alias = "Europe/Istanbul")]
    EuropeIstanbul,
    #[serde(alias = "Europe/Kaliningrad")]
    EuropeKaliningrad,
    #[serde(alias = "Europe/Kiev")]
    EuropeKiev,
    #[serde(alias = "Europe/Kirov")]
    EuropeKirov,
    #[serde(alias = "Europe/Lisbon")]
    EuropeLisbon,
    #[serde(alias = "Europe/London")]
    EuropeLondon,
    #[serde(alias = "Europe/Luxembourg")]
    EuropeLuxembourg,
    #[serde(alias = "Europe/Madrid")]
    EuropeMadrid,
    #[serde(alias = "Europe/Malta")]
    EuropeMalta,
    #[serde(alias = "Europe/Minsk")]
    EuropeMinsk,
    #[serde(alias = "Europe/Monaco")]
    EuropeMonaco,
    #[serde(alias = "Europe/Moscow")]
    EuropeMoscow,
    #[serde(alias = "Europe/Oslo")]
    EuropeOslo,
    #[serde(alias = "Europe/Paris")]
    EuropeParis,
    #[serde(alias = "Europe/Prague")]
    EuropePrague,
    #[serde(alias = "Europe/Riga")]
    EuropeRiga,
    #[serde(alias = "Europe/Rome")]
    EuropeRome,
    #[serde(alias = "Europe/Samara")]
    EuropeSamara,
    #[serde(alias = "Europe/Saratov")]
    EuropeSaratov,
    #[serde(alias = "Europe/Simferopol")]
    EuropeSimferopol,
    #[serde(alias = "Europe/Sofia")]
    EuropeSofia,
    #[serde(alias = "Europe/Stockholm")]
    EuropeStockholm,
    #[serde(alias = "Europe/Tallinn")]
    EuropeTallinn,
    #[serde(alias = "Europe/Tirane")]
    EuropeTirane,
    #[serde(alias = "Europe/Ulyanovsk")]
    EuropeUlyanovsk,
    #[serde(alias = "Europe/Uzhgorod")]
    EuropeUzhgorod,
    #[serde(alias = "Europe/Vienna")]
    EuropeVienna,
    #[serde(alias = "Europe/Vilnius")]
    EuropeVilnius,
    #[serde(alias = "Europe/Volgograd")]
    EuropeVolgograd,
    #[serde(alias = "Europe/Warsaw")]
    EuropeWarsaw,
    #[serde(alias = "Europe/Zaporozhye")]
    EuropeZaporozhye,
    #[serde(alias = "Europe/Zurich")]
    EuropeZurich,
    #[serde(alias = "Indian/Chagos")]
    IndianChagos,
    #[serde(alias = "Indian/Christmas")]
    IndianChristmas,
    #[serde(alias = "Indian/Cocos")]
    IndianCocos,
    #[serde(alias = "Indian/Kerguelen")]
    IndianKerguelen,
    #[serde(alias = "Indian/Mahe")]
    IndianMahe,
    #[serde(alias = "Indian/Maldives")]
    IndianMaldives,
    #[serde(alias = "Indian/Mauritius")]
    IndianMauritius,
    #[serde(alias = "Indian/Reunion")]
    IndianReunion,
    #[serde(alias = "Pacific/Apia")]
    PacificApia,
    #[serde(alias = "Pacific/Auckland")]
    PacificAuckland,
    #[serde(alias = "Pacific/Bougainville")]
    PacificBougainville,
    #[serde(alias = "Pacific/Chatham")]
    PacificChatham,
    #[serde(alias = "Pacific/Chuuk")]
    PacificChuuk,
    #[serde(alias = "Pacific/Easter")]
    PacificEaster,
    #[serde(alias = "Pacific/Efate")]
    PacificEfate,
    #[serde(alias = "Pacific/Enderbury")]
    PacificEnderbury,
    #[serde(alias = "Pacific/Fakaofo")]
    PacificFakaofo,
    #[serde(alias = "Pacific/Fiji")]
    PacificFiji,
    #[serde(alias = "Pacific/Funafuti")]
    PacificFunafuti,
    #[serde(alias = "Pacific/Galapagos")]
    PacificGalapagos,
    #[serde(alias = "Pacific/Gambier")]
    PacificGambier,
    #[serde(alias = "Pacific/Guadalcanal")]
    PacificGuadalcanal,
    #[serde(alias = "Pacific/Guam")]
    PacificGuam,
    #[serde(alias = "Pacific/Honolulu")]
    PacificHonolulu,
    #[serde(alias = "Pacific/Kiritimati")]
    PacificKiritimati,
    #[serde(alias = "Pacific/Kosrae")]
    PacificKosrae,
    #[serde(alias = "Pacific/Kwajalein")]
    PacificKwajalein,
    #[serde(alias = "Pacific/Majuro")]
    PacificMajuro,
    #[serde(alias = "Pacific/Marquesas")]
    PacificMarquesas,
    #[serde(alias = "Pacific/Nauru")]
    PacificNauru,
    #[serde(alias = "Pacific/Niue")]
    PacificNiue,
    #[serde(alias = "Pacific/Norfolk")]
    PacificNorfolk,
    #[serde(alias = "Pacific/Noumea")]
    PacificNoumea,
    #[serde(alias = "Pacific/Pago_Pago")]
    PacificPagoPago,
    #[serde(alias = "Pacific/Palau")]
    PacificPalau,
    #[serde(alias = "Pacific/Pitcairn")]
    PacificPitcairn,
    #[serde(alias = "Pacific/Pohnpei")]
    PacificPohnpei,
    #[serde(alias = "Pacific/Port_Moresby")]
    PacificPortMoresby,
    #[serde(alias = "Pacific/Rarotonga")]
    PacificRarotonga,
    #[serde(alias = "Pacific/Tahiti")]
    PacificTahiti,
    #[serde(alias = "Pacific/Tarawa")]
    PacificTarawa,
    #[serde(alias = "Pacific/Tongatapu")]
    PacificTongatapu,
    #[serde(alias = "Pacific/Wake")]
    PacificWake,
    #[serde(alias = "Pacific/Wallis")]
    PacificWallis,
} // enum

impl std::convert::From<&TransitTimeZone> for String {
    /// Converts a `TransitTimeZone` enum to a `String` that contains a [time
    /// zone](https://www.iana.org/time-zones) name.
    fn from(time_zone: &TransitTimeZone) -> String {
        match time_zone {
            TransitTimeZone::AfricaAbidjan => String::from("Africa/Abidjan"),
            TransitTimeZone::AfricaAccra => String::from("Africa/Accra"),
            TransitTimeZone::AfricaAlgiers => String::from("Africa/Algiers"),
            TransitTimeZone::AfricaBissau => String::from("Africa/Bissau"),
            TransitTimeZone::AfricaCairo => String::from("Africa/Cairo"),
            TransitTimeZone::AfricaCasablanca => String::from("Africa/Casablanca"),
            TransitTimeZone::AfricaCeuta => String::from("Africa/Ceuta"),
            TransitTimeZone::AfricaElAaiun => String::from("Africa/El_Aaiun"),
            TransitTimeZone::AfricaJohannesburg => String::from("Africa/Johannesburg"),
            TransitTimeZone::AfricaJuba => String::from("Africa/Juba"),
            TransitTimeZone::AfricaKhartoum => String::from("Africa/Khartoum"),
            TransitTimeZone::AfricaLagos => String::from("Africa/Lagos"),
            TransitTimeZone::AfricaMaputo => String::from("Africa/Maputo"),
            TransitTimeZone::AfricaMonrovia => String::from("Africa/Monrovia"),
            TransitTimeZone::AfricaNairobi => String::from("Africa/Nairobi"),
            TransitTimeZone::AfricaNdjamena => String::from("Africa/Ndjamena"),
            TransitTimeZone::AfricaSaoTome => String::from("Africa/Sao_Tome"),
            TransitTimeZone::AfricaTripoli => String::from("Africa/Tripoli"),
            TransitTimeZone::AfricaTunis => String::from("Africa/Tunis"),
            TransitTimeZone::AfricaWindhoek => String::from("Africa/Windhoek"),
            TransitTimeZone::AmericaAdak => String::from("America/Adak"),
            TransitTimeZone::AmericaAnchorage => String::from("America/Anchorage"),
            TransitTimeZone::AmericaAraguaina => String::from("America/Araguaina"),
            TransitTimeZone::AmericaArgentinaBuenosAires => String::from("America/Argentina/Buenos_Aires"),
            TransitTimeZone::AmericaArgentinaCatamarca => String::from("America/Argentina/Catamarca"),
            TransitTimeZone::AmericaArgentinaCordoba => String::from("America/Argentina/Cordoba"),
            TransitTimeZone::AmericaArgentinaJujuy => String::from("America/Argentina/Jujuy"),
            TransitTimeZone::AmericaArgentinaLaRioja => String::from("America/Argentina/La_Rioja"),
            TransitTimeZone::AmericaArgentinaMendoza => String::from("America/Argentina/Mendoza"),
            TransitTimeZone::AmericaArgentinaRioGallegos => String::from("America/Argentina/Rio_Gallegos"),
            TransitTimeZone::AmericaArgentinaSalta => String::from("AmAmerica/Argentina/Salta"),
            TransitTimeZone::AmericaArgentinaSanJuan => String::from("America/Argentina/San_Juan"),
            TransitTimeZone::AmericaArgentinaSanLuis => String::from("America/Argentina/San_Luis"),
            TransitTimeZone::AmericaArgentinaTucuman => String::from("America/Argentina/Tucuman"),
            TransitTimeZone::AmericaArgentinaUshuaia => String::from("America/Argentina/Ushuaia"),
            TransitTimeZone::AmericaAsuncion => String::from("America/Asuncion"),
            TransitTimeZone::AmericaAtikokan => String::from("America/Atikokan"),
            TransitTimeZone::AmericaBahia => String::from("America/Bahia"),
            TransitTimeZone::AmericaBahiaBanderas => String::from("America/Bahia_Banderas"),
            TransitTimeZone::AmericaBarbados => String::from("America/Barbados"),
            TransitTimeZone::AmericaBelem => String::from("America/Belem"),
            TransitTimeZone::AmericaBelize => String::from("America/Belize"),
            TransitTimeZone::AmericaBlancSablon => String::from("America/Blanc-Sablon"),
            TransitTimeZone::AmericaBoaVista => String::from("America/Boa_Vista"),
            TransitTimeZone::AmericaBogota => String::from("America/Bogota"),
            TransitTimeZone::AmericaBoise => String::from("America/Boise"),
            TransitTimeZone::AmericaCambridgeBay => String::from("America/Cambridge_Bay"),
            TransitTimeZone::AmericaCampoGrande => String::from("America/Campo_Grande"),
            TransitTimeZone::AmericaCancun => String::from("America/Cancun"),
            TransitTimeZone::AmericaCaracas => String::from("America/Caracas"),
            TransitTimeZone::AmericaCayenne => String::from("America/Cayenne"),
            TransitTimeZone::AmericaChicago => String::from("America/Chicago"),
            TransitTimeZone::AmericaChihuahua => String::from("America/Chihuahua"),
            TransitTimeZone::AmericaCostaRica => String::from("America/Costa_Rica"),
            TransitTimeZone::AmericaCreston => String::from("America/Creston"),
            TransitTimeZone::AmericaCuiaba => String::from("America/Cuiaba"),
            TransitTimeZone::AmericaCuracao => String::from("America/Curacao"),
            TransitTimeZone::AmericaDanmarkshavn => String::from("America/Danmarkshavn"),
            TransitTimeZone::AmericaDawson => String::from("America/Dawson"),
            TransitTimeZone::AmericaDawsonCreek => String::from("America/Dawson_Creek"),
            TransitTimeZone::AmericaDenver => String::from("America/Denver"),
            TransitTimeZone::AmericaDetroit => String::from("America/Detroit"),
            TransitTimeZone::AmericaEdmonton => String::from("America/Edmonton"),
            TransitTimeZone::AmericaEirunepe => String::from("America/Eirunepe"),
            TransitTimeZone::AmericaElSalvador => String::from("America/El_Salvador"),
            TransitTimeZone::AmericaFortNelson => String::from("America/Fort_Nelson"),
            TransitTimeZone::AmericaFortaleza => String::from("America/Fortaleza"),
            TransitTimeZone::AmericaGlaceBay => String::from("America/Glace_Bay"),
            TransitTimeZone::AmericaGodthab => String::from("America/Godthab"),
            TransitTimeZone::AmericaGooseBay => String::from("America/Goose_Bay"),
            TransitTimeZone::AmericaGrandTurk => String::from("America/Grand_Turk"),
            TransitTimeZone::AmericaGuatemala => String::from("America/Guatemala"),
            TransitTimeZone::AmericaGuayaquil => String::from("America/Guayaquil"),
            TransitTimeZone::AmericaGuyana => String::from("America/Guyana"),
            TransitTimeZone::AmericaHalifax => String::from("America/Halifax"),
            TransitTimeZone::AmericaHavana => String::from("America/Havana"),
            TransitTimeZone::AmericaHermosillo => String::from("America/Hermosillo"),
            TransitTimeZone::AmericaIndianaIndianapolis => String::from("America/Indiana/Indianapolis"),
            TransitTimeZone::AmericaIndianaKnox => String::from("America/Indiana/Knox"),
            TransitTimeZone::AmericaIndianaMarengo => String::from("America/Indiana/Marengo"),
            TransitTimeZone::AmericaIndianaPetersburg => String::from("America/Indiana/Petersburg"),
            TransitTimeZone::AmericaIndianaTellCity => String::from("America/Indiana/Tell_City"),
            TransitTimeZone::AmericaIndianaVevay => String::from("America/Indiana/Vevay"),
            TransitTimeZone::AmericaIndianaVincennes => String::from("America/Indiana/Vincennes"),
            TransitTimeZone::AmericaIndianaWinamac => String::from("America/Indiana/Winamac"),
            TransitTimeZone::AmericaInuvik => String::from("America/Inuvik"),
            TransitTimeZone::AmericaIqaluit => String::from("America/Iqaluit"),
            TransitTimeZone::AmericaJamaica => String::from("America/Jamaica"),
            TransitTimeZone::AmericaJuneau => String::from("America/Juneau"),
            TransitTimeZone::AmericaKentuckyLouisville => String::from("America/Kentucky/Louisville"),
            TransitTimeZone::AmericaKentuckyMonticello => String::from("America/Kentucky/Monticello"),
            TransitTimeZone::AmericaLaPaz => String::from("America/La_Paz"),
            TransitTimeZone::AmericaLima => String::from("America/Lima"),
            TransitTimeZone::AmericaLosAngeles => String::from("America/Los_Angeles"),
            TransitTimeZone::AmericaMaceio => String::from("America/Maceio"),
            TransitTimeZone::AmericaManagua => String::from("America/Managua"),
            TransitTimeZone::AmericaManaus => String::from("America/Manaus"),
            TransitTimeZone::AmericaMartinique => String::from("America/Martinique"),
            TransitTimeZone::AmericaMatamoros => String::from("America/Matamoros"),
            TransitTimeZone::AmericaMazatlan => String::from("America/Mazatlan"),
            TransitTimeZone::AmericaMenominee => String::from("America/Menominee"),
            TransitTimeZone::AmericaMerida => String::from("America/Merida"),
            TransitTimeZone::AmericaMetlakatla => String::from("America/Metlakatla"),
            TransitTimeZone::AmericaMexicoCity => String::from("America/Mexico_City"),
            TransitTimeZone::AmericaMiquelon => String::from("America/Miquelon"),
            TransitTimeZone::AmericaMoncton => String::from("America/Moncton"),
            TransitTimeZone::AmericaMonterrey => String::from("America/Monterrey"),
            TransitTimeZone::AmericaMontevideo => String::from("America/Montevideo"),
            TransitTimeZone::AmericaNassau => String::from("America/Nassau"),
            TransitTimeZone::AmericaNewYork => String::from("America/New_York"),
            TransitTimeZone::AmericaNipigon => String::from("America/Nipigon"),
            TransitTimeZone::AmericaNome => String::from("America/Nome"),
            TransitTimeZone::AmericaNoronha => String::from("America/Noronha"),
            TransitTimeZone::AmericaNorthDakotaBeulah => String::from("America/North_Dakota/Beulah"),
            TransitTimeZone::AmericaNorthDakotaCenter => String::from("America/North_Dakota/Center"),
            TransitTimeZone::AmericaNorthDakotaNewSalem => String::from("America/North_Dakota/New_Salem"),
            TransitTimeZone::AmericaOjinaga => String::from("America/Ojinaga"),
            TransitTimeZone::AmericaPanama => String::from("America/Panama"),
            TransitTimeZone::AmericaPangnirtung => String::from("America/Pangnirtung"),
            TransitTimeZone::AmericaParamaribo => String::from("America/Paramaribo"),
            TransitTimeZone::AmericaPhoenix => String::from("America/Phoenix"),
            TransitTimeZone::AmericaPortAuPrince => String::from("America/Port-au-Prince"),
            TransitTimeZone::AmericaPortOfSpain => String::from("America/Port_of_Spain"),
            TransitTimeZone::AmericaPortoVelho => String::from("America/Porto_Velho"),
            TransitTimeZone::AmericaPuertoRico => String::from("America/Puerto_Rico"),
            TransitTimeZone::AmericaPuntaArenas => String::from("America/Punta_Arenas"),
            TransitTimeZone::AmericaRainyRiver => String::from("America/Rainy_River"),
            TransitTimeZone::AmericaRankinInlet => String::from("America/Rankin_Inlet"),
            TransitTimeZone::AmericaRecife => String::from("America/Recife"),
            TransitTimeZone::AmericaRegina => String::from("America/Regina"),
            TransitTimeZone::AmericaResolute => String::from("America/Resolute"),
            TransitTimeZone::AmericaRioBranco => String::from("America/Rio_Branco"),
            TransitTimeZone::AmericaSantarem => String::from("America/Santarem"),
            TransitTimeZone::AmericaSantiago => String::from("America/Santiago"),
            TransitTimeZone::AmericaSantoDomingo => String::from("America/Santo_Domingo"),
            TransitTimeZone::AmericaSaoPaulo => String::from("America/Sao_Paulo"),
            TransitTimeZone::AmericaScoresbysund => String::from("America/Scoresbysund"),
            TransitTimeZone::AmericaSitka => String::from("America/Sitka"),
            TransitTimeZone::AmericaStJohns => String::from("America/St_Johns"),
            TransitTimeZone::AmericaSwiftCurrent => String::from("America/Swift_Current"),
            TransitTimeZone::AmericaTegucigalpa => String::from("America/Tegucigalpa"),
            TransitTimeZone::AmericaThule => String::from("America/Thule"),
            TransitTimeZone::AmericaThunderBay => String::from("America/Thunder_Bay"),
            TransitTimeZone::AmericaTijuana => String::from("America/Tijuana"),
            TransitTimeZone::AmericaToronto => String::from("America/Toronto"),
            TransitTimeZone::AmericaVancouver => String::from("America/Vancouver"),
            TransitTimeZone::AmericaWhitehorse => String::from("America/Whitehorse"),
            TransitTimeZone::AmericaWinnipeg => String::from("America/Winnipeg"),
            TransitTimeZone::AmericaYakutat => String::from("America/Yakutat"),
            TransitTimeZone::AmericaYellowknife => String::from("America/Yellowknife"),
            TransitTimeZone::AntarcticaCasey => String::from("Antarctica/Casey"),
            TransitTimeZone::AntarcticaDavis => String::from("Antarctica/Davis"),
            TransitTimeZone::AntarcticaDumontDUrville => String::from("Antarctica/DumontDUrville"),
            TransitTimeZone::AntarcticaMacquarie => String::from("Antarctica/Macquarie"),
            TransitTimeZone::AntarcticaMawson => String::from("Antarctica/Mawson"),
            TransitTimeZone::AntarcticaPalmer => String::from("Antarctica/Palmer"),
            TransitTimeZone::AntarcticaRothera => String::from("Antarctica/Rothera"),
            TransitTimeZone::AntarcticaSyowa => String::from("Antarctica/Syowa"),
            TransitTimeZone::AntarcticaTroll => String::from("Antarctica/Troll"),
            TransitTimeZone::AntarcticaVostok => String::from("Antarctica/Vostok"),
            TransitTimeZone::AsiaAlmaty => String::from("Asia/Almaty"),
            TransitTimeZone::AsiaAmman => String::from("Asia/Amman"),
            TransitTimeZone::AsiaAnadyr => String::from("Asia/Anadyr"),
            TransitTimeZone::AsiaAqtau => String::from("Asia/Aqtau"),
            TransitTimeZone::AsiaAqtobe => String::from("Asia/Aqtobe"),
            TransitTimeZone::AsiaAshgabat => String::from("Asia/Ashgabat"),
            TransitTimeZone::AsiaAtyrau => String::from("Asia/Atyrau"),
            TransitTimeZone::AsiaBaghdad => String::from("Asia/Baghdad"),
            TransitTimeZone::AsiaBaku => String::from("Asia/Baku"),
            TransitTimeZone::AsiaBangkok => String::from("Asia/Bangkok"),
            TransitTimeZone::AsiaBarnaul => String::from("Asia/Barnaul"),
            TransitTimeZone::AsiaBeirut => String::from("Asia/Beirut"),
            TransitTimeZone::AsiaBishkek => String::from("Asia/Bishkek"),
            TransitTimeZone::AsiaBrunei => String::from("Asia/Brunei"),
            TransitTimeZone::AsiaChita => String::from("Asia/Chita"),
            TransitTimeZone::AsiaChoibalsan => String::from("Asia/Choibalsan"),
            TransitTimeZone::AsiaColombo => String::from("Asia/Colombo"),
            TransitTimeZone::AsiaDamascus => String::from("Asia/Damascus"),
            TransitTimeZone::AsiaDhaka => String::from("Asia/Dhaka"),
            TransitTimeZone::AsiaDili => String::from("Asia/Dili"),
            TransitTimeZone::AsiaDubai => String::from("Asia/Dubai"),
            TransitTimeZone::AsiaDushanbe => String::from("Asia/Dushanbe"),
            TransitTimeZone::AsiaFamagusta => String::from("Asia/Famagusta"),
            TransitTimeZone::AsiaGaza => String::from("Asia/Gaza"),
            TransitTimeZone::AsiaHebron => String::from("Asia/Hebron"),
            TransitTimeZone::AsiaHoChiMinh => String::from("Asia/Ho_Chi_Minh"),
            TransitTimeZone::AsiaHongKong => String::from("Asia/Hong_Kong"),
            TransitTimeZone::AsiaHovd => String::from("Asia/Hovd"),
            TransitTimeZone::AsiaIrkutsk => String::from("Asia/Irkutsk"),
            TransitTimeZone::AsiaJakarta => String::from("Asia/Jakarta"),
            TransitTimeZone::AsiaJayapura => String::from("Asia/Jayapura"),
            TransitTimeZone::AsiaJerusalem => String::from("Asia/Jerusalem"),
            TransitTimeZone::AsiaKabul => String::from("Asia/Kabul"),
            TransitTimeZone::AsiaKamchatka => String::from("Asia/Kamchatka"),
            TransitTimeZone::AsiaKarachi => String::from("Asia/Karachi"),
            TransitTimeZone::AsiaKathmandu => String::from("Asia/Kathmandu"),
            TransitTimeZone::AsiaKhandyga => String::from("Asia/Khandyga"),
            TransitTimeZone::AsiaKolkata => String::from("Asia/Kolkata"),
            TransitTimeZone::AsiaKrasnoyarsk => String::from("Asia/Krasnoyarsk"),
            TransitTimeZone::AsiaKualaLumpur => String::from("Asia/Kuala_Lumpur"),
            TransitTimeZone::AsiaKuching => String::from("Asia/Kuching"),
            TransitTimeZone::AsiaMacau => String::from("Asia/Macau"),
            TransitTimeZone::AsiaMagadan => String::from("Asia/Magadan"),
            TransitTimeZone::AsiaMakassar => String::from("Asia/Makassar"),
            TransitTimeZone::AsiaManila => String::from("Asia/Manila"),
            TransitTimeZone::AsiaNicosia => String::from("Asia/Nicosia"),
            TransitTimeZone::AsiaNovokuznetsk => String::from("Asia/Novokuznetsk"),
            TransitTimeZone::AsiaNovosibirsk => String::from("Asia/Novosibirsk"),
            TransitTimeZone::AsiaOmsk => String::from("Asia/Omsk"),
            TransitTimeZone::AsiaOral => String::from("Asia/Oral"),
            TransitTimeZone::AsiaPontianak => String::from("Asia/Pontianak"),
            TransitTimeZone::AsiaPyongyang => String::from("Asia/Pyongyang"),
            TransitTimeZone::AsiaQatar => String::from("Asia/Qatar"),
            TransitTimeZone::AsiaQostanay => String::from("Asia/Qostanay"),
            TransitTimeZone::AsiaQyzylorda => String::from("Asia/Qyzylorda"),
            TransitTimeZone::AsiaRiyadh => String::from("Asia/Riyadh"),
            TransitTimeZone::AsiaSakhalin => String::from("Asia/Sakhalin"),
            TransitTimeZone::AsiaSamarkand => String::from("Asia/Samarkand"),
            TransitTimeZone::AsiaSeoul => String::from("Asia/Seoul"),
            TransitTimeZone::AsiaShanghai => String::from("Asia/Shanghai"),
            TransitTimeZone::AsiaSingapore => String::from("Asia/Singapore"),
            TransitTimeZone::AsiaSrednekolymsk => String::from("Asia/Srednekolymsk"),
            TransitTimeZone::AsiaTaipei => String::from("Asia/Taipei"),
            TransitTimeZone::AsiaTashkent => String::from("Asia/Tashkent"),
            TransitTimeZone::AsiaTbilisi => String::from("Asia/Tbilisi"),
            TransitTimeZone::AsiaTehran => String::from("Asia/Tehran"),
            TransitTimeZone::AsiaThimphu => String::from("Asia/Thimphu"),
            TransitTimeZone::AsiaTokyo => String::from("Asia/Tokyo"),
            TransitTimeZone::AsiaTomsk => String::from("Asia/Tomsk"),
            TransitTimeZone::AsiaUlaanbaatar => String::from("Asia/Ulaanbaatar"),
            TransitTimeZone::AsiaUrumqi => String::from("Asia/Urumqi"),
            TransitTimeZone::AsiaUstNera => String::from("Asia/Ust-Nera"),
            TransitTimeZone::AsiaVladivostok => String::from("Asia/Vladivostok"),
            TransitTimeZone::AsiaYakutsk => String::from("Asia/Yakutsk"),
            TransitTimeZone::AsiaYangon => String::from("Asia/Yangon"),
            TransitTimeZone::AsiaYekaterinburg => String::from("Asia/Yekaterinburg"),
            TransitTimeZone::AsiaYerevan => String::from("Asia/Yerevan"),
            TransitTimeZone::AtlanticAzores => String::from("Atlantic/Azores"),
            TransitTimeZone::AtlanticBermuda => String::from("Atlantic/Bermuda"),
            TransitTimeZone::AtlanticCanary => String::from("Atlantic/Canary"),
            TransitTimeZone::AtlanticCapeVerde => String::from("Atlantic/Cape_Verde"),
            TransitTimeZone::AtlanticFaroe => String::from("Atlantic/Faroe"),
            TransitTimeZone::AtlanticMadeira => String::from("Atlantic/Madeira"),
            TransitTimeZone::AtlanticReykjavik => String::from("Atlantic/Reykjavik"),
            TransitTimeZone::AtlanticSouthGeorgia => String::from("Atlantic/South_Georgia"),
            TransitTimeZone::AtlanticStanley => String::from("Atlantic/Stanley"),
            TransitTimeZone::AustraliaAdelaide => String::from("Australia/Adelaide"),
            TransitTimeZone::AustraliaBrisbane => String::from("Australia/Brisbane"),
            TransitTimeZone::AustraliaBrokenHill => String::from("Australia/Broken_Hill"),
            TransitTimeZone::AustraliaCurrie => String::from("Australia/Currie"),
            TransitTimeZone::AustraliaDarwin => String::from("Australia/Darwin"),
            TransitTimeZone::AustraliaEucla => String::from("Australia/Eucla"),
            TransitTimeZone::AustraliaHobart => String::from("Australia/Hobart"),
            TransitTimeZone::AustraliaLindeman => String::from("Australia/Lindeman"),
            TransitTimeZone::AustraliaLordHowe => String::from("Australia/Lord_Howe"),
            TransitTimeZone::AustraliaMelbourne => String::from("Australia/Melbourne"),
            TransitTimeZone::AustraliaPerth => String::from("Australia/Perth"),
            TransitTimeZone::AustraliaSydney => String::from("Australia/Sydney"),
            TransitTimeZone::EuropeAmsterdam => String::from("Europe/Amsterdam"),
            TransitTimeZone::EuropeAndorra => String::from("Europe/Andorra"),
            TransitTimeZone::EuropeAstrakhan => String::from("Europe/Astrakhan"),
            TransitTimeZone::EuropeAthens => String::from("Europe/Athens"),
            TransitTimeZone::EuropeBelgrade => String::from("Europe/Belgrade"),
            TransitTimeZone::EuropeBerlin => String::from("Europe/Berlin"),
            TransitTimeZone::EuropeBrussels => String::from("Europe/Brussels"),
            TransitTimeZone::EuropeBucharest => String::from("Europe/Bucharest"),
            TransitTimeZone::EuropeBudapest => String::from("Europe/Budapest"),
            TransitTimeZone::EuropeChisinau => String::from("Europe/Chisinau"),
            TransitTimeZone::EuropeCopenhagen => String::from("Europe/Copenhagen"),
            TransitTimeZone::EuropeDublin => String::from("Europe/Dublin"),
            TransitTimeZone::EuropeGibraltar => String::from("Europe/Gibraltar"),
            TransitTimeZone::EuropeHelsinki => String::from("Europe/Helsinki"),
            TransitTimeZone::EuropeIstanbul => String::from("Europe/Istanbul"),
            TransitTimeZone::EuropeKaliningrad => String::from("Europe/Kaliningrad"),
            TransitTimeZone::EuropeKiev => String::from("Europe/Kiev"),
            TransitTimeZone::EuropeKirov => String::from("Europe/Kirov"),
            TransitTimeZone::EuropeLisbon => String::from("Europe/Lisbon"),
            TransitTimeZone::EuropeLondon => String::from("Europe/London"),
            TransitTimeZone::EuropeLuxembourg => String::from("Europe/Luxembourg"),
            TransitTimeZone::EuropeMadrid => String::from("Europe/Madrid"),
            TransitTimeZone::EuropeMalta => String::from("Europe/Malta"),
            TransitTimeZone::EuropeMinsk => String::from("Europe/Minsk"),
            TransitTimeZone::EuropeMonaco => String::from("Europe/Monaco"),
            TransitTimeZone::EuropeMoscow => String::from("Europe/Moscow"),
            TransitTimeZone::EuropeOslo => String::from("Europe/Oslo"),
            TransitTimeZone::EuropeParis => String::from("Europe/Paris"),
            TransitTimeZone::EuropePrague => String::from("Europe/Prague"),
            TransitTimeZone::EuropeRiga => String::from("Europe/Riga"),
            TransitTimeZone::EuropeRome => String::from("Europe/Rome"),
            TransitTimeZone::EuropeSamara => String::from("Europe/Samara"),
            TransitTimeZone::EuropeSaratov => String::from("Europe/Saratov"),
            TransitTimeZone::EuropeSimferopol => String::from("Europe/Simferopol"),
            TransitTimeZone::EuropeSofia => String::from("Europe/Sofia"),
            TransitTimeZone::EuropeStockholm => String::from("Europe/Stockholm"),
            TransitTimeZone::EuropeTallinn => String::from("Europe/Tallinn"),
            TransitTimeZone::EuropeTirane => String::from("Europe/Tirane"),
            TransitTimeZone::EuropeUlyanovsk => String::from("Europe/Ulyanovsk"),
            TransitTimeZone::EuropeUzhgorod => String::from("Europe/Uzhgorod"),
            TransitTimeZone::EuropeVienna => String::from("Europe/Vienna"),
            TransitTimeZone::EuropeVilnius => String::from("Europe/Vilnius"),
            TransitTimeZone::EuropeVolgograd => String::from("Europe/Volgograd"),
            TransitTimeZone::EuropeWarsaw => String::from("Europe/Warsaw"),
            TransitTimeZone::EuropeZaporozhye => String::from("Europe/Zaporozhye"),
            TransitTimeZone::EuropeZurich => String::from("Europe/Zurich"),
            TransitTimeZone::IndianChagos => String::from("Indian/Chagos"),
            TransitTimeZone::IndianChristmas => String::from("Indian/Christmas"),
            TransitTimeZone::IndianCocos => String::from("Indian/Cocos"),
            TransitTimeZone::IndianKerguelen => String::from("Indian/Kerguelen"),
            TransitTimeZone::IndianMahe => String::from("Indian/Mahe"),
            TransitTimeZone::IndianMaldives => String::from("Indian/Maldives"),
            TransitTimeZone::IndianMauritius => String::from("Indian/Mauritius"),
            TransitTimeZone::IndianReunion => String::from("Indian/Reunion"),
            TransitTimeZone::PacificApia => String::from("Pacific/Apia"),
            TransitTimeZone::PacificAuckland => String::from("Pacific/Auckland"),
            TransitTimeZone::PacificBougainville => String::from("Pacific/Bougainville"),
            TransitTimeZone::PacificChatham => String::from("Pacific/Chatham"),
            TransitTimeZone::PacificChuuk => String::from("Pacific/Chuuk"),
            TransitTimeZone::PacificEaster => String::from("Pacific/Easter"),
            TransitTimeZone::PacificEfate => String::from("Pacific/Efate"),
            TransitTimeZone::PacificEnderbury => String::from("Pacific/Enderbury"),
            TransitTimeZone::PacificFakaofo => String::from("Pacific/Fakaofo"),
            TransitTimeZone::PacificFiji => String::from("Pacific/Fiji"),
            TransitTimeZone::PacificFunafuti => String::from("Pacific/Funafuti"),
            TransitTimeZone::PacificGalapagos => String::from("Pacific/Galapagos"),
            TransitTimeZone::PacificGambier => String::from("Pacific/Gambier"),
            TransitTimeZone::PacificGuadalcanal => String::from("Pacific/Guadalcanal"),
            TransitTimeZone::PacificGuam => String::from("Pacific/Guam"),
            TransitTimeZone::PacificHonolulu => String::from("Pacific/Honolulu"),
            TransitTimeZone::PacificKiritimati => String::from("Pacific/Kiritimati"),
            TransitTimeZone::PacificKosrae => String::from("Pacific/Kosrae"),
            TransitTimeZone::PacificKwajalein => String::from("Pacific/Kwajalein"),
            TransitTimeZone::PacificMajuro => String::from("Pacific/Majuro"),
            TransitTimeZone::PacificMarquesas => String::from("Pacific/Marquesas"),
            TransitTimeZone::PacificNauru => String::from("Pacific/Nauru"),
            TransitTimeZone::PacificNiue => String::from("Pacific/Niue"),
            TransitTimeZone::PacificNorfolk => String::from("Pacific/Norfolk"),
            TransitTimeZone::PacificNoumea => String::from("Pacific/Noumea"),
            TransitTimeZone::PacificPagoPago => String::from("Pacific/Pago_Pago"),
            TransitTimeZone::PacificPalau => String::from("Pacific/Palau"),
            TransitTimeZone::PacificPitcairn => String::from("Pacific/Pitcairn"),
            TransitTimeZone::PacificPohnpei => String::from("Pacific/Pohnpei"),
            TransitTimeZone::PacificPortMoresby => String::from("Pacific/Port_Moresby"),
            TransitTimeZone::PacificRarotonga => String::from("Pacific/Rarotonga"),
            TransitTimeZone::PacificTahiti => String::from("Pacific/Tahiti"),
            TransitTimeZone::PacificTarawa => String::from("Pacific/Tarawa"),
            TransitTimeZone::PacificTongatapu => String::from("Pacific/Tongatapu"),
            TransitTimeZone::PacificWake => String::from("Pacific/Wake"),
            TransitTimeZone::PacificWallis => String::from("Pacific/Wallis"),
        } // match
    } // fn
} // impl

impl std::convert::TryFrom<String> for TransitTimeZone {

    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;

    /// Gets a `TransitTimeZone` enum from a `String` that contains a valid
    /// [time zone](https://www.iana.org/time-zones) name.
    fn try_from(time_zone: String) -> Result<TransitTimeZone, Error> {
        match time_zone.as_ref() {
            "Africa/Abidjan" => Ok(TransitTimeZone::AfricaAbidjan),
            "Africa/Accra" => Ok(TransitTimeZone::AfricaAccra),
            "Africa/Algiers" => Ok(TransitTimeZone::AfricaAlgiers),
            "Africa/Bissau" => Ok(TransitTimeZone::AfricaBissau),
            "Africa/Cairo" => Ok(TransitTimeZone::AfricaCairo),
            "Africa/Casablanca" => Ok(TransitTimeZone::AfricaCasablanca),
            "Africa/Ceuta" => Ok(TransitTimeZone::AfricaCeuta),
            "Africa/El_Aaiun" => Ok(TransitTimeZone::AfricaElAaiun),
            "Africa/Johannesburg" => Ok(TransitTimeZone::AfricaJohannesburg),
            "Africa/Juba" => Ok(TransitTimeZone::AfricaJuba),
            "Africa/Khartoum" => Ok(TransitTimeZone::AfricaKhartoum),
            "Africa/Lagos" => Ok(TransitTimeZone::AfricaLagos),
            "Africa/Maputo" => Ok(TransitTimeZone::AfricaMaputo),
            "Africa/Monrovia" => Ok(TransitTimeZone::AfricaMonrovia),
            "Africa/Nairobi" => Ok(TransitTimeZone::AfricaNairobi),
            "Africa/Ndjamena" => Ok(TransitTimeZone::AfricaNdjamena),
            "Africa/Sao_Tome" => Ok(TransitTimeZone::AfricaSaoTome),
            "Africa/Tripoli" => Ok(TransitTimeZone::AfricaTripoli),
            "Africa/Tunis" => Ok(TransitTimeZone::AfricaTunis),
            "Africa/Windhoek" => Ok(TransitTimeZone::AfricaWindhoek),
            "America/Adak" => Ok(TransitTimeZone::AmericaAdak),
            "America/Anchorage" => Ok(TransitTimeZone::AmericaAnchorage),
            "America/Araguaina" => Ok(TransitTimeZone::AmericaAraguaina),
            "America/Argentina/Buenos_Aires" => Ok(TransitTimeZone::AmericaArgentinaBuenosAires),
            "America/Argentina/Catamarca" => Ok(TransitTimeZone::AmericaArgentinaCatamarca),
            "America/Argentina/Cordoba" => Ok(TransitTimeZone::AmericaArgentinaCordoba),
            "America/Argentina/Jujuy" => Ok(TransitTimeZone::AmericaArgentinaJujuy),
            "America/Argentina/La_Rioja" => Ok(TransitTimeZone::AmericaArgentinaLaRioja),
            "America/Argentina/Mendoza" => Ok(TransitTimeZone::AmericaArgentinaMendoza),
            "America/Argentina/Rio_Gallegos" => Ok(TransitTimeZone::AmericaArgentinaRioGallegos),
            "AmAmerica/Argentina/Salta" => Ok(TransitTimeZone::AmericaArgentinaSalta),
            "America/Argentina/San_Juan" => Ok(TransitTimeZone::AmericaArgentinaSanJuan),
            "America/Argentina/San_Luis" => Ok(TransitTimeZone::AmericaArgentinaSanLuis),
            "America/Argentina/Tucuman" => Ok(TransitTimeZone::AmericaArgentinaTucuman),
            "America/Argentina/Ushuaia" => Ok(TransitTimeZone::AmericaArgentinaUshuaia),
            "America/Asuncion" => Ok(TransitTimeZone::AmericaAsuncion),
            "America/Atikokan" => Ok(TransitTimeZone::AmericaAtikokan),
            "America/Bahia" => Ok(TransitTimeZone::AmericaBahia),
            "America/Bahia_Banderas" => Ok(TransitTimeZone::AmericaBahiaBanderas),
            "America/Barbados" => Ok(TransitTimeZone::AmericaBarbados),
            "America/Belem" => Ok(TransitTimeZone::AmericaBelem),
            "America/Belize" => Ok(TransitTimeZone::AmericaBelize),
            "America/Blanc-Sablon" => Ok(TransitTimeZone::AmericaBlancSablon),
            "America/Boa_Vista" => Ok(TransitTimeZone::AmericaBoaVista),
            "America/Bogota" => Ok(TransitTimeZone::AmericaBogota),
            "America/Boise" => Ok(TransitTimeZone::AmericaBoise),
            "America/Cambridge_Bay" => Ok(TransitTimeZone::AmericaCambridgeBay),
            "America/Campo_Grande" => Ok(TransitTimeZone::AmericaCampoGrande),
            "America/Cancun" => Ok(TransitTimeZone::AmericaCancun),
            "America/Caracas" => Ok(TransitTimeZone::AmericaCaracas),
            "America/Cayenne" => Ok(TransitTimeZone::AmericaCayenne),
            "America/Chicago" => Ok(TransitTimeZone::AmericaChicago),
            "America/Chihuahua" => Ok(TransitTimeZone::AmericaChihuahua),
            "America/Costa_Rica" => Ok(TransitTimeZone::AmericaCostaRica),
            "America/Creston" => Ok(TransitTimeZone::AmericaCreston),
            "America/Cuiaba" => Ok(TransitTimeZone::AmericaCuiaba),
            "America/Curacao" => Ok(TransitTimeZone::AmericaCuracao),
            "America/Danmarkshavn" => Ok(TransitTimeZone::AmericaDanmarkshavn),
            "America/Dawson" => Ok(TransitTimeZone::AmericaDawson),
            "America/Dawson_Creek" => Ok(TransitTimeZone::AmericaDawsonCreek),
            "America/Denver" => Ok(TransitTimeZone::AmericaDenver),
            "America/Detroit" => Ok(TransitTimeZone::AmericaDetroit),
            "America/Edmonton" => Ok(TransitTimeZone::AmericaEdmonton),
            "America/Eirunepe" => Ok(TransitTimeZone::AmericaEirunepe),
            "America/El_Salvador" => Ok(TransitTimeZone::AmericaElSalvador),
            "America/Fort_Nelson" => Ok(TransitTimeZone::AmericaFortNelson),
            "America/Fortaleza" => Ok(TransitTimeZone::AmericaFortaleza),
            "America/Glace_Bay" => Ok(TransitTimeZone::AmericaGlaceBay),
            "America/Godthab" => Ok(TransitTimeZone::AmericaGodthab),
            "America/Goose_Bay" => Ok(TransitTimeZone::AmericaGooseBay),
            "America/Grand_Turk" => Ok(TransitTimeZone::AmericaGrandTurk),
            "America/Guatemala" => Ok(TransitTimeZone::AmericaGuatemala),
            "America/Guayaquil" => Ok(TransitTimeZone::AmericaGuayaquil),
            "America/Guyana" => Ok(TransitTimeZone::AmericaGuyana),
            "America/Halifax" => Ok(TransitTimeZone::AmericaHalifax),
            "America/Havana" => Ok(TransitTimeZone::AmericaHavana),
            "America/Hermosillo" => Ok(TransitTimeZone::AmericaHermosillo),
            "America/Indiana/Indianapolis" => Ok(TransitTimeZone::AmericaIndianaIndianapolis),
            "America/Indiana/Knox" => Ok(TransitTimeZone::AmericaIndianaKnox),
            "America/Indiana/Marengo" => Ok(TransitTimeZone::AmericaIndianaMarengo),
            "America/Indiana/Petersburg" => Ok(TransitTimeZone::AmericaIndianaPetersburg),
            "America/Indiana/Tell_City" => Ok(TransitTimeZone::AmericaIndianaTellCity),
            "America/Indiana/Vevay" => Ok(TransitTimeZone::AmericaIndianaVevay),
            "America/Indiana/Vincennes" => Ok(TransitTimeZone::AmericaIndianaVincennes),
            "America/Indiana/Winamac" => Ok(TransitTimeZone::AmericaIndianaWinamac),
            "America/Inuvik" => Ok(TransitTimeZone::AmericaInuvik),
            "America/Iqaluit" => Ok(TransitTimeZone::AmericaIqaluit),
            "America/Jamaica" => Ok(TransitTimeZone::AmericaJamaica),
            "America/Juneau" => Ok(TransitTimeZone::AmericaJuneau),
            "America/Kentucky/Louisville" => Ok(TransitTimeZone::AmericaKentuckyLouisville),
            "America/Kentucky/Monticello" => Ok(TransitTimeZone::AmericaKentuckyMonticello),
            "America/La_Paz" => Ok(TransitTimeZone::AmericaLaPaz),
            "America/Lima" => Ok(TransitTimeZone::AmericaLima),
            "America/Los_Angeles" => Ok(TransitTimeZone::AmericaLosAngeles),
            "America/Maceio" => Ok(TransitTimeZone::AmericaMaceio),
            "America/Managua" => Ok(TransitTimeZone::AmericaManagua),
            "America/Manaus" => Ok(TransitTimeZone::AmericaManaus),
            "America/Martinique" => Ok(TransitTimeZone::AmericaMartinique),
            "America/Matamoros" => Ok(TransitTimeZone::AmericaMatamoros),
            "America/Mazatlan" => Ok(TransitTimeZone::AmericaMazatlan),
            "America/Menominee" => Ok(TransitTimeZone::AmericaMenominee),
            "America/Merida" => Ok(TransitTimeZone::AmericaMerida),
            "America/Metlakatla" => Ok(TransitTimeZone::AmericaMetlakatla),
            "America/Mexico_City" => Ok(TransitTimeZone::AmericaMexicoCity),
            "America/Miquelon" => Ok(TransitTimeZone::AmericaMiquelon),
            "America/Moncton" => Ok(TransitTimeZone::AmericaMoncton),
            "America/Monterrey" => Ok(TransitTimeZone::AmericaMonterrey),
            "America/Montevideo" => Ok(TransitTimeZone::AmericaMontevideo),
            "America/Nassau" => Ok(TransitTimeZone::AmericaNassau),
            "America/New_York" => Ok(TransitTimeZone::AmericaNewYork),
            "America/Nipigon" => Ok(TransitTimeZone::AmericaNipigon),
            "America/Nome" => Ok(TransitTimeZone::AmericaNome),
            "America/Noronha" => Ok(TransitTimeZone::AmericaNoronha),
            "America/North_Dakota/Beulah" => Ok(TransitTimeZone::AmericaNorthDakotaBeulah),
            "America/North_Dakota/Center" => Ok(TransitTimeZone::AmericaNorthDakotaCenter),
            "America/North_Dakota/New_Salem" => Ok(TransitTimeZone::AmericaNorthDakotaNewSalem),
            "America/Ojinaga" => Ok(TransitTimeZone::AmericaOjinaga),
            "America/Panama" => Ok(TransitTimeZone::AmericaPanama),
            "America/Pangnirtung" => Ok(TransitTimeZone::AmericaPangnirtung),
            "America/Paramaribo" => Ok(TransitTimeZone::AmericaParamaribo),
            "America/Phoenix" => Ok(TransitTimeZone::AmericaPhoenix),
            "America/Port-au-Prince" => Ok(TransitTimeZone::AmericaPortAuPrince),
            "America/Port_of_Spain" => Ok(TransitTimeZone::AmericaPortOfSpain),
            "America/Porto_Velho" => Ok(TransitTimeZone::AmericaPortoVelho),
            "America/Puerto_Rico" => Ok(TransitTimeZone::AmericaPuertoRico),
            "America/Punta_Arenas" => Ok(TransitTimeZone::AmericaPuntaArenas),
            "America/Rainy_River" => Ok(TransitTimeZone::AmericaRainyRiver),
            "America/Rankin_Inlet" => Ok(TransitTimeZone::AmericaRankinInlet),
            "America/Recife" => Ok(TransitTimeZone::AmericaRecife),
            "America/Regina" => Ok(TransitTimeZone::AmericaRegina),
            "America/Resolute" => Ok(TransitTimeZone::AmericaResolute),
            "America/Rio_Branco" => Ok(TransitTimeZone::AmericaRioBranco),
            "America/Santarem" => Ok(TransitTimeZone::AmericaSantarem),
            "America/Santiago" => Ok(TransitTimeZone::AmericaSantiago),
            "America/Santo_Domingo" => Ok(TransitTimeZone::AmericaSantoDomingo),
            "America/Sao_Paulo" => Ok(TransitTimeZone::AmericaSaoPaulo),
            "America/Scoresbysund" => Ok(TransitTimeZone::AmericaScoresbysund),
            "America/Sitka" => Ok(TransitTimeZone::AmericaSitka),
            "America/St_Johns" => Ok(TransitTimeZone::AmericaStJohns),
            "America/Swift_Current" => Ok(TransitTimeZone::AmericaSwiftCurrent),
            "America/Tegucigalpa" => Ok(TransitTimeZone::AmericaTegucigalpa),
            "America/Thule" => Ok(TransitTimeZone::AmericaThule),
            "America/Thunder_Bay" => Ok(TransitTimeZone::AmericaThunderBay),
            "America/Tijuana" => Ok(TransitTimeZone::AmericaTijuana),
            "America/Toronto" => Ok(TransitTimeZone::AmericaToronto),
            "America/Vancouver" => Ok(TransitTimeZone::AmericaVancouver),
            "America/Whitehorse" => Ok(TransitTimeZone::AmericaWhitehorse),
            "America/Winnipeg" => Ok(TransitTimeZone::AmericaWinnipeg),
            "America/Yakutat" => Ok(TransitTimeZone::AmericaYakutat),
            "America/Yellowknife" => Ok(TransitTimeZone::AmericaYellowknife),
            "Antarctica/Casey" => Ok(TransitTimeZone::AntarcticaCasey),
            "Antarctica/Davis" => Ok(TransitTimeZone::AntarcticaDavis),
            "Antarctica/DumontDUrville" => Ok(TransitTimeZone::AntarcticaDumontDUrville),
            "Antarctica/Macquarie" => Ok(TransitTimeZone::AntarcticaMacquarie),
            "Antarctica/Mawson" => Ok(TransitTimeZone::AntarcticaMawson),
            "Antarctica/Palmer" => Ok(TransitTimeZone::AntarcticaPalmer),
            "Antarctica/Rothera" => Ok(TransitTimeZone::AntarcticaRothera),
            "Antarctica/Syowa" => Ok(TransitTimeZone::AntarcticaSyowa),
            "Antarctica/Troll" => Ok(TransitTimeZone::AntarcticaTroll),
            "Antarctica/Vostok" => Ok(TransitTimeZone::AntarcticaVostok),
            "Asia/Almaty" => Ok(TransitTimeZone::AsiaAlmaty),
            "Asia/Amman" => Ok(TransitTimeZone::AsiaAmman),
            "Asia/Anadyr" => Ok(TransitTimeZone::AsiaAnadyr),
            "Asia/Aqtau" => Ok(TransitTimeZone::AsiaAqtau),
            "Asia/Aqtobe" => Ok(TransitTimeZone::AsiaAqtobe),
            "Asia/Ashgabat" => Ok(TransitTimeZone::AsiaAshgabat),
            "Asia/Atyrau" => Ok(TransitTimeZone::AsiaAtyrau),
            "Asia/Baghdad" => Ok(TransitTimeZone::AsiaBaghdad),
            "Asia/Baku" => Ok(TransitTimeZone::AsiaBaku),
            "Asia/Bangkok" => Ok(TransitTimeZone::AsiaBangkok),
            "Asia/Barnaul" => Ok(TransitTimeZone::AsiaBarnaul),
            "Asia/Beirut" => Ok(TransitTimeZone::AsiaBeirut),
            "Asia/Bishkek" => Ok(TransitTimeZone::AsiaBishkek),
            "Asia/Brunei" => Ok(TransitTimeZone::AsiaBrunei),
            "Asia/Chita" => Ok(TransitTimeZone::AsiaChita),
            "Asia/Choibalsan" => Ok(TransitTimeZone::AsiaChoibalsan),
            "Asia/Colombo" => Ok(TransitTimeZone::AsiaColombo),
            "Asia/Damascus" => Ok(TransitTimeZone::AsiaDamascus),
            "Asia/Dhaka" => Ok(TransitTimeZone::AsiaDhaka),
            "Asia/Dili" => Ok(TransitTimeZone::AsiaDili),
            "Asia/Dubai" => Ok(TransitTimeZone::AsiaDubai),
            "Asia/Dushanbe" => Ok(TransitTimeZone::AsiaDushanbe),
            "Asia/Famagusta" => Ok(TransitTimeZone::AsiaFamagusta),
            "Asia/Gaza" => Ok(TransitTimeZone::AsiaGaza),
            "Asia/Hebron" => Ok(TransitTimeZone::AsiaHebron),
            "Asia/Ho_Chi_Minh" => Ok(TransitTimeZone::AsiaHoChiMinh),
            "Asia/Hong_Kong" => Ok(TransitTimeZone::AsiaHongKong),
            "Asia/Hovd" => Ok(TransitTimeZone::AsiaHovd),
            "Asia/Irkutsk" => Ok(TransitTimeZone::AsiaIrkutsk),
            "Asia/Jakarta" => Ok(TransitTimeZone::AsiaJakarta),
            "Asia/Jayapura" => Ok(TransitTimeZone::AsiaJayapura),
            "Asia/Jerusalem" => Ok(TransitTimeZone::AsiaJerusalem),
            "Asia/Kabul" => Ok(TransitTimeZone::AsiaKabul),
            "Asia/Kamchatka" => Ok(TransitTimeZone::AsiaKamchatka),
            "Asia/Karachi" => Ok(TransitTimeZone::AsiaKarachi),
            "Asia/Kathmandu" => Ok(TransitTimeZone::AsiaKathmandu),
            "Asia/Khandyga" => Ok(TransitTimeZone::AsiaKhandyga),
            "Asia/Kolkata" => Ok(TransitTimeZone::AsiaKolkata),
            "Asia/Krasnoyarsk" => Ok(TransitTimeZone::AsiaKrasnoyarsk),
            "Asia/Kuala_Lumpur" => Ok(TransitTimeZone::AsiaKualaLumpur),
            "Asia/Kuching" => Ok(TransitTimeZone::AsiaKuching),
            "Asia/Macau" => Ok(TransitTimeZone::AsiaMacau),
            "Asia/Magadan" => Ok(TransitTimeZone::AsiaMagadan),
            "Asia/Makassar" => Ok(TransitTimeZone::AsiaMakassar),
            "Asia/Manila" => Ok(TransitTimeZone::AsiaManila),
            "Asia/Nicosia" => Ok(TransitTimeZone::AsiaNicosia),
            "Asia/Novokuznetsk" => Ok(TransitTimeZone::AsiaNovokuznetsk),
            "Asia/Novosibirsk" => Ok(TransitTimeZone::AsiaNovosibirsk),
            "Asia/Omsk" => Ok(TransitTimeZone::AsiaOmsk),
            "Asia/Oral" => Ok(TransitTimeZone::AsiaOral),
            "Asia/Pontianak" => Ok(TransitTimeZone::AsiaPontianak),
            "Asia/Pyongyang" => Ok(TransitTimeZone::AsiaPyongyang),
            "Asia/Qatar" => Ok(TransitTimeZone::AsiaQatar),
            "Asia/Qostanay" => Ok(TransitTimeZone::AsiaQostanay),
            "Asia/Qyzylorda" => Ok(TransitTimeZone::AsiaQyzylorda),
            "Asia/Riyadh" => Ok(TransitTimeZone::AsiaRiyadh),
            "Asia/Sakhalin" => Ok(TransitTimeZone::AsiaSakhalin),
            "Asia/Samarkand" => Ok(TransitTimeZone::AsiaSamarkand),
            "Asia/Seoul" => Ok(TransitTimeZone::AsiaSeoul),
            "Asia/Shanghai" => Ok(TransitTimeZone::AsiaShanghai),
            "Asia/Singapore" => Ok(TransitTimeZone::AsiaSingapore),
            "Asia/Srednekolymsk" => Ok(TransitTimeZone::AsiaSrednekolymsk),
            "Asia/Taipei" => Ok(TransitTimeZone::AsiaTaipei),
            "Asia/Tashkent" => Ok(TransitTimeZone::AsiaTashkent),
            "Asia/Tbilisi" => Ok(TransitTimeZone::AsiaTbilisi),
            "Asia/Tehran" => Ok(TransitTimeZone::AsiaTehran),
            "Asia/Thimphu" => Ok(TransitTimeZone::AsiaThimphu),
            "Asia/Tokyo" => Ok(TransitTimeZone::AsiaTokyo),
            "Asia/Tomsk" => Ok(TransitTimeZone::AsiaTomsk),
            "Asia/Ulaanbaatar" => Ok(TransitTimeZone::AsiaUlaanbaatar),
            "Asia/Urumqi" => Ok(TransitTimeZone::AsiaUrumqi),
            "Asia/Ust-Nera" => Ok(TransitTimeZone::AsiaUstNera),
            "Asia/Vladivostok" => Ok(TransitTimeZone::AsiaVladivostok),
            "Asia/Yakutsk" => Ok(TransitTimeZone::AsiaYakutsk),
            "Asia/Yangon" => Ok(TransitTimeZone::AsiaYangon),
            "Asia/Yekaterinburg" => Ok(TransitTimeZone::AsiaYekaterinburg),
            "Asia/Yerevan" => Ok(TransitTimeZone::AsiaYerevan),
            "Atlantic/Azores" => Ok(TransitTimeZone::AtlanticAzores),
            "Atlantic/Bermuda" => Ok(TransitTimeZone::AtlanticBermuda),
            "Atlantic/Canary" => Ok(TransitTimeZone::AtlanticCanary),
            "Atlantic/Cape_Verde" => Ok(TransitTimeZone::AtlanticCapeVerde),
            "Atlantic/Faroe" => Ok(TransitTimeZone::AtlanticFaroe),
            "Atlantic/Madeira" => Ok(TransitTimeZone::AtlanticMadeira),
            "Atlantic/Reykjavik" => Ok(TransitTimeZone::AtlanticReykjavik),
            "Atlantic/South_Georgia" => Ok(TransitTimeZone::AtlanticSouthGeorgia),
            "Atlantic/Stanley" => Ok(TransitTimeZone::AtlanticStanley),
            "Australia/Adelaide" => Ok(TransitTimeZone::AustraliaAdelaide),
            "Australia/Brisbane" => Ok(TransitTimeZone::AustraliaBrisbane),
            "Australia/Broken_Hill" => Ok(TransitTimeZone::AustraliaBrokenHill),
            "Australia/Currie" => Ok(TransitTimeZone::AustraliaCurrie),
            "Australia/Darwin" => Ok(TransitTimeZone::AustraliaDarwin),
            "Australia/Eucla" => Ok(TransitTimeZone::AustraliaEucla),
            "Australia/Hobart" => Ok(TransitTimeZone::AustraliaHobart),
            "Australia/Lindeman" => Ok(TransitTimeZone::AustraliaLindeman),
            "Australia/Lord_Howe" => Ok(TransitTimeZone::AustraliaLordHowe),
            "Australia/Melbourne" => Ok(TransitTimeZone::AustraliaMelbourne),
            "Australia/Perth" => Ok(TransitTimeZone::AustraliaPerth),
            "Australia/Sydney" => Ok(TransitTimeZone::AustraliaSydney),
            "Europe/Amsterdam" => Ok(TransitTimeZone::EuropeAmsterdam),
            "Europe/Andorra" => Ok(TransitTimeZone::EuropeAndorra),
            "Europe/Astrakhan" => Ok(TransitTimeZone::EuropeAstrakhan),
            "Europe/Athens" => Ok(TransitTimeZone::EuropeAthens),
            "Europe/Belgrade" => Ok(TransitTimeZone::EuropeBelgrade),
            "Europe/Berlin" => Ok(TransitTimeZone::EuropeBerlin),
            "Europe/Brussels" => Ok(TransitTimeZone::EuropeBrussels),
            "Europe/Bucharest" => Ok(TransitTimeZone::EuropeBucharest),
            "Europe/Budapest" => Ok(TransitTimeZone::EuropeBudapest),
            "Europe/Chisinau" => Ok(TransitTimeZone::EuropeChisinau),
            "Europe/Copenhagen" => Ok(TransitTimeZone::EuropeCopenhagen),
            "Europe/Dublin" => Ok(TransitTimeZone::EuropeDublin),
            "Europe/Gibraltar" => Ok(TransitTimeZone::EuropeGibraltar),
            "Europe/Helsinki" => Ok(TransitTimeZone::EuropeHelsinki),
            "Europe/Istanbul" => Ok(TransitTimeZone::EuropeIstanbul),
            "Europe/Kaliningrad" => Ok(TransitTimeZone::EuropeKaliningrad),
            "Europe/Kiev" => Ok(TransitTimeZone::EuropeKiev),
            "Europe/Kirov" => Ok(TransitTimeZone::EuropeKirov),
            "Europe/Lisbon" => Ok(TransitTimeZone::EuropeLisbon),
            "Europe/London" => Ok(TransitTimeZone::EuropeLondon),
            "Europe/Luxembourg" => Ok(TransitTimeZone::EuropeLuxembourg),
            "Europe/Madrid" => Ok(TransitTimeZone::EuropeMadrid),
            "Europe/Malta" => Ok(TransitTimeZone::EuropeMalta),
            "Europe/Minsk" => Ok(TransitTimeZone::EuropeMinsk),
            "Europe/Monaco" => Ok(TransitTimeZone::EuropeMonaco),
            "Europe/Moscow" => Ok(TransitTimeZone::EuropeMoscow),
            "Europe/Oslo" => Ok(TransitTimeZone::EuropeOslo),
            "Europe/Paris" => Ok(TransitTimeZone::EuropeParis),
            "Europe/Prague" => Ok(TransitTimeZone::EuropePrague),
            "Europe/Riga" => Ok(TransitTimeZone::EuropeRiga),
            "Europe/Rome" => Ok(TransitTimeZone::EuropeRome),
            "Europe/Samara" => Ok(TransitTimeZone::EuropeSamara),
            "Europe/Saratov" => Ok(TransitTimeZone::EuropeSaratov),
            "Europe/Simferopol" => Ok(TransitTimeZone::EuropeSimferopol),
            "Europe/Sofia" => Ok(TransitTimeZone::EuropeSofia),
            "Europe/Stockholm" => Ok(TransitTimeZone::EuropeStockholm),
            "Europe/Tallinn" => Ok(TransitTimeZone::EuropeTallinn),
            "Europe/Tirane" => Ok(TransitTimeZone::EuropeTirane),
            "Europe/Ulyanovsk" => Ok(TransitTimeZone::EuropeUlyanovsk),
            "Europe/Uzhgorod" => Ok(TransitTimeZone::EuropeUzhgorod),
            "Europe/Vienna" => Ok(TransitTimeZone::EuropeVienna),
            "Europe/Vilnius" => Ok(TransitTimeZone::EuropeVilnius),
            "Europe/Volgograd" => Ok(TransitTimeZone::EuropeVolgograd),
            "Europe/Warsaw" => Ok(TransitTimeZone::EuropeWarsaw),
            "Europe/Zaporozhye" => Ok(TransitTimeZone::EuropeZaporozhye),
            "Europe/Zurich" => Ok(TransitTimeZone::EuropeZurich),
            "Indian/Chagos" => Ok(TransitTimeZone::IndianChagos),
            "Indian/Christmas" => Ok(TransitTimeZone::IndianChristmas),
            "Indian/Cocos" => Ok(TransitTimeZone::IndianCocos),
            "Indian/Kerguelen" => Ok(TransitTimeZone::IndianKerguelen),
            "Indian/Mahe" => Ok(TransitTimeZone::IndianMahe),
            "Indian/Maldives" => Ok(TransitTimeZone::IndianMaldives),
            "Indian/Mauritius" => Ok(TransitTimeZone::IndianMauritius),
            "Indian/Reunion" => Ok(TransitTimeZone::IndianReunion),
            "Pacific/Apia" => Ok(TransitTimeZone::PacificApia),
            "Pacific/Auckland" => Ok(TransitTimeZone::PacificAuckland),
            "Pacific/Bougainville" => Ok(TransitTimeZone::PacificBougainville),
            "Pacific/Chatham" => Ok(TransitTimeZone::PacificChatham),
            "Pacific/Chuuk" => Ok(TransitTimeZone::PacificChuuk),
            "Pacific/Easter" => Ok(TransitTimeZone::PacificEaster),
            "Pacific/Efate" => Ok(TransitTimeZone::PacificEfate),
            "Pacific/Enderbury" => Ok(TransitTimeZone::PacificEnderbury),
            "Pacific/Fakaofo" => Ok(TransitTimeZone::PacificFakaofo),
            "Pacific/Fiji" => Ok(TransitTimeZone::PacificFiji),
            "Pacific/Funafuti" => Ok(TransitTimeZone::PacificFunafuti),
            "Pacific/Galapagos" => Ok(TransitTimeZone::PacificGalapagos),
            "Pacific/Gambier" => Ok(TransitTimeZone::PacificGambier),
            "Pacific/Guadalcanal" => Ok(TransitTimeZone::PacificGuadalcanal),
            "Pacific/Guam" => Ok(TransitTimeZone::PacificGuam),
            "Pacific/Honolulu" => Ok(TransitTimeZone::PacificHonolulu),
            "Pacific/Kiritimati" => Ok(TransitTimeZone::PacificKiritimati),
            "Pacific/Kosrae" => Ok(TransitTimeZone::PacificKosrae),
            "Pacific/Kwajalein" => Ok(TransitTimeZone::PacificKwajalein),
            "Pacific/Majuro" => Ok(TransitTimeZone::PacificMajuro),
            "Pacific/Marquesas" => Ok(TransitTimeZone::PacificMarquesas),
            "Pacific/Nauru" => Ok(TransitTimeZone::PacificNauru),
            "Pacific/Niue" => Ok(TransitTimeZone::PacificNiue),
            "Pacific/Norfolk" => Ok(TransitTimeZone::PacificNorfolk),
            "Pacific/Noumea" => Ok(TransitTimeZone::PacificNoumea),
            "Pacific/Pago_Pago" => Ok(TransitTimeZone::PacificPagoPago),
            "Pacific/Palau" => Ok(TransitTimeZone::PacificPalau),
            "Pacific/Pitcairn" => Ok(TransitTimeZone::PacificPitcairn),
            "Pacific/Pohnpei" => Ok(TransitTimeZone::PacificPohnpei),
            "Pacific/Port_Moresby" => Ok(TransitTimeZone::PacificPortMoresby),
            "Pacific/Rarotonga" => Ok(TransitTimeZone::PacificRarotonga),
            "Pacific/Tahiti" => Ok(TransitTimeZone::PacificTahiti),
            "Pacific/Tarawa" => Ok(TransitTimeZone::PacificTarawa),
            "Pacific/Tongatapu" => Ok(TransitTimeZone::PacificTongatapu),
            "Pacific/Wake" => Ok(TransitTimeZone::PacificWake),
            "Pacific/Wallis" => Ok(TransitTimeZone::PacificWallis),
            _ => Err(Error::InvalidTimeZoneName(time_zone)),
        } // match
    } // fn
} // impl

impl std::default::Default for TransitTimeZone {
    /// Returns a reasonable default variant for the `TransitTimeZone` enum
    /// type.
    fn default() -> Self {
        TransitTimeZone::AmericaEdmonton
    } // fn
} // impl

impl std::fmt::Display for TransitTimeZone {
    /// Formats a `TransitTimeZone` enum into a string that is presentable to
    /// the end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       write!(f, "{}", String::from(self))
    } // fn
} // impl