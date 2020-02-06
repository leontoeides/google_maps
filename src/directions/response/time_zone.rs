//! Contains the `TimeZone` enum and its associated traits. It is used to
//! specify the time zone for times in the transit directions from the Google
//! Maps Directions API.

use serde::{Serialize, Deserialize};

/// A comprehensive list of time zones. At the moment this is used only for
/// Google Maps Directions for Transit. The intent behind having _Serde_ convert
/// the time zone `String` to an `enum` is for more efficient time zone
/// conversions and information lookups in the future.

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