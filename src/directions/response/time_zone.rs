//! Contains the `TimeZone` enum and its associated traits. It is used to
//! work with the times returned by transit directions from the Google Maps
//! Directions API.

use crate::directions::error::Error;
use serde::{Serialize, Deserialize};

/// A comprehensive list of time zones. At the moment this is used only for
/// Google Maps Transit Directions. The intent behind having _Serde_ convert
/// the time zone `String` to an `enum` is for efficient time zone conversions,
/// information lookups, and manipulation in the future.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum TimeZone {
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

impl std::convert::From<&TimeZone> for String {
    /// Converts a `TimeZone` enum to a `String` that contains an [time
    /// zone](https://www.iana.org/time-zones) name.
    fn from(time_zone: &TimeZone) -> String {
        match time_zone {
            TimeZone::AfricaAbidjan => String::from("Africa/Abidjan"),
            TimeZone::AfricaAccra => String::from("Africa/Accra"),
            TimeZone::AfricaAlgiers => String::from("Africa/Algiers"),
            TimeZone::AfricaBissau => String::from("Africa/Bissau"),
            TimeZone::AfricaCairo => String::from("Africa/Cairo"),
            TimeZone::AfricaCasablanca => String::from("Africa/Casablanca"),
            TimeZone::AfricaCeuta => String::from("Africa/Ceuta"),
            TimeZone::AfricaElAaiun => String::from("Africa/El_Aaiun"),
            TimeZone::AfricaJohannesburg => String::from("Africa/Johannesburg"),
            TimeZone::AfricaJuba => String::from("Africa/Juba"),
            TimeZone::AfricaKhartoum => String::from("Africa/Khartoum"),
            TimeZone::AfricaLagos => String::from("Africa/Lagos"),
            TimeZone::AfricaMaputo => String::from("Africa/Maputo"),
            TimeZone::AfricaMonrovia => String::from("Africa/Monrovia"),
            TimeZone::AfricaNairobi => String::from("Africa/Nairobi"),
            TimeZone::AfricaNdjamena => String::from("Africa/Ndjamena"),
            TimeZone::AfricaSaoTome => String::from("Africa/Sao_Tome"),
            TimeZone::AfricaTripoli => String::from("Africa/Tripoli"),
            TimeZone::AfricaTunis => String::from("Africa/Tunis"),
            TimeZone::AfricaWindhoek => String::from("Africa/Windhoek"),
            TimeZone::AmericaAdak => String::from("America/Adak"),
            TimeZone::AmericaAnchorage => String::from("America/Anchorage"),
            TimeZone::AmericaAraguaina => String::from("America/Araguaina"),
            TimeZone::AmericaArgentinaBuenosAires => String::from("America/Argentina/Buenos_Aires"),
            TimeZone::AmericaArgentinaCatamarca => String::from("America/Argentina/Catamarca"),
            TimeZone::AmericaArgentinaCordoba => String::from("America/Argentina/Cordoba"),
            TimeZone::AmericaArgentinaJujuy => String::from("America/Argentina/Jujuy"),
            TimeZone::AmericaArgentinaLaRioja => String::from("America/Argentina/La_Rioja"),
            TimeZone::AmericaArgentinaMendoza => String::from("America/Argentina/Mendoza"),
            TimeZone::AmericaArgentinaRioGallegos => String::from("America/Argentina/Rio_Gallegos"),
            TimeZone::AmericaArgentinaSalta => String::from("AmAmerica/Argentina/Salta"),
            TimeZone::AmericaArgentinaSanJuan => String::from("America/Argentina/San_Juan"),
            TimeZone::AmericaArgentinaSanLuis => String::from("America/Argentina/San_Luis"),
            TimeZone::AmericaArgentinaTucuman => String::from("America/Argentina/Tucuman"),
            TimeZone::AmericaArgentinaUshuaia => String::from("America/Argentina/Ushuaia"),
            TimeZone::AmericaAsuncion => String::from("America/Asuncion"),
            TimeZone::AmericaAtikokan => String::from("America/Atikokan"),
            TimeZone::AmericaBahia => String::from("America/Bahia"),
            TimeZone::AmericaBahiaBanderas => String::from("America/Bahia_Banderas"),
            TimeZone::AmericaBarbados => String::from("America/Barbados"),
            TimeZone::AmericaBelem => String::from("America/Belem"),
            TimeZone::AmericaBelize => String::from("America/Belize"),
            TimeZone::AmericaBlancSablon => String::from("America/Blanc-Sablon"),
            TimeZone::AmericaBoaVista => String::from("America/Boa_Vista"),
            TimeZone::AmericaBogota => String::from("America/Bogota"),
            TimeZone::AmericaBoise => String::from("America/Boise"),
            TimeZone::AmericaCambridgeBay => String::from("America/Cambridge_Bay"),
            TimeZone::AmericaCampoGrande => String::from("America/Campo_Grande"),
            TimeZone::AmericaCancun => String::from("America/Cancun"),
            TimeZone::AmericaCaracas => String::from("America/Caracas"),
            TimeZone::AmericaCayenne => String::from("America/Cayenne"),
            TimeZone::AmericaChicago => String::from("America/Chicago"),
            TimeZone::AmericaChihuahua => String::from("America/Chihuahua"),
            TimeZone::AmericaCostaRica => String::from("America/Costa_Rica"),
            TimeZone::AmericaCreston => String::from("America/Creston"),
            TimeZone::AmericaCuiaba => String::from("America/Cuiaba"),
            TimeZone::AmericaCuracao => String::from("America/Curacao"),
            TimeZone::AmericaDanmarkshavn => String::from("America/Danmarkshavn"),
            TimeZone::AmericaDawson => String::from("America/Dawson"),
            TimeZone::AmericaDawsonCreek => String::from("America/Dawson_Creek"),
            TimeZone::AmericaDenver => String::from("America/Denver"),
            TimeZone::AmericaDetroit => String::from("America/Detroit"),
            TimeZone::AmericaEdmonton => String::from("America/Edmonton"),
            TimeZone::AmericaEirunepe => String::from("America/Eirunepe"),
            TimeZone::AmericaElSalvador => String::from("America/El_Salvador"),
            TimeZone::AmericaFortNelson => String::from("America/Fort_Nelson"),
            TimeZone::AmericaFortaleza => String::from("America/Fortaleza"),
            TimeZone::AmericaGlaceBay => String::from("America/Glace_Bay"),
            TimeZone::AmericaGodthab => String::from("America/Godthab"),
            TimeZone::AmericaGooseBay => String::from("America/Goose_Bay"),
            TimeZone::AmericaGrandTurk => String::from("America/Grand_Turk"),
            TimeZone::AmericaGuatemala => String::from("America/Guatemala"),
            TimeZone::AmericaGuayaquil => String::from("America/Guayaquil"),
            TimeZone::AmericaGuyana => String::from("America/Guyana"),
            TimeZone::AmericaHalifax => String::from("America/Halifax"),
            TimeZone::AmericaHavana => String::from("America/Havana"),
            TimeZone::AmericaHermosillo => String::from("America/Hermosillo"),
            TimeZone::AmericaIndianaIndianapolis => String::from("America/Indiana/Indianapolis"),
            TimeZone::AmericaIndianaKnox => String::from("America/Indiana/Knox"),
            TimeZone::AmericaIndianaMarengo => String::from("America/Indiana/Marengo"),
            TimeZone::AmericaIndianaPetersburg => String::from("America/Indiana/Petersburg"),
            TimeZone::AmericaIndianaTellCity => String::from("America/Indiana/Tell_City"),
            TimeZone::AmericaIndianaVevay => String::from("America/Indiana/Vevay"),
            TimeZone::AmericaIndianaVincennes => String::from("America/Indiana/Vincennes"),
            TimeZone::AmericaIndianaWinamac => String::from("America/Indiana/Winamac"),
            TimeZone::AmericaInuvik => String::from("America/Inuvik"),
            TimeZone::AmericaIqaluit => String::from("America/Iqaluit"),
            TimeZone::AmericaJamaica => String::from("America/Jamaica"),
            TimeZone::AmericaJuneau => String::from("America/Juneau"),
            TimeZone::AmericaKentuckyLouisville => String::from("America/Kentucky/Louisville"),
            TimeZone::AmericaKentuckyMonticello => String::from("America/Kentucky/Monticello"),
            TimeZone::AmericaLaPaz => String::from("America/La_Paz"),
            TimeZone::AmericaLima => String::from("America/Lima"),
            TimeZone::AmericaLosAngeles => String::from("America/Los_Angeles"),
            TimeZone::AmericaMaceio => String::from("America/Maceio"),
            TimeZone::AmericaManagua => String::from("America/Managua"),
            TimeZone::AmericaManaus => String::from("America/Manaus"),
            TimeZone::AmericaMartinique => String::from("America/Martinique"),
            TimeZone::AmericaMatamoros => String::from("America/Matamoros"),
            TimeZone::AmericaMazatlan => String::from("America/Mazatlan"),
            TimeZone::AmericaMenominee => String::from("America/Menominee"),
            TimeZone::AmericaMerida => String::from("America/Merida"),
            TimeZone::AmericaMetlakatla => String::from("America/Metlakatla"),
            TimeZone::AmericaMexicoCity => String::from("America/Mexico_City"),
            TimeZone::AmericaMiquelon => String::from("America/Miquelon"),
            TimeZone::AmericaMoncton => String::from("America/Moncton"),
            TimeZone::AmericaMonterrey => String::from("America/Monterrey"),
            TimeZone::AmericaMontevideo => String::from("America/Montevideo"),
            TimeZone::AmericaNassau => String::from("America/Nassau"),
            TimeZone::AmericaNewYork => String::from("America/New_York"),
            TimeZone::AmericaNipigon => String::from("America/Nipigon"),
            TimeZone::AmericaNome => String::from("America/Nome"),
            TimeZone::AmericaNoronha => String::from("America/Noronha"),
            TimeZone::AmericaNorthDakotaBeulah => String::from("America/North_Dakota/Beulah"),
            TimeZone::AmericaNorthDakotaCenter => String::from("America/North_Dakota/Center"),
            TimeZone::AmericaNorthDakotaNewSalem => String::from("America/North_Dakota/New_Salem"),
            TimeZone::AmericaOjinaga => String::from("America/Ojinaga"),
            TimeZone::AmericaPanama => String::from("America/Panama"),
            TimeZone::AmericaPangnirtung => String::from("America/Pangnirtung"),
            TimeZone::AmericaParamaribo => String::from("America/Paramaribo"),
            TimeZone::AmericaPhoenix => String::from("America/Phoenix"),
            TimeZone::AmericaPortAuPrince => String::from("America/Port-au-Prince"),
            TimeZone::AmericaPortOfSpain => String::from("America/Port_of_Spain"),
            TimeZone::AmericaPortoVelho => String::from("America/Porto_Velho"),
            TimeZone::AmericaPuertoRico => String::from("America/Puerto_Rico"),
            TimeZone::AmericaPuntaArenas => String::from("America/Punta_Arenas"),
            TimeZone::AmericaRainyRiver => String::from("America/Rainy_River"),
            TimeZone::AmericaRankinInlet => String::from("America/Rankin_Inlet"),
            TimeZone::AmericaRecife => String::from("America/Recife"),
            TimeZone::AmericaRegina => String::from("America/Regina"),
            TimeZone::AmericaResolute => String::from("America/Resolute"),
            TimeZone::AmericaRioBranco => String::from("America/Rio_Branco"),
            TimeZone::AmericaSantarem => String::from("America/Santarem"),
            TimeZone::AmericaSantiago => String::from("America/Santiago"),
            TimeZone::AmericaSantoDomingo => String::from("America/Santo_Domingo"),
            TimeZone::AmericaSaoPaulo => String::from("America/Sao_Paulo"),
            TimeZone::AmericaScoresbysund => String::from("America/Scoresbysund"),
            TimeZone::AmericaSitka => String::from("America/Sitka"),
            TimeZone::AmericaStJohns => String::from("America/St_Johns"),
            TimeZone::AmericaSwiftCurrent => String::from("America/Swift_Current"),
            TimeZone::AmericaTegucigalpa => String::from("America/Tegucigalpa"),
            TimeZone::AmericaThule => String::from("America/Thule"),
            TimeZone::AmericaThunderBay => String::from("America/Thunder_Bay"),
            TimeZone::AmericaTijuana => String::from("America/Tijuana"),
            TimeZone::AmericaToronto => String::from("America/Toronto"),
            TimeZone::AmericaVancouver => String::from("America/Vancouver"),
            TimeZone::AmericaWhitehorse => String::from("America/Whitehorse"),
            TimeZone::AmericaWinnipeg => String::from("America/Winnipeg"),
            TimeZone::AmericaYakutat => String::from("America/Yakutat"),
            TimeZone::AmericaYellowknife => String::from("America/Yellowknife"),
            TimeZone::AntarcticaCasey => String::from("Antarctica/Casey"),
            TimeZone::AntarcticaDavis => String::from("Antarctica/Davis"),
            TimeZone::AntarcticaDumontDUrville => String::from("Antarctica/DumontDUrville"),
            TimeZone::AntarcticaMacquarie => String::from("Antarctica/Macquarie"),
            TimeZone::AntarcticaMawson => String::from("Antarctica/Mawson"),
            TimeZone::AntarcticaPalmer => String::from("Antarctica/Palmer"),
            TimeZone::AntarcticaRothera => String::from("Antarctica/Rothera"),
            TimeZone::AntarcticaSyowa => String::from("Antarctica/Syowa"),
            TimeZone::AntarcticaTroll => String::from("Antarctica/Troll"),
            TimeZone::AntarcticaVostok => String::from("Antarctica/Vostok"),
            TimeZone::AsiaAlmaty => String::from("Asia/Almaty"),
            TimeZone::AsiaAmman => String::from("Asia/Amman"),
            TimeZone::AsiaAnadyr => String::from("Asia/Anadyr"),
            TimeZone::AsiaAqtau => String::from("Asia/Aqtau"),
            TimeZone::AsiaAqtobe => String::from("Asia/Aqtobe"),
            TimeZone::AsiaAshgabat => String::from("Asia/Ashgabat"),
            TimeZone::AsiaAtyrau => String::from("Asia/Atyrau"),
            TimeZone::AsiaBaghdad => String::from("Asia/Baghdad"),
            TimeZone::AsiaBaku => String::from("Asia/Baku"),
            TimeZone::AsiaBangkok => String::from("Asia/Bangkok"),
            TimeZone::AsiaBarnaul => String::from("Asia/Barnaul"),
            TimeZone::AsiaBeirut => String::from("Asia/Beirut"),
            TimeZone::AsiaBishkek => String::from("Asia/Bishkek"),
            TimeZone::AsiaBrunei => String::from("Asia/Brunei"),
            TimeZone::AsiaChita => String::from("Asia/Chita"),
            TimeZone::AsiaChoibalsan => String::from("Asia/Choibalsan"),
            TimeZone::AsiaColombo => String::from("Asia/Colombo"),
            TimeZone::AsiaDamascus => String::from("Asia/Damascus"),
            TimeZone::AsiaDhaka => String::from("Asia/Dhaka"),
            TimeZone::AsiaDili => String::from("Asia/Dili"),
            TimeZone::AsiaDubai => String::from("Asia/Dubai"),
            TimeZone::AsiaDushanbe => String::from("Asia/Dushanbe"),
            TimeZone::AsiaFamagusta => String::from("Asia/Famagusta"),
            TimeZone::AsiaGaza => String::from("Asia/Gaza"),
            TimeZone::AsiaHebron => String::from("Asia/Hebron"),
            TimeZone::AsiaHoChiMinh => String::from("Asia/Ho_Chi_Minh"),
            TimeZone::AsiaHongKong => String::from("Asia/Hong_Kong"),
            TimeZone::AsiaHovd => String::from("Asia/Hovd"),
            TimeZone::AsiaIrkutsk => String::from("Asia/Irkutsk"),
            TimeZone::AsiaJakarta => String::from("Asia/Jakarta"),
            TimeZone::AsiaJayapura => String::from("Asia/Jayapura"),
            TimeZone::AsiaJerusalem => String::from("Asia/Jerusalem"),
            TimeZone::AsiaKabul => String::from("Asia/Kabul"),
            TimeZone::AsiaKamchatka => String::from("Asia/Kamchatka"),
            TimeZone::AsiaKarachi => String::from("Asia/Karachi"),
            TimeZone::AsiaKathmandu => String::from("Asia/Kathmandu"),
            TimeZone::AsiaKhandyga => String::from("Asia/Khandyga"),
            TimeZone::AsiaKolkata => String::from("Asia/Kolkata"),
            TimeZone::AsiaKrasnoyarsk => String::from("Asia/Krasnoyarsk"),
            TimeZone::AsiaKualaLumpur => String::from("Asia/Kuala_Lumpur"),
            TimeZone::AsiaKuching => String::from("Asia/Kuching"),
            TimeZone::AsiaMacau => String::from("Asia/Macau"),
            TimeZone::AsiaMagadan => String::from("Asia/Magadan"),
            TimeZone::AsiaMakassar => String::from("Asia/Makassar"),
            TimeZone::AsiaManila => String::from("Asia/Manila"),
            TimeZone::AsiaNicosia => String::from("Asia/Nicosia"),
            TimeZone::AsiaNovokuznetsk => String::from("Asia/Novokuznetsk"),
            TimeZone::AsiaNovosibirsk => String::from("Asia/Novosibirsk"),
            TimeZone::AsiaOmsk => String::from("Asia/Omsk"),
            TimeZone::AsiaOral => String::from("Asia/Oral"),
            TimeZone::AsiaPontianak => String::from("Asia/Pontianak"),
            TimeZone::AsiaPyongyang => String::from("Asia/Pyongyang"),
            TimeZone::AsiaQatar => String::from("Asia/Qatar"),
            TimeZone::AsiaQostanay => String::from("Asia/Qostanay"),
            TimeZone::AsiaQyzylorda => String::from("Asia/Qyzylorda"),
            TimeZone::AsiaRiyadh => String::from("Asia/Riyadh"),
            TimeZone::AsiaSakhalin => String::from("Asia/Sakhalin"),
            TimeZone::AsiaSamarkand => String::from("Asia/Samarkand"),
            TimeZone::AsiaSeoul => String::from("Asia/Seoul"),
            TimeZone::AsiaShanghai => String::from("Asia/Shanghai"),
            TimeZone::AsiaSingapore => String::from("Asia/Singapore"),
            TimeZone::AsiaSrednekolymsk => String::from("Asia/Srednekolymsk"),
            TimeZone::AsiaTaipei => String::from("Asia/Taipei"),
            TimeZone::AsiaTashkent => String::from("Asia/Tashkent"),
            TimeZone::AsiaTbilisi => String::from("Asia/Tbilisi"),
            TimeZone::AsiaTehran => String::from("Asia/Tehran"),
            TimeZone::AsiaThimphu => String::from("Asia/Thimphu"),
            TimeZone::AsiaTokyo => String::from("Asia/Tokyo"),
            TimeZone::AsiaTomsk => String::from("Asia/Tomsk"),
            TimeZone::AsiaUlaanbaatar => String::from("Asia/Ulaanbaatar"),
            TimeZone::AsiaUrumqi => String::from("Asia/Urumqi"),
            TimeZone::AsiaUstNera => String::from("Asia/Ust-Nera"),
            TimeZone::AsiaVladivostok => String::from("Asia/Vladivostok"),
            TimeZone::AsiaYakutsk => String::from("Asia/Yakutsk"),
            TimeZone::AsiaYangon => String::from("Asia/Yangon"),
            TimeZone::AsiaYekaterinburg => String::from("Asia/Yekaterinburg"),
            TimeZone::AsiaYerevan => String::from("Asia/Yerevan"),
            TimeZone::AtlanticAzores => String::from("Atlantic/Azores"),
            TimeZone::AtlanticBermuda => String::from("Atlantic/Bermuda"),
            TimeZone::AtlanticCanary => String::from("Atlantic/Canary"),
            TimeZone::AtlanticCapeVerde => String::from("Atlantic/Cape_Verde"),
            TimeZone::AtlanticFaroe => String::from("Atlantic/Faroe"),
            TimeZone::AtlanticMadeira => String::from("Atlantic/Madeira"),
            TimeZone::AtlanticReykjavik => String::from("Atlantic/Reykjavik"),
            TimeZone::AtlanticSouthGeorgia => String::from("Atlantic/South_Georgia"),
            TimeZone::AtlanticStanley => String::from("Atlantic/Stanley"),
            TimeZone::AustraliaAdelaide => String::from("Australia/Adelaide"),
            TimeZone::AustraliaBrisbane => String::from("Australia/Brisbane"),
            TimeZone::AustraliaBrokenHill => String::from("Australia/Broken_Hill"),
            TimeZone::AustraliaCurrie => String::from("Australia/Currie"),
            TimeZone::AustraliaDarwin => String::from("Australia/Darwin"),
            TimeZone::AustraliaEucla => String::from("Australia/Eucla"),
            TimeZone::AustraliaHobart => String::from("Australia/Hobart"),
            TimeZone::AustraliaLindeman => String::from("Australia/Lindeman"),
            TimeZone::AustraliaLordHowe => String::from("Australia/Lord_Howe"),
            TimeZone::AustraliaMelbourne => String::from("Australia/Melbourne"),
            TimeZone::AustraliaPerth => String::from("Australia/Perth"),
            TimeZone::AustraliaSydney => String::from("Australia/Sydney"),
            TimeZone::EuropeAmsterdam => String::from("Europe/Amsterdam"),
            TimeZone::EuropeAndorra => String::from("Europe/Andorra"),
            TimeZone::EuropeAstrakhan => String::from("Europe/Astrakhan"),
            TimeZone::EuropeAthens => String::from("Europe/Athens"),
            TimeZone::EuropeBelgrade => String::from("Europe/Belgrade"),
            TimeZone::EuropeBerlin => String::from("Europe/Berlin"),
            TimeZone::EuropeBrussels => String::from("Europe/Brussels"),
            TimeZone::EuropeBucharest => String::from("Europe/Bucharest"),
            TimeZone::EuropeBudapest => String::from("Europe/Budapest"),
            TimeZone::EuropeChisinau => String::from("Europe/Chisinau"),
            TimeZone::EuropeCopenhagen => String::from("Europe/Copenhagen"),
            TimeZone::EuropeDublin => String::from("Europe/Dublin"),
            TimeZone::EuropeGibraltar => String::from("Europe/Gibraltar"),
            TimeZone::EuropeHelsinki => String::from("Europe/Helsinki"),
            TimeZone::EuropeIstanbul => String::from("Europe/Istanbul"),
            TimeZone::EuropeKaliningrad => String::from("Europe/Kaliningrad"),
            TimeZone::EuropeKiev => String::from("Europe/Kiev"),
            TimeZone::EuropeKirov => String::from("Europe/Kirov"),
            TimeZone::EuropeLisbon => String::from("Europe/Lisbon"),
            TimeZone::EuropeLondon => String::from("Europe/London"),
            TimeZone::EuropeLuxembourg => String::from("Europe/Luxembourg"),
            TimeZone::EuropeMadrid => String::from("Europe/Madrid"),
            TimeZone::EuropeMalta => String::from("Europe/Malta"),
            TimeZone::EuropeMinsk => String::from("Europe/Minsk"),
            TimeZone::EuropeMonaco => String::from("Europe/Monaco"),
            TimeZone::EuropeMoscow => String::from("Europe/Moscow"),
            TimeZone::EuropeOslo => String::from("Europe/Oslo"),
            TimeZone::EuropeParis => String::from("Europe/Paris"),
            TimeZone::EuropePrague => String::from("Europe/Prague"),
            TimeZone::EuropeRiga => String::from("Europe/Riga"),
            TimeZone::EuropeRome => String::from("Europe/Rome"),
            TimeZone::EuropeSamara => String::from("Europe/Samara"),
            TimeZone::EuropeSaratov => String::from("Europe/Saratov"),
            TimeZone::EuropeSimferopol => String::from("Europe/Simferopol"),
            TimeZone::EuropeSofia => String::from("Europe/Sofia"),
            TimeZone::EuropeStockholm => String::from("Europe/Stockholm"),
            TimeZone::EuropeTallinn => String::from("Europe/Tallinn"),
            TimeZone::EuropeTirane => String::from("Europe/Tirane"),
            TimeZone::EuropeUlyanovsk => String::from("Europe/Ulyanovsk"),
            TimeZone::EuropeUzhgorod => String::from("Europe/Uzhgorod"),
            TimeZone::EuropeVienna => String::from("Europe/Vienna"),
            TimeZone::EuropeVilnius => String::from("Europe/Vilnius"),
            TimeZone::EuropeVolgograd => String::from("Europe/Volgograd"),
            TimeZone::EuropeWarsaw => String::from("Europe/Warsaw"),
            TimeZone::EuropeZaporozhye => String::from("Europe/Zaporozhye"),
            TimeZone::EuropeZurich => String::from("Europe/Zurich"),
            TimeZone::IndianChagos => String::from("Indian/Chagos"),
            TimeZone::IndianChristmas => String::from("Indian/Christmas"),
            TimeZone::IndianCocos => String::from("Indian/Cocos"),
            TimeZone::IndianKerguelen => String::from("Indian/Kerguelen"),
            TimeZone::IndianMahe => String::from("Indian/Mahe"),
            TimeZone::IndianMaldives => String::from("Indian/Maldives"),
            TimeZone::IndianMauritius => String::from("Indian/Mauritius"),
            TimeZone::IndianReunion => String::from("Indian/Reunion"),
            TimeZone::PacificApia => String::from("Pacific/Apia"),
            TimeZone::PacificAuckland => String::from("Pacific/Auckland"),
            TimeZone::PacificBougainville => String::from("Pacific/Bougainville"),
            TimeZone::PacificChatham => String::from("Pacific/Chatham"),
            TimeZone::PacificChuuk => String::from("Pacific/Chuuk"),
            TimeZone::PacificEaster => String::from("Pacific/Easter"),
            TimeZone::PacificEfate => String::from("Pacific/Efate"),
            TimeZone::PacificEnderbury => String::from("Pacific/Enderbury"),
            TimeZone::PacificFakaofo => String::from("Pacific/Fakaofo"),
            TimeZone::PacificFiji => String::from("Pacific/Fiji"),
            TimeZone::PacificFunafuti => String::from("Pacific/Funafuti"),
            TimeZone::PacificGalapagos => String::from("Pacific/Galapagos"),
            TimeZone::PacificGambier => String::from("Pacific/Gambier"),
            TimeZone::PacificGuadalcanal => String::from("Pacific/Guadalcanal"),
            TimeZone::PacificGuam => String::from("Pacific/Guam"),
            TimeZone::PacificHonolulu => String::from("Pacific/Honolulu"),
            TimeZone::PacificKiritimati => String::from("Pacific/Kiritimati"),
            TimeZone::PacificKosrae => String::from("Pacific/Kosrae"),
            TimeZone::PacificKwajalein => String::from("Pacific/Kwajalein"),
            TimeZone::PacificMajuro => String::from("Pacific/Majuro"),
            TimeZone::PacificMarquesas => String::from("Pacific/Marquesas"),
            TimeZone::PacificNauru => String::from("Pacific/Nauru"),
            TimeZone::PacificNiue => String::from("Pacific/Niue"),
            TimeZone::PacificNorfolk => String::from("Pacific/Norfolk"),
            TimeZone::PacificNoumea => String::from("Pacific/Noumea"),
            TimeZone::PacificPagoPago => String::from("Pacific/Pago_Pago"),
            TimeZone::PacificPalau => String::from("Pacific/Palau"),
            TimeZone::PacificPitcairn => String::from("Pacific/Pitcairn"),
            TimeZone::PacificPohnpei => String::from("Pacific/Pohnpei"),
            TimeZone::PacificPortMoresby => String::from("Pacific/Port_Moresby"),
            TimeZone::PacificRarotonga => String::from("Pacific/Rarotonga"),
            TimeZone::PacificTahiti => String::from("Pacific/Tahiti"),
            TimeZone::PacificTarawa => String::from("Pacific/Tarawa"),
            TimeZone::PacificTongatapu => String::from("Pacific/Tongatapu"),
            TimeZone::PacificWake => String::from("Pacific/Wake"),
            TimeZone::PacificWallis => String::from("Pacific/Wallis"),
        } // match
    } // fn
} // impl

impl std::convert::TryFrom<String> for TimeZone {

    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;

    /// Gets a `TimeZone` enum from a `String` that contains a valid [time
    /// zone](https://www.iana.org/time-zones) name.
    fn try_from(time_zone: String) -> Result<TimeZone, Error> {
        match time_zone.as_ref() {
            "Africa/Abidjan" => Ok(TimeZone::AfricaAbidjan),
            "Africa/Accra" => Ok(TimeZone::AfricaAccra),
            "Africa/Algiers" => Ok(TimeZone::AfricaAlgiers),
            "Africa/Bissau" => Ok(TimeZone::AfricaBissau),
            "Africa/Cairo" => Ok(TimeZone::AfricaCairo),
            "Africa/Casablanca" => Ok(TimeZone::AfricaCasablanca),
            "Africa/Ceuta" => Ok(TimeZone::AfricaCeuta),
            "Africa/El_Aaiun" => Ok(TimeZone::AfricaElAaiun),
            "Africa/Johannesburg" => Ok(TimeZone::AfricaJohannesburg),
            "Africa/Juba" => Ok(TimeZone::AfricaJuba),
            "Africa/Khartoum" => Ok(TimeZone::AfricaKhartoum),
            "Africa/Lagos" => Ok(TimeZone::AfricaLagos),
            "Africa/Maputo" => Ok(TimeZone::AfricaMaputo),
            "Africa/Monrovia" => Ok(TimeZone::AfricaMonrovia),
            "Africa/Nairobi" => Ok(TimeZone::AfricaNairobi),
            "Africa/Ndjamena" => Ok(TimeZone::AfricaNdjamena),
            "Africa/Sao_Tome" => Ok(TimeZone::AfricaSaoTome),
            "Africa/Tripoli" => Ok(TimeZone::AfricaTripoli),
            "Africa/Tunis" => Ok(TimeZone::AfricaTunis),
            "Africa/Windhoek" => Ok(TimeZone::AfricaWindhoek),
            "America/Adak" => Ok(TimeZone::AmericaAdak),
            "America/Anchorage" => Ok(TimeZone::AmericaAnchorage),
            "America/Araguaina" => Ok(TimeZone::AmericaAraguaina),
            "America/Argentina/Buenos_Aires" => Ok(TimeZone::AmericaArgentinaBuenosAires),
            "America/Argentina/Catamarca" => Ok(TimeZone::AmericaArgentinaCatamarca),
            "America/Argentina/Cordoba" => Ok(TimeZone::AmericaArgentinaCordoba),
            "America/Argentina/Jujuy" => Ok(TimeZone::AmericaArgentinaJujuy),
            "America/Argentina/La_Rioja" => Ok(TimeZone::AmericaArgentinaLaRioja),
            "America/Argentina/Mendoza" => Ok(TimeZone::AmericaArgentinaMendoza),
            "America/Argentina/Rio_Gallegos" => Ok(TimeZone::AmericaArgentinaRioGallegos),
            "AmAmerica/Argentina/Salta" => Ok(TimeZone::AmericaArgentinaSalta),
            "America/Argentina/San_Juan" => Ok(TimeZone::AmericaArgentinaSanJuan),
            "America/Argentina/San_Luis" => Ok(TimeZone::AmericaArgentinaSanLuis),
            "America/Argentina/Tucuman" => Ok(TimeZone::AmericaArgentinaTucuman),
            "America/Argentina/Ushuaia" => Ok(TimeZone::AmericaArgentinaUshuaia),
            "America/Asuncion" => Ok(TimeZone::AmericaAsuncion),
            "America/Atikokan" => Ok(TimeZone::AmericaAtikokan),
            "America/Bahia" => Ok(TimeZone::AmericaBahia),
            "America/Bahia_Banderas" => Ok(TimeZone::AmericaBahiaBanderas),
            "America/Barbados" => Ok(TimeZone::AmericaBarbados),
            "America/Belem" => Ok(TimeZone::AmericaBelem),
            "America/Belize" => Ok(TimeZone::AmericaBelize),
            "America/Blanc-Sablon" => Ok(TimeZone::AmericaBlancSablon),
            "America/Boa_Vista" => Ok(TimeZone::AmericaBoaVista),
            "America/Bogota" => Ok(TimeZone::AmericaBogota),
            "America/Boise" => Ok(TimeZone::AmericaBoise),
            "America/Cambridge_Bay" => Ok(TimeZone::AmericaCambridgeBay),
            "America/Campo_Grande" => Ok(TimeZone::AmericaCampoGrande),
            "America/Cancun" => Ok(TimeZone::AmericaCancun),
            "America/Caracas" => Ok(TimeZone::AmericaCaracas),
            "America/Cayenne" => Ok(TimeZone::AmericaCayenne),
            "America/Chicago" => Ok(TimeZone::AmericaChicago),
            "America/Chihuahua" => Ok(TimeZone::AmericaChihuahua),
            "America/Costa_Rica" => Ok(TimeZone::AmericaCostaRica),
            "America/Creston" => Ok(TimeZone::AmericaCreston),
            "America/Cuiaba" => Ok(TimeZone::AmericaCuiaba),
            "America/Curacao" => Ok(TimeZone::AmericaCuracao),
            "America/Danmarkshavn" => Ok(TimeZone::AmericaDanmarkshavn),
            "America/Dawson" => Ok(TimeZone::AmericaDawson),
            "America/Dawson_Creek" => Ok(TimeZone::AmericaDawsonCreek),
            "America/Denver" => Ok(TimeZone::AmericaDenver),
            "America/Detroit" => Ok(TimeZone::AmericaDetroit),
            "America/Edmonton" => Ok(TimeZone::AmericaEdmonton),
            "America/Eirunepe" => Ok(TimeZone::AmericaEirunepe),
            "America/El_Salvador" => Ok(TimeZone::AmericaElSalvador),
            "America/Fort_Nelson" => Ok(TimeZone::AmericaFortNelson),
            "America/Fortaleza" => Ok(TimeZone::AmericaFortaleza),
            "America/Glace_Bay" => Ok(TimeZone::AmericaGlaceBay),
            "America/Godthab" => Ok(TimeZone::AmericaGodthab),
            "America/Goose_Bay" => Ok(TimeZone::AmericaGooseBay),
            "America/Grand_Turk" => Ok(TimeZone::AmericaGrandTurk),
            "America/Guatemala" => Ok(TimeZone::AmericaGuatemala),
            "America/Guayaquil" => Ok(TimeZone::AmericaGuayaquil),
            "America/Guyana" => Ok(TimeZone::AmericaGuyana),
            "America/Halifax" => Ok(TimeZone::AmericaHalifax),
            "America/Havana" => Ok(TimeZone::AmericaHavana),
            "America/Hermosillo" => Ok(TimeZone::AmericaHermosillo),
            "America/Indiana/Indianapolis" => Ok(TimeZone::AmericaIndianaIndianapolis),
            "America/Indiana/Knox" => Ok(TimeZone::AmericaIndianaKnox),
            "America/Indiana/Marengo" => Ok(TimeZone::AmericaIndianaMarengo),
            "America/Indiana/Petersburg" => Ok(TimeZone::AmericaIndianaPetersburg),
            "America/Indiana/Tell_City" => Ok(TimeZone::AmericaIndianaTellCity),
            "America/Indiana/Vevay" => Ok(TimeZone::AmericaIndianaVevay),
            "America/Indiana/Vincennes" => Ok(TimeZone::AmericaIndianaVincennes),
            "America/Indiana/Winamac" => Ok(TimeZone::AmericaIndianaWinamac),
            "America/Inuvik" => Ok(TimeZone::AmericaInuvik),
            "America/Iqaluit" => Ok(TimeZone::AmericaIqaluit),
            "America/Jamaica" => Ok(TimeZone::AmericaJamaica),
            "America/Juneau" => Ok(TimeZone::AmericaJuneau),
            "America/Kentucky/Louisville" => Ok(TimeZone::AmericaKentuckyLouisville),
            "America/Kentucky/Monticello" => Ok(TimeZone::AmericaKentuckyMonticello),
            "America/La_Paz" => Ok(TimeZone::AmericaLaPaz),
            "America/Lima" => Ok(TimeZone::AmericaLima),
            "America/Los_Angeles" => Ok(TimeZone::AmericaLosAngeles),
            "America/Maceio" => Ok(TimeZone::AmericaMaceio),
            "America/Managua" => Ok(TimeZone::AmericaManagua),
            "America/Manaus" => Ok(TimeZone::AmericaManaus),
            "America/Martinique" => Ok(TimeZone::AmericaMartinique),
            "America/Matamoros" => Ok(TimeZone::AmericaMatamoros),
            "America/Mazatlan" => Ok(TimeZone::AmericaMazatlan),
            "America/Menominee" => Ok(TimeZone::AmericaMenominee),
            "America/Merida" => Ok(TimeZone::AmericaMerida),
            "America/Metlakatla" => Ok(TimeZone::AmericaMetlakatla),
            "America/Mexico_City" => Ok(TimeZone::AmericaMexicoCity),
            "America/Miquelon" => Ok(TimeZone::AmericaMiquelon),
            "America/Moncton" => Ok(TimeZone::AmericaMoncton),
            "America/Monterrey" => Ok(TimeZone::AmericaMonterrey),
            "America/Montevideo" => Ok(TimeZone::AmericaMontevideo),
            "America/Nassau" => Ok(TimeZone::AmericaNassau),
            "America/New_York" => Ok(TimeZone::AmericaNewYork),
            "America/Nipigon" => Ok(TimeZone::AmericaNipigon),
            "America/Nome" => Ok(TimeZone::AmericaNome),
            "America/Noronha" => Ok(TimeZone::AmericaNoronha),
            "America/North_Dakota/Beulah" => Ok(TimeZone::AmericaNorthDakotaBeulah),
            "America/North_Dakota/Center" => Ok(TimeZone::AmericaNorthDakotaCenter),
            "America/North_Dakota/New_Salem" => Ok(TimeZone::AmericaNorthDakotaNewSalem),
            "America/Ojinaga" => Ok(TimeZone::AmericaOjinaga),
            "America/Panama" => Ok(TimeZone::AmericaPanama),
            "America/Pangnirtung" => Ok(TimeZone::AmericaPangnirtung),
            "America/Paramaribo" => Ok(TimeZone::AmericaParamaribo),
            "America/Phoenix" => Ok(TimeZone::AmericaPhoenix),
            "America/Port-au-Prince" => Ok(TimeZone::AmericaPortAuPrince),
            "America/Port_of_Spain" => Ok(TimeZone::AmericaPortOfSpain),
            "America/Porto_Velho" => Ok(TimeZone::AmericaPortoVelho),
            "America/Puerto_Rico" => Ok(TimeZone::AmericaPuertoRico),
            "America/Punta_Arenas" => Ok(TimeZone::AmericaPuntaArenas),
            "America/Rainy_River" => Ok(TimeZone::AmericaRainyRiver),
            "America/Rankin_Inlet" => Ok(TimeZone::AmericaRankinInlet),
            "America/Recife" => Ok(TimeZone::AmericaRecife),
            "America/Regina" => Ok(TimeZone::AmericaRegina),
            "America/Resolute" => Ok(TimeZone::AmericaResolute),
            "America/Rio_Branco" => Ok(TimeZone::AmericaRioBranco),
            "America/Santarem" => Ok(TimeZone::AmericaSantarem),
            "America/Santiago" => Ok(TimeZone::AmericaSantiago),
            "America/Santo_Domingo" => Ok(TimeZone::AmericaSantoDomingo),
            "America/Sao_Paulo" => Ok(TimeZone::AmericaSaoPaulo),
            "America/Scoresbysund" => Ok(TimeZone::AmericaScoresbysund),
            "America/Sitka" => Ok(TimeZone::AmericaSitka),
            "America/St_Johns" => Ok(TimeZone::AmericaStJohns),
            "America/Swift_Current" => Ok(TimeZone::AmericaSwiftCurrent),
            "America/Tegucigalpa" => Ok(TimeZone::AmericaTegucigalpa),
            "America/Thule" => Ok(TimeZone::AmericaThule),
            "America/Thunder_Bay" => Ok(TimeZone::AmericaThunderBay),
            "America/Tijuana" => Ok(TimeZone::AmericaTijuana),
            "America/Toronto" => Ok(TimeZone::AmericaToronto),
            "America/Vancouver" => Ok(TimeZone::AmericaVancouver),
            "America/Whitehorse" => Ok(TimeZone::AmericaWhitehorse),
            "America/Winnipeg" => Ok(TimeZone::AmericaWinnipeg),
            "America/Yakutat" => Ok(TimeZone::AmericaYakutat),
            "America/Yellowknife" => Ok(TimeZone::AmericaYellowknife),
            "Antarctica/Casey" => Ok(TimeZone::AntarcticaCasey),
            "Antarctica/Davis" => Ok(TimeZone::AntarcticaDavis),
            "Antarctica/DumontDUrville" => Ok(TimeZone::AntarcticaDumontDUrville),
            "Antarctica/Macquarie" => Ok(TimeZone::AntarcticaMacquarie),
            "Antarctica/Mawson" => Ok(TimeZone::AntarcticaMawson),
            "Antarctica/Palmer" => Ok(TimeZone::AntarcticaPalmer),
            "Antarctica/Rothera" => Ok(TimeZone::AntarcticaRothera),
            "Antarctica/Syowa" => Ok(TimeZone::AntarcticaSyowa),
            "Antarctica/Troll" => Ok(TimeZone::AntarcticaTroll),
            "Antarctica/Vostok" => Ok(TimeZone::AntarcticaVostok),
            "Asia/Almaty" => Ok(TimeZone::AsiaAlmaty),
            "Asia/Amman" => Ok(TimeZone::AsiaAmman),
            "Asia/Anadyr" => Ok(TimeZone::AsiaAnadyr),
            "Asia/Aqtau" => Ok(TimeZone::AsiaAqtau),
            "Asia/Aqtobe" => Ok(TimeZone::AsiaAqtobe),
            "Asia/Ashgabat" => Ok(TimeZone::AsiaAshgabat),
            "Asia/Atyrau" => Ok(TimeZone::AsiaAtyrau),
            "Asia/Baghdad" => Ok(TimeZone::AsiaBaghdad),
            "Asia/Baku" => Ok(TimeZone::AsiaBaku),
            "Asia/Bangkok" => Ok(TimeZone::AsiaBangkok),
            "Asia/Barnaul" => Ok(TimeZone::AsiaBarnaul),
            "Asia/Beirut" => Ok(TimeZone::AsiaBeirut),
            "Asia/Bishkek" => Ok(TimeZone::AsiaBishkek),
            "Asia/Brunei" => Ok(TimeZone::AsiaBrunei),
            "Asia/Chita" => Ok(TimeZone::AsiaChita),
            "Asia/Choibalsan" => Ok(TimeZone::AsiaChoibalsan),
            "Asia/Colombo" => Ok(TimeZone::AsiaColombo),
            "Asia/Damascus" => Ok(TimeZone::AsiaDamascus),
            "Asia/Dhaka" => Ok(TimeZone::AsiaDhaka),
            "Asia/Dili" => Ok(TimeZone::AsiaDili),
            "Asia/Dubai" => Ok(TimeZone::AsiaDubai),
            "Asia/Dushanbe" => Ok(TimeZone::AsiaDushanbe),
            "Asia/Famagusta" => Ok(TimeZone::AsiaFamagusta),
            "Asia/Gaza" => Ok(TimeZone::AsiaGaza),
            "Asia/Hebron" => Ok(TimeZone::AsiaHebron),
            "Asia/Ho_Chi_Minh" => Ok(TimeZone::AsiaHoChiMinh),
            "Asia/Hong_Kong" => Ok(TimeZone::AsiaHongKong),
            "Asia/Hovd" => Ok(TimeZone::AsiaHovd),
            "Asia/Irkutsk" => Ok(TimeZone::AsiaIrkutsk),
            "Asia/Jakarta" => Ok(TimeZone::AsiaJakarta),
            "Asia/Jayapura" => Ok(TimeZone::AsiaJayapura),
            "Asia/Jerusalem" => Ok(TimeZone::AsiaJerusalem),
            "Asia/Kabul" => Ok(TimeZone::AsiaKabul),
            "Asia/Kamchatka" => Ok(TimeZone::AsiaKamchatka),
            "Asia/Karachi" => Ok(TimeZone::AsiaKarachi),
            "Asia/Kathmandu" => Ok(TimeZone::AsiaKathmandu),
            "Asia/Khandyga" => Ok(TimeZone::AsiaKhandyga),
            "Asia/Kolkata" => Ok(TimeZone::AsiaKolkata),
            "Asia/Krasnoyarsk" => Ok(TimeZone::AsiaKrasnoyarsk),
            "Asia/Kuala_Lumpur" => Ok(TimeZone::AsiaKualaLumpur),
            "Asia/Kuching" => Ok(TimeZone::AsiaKuching),
            "Asia/Macau" => Ok(TimeZone::AsiaMacau),
            "Asia/Magadan" => Ok(TimeZone::AsiaMagadan),
            "Asia/Makassar" => Ok(TimeZone::AsiaMakassar),
            "Asia/Manila" => Ok(TimeZone::AsiaManila),
            "Asia/Nicosia" => Ok(TimeZone::AsiaNicosia),
            "Asia/Novokuznetsk" => Ok(TimeZone::AsiaNovokuznetsk),
            "Asia/Novosibirsk" => Ok(TimeZone::AsiaNovosibirsk),
            "Asia/Omsk" => Ok(TimeZone::AsiaOmsk),
            "Asia/Oral" => Ok(TimeZone::AsiaOral),
            "Asia/Pontianak" => Ok(TimeZone::AsiaPontianak),
            "Asia/Pyongyang" => Ok(TimeZone::AsiaPyongyang),
            "Asia/Qatar" => Ok(TimeZone::AsiaQatar),
            "Asia/Qostanay" => Ok(TimeZone::AsiaQostanay),
            "Asia/Qyzylorda" => Ok(TimeZone::AsiaQyzylorda),
            "Asia/Riyadh" => Ok(TimeZone::AsiaRiyadh),
            "Asia/Sakhalin" => Ok(TimeZone::AsiaSakhalin),
            "Asia/Samarkand" => Ok(TimeZone::AsiaSamarkand),
            "Asia/Seoul" => Ok(TimeZone::AsiaSeoul),
            "Asia/Shanghai" => Ok(TimeZone::AsiaShanghai),
            "Asia/Singapore" => Ok(TimeZone::AsiaSingapore),
            "Asia/Srednekolymsk" => Ok(TimeZone::AsiaSrednekolymsk),
            "Asia/Taipei" => Ok(TimeZone::AsiaTaipei),
            "Asia/Tashkent" => Ok(TimeZone::AsiaTashkent),
            "Asia/Tbilisi" => Ok(TimeZone::AsiaTbilisi),
            "Asia/Tehran" => Ok(TimeZone::AsiaTehran),
            "Asia/Thimphu" => Ok(TimeZone::AsiaThimphu),
            "Asia/Tokyo" => Ok(TimeZone::AsiaTokyo),
            "Asia/Tomsk" => Ok(TimeZone::AsiaTomsk),
            "Asia/Ulaanbaatar" => Ok(TimeZone::AsiaUlaanbaatar),
            "Asia/Urumqi" => Ok(TimeZone::AsiaUrumqi),
            "Asia/Ust-Nera" => Ok(TimeZone::AsiaUstNera),
            "Asia/Vladivostok" => Ok(TimeZone::AsiaVladivostok),
            "Asia/Yakutsk" => Ok(TimeZone::AsiaYakutsk),
            "Asia/Yangon" => Ok(TimeZone::AsiaYangon),
            "Asia/Yekaterinburg" => Ok(TimeZone::AsiaYekaterinburg),
            "Asia/Yerevan" => Ok(TimeZone::AsiaYerevan),
            "Atlantic/Azores" => Ok(TimeZone::AtlanticAzores),
            "Atlantic/Bermuda" => Ok(TimeZone::AtlanticBermuda),
            "Atlantic/Canary" => Ok(TimeZone::AtlanticCanary),
            "Atlantic/Cape_Verde" => Ok(TimeZone::AtlanticCapeVerde),
            "Atlantic/Faroe" => Ok(TimeZone::AtlanticFaroe),
            "Atlantic/Madeira" => Ok(TimeZone::AtlanticMadeira),
            "Atlantic/Reykjavik" => Ok(TimeZone::AtlanticReykjavik),
            "Atlantic/South_Georgia" => Ok(TimeZone::AtlanticSouthGeorgia),
            "Atlantic/Stanley" => Ok(TimeZone::AtlanticStanley),
            "Australia/Adelaide" => Ok(TimeZone::AustraliaAdelaide),
            "Australia/Brisbane" => Ok(TimeZone::AustraliaBrisbane),
            "Australia/Broken_Hill" => Ok(TimeZone::AustraliaBrokenHill),
            "Australia/Currie" => Ok(TimeZone::AustraliaCurrie),
            "Australia/Darwin" => Ok(TimeZone::AustraliaDarwin),
            "Australia/Eucla" => Ok(TimeZone::AustraliaEucla),
            "Australia/Hobart" => Ok(TimeZone::AustraliaHobart),
            "Australia/Lindeman" => Ok(TimeZone::AustraliaLindeman),
            "Australia/Lord_Howe" => Ok(TimeZone::AustraliaLordHowe),
            "Australia/Melbourne" => Ok(TimeZone::AustraliaMelbourne),
            "Australia/Perth" => Ok(TimeZone::AustraliaPerth),
            "Australia/Sydney" => Ok(TimeZone::AustraliaSydney),
            "Europe/Amsterdam" => Ok(TimeZone::EuropeAmsterdam),
            "Europe/Andorra" => Ok(TimeZone::EuropeAndorra),
            "Europe/Astrakhan" => Ok(TimeZone::EuropeAstrakhan),
            "Europe/Athens" => Ok(TimeZone::EuropeAthens),
            "Europe/Belgrade" => Ok(TimeZone::EuropeBelgrade),
            "Europe/Berlin" => Ok(TimeZone::EuropeBerlin),
            "Europe/Brussels" => Ok(TimeZone::EuropeBrussels),
            "Europe/Bucharest" => Ok(TimeZone::EuropeBucharest),
            "Europe/Budapest" => Ok(TimeZone::EuropeBudapest),
            "Europe/Chisinau" => Ok(TimeZone::EuropeChisinau),
            "Europe/Copenhagen" => Ok(TimeZone::EuropeCopenhagen),
            "Europe/Dublin" => Ok(TimeZone::EuropeDublin),
            "Europe/Gibraltar" => Ok(TimeZone::EuropeGibraltar),
            "Europe/Helsinki" => Ok(TimeZone::EuropeHelsinki),
            "Europe/Istanbul" => Ok(TimeZone::EuropeIstanbul),
            "Europe/Kaliningrad" => Ok(TimeZone::EuropeKaliningrad),
            "Europe/Kiev" => Ok(TimeZone::EuropeKiev),
            "Europe/Kirov" => Ok(TimeZone::EuropeKirov),
            "Europe/Lisbon" => Ok(TimeZone::EuropeLisbon),
            "Europe/London" => Ok(TimeZone::EuropeLondon),
            "Europe/Luxembourg" => Ok(TimeZone::EuropeLuxembourg),
            "Europe/Madrid" => Ok(TimeZone::EuropeMadrid),
            "Europe/Malta" => Ok(TimeZone::EuropeMalta),
            "Europe/Minsk" => Ok(TimeZone::EuropeMinsk),
            "Europe/Monaco" => Ok(TimeZone::EuropeMonaco),
            "Europe/Moscow" => Ok(TimeZone::EuropeMoscow),
            "Europe/Oslo" => Ok(TimeZone::EuropeOslo),
            "Europe/Paris" => Ok(TimeZone::EuropeParis),
            "Europe/Prague" => Ok(TimeZone::EuropePrague),
            "Europe/Riga" => Ok(TimeZone::EuropeRiga),
            "Europe/Rome" => Ok(TimeZone::EuropeRome),
            "Europe/Samara" => Ok(TimeZone::EuropeSamara),
            "Europe/Saratov" => Ok(TimeZone::EuropeSaratov),
            "Europe/Simferopol" => Ok(TimeZone::EuropeSimferopol),
            "Europe/Sofia" => Ok(TimeZone::EuropeSofia),
            "Europe/Stockholm" => Ok(TimeZone::EuropeStockholm),
            "Europe/Tallinn" => Ok(TimeZone::EuropeTallinn),
            "Europe/Tirane" => Ok(TimeZone::EuropeTirane),
            "Europe/Ulyanovsk" => Ok(TimeZone::EuropeUlyanovsk),
            "Europe/Uzhgorod" => Ok(TimeZone::EuropeUzhgorod),
            "Europe/Vienna" => Ok(TimeZone::EuropeVienna),
            "Europe/Vilnius" => Ok(TimeZone::EuropeVilnius),
            "Europe/Volgograd" => Ok(TimeZone::EuropeVolgograd),
            "Europe/Warsaw" => Ok(TimeZone::EuropeWarsaw),
            "Europe/Zaporozhye" => Ok(TimeZone::EuropeZaporozhye),
            "Europe/Zurich" => Ok(TimeZone::EuropeZurich),
            "Indian/Chagos" => Ok(TimeZone::IndianChagos),
            "Indian/Christmas" => Ok(TimeZone::IndianChristmas),
            "Indian/Cocos" => Ok(TimeZone::IndianCocos),
            "Indian/Kerguelen" => Ok(TimeZone::IndianKerguelen),
            "Indian/Mahe" => Ok(TimeZone::IndianMahe),
            "Indian/Maldives" => Ok(TimeZone::IndianMaldives),
            "Indian/Mauritius" => Ok(TimeZone::IndianMauritius),
            "Indian/Reunion" => Ok(TimeZone::IndianReunion),
            "Pacific/Apia" => Ok(TimeZone::PacificApia),
            "Pacific/Auckland" => Ok(TimeZone::PacificAuckland),
            "Pacific/Bougainville" => Ok(TimeZone::PacificBougainville),
            "Pacific/Chatham" => Ok(TimeZone::PacificChatham),
            "Pacific/Chuuk" => Ok(TimeZone::PacificChuuk),
            "Pacific/Easter" => Ok(TimeZone::PacificEaster),
            "Pacific/Efate" => Ok(TimeZone::PacificEfate),
            "Pacific/Enderbury" => Ok(TimeZone::PacificEnderbury),
            "Pacific/Fakaofo" => Ok(TimeZone::PacificFakaofo),
            "Pacific/Fiji" => Ok(TimeZone::PacificFiji),
            "Pacific/Funafuti" => Ok(TimeZone::PacificFunafuti),
            "Pacific/Galapagos" => Ok(TimeZone::PacificGalapagos),
            "Pacific/Gambier" => Ok(TimeZone::PacificGambier),
            "Pacific/Guadalcanal" => Ok(TimeZone::PacificGuadalcanal),
            "Pacific/Guam" => Ok(TimeZone::PacificGuam),
            "Pacific/Honolulu" => Ok(TimeZone::PacificHonolulu),
            "Pacific/Kiritimati" => Ok(TimeZone::PacificKiritimati),
            "Pacific/Kosrae" => Ok(TimeZone::PacificKosrae),
            "Pacific/Kwajalein" => Ok(TimeZone::PacificKwajalein),
            "Pacific/Majuro" => Ok(TimeZone::PacificMajuro),
            "Pacific/Marquesas" => Ok(TimeZone::PacificMarquesas),
            "Pacific/Nauru" => Ok(TimeZone::PacificNauru),
            "Pacific/Niue" => Ok(TimeZone::PacificNiue),
            "Pacific/Norfolk" => Ok(TimeZone::PacificNorfolk),
            "Pacific/Noumea" => Ok(TimeZone::PacificNoumea),
            "Pacific/Pago_Pago" => Ok(TimeZone::PacificPagoPago),
            "Pacific/Palau" => Ok(TimeZone::PacificPalau),
            "Pacific/Pitcairn" => Ok(TimeZone::PacificPitcairn),
            "Pacific/Pohnpei" => Ok(TimeZone::PacificPohnpei),
            "Pacific/Port_Moresby" => Ok(TimeZone::PacificPortMoresby),
            "Pacific/Rarotonga" => Ok(TimeZone::PacificRarotonga),
            "Pacific/Tahiti" => Ok(TimeZone::PacificTahiti),
            "Pacific/Tarawa" => Ok(TimeZone::PacificTarawa),
            "Pacific/Tongatapu" => Ok(TimeZone::PacificTongatapu),
            "Pacific/Wake" => Ok(TimeZone::PacificWake),
            "Pacific/Wallis" => Ok(TimeZone::PacificWallis),
            _ => Err(Error::InvalidTimeZoneName(time_zone)),
        } // match
    } // fn
} // impl

impl std::default::Default for TimeZone {
    /// Returns a reasonable default variant for the `TimeZone` enum type.
    fn default() -> Self {
        TimeZone::AmericaEdmonton
    } // fn
} // impl

impl std::fmt::Display for TimeZone {
    /// Formats a `TimeZone` enum into a string that is presentable to the end
    /// user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       write!(f, "{}", String::from(self))
    } // fn
} // impl