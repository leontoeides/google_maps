use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

// -------------------------------------------------------------------------------------------------
//
/// Last updated: 2026-07-12, includes the types added in Google's February 12, 2026. **Note that
/// discriminant values were renumbered to accomodate larger category blocks.**
///
/// **Place types** are categories that identify the characteristics of a place. A place can have
/// one or more place types assigned to it.
///
/// A place's types are included in the response from a Place Details (New), Nearby Search (New),
/// Text Search (New), and Autocomplete (New) request:
///
/// - **A place can have a single primary type** from
///   type [Table A](https://developers.google.com/maps/documentation/places/web-service/place-types#table-a) or
///   type [Table B](https://developers.google.com/maps/documentation/places/web-service/place-types#table-b)
///   associated with it. For example, the primary type might be `mexican_restaurant` or
///   `steak_house`. The primary type may be missing if the place's primary type is not a supported
///   type. When a primary type is present, it is always one of the types in the `types` field.
///
/// - **A place can have multiple type values** from
///   type [Table A](https://developers.google.com/maps/documentation/places/web-service/place-types#table-a) or
///   type [Table B](https://developers.google.com/maps/documentation/places/web-service/place-types#table-b)
///   associated with it. For example a restaurant might have the following types:
///   `seafood_restaurant`, `restaurant`, `food`, `point_of_interest`, `establishment`.
///
/// - **The address and address components** of a place can be tagged with certain types from the
///   [Address types and address component types](https://developers.google.com/maps/documentation/places/web-service/place-types#address-types)
///   table. For example, an address might be tagged as an `street_address` and a component of the
///   address might be tagged as a `street_number`.
///
/// You can also specify place types as part of a request. When specified in the request, the type
/// acts as a filter to restrict the response to only include places that match the specified types.
///
/// ## About the Type Tables
///
/// **[Table A](https://developers.google.com/maps/documentation/places/web-service/place-types#table-a)** lists the types that are used in the following ways:
///
/// - As part of a response from
///   [Place Details (New)](https://developers.google.com/maps/documentation/places/web-service/place-details),
///   [Nearby Search (New)](https://developers.google.com/maps/documentation/places/web-service/nearby-search), and
///   [Text Search (New)](https://developers.google.com/maps/documentation/places/web-service/text-search).
///   The request must specify at least one of the `places.types` or `places.primaryType` fields in
///   the field mask. The values in Table A are then used to populate those fields.
///
/// - As part of a
///   [Nearby Search (New)](https://developers.google.com/maps/documentation/places/web-service/nearby-search)
///   request, used as the value of the `includedTypes`, `excludedTypes`, `includedPrimaryTypes`,
///   and `excludedPrimaryTypes` parameter. The values in Table A are then used to populate those
///   fields.
///
/// - As part of a
///   [Text Search (New)](https://developers.google.com/maps/documentation/places/web-service/text-search)
///   request, used as the value of the `includedType` parameter.
///
/// - As part of a
///   [Autocomplete (New)](https://developers.google.com/maps/documentation/places/web-service/place-autocomplete)
///   request, use as the values to the `includedPrimaryTypes` parameter.
///
/// - As part of a
///   [Autocomplete (New)](https://developers.google.com/maps/documentation/places/web-service/place-autocomplete)
///   response.
///
/// **[Table B](https://developers.google.com/maps/documentation/places/web-service/place-types#table-b)**
/// lists additional place type values which may also be returned as part of a
/// [Place Details (New)](https://developers.google.com/maps/documentation/places/web-service/place-details),
/// [Nearby Search (New)](https://developers.google.com/maps/documentation/places/web-service/nearby-search),
/// [Text Search (New)](https://developers.google.com/maps/documentation/places/web-service/text-search), and
/// [Autocomplete (New)](https://developers.google.com/maps/documentation/places/web-service/place-autocomplete)
/// response. The request must specify at least one of the `places.types` or `places.primaryType`
/// fields in the field mask. **Values from Table B may NOT be used as part of a request, except as
/// the values to the `includedPrimaryTypes` parameter for a
/// [Autocomplete (New)](https://developers.google.com/maps/documentation/places/web-service/place-autocomplete)
/// request.**
///
/// **[Address types and address component types](https://developers.google.com/maps/documentation/places/web-service/place-types#address-types)**
/// list types that may appear in either or both address type and address component type arrays in
/// the response body. Address component types are subject to change.
#[derive(
    // std
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    // serde
    Serialize,
    Deserialize,
    // strum
    AsRefStr,
    Display,
    EnumIter,
    EnumString,
    FromRepr,
    IntoStaticStr
)]
#[non_exhaustive]
#[repr(u16)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PlaceType {
    //   ______      __    __          ___
    //  /_  __/___ _/ /_  / /__       /   |
    //   / / / __ `/ __ \/ / _ \     / /| |
    //  / / / /_/ / /_/ / /  __/    / ___ |
    // /_/  \__,_/_.___/_/\___/    /_/  |_|
    //
    // <https://developers.google.com/maps/documentation/places/web-service/place-types#table-a>

    // ---------------------------------------------------------------------------------------------
    // Automotive (0-99)
    // ---------------------------------------------------------------------------------------------

    /// A business that sells cars.
    CarDealer = 0,

    /// A business that rents cars for short-term use.
    CarRental = 1,

    /// A business that provides automotive repair services.
    CarRepair = 2,

    /// A facility for washing vehicles.
    CarWash = 3,

    /// A station for charging electric bicycles.
    EbikeChargingStation = 4,

    /// A station for charging electric vehicles.
    ElectricVehicleChargingStation = 5,

    /// A facility that sells fuel for motor vehicles.
    GasStation = 6,

    /// A facility for parking vehicles.
    Parking = 7,

    /// A multi-level structure for parking vehicles.
    ParkingGarage = 8,

    /// An open area for parking vehicles.
    ParkingLot = 9,

    /// A roadside facility for travelers to rest.
    RestStop = 10,

    /// A business that sells and installs vehicle tires.
    TireShop = 11,

    /// A business that sells trucks.
    TruckDealer = 12,

    // ---------------------------------------------------------------------------------------------
    // Business (100-199)
    // ---------------------------------------------------------------------------------------------

    /// A facility providing office space and business services.
    BusinessCenter = 100,

    /// A building or office space designated for business activities.
    CorporateOffice = 101,

    /// A shared workspace rented by individuals or small teams.
    CoworkingSpace = 102,

    /// An agricultural facility for growing crops or raising livestock.
    Farm = 103,

    /// A facility that produces goods, typically on an industrial scale.
    Manufacturer = 104,

    /// A large farm for raising horses, cattle, or other livestock.
    Ranch = 105,

    /// A business that supplies goods or materials to other businesses.
    Supplier = 106,

    /// A facility for producing television programs.
    TelevisionStudio = 107,

    // ---------------------------------------------------------------------------------------------
    // Culture (200-299)
    // ---------------------------------------------------------------------------------------------

    /// A facility for displaying works of art.
    ArtGallery = 200,

    /// A museum dedicated to the display of art.
    ArtMuseum = 201,

    /// A workspace for creating art.
    ArtStudio = 202,

    /// A large room or hall for public gatherings or performances.
    Auditorium = 203,

    /// A fortified historic residence or stronghold.
    Castle = 204,

    /// A site of cultural or historical significance.
    CulturalLandmark = 205,

    /// A decorative structure that ejects water, often a public landmark.
    Fountain = 206,

    /// A place of historical importance.
    HistoricalPlace = 207,

    /// A museum dedicated to historical artifacts and exhibits.
    HistoryMuseum = 208,

    /// A structure built to commemorate a person or event.
    Monument = 209,

    /// An institution for preserving and exhibiting objects of cultural importance.
    Museum = 210,

    /// A venue for theatrical performances.
    PerformingArtsTheater = 211,

    /// A three-dimensional work of art.
    Sculpture = 212,

    // ---------------------------------------------------------------------------------------------
    // Education (300-399)
    // ---------------------------------------------------------------------------------------------

    /// A department within a school or university dedicated to a field of study.
    AcademicDepartment = 300,

    /// An organization dedicated to education, such as a school or academy.
    EducationalInstitution = 301,

    /// A facility containing books and other materials for reading and study.
    Library = 302,

    /// An educational institution for young children before primary school.
    Preschool = 303,

    /// An elementary school for children in early grades.
    PrimarySchool = 304,

    /// An organization dedicated to conducting research.
    ResearchInstitute = 305,

    /// An educational institution for children and adolescents.
    School = 306,

    /// A high school for adolescents.
    SecondarySchool = 307,

    /// An institution of higher education.
    University = 308,

    // ---------------------------------------------------------------------------------------------
    // Entertainment and Recreation (400-599)
    // ---------------------------------------------------------------------------------------------

    /// A facility offering adventure sports activities.
    AdventureSportsCenter = 400,

    /// An outdoor venue for performances with tiered seating.
    Amphitheatre = 401,

    /// A facility providing various forms of entertainment.
    AmusementCenter = 402,

    /// A park with rides and attractions for entertainment.
    AmusementPark = 403,

    /// A facility for displaying marine life.
    Aquarium = 404,

    /// A hall for hosting banquets and large dining events.
    BanquetHall = 405,

    /// An area designated for outdoor cooking and grilling.
    BarbecueArea = 406,

    /// A garden featuring a wide variety of plants for scientific study and display.
    BotanicalGarden = 407,

    /// A facility for the sport of bowling.
    BowlingAlley = 408,

    /// A facility for gambling and gaming.
    Casino = 409,

    /// A recreational facility for children's camping activities.
    ChildrensCamp = 410,

    /// A public park within a city.
    CityPark = 411,

    /// A venue for comedy performances.
    ComedyClub = 412,

    /// A facility serving the local community for various activities.
    CommunityCenter = 413,

    /// A venue designed for musical performances.
    ConcertHall = 414,

    /// A facility for large meetings, conferences, and exhibitions.
    ConventionCenter = 415,

    /// A facility dedicated to cultural activities and events.
    CulturalCenter = 416,

    /// A park designed for bicycle riding and cycling activities.
    CyclingPark = 417,

    /// A venue for dancing and dance events.
    DanceHall = 418,

    /// A park specifically designed for dogs to exercise and play.
    DogPark = 419,

    /// A facility for hosting events and gatherings.
    EventVenue = 420,

    /// A large rotating wheel ride at an amusement park.
    FerrisWheel = 421,

    /// A cultivated area for growing flowers, plants, or vegetables.
    Garden = 422,

    /// A venue for recreational go-kart racing.
    GoKartingVenue = 423,

    /// An area designated for hiking and walking trails.
    HikingArea = 424,

    /// A site of historical significance.
    HistoricalLandmark = 425,

    /// An indoor play facility for children.
    IndoorPlayground = 426,

    /// A facility providing internet access to customers.
    InternetCafe = 427,

    /// A venue for karaoke entertainment.
    Karaoke = 428,

    /// A venue that hosts live musical performances.
    LiveMusicVenue = 429,

    /// A harbor facility for boats and yachts.
    Marina = 430,

    /// A course for playing miniature golf.
    MiniatureGolfCourse = 431,

    /// A business that rents movies for home viewing.
    MovieRental = 432,

    /// A venue for showing films.
    MovieTheater = 433,

    /// A park protected by the national government.
    NationalPark = 434,

    /// An entertainment venue operating primarily at night.
    NightClub = 435,

    /// A platform offering panoramic views of the surrounding area.
    ObservationDeck = 436,

    /// An area designated for off-road vehicle activities.
    OffRoadingArea = 437,

    /// A venue for opera performances.
    OperaHouse = 438,

    /// A venue for playing paintball.
    PaintballCenter = 439,

    /// A public area of land for recreation and enjoyment.
    Park = 440,

    /// A concert hall specifically for philharmonic orchestras.
    PhilharmonicHall = 441,

    /// An area designated for picnicking.
    PicnicGround = 442,

    /// A facility for astronomical observation and education.
    Planetarium = 443,

    /// An open public square or marketplace.
    Plaza = 444,

    /// A ride at an amusement park featuring a track with steep slopes and sharp turns.
    RollerCoaster = 445,

    /// A park designed for skateboarding activities.
    SkateboardPark = 446,

    /// A park protected by state government.
    StatePark = 447,

    /// A place of interest for tourists.
    TouristAttraction = 448,

    /// A facility with coin-operated video games.
    VideoArcade = 449,

    /// An area planted with grapevines, typically for winemaking.
    Vineyard = 450,

    /// A facility providing information and services to tourists.
    VisitorCenter = 451,

    /// A recreational facility with water attractions.
    WaterPark = 452,

    /// A facility for hosting wedding ceremonies and receptions.
    WeddingVenue = 453,

    /// A park for observing wildlife in natural habitats.
    WildlifePark = 454,

    /// A protected area for wildlife conservation.
    WildlifeRefuge = 455,

    /// A facility for displaying animals for public viewing.
    Zoo = 456,

    // ---------------------------------------------------------------------------------------------
    // Facilities (600-699)
    // ---------------------------------------------------------------------------------------------

    /// A facility for public bathing.
    PublicBath = 600,

    /// A public restroom facility.
    PublicBathroom = 601,

    /// A building for housing horses or livestock.
    Stable = 602,

    // ---------------------------------------------------------------------------------------------
    // Finance (700-799)
    // ---------------------------------------------------------------------------------------------

    /// A business providing accounting and bookkeeping services.
    Accounting = 700,

    /// An automated teller machine for banking transactions.
    Atm = 701,

    /// A financial institution that accepts deposits and makes loans.
    Bank = 702,

    // ---------------------------------------------------------------------------------------------
    // Food and Drink (800-1499)
    // ---------------------------------------------------------------------------------------------

    /// A shop specializing in acai bowls and related products.
    AcaiShop = 800,

    /// A restaurant serving Afghan cuisine.
    AfghaniRestaurant = 801,

    /// A restaurant serving African cuisine.
    AfricanRestaurant = 802,

    /// A restaurant serving American cuisine.
    AmericanRestaurant = 803,

    /// A restaurant serving Argentinian cuisine.
    ArgentinianRestaurant = 804,

    /// A restaurant blending Asian cuisines with other culinary traditions.
    AsianFusionRestaurant = 805,

    /// A restaurant serving Asian cuisine.
    AsianRestaurant = 806,

    /// A restaurant serving Australian cuisine.
    AustralianRestaurant = 807,

    /// A restaurant serving Austrian cuisine.
    AustrianRestaurant = 808,

    /// A shop specializing in bagels.
    BagelShop = 809,

    /// A shop that bakes and sells bread, cakes, and pastries.
    Bakery = 810,

    /// A restaurant serving Bangladeshi cuisine.
    BangladeshiRestaurant = 811,

    /// An establishment serving alcoholic beverages.
    Bar = 812,

    /// A restaurant that combines bar and grill services.
    BarAndGrill = 813,

    /// A restaurant specializing in barbecued food.
    BarbecueRestaurant = 814,

    /// A restaurant serving Basque cuisine.
    BasqueRestaurant = 815,

    /// A restaurant serving Bavarian cuisine.
    BavarianRestaurant = 816,

    /// An outdoor venue serving beer and often food.
    BeerGarden = 817,

    /// A restaurant serving Belgian cuisine.
    BelgianRestaurant = 818,

    /// A small, casual restaurant serving simple meals.
    Bistro = 819,

    /// A restaurant serving Brazilian cuisine.
    BrazilianRestaurant = 820,

    /// A restaurant serving breakfast meals.
    BreakfastRestaurant = 821,

    /// A facility that produces beer, often with an attached taproom.
    Brewery = 822,

    /// A pub that brews and serves its own beer on the premises.
    Brewpub = 823,

    /// A restaurant serving British cuisine.
    BritishRestaurant = 824,

    /// A restaurant serving brunch meals.
    BrunchRestaurant = 825,

    /// A restaurant offering self-service dining from a variety of dishes.
    BuffetRestaurant = 826,

    /// A restaurant serving Burmese cuisine.
    BurmeseRestaurant = 827,

    /// A restaurant specializing in burritos.
    BurritoRestaurant = 828,

    /// A casual establishment serving coffee, light meals, and snacks.
    Cafe = 829,

    /// A dining facility typically found in institutions like schools or offices.
    Cafeteria = 830,

    /// A restaurant serving Cajun cuisine.
    CajunRestaurant = 831,

    /// A shop that sells cakes.
    CakeShop = 832,

    /// A restaurant serving Californian cuisine.
    CalifornianRestaurant = 833,

    /// A restaurant serving Cambodian cuisine.
    CambodianRestaurant = 834,

    /// A shop selling confections and candy.
    CandyStore = 835,

    /// A restaurant serving Cantonese cuisine.
    CantoneseRestaurant = 836,

    /// A restaurant serving Caribbean cuisine.
    CaribbeanRestaurant = 837,

    /// A cafe where customers can interact with cats.
    CatCafe = 838,

    /// A restaurant specializing in chicken dishes.
    ChickenRestaurant = 839,

    /// A restaurant specializing in chicken wings.
    ChickenWingsRestaurant = 840,

    /// A restaurant serving Chilean cuisine.
    ChileanRestaurant = 841,

    /// A restaurant specializing in Chinese noodle dishes.
    ChineseNoodleRestaurant = 842,

    /// A restaurant serving Chinese cuisine.
    ChineseRestaurant = 843,

    /// A facility that manufactures chocolate products.
    ChocolateFactory = 844,

    /// A shop specializing in chocolate products.
    ChocolateShop = 845,

    /// A bar specializing in cocktails.
    CocktailBar = 846,

    /// A business that roasts coffee beans, often serving coffee on site.
    CoffeeRoastery = 847,

    /// An establishment primarily serving coffee and coffee-based beverages.
    CoffeeShop = 848,

    /// A small stand or kiosk serving coffee.
    CoffeeStand = 849,

    /// A restaurant serving Colombian cuisine.
    ColombianRestaurant = 850,

    /// A shop selling confections and sweets.
    Confectionery = 851,

    /// A restaurant serving Croatian cuisine.
    CroatianRestaurant = 852,

    /// A restaurant serving Cuban cuisine.
    CubanRestaurant = 853,

    /// A restaurant serving Czech cuisine.
    CzechRestaurant = 854,

    /// A restaurant serving Danish cuisine.
    DanishRestaurant = 855,

    /// A shop selling sliced meats, cheeses, and prepared foods.
    Deli = 856,

    /// A restaurant specializing in desserts.
    DessertRestaurant = 857,

    /// A shop specializing in desserts and sweets.
    DessertShop = 858,

    /// A restaurant specializing in dim sum.
    DimSumRestaurant = 859,

    /// A casual restaurant, typically serving American fare.
    Diner = 860,

    /// A cafe where customers can interact with dogs.
    DogCafe = 861,

    /// A shop specializing in donuts and related pastries.
    DonutShop = 862,

    /// A restaurant specializing in dumplings.
    DumplingRestaurant = 863,

    /// A restaurant serving Dutch cuisine.
    DutchRestaurant = 864,

    /// A restaurant serving Eastern European cuisine.
    EasternEuropeanRestaurant = 865,

    /// A restaurant serving Ethiopian cuisine.
    EthiopianRestaurant = 866,

    /// A restaurant serving European cuisine.
    EuropeanRestaurant = 867,

    /// A restaurant specializing in falafel.
    FalafelRestaurant = 868,

    /// A restaurant suitable for family dining.
    FamilyRestaurant = 869,

    /// A restaurant offering quick service meals.
    FastFoodRestaurant = 870,

    /// A restaurant serving Filipino cuisine.
    FilipinoRestaurant = 871,

    /// An upscale restaurant offering high-quality cuisine and service.
    FineDiningRestaurant = 872,

    /// A restaurant specializing in fish and chips.
    FishAndChipsRestaurant = 873,

    /// A restaurant specializing in fondue.
    FondueRestaurant = 874,

    /// An area with multiple food vendors and shared seating.
    FoodCourt = 875,

    /// A restaurant serving French cuisine.
    FrenchRestaurant = 876,

    /// A restaurant blending multiple culinary traditions.
    FusionRestaurant = 877,

    /// A pub serving high-quality food alongside drinks.
    Gastropub = 878,

    /// A restaurant serving German cuisine.
    GermanRestaurant = 879,

    /// A restaurant serving Greek cuisine.
    GreekRestaurant = 880,

    /// A restaurant specializing in gyros.
    GyroRestaurant = 881,

    /// A restaurant serving halal food.
    HalalRestaurant = 882,

    /// A restaurant specializing in hamburgers.
    HamburgerRestaurant = 883,

    /// A restaurant serving Hawaiian cuisine.
    HawaiianRestaurant = 884,

    /// A bar where patrons smoke flavored tobacco from hookahs.
    HookahBar = 885,

    /// A restaurant specializing in hot dogs.
    HotDogRestaurant = 886,

    /// A stand or cart selling hot dogs.
    HotDogStand = 887,

    /// A restaurant specializing in hot pot dining.
    HotPotRestaurant = 888,

    /// A restaurant serving Hungarian cuisine.
    HungarianRestaurant = 889,

    /// A shop selling ice cream and frozen treats.
    IceCreamShop = 890,

    /// A restaurant serving Indian cuisine.
    IndianRestaurant = 891,

    /// A restaurant serving Indonesian cuisine.
    IndonesianRestaurant = 892,

    /// A pub in the Irish style, typically serving beer and pub food.
    IrishPub = 893,

    /// A restaurant serving Irish cuisine.
    IrishRestaurant = 894,

    /// A restaurant serving Israeli cuisine.
    IsraeliRestaurant = 895,

    /// A restaurant serving Italian cuisine.
    ItalianRestaurant = 896,

    /// A restaurant specializing in Japanese curry.
    JapaneseCurryRestaurant = 897,

    /// A Japanese-style pub serving drinks and small dishes.
    JapaneseIzakayaRestaurant = 898,

    /// A restaurant serving Japanese cuisine.
    JapaneseRestaurant = 899,

    /// A shop specializing in fresh juices and smoothies.
    JuiceShop = 900,

    /// A shop or restaurant specializing in kebabs.
    KebabShop = 901,

    /// A restaurant specializing in Korean barbecue.
    KoreanBarbecueRestaurant = 902,

    /// A restaurant serving Korean cuisine.
    KoreanRestaurant = 903,

    /// A restaurant serving Latin American cuisine.
    LatinAmericanRestaurant = 904,

    /// A restaurant serving Lebanese cuisine.
    LebaneseRestaurant = 905,

    /// A bar with a relaxed lounge atmosphere.
    LoungeBar = 906,

    /// A restaurant serving Malaysian cuisine.
    MalaysianRestaurant = 907,

    /// A service that delivers prepared meals.
    MealDelivery = 908,

    /// A restaurant offering meals for takeaway.
    MealTakeaway = 909,

    /// A restaurant serving Mediterranean cuisine.
    MediterraneanRestaurant = 910,

    /// A restaurant serving Mexican cuisine.
    MexicanRestaurant = 911,

    /// A restaurant serving Middle Eastern cuisine.
    MiddleEasternRestaurant = 912,

    /// A restaurant specializing in Mongolian barbecue.
    MongolianBarbecueRestaurant = 913,

    /// A restaurant serving Moroccan cuisine.
    MoroccanRestaurant = 914,

    /// A restaurant specializing in noodle dishes.
    NoodleShop = 915,

    /// A restaurant serving North Indian cuisine.
    NorthIndianRestaurant = 916,

    /// A restaurant specializing in oysters and other shellfish.
    OysterBarRestaurant = 917,

    /// A restaurant serving Pakistani cuisine.
    PakistaniRestaurant = 918,

    /// A shop that sells pastries.
    PastryShop = 919,

    /// A restaurant serving Persian cuisine.
    PersianRestaurant = 920,

    /// A restaurant serving Peruvian cuisine.
    PeruvianRestaurant = 921,

    /// A business specializing in pizza delivery.
    PizzaDelivery = 922,

    /// A restaurant specializing in pizza.
    PizzaRestaurant = 923,

    /// A restaurant serving Polish cuisine.
    PolishRestaurant = 924,

    /// A restaurant serving Portuguese cuisine.
    PortugueseRestaurant = 925,

    /// A tavern or bar serving food and alcoholic beverages.
    Pub = 926,

    /// A restaurant specializing in ramen noodle soup.
    RamenRestaurant = 927,

    /// A business serving prepared food and beverages to customers.
    Restaurant = 928,

    /// A restaurant serving Romanian cuisine.
    RomanianRestaurant = 929,

    /// A restaurant serving Russian cuisine.
    RussianRestaurant = 930,

    /// A restaurant specializing in salads.
    SaladShop = 931,

    /// A shop specializing in sandwiches.
    SandwichShop = 932,

    /// A restaurant serving Scandinavian cuisine.
    ScandinavianRestaurant = 933,

    /// A restaurant serving seafood.
    SeafoodRestaurant = 934,

    /// A restaurant specializing in shawarma.
    ShawarmaRestaurant = 935,

    /// A small eatery serving snacks and light meals.
    SnackBar = 936,

    /// A restaurant serving soul food.
    SoulFoodRestaurant = 937,

    /// A restaurant specializing in soups.
    SoupRestaurant = 938,

    /// A restaurant serving South American cuisine.
    SouthAmericanRestaurant = 939,

    /// A restaurant serving South Indian cuisine.
    SouthIndianRestaurant = 940,

    /// A restaurant serving Southwestern United States cuisine.
    SouthwesternUsRestaurant = 941,

    /// A restaurant serving Spanish cuisine.
    SpanishRestaurant = 942,

    /// A bar featuring televised sports and casual food.
    SportsBar = 943,

    /// A restaurant serving Sri Lankan cuisine.
    SriLankanRestaurant = 944,

    /// A restaurant specializing in steak and other grilled meats.
    SteakHouse = 945,

    /// A restaurant specializing in sushi.
    SushiRestaurant = 946,

    /// A restaurant serving Swiss cuisine.
    SwissRestaurant = 947,

    /// A restaurant specializing in tacos.
    TacoRestaurant = 948,

    /// A restaurant serving Taiwanese cuisine.
    TaiwaneseRestaurant = 949,

    /// A restaurant serving Spanish-style tapas.
    TapasRestaurant = 950,

    /// An establishment serving tea and light refreshments.
    TeaHouse = 951,

    /// A restaurant serving Tex-Mex cuisine.
    TexMexRestaurant = 952,

    /// A restaurant serving Thai cuisine.
    ThaiRestaurant = 953,

    /// A restaurant serving Tibetan cuisine.
    TibetanRestaurant = 954,

    /// A restaurant specializing in Japanese tonkatsu.
    TonkatsuRestaurant = 955,

    /// A restaurant serving Turkish cuisine.
    TurkishRestaurant = 956,

    /// A restaurant serving Ukrainian cuisine.
    UkrainianRestaurant = 957,

    /// A restaurant serving vegan food.
    VeganRestaurant = 958,

    /// A restaurant serving vegetarian food.
    VegetarianRestaurant = 959,

    /// A restaurant serving Vietnamese cuisine.
    VietnameseRestaurant = 960,

    /// A restaurant serving Western-style cuisine.
    WesternRestaurant = 961,

    /// A bar specializing in wine.
    WineBar = 962,

    /// A facility that produces wine, often with tastings on site.
    Winery = 963,

    /// A restaurant specializing in Japanese grilled meat.
    YakinikuRestaurant = 964,

    /// A restaurant specializing in Japanese grilled chicken skewers.
    YakitoriRestaurant = 965,

    // ---------------------------------------------------------------------------------------------
    // Geographical Areas (1500-1599)
    // ---------------------------------------------------------------------------------------------

    /// A first-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_1")]
    #[strum(serialize = "administrative_area_level_1")]
    AdministrativeAreaLevel1 = 1500,

    /// A second-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_2")]
    #[strum(serialize = "administrative_area_level_2")]
    AdministrativeAreaLevel2 = 1501,

    /// A sovereign nation.
    Country = 1502,

    /// An incorporated city or town.
    Locality = 1503,

    /// A postal code area.
    PostalCode = 1504,

    /// A district defined for school administrative purposes.
    SchoolDistrict = 1505,

    // ---------------------------------------------------------------------------------------------
    // Government (1600-1699)
    // ---------------------------------------------------------------------------------------------

    /// The administrative center of a city or town.
    CityHall = 1600,

    /// A building housing judicial courts.
    Courthouse = 1601,

    /// The diplomatic mission of one country in another.
    Embassy = 1602,

    /// A facility for firefighting services.
    FireStation = 1603,

    /// A building housing government offices.
    GovernmentOffice = 1604,

    /// A local government administrative office.
    LocalGovernmentOffice = 1605,

    /// A local police station (specific to Japan).
    NeighborhoodPoliceStation = 1606,

    /// A law enforcement facility.
    Police = 1607,

    /// A facility for postal and mailing services.
    PostOffice = 1608,

    // ---------------------------------------------------------------------------------------------
    // Health and Wellness (1700-1799)
    // ---------------------------------------------------------------------------------------------

    /// A healthcare practitioner specializing in spinal adjustment.
    Chiropractor = 1700,

    /// A clinic specializing in dental care.
    DentalClinic = 1701,

    /// A healthcare professional specializing in dental care.
    Dentist = 1702,

    /// A medical practitioner.
    Doctor = 1703,

    /// A retail pharmacy.
    Drugstore = 1704,

    /// A hospital providing a broad range of medical care.
    GeneralHospital = 1705,

    /// A medical facility for patient care.
    Hospital = 1706,

    /// A facility offering therapeutic massage services.
    Massage = 1707,

    /// A spa specializing in massage services.
    MassageSpa = 1708,

    /// A facility providing a range of medical services.
    MedicalCenter = 1709,

    /// A clinic providing outpatient medical care.
    MedicalClinic = 1710,

    /// A facility for medical testing and analysis.
    MedicalLab = 1711,

    /// A facility dispensing prescription medications.
    Pharmacy = 1712,

    /// A healthcare professional specializing in physical therapy.
    Physiotherapist = 1713,

    /// A facility offering sauna services.
    Sauna = 1714,

    /// A clinic specializing in skin care treatments.
    SkinCareClinic = 1715,

    /// A facility offering health and wellness treatments.
    Spa = 1716,

    /// A facility offering tanning services.
    TanningStudio = 1717,

    /// A facility focused on health and wellness services.
    WellnessCenter = 1718,

    /// A studio offering yoga classes and instruction.
    YogaStudio = 1719,

    // ---------------------------------------------------------------------------------------------
    // Housing (1800-1899)
    // ---------------------------------------------------------------------------------------------

    /// A residential building divided into individual apartments.
    ApartmentBuilding = 1800,

    /// A group of apartment buildings managed together.
    ApartmentComplex = 1801,

    /// A residential complex of individually owned units.
    CondominiumComplex = 1802,

    /// A residential development with multiple housing units.
    HousingComplex = 1803,

    // ---------------------------------------------------------------------------------------------
    // Lodging (1900-1999)
    // ---------------------------------------------------------------------------------------------

    /// A small lodging establishment offering overnight accommodation and breakfast.
    BedAndBreakfast = 1900,

    /// An affordable Japanese-style inn.
    BudgetJapaneseInn = 1901,

    /// A recreational area for camping with facilities.
    Campground = 1902,

    /// A small cabin for camping and outdoor recreation.
    CampingCabin = 1903,

    /// A small house, typically in a rural area.
    Cottage = 1904,

    /// A hotel designed for extended stays.
    ExtendedStayHotel = 1905,

    /// A working farm offering accommodation to visitors.
    Farmstay = 1906,

    /// A private home offering accommodation to travelers.
    GuestHouse = 1907,

    /// A budget accommodation with shared facilities.
    Hostel = 1908,

    /// An establishment providing lodging and services for travelers.
    Hotel = 1909,

    /// A small hotel or lodging establishment.
    Inn = 1910,

    /// A traditional Japanese inn.
    JapaneseInn = 1911,

    /// A place that provides temporary accommodation.
    Lodging = 1912,

    /// A park designed for mobile homes.
    MobileHomePark = 1913,

    /// A roadside hotel for motorists.
    Motel = 1914,

    /// A private room in someone's home for guests.
    PrivateGuestRoom = 1915,

    /// A hotel at a resort destination.
    ResortHotel = 1916,

    /// A park for recreational vehicles.
    RvPark = 1917,

    // ---------------------------------------------------------------------------------------------
    // Natural Features (2000-2099)
    // ---------------------------------------------------------------------------------------------

    /// A sandy or pebbly shore along a body of water.
    Beach = 2000,

    /// A body of land surrounded by water.
    Island = 2001,

    /// A large inland body of water.
    Lake = 2002,

    /// The summit of a mountain.
    MountainPeak = 2003,

    /// A protected area for conserving wildlife and natural habitats.
    NaturePreserve = 2004,

    /// A large natural stream of flowing water.
    River = 2005,

    /// A location known for its scenic views.
    ScenicSpot = 2006,

    /// An area covered with trees, smaller than a forest.
    Woods = 2007,

    // ---------------------------------------------------------------------------------------------
    // Places of Worship (2100-2199)
    // ---------------------------------------------------------------------------------------------

    /// A place of worship for Buddhists.
    BuddhistTemple = 2100,

    /// A Christian place of worship.
    Church = 2101,

    /// A Hindu place of worship.
    HinduTemple = 2102,

    /// An Islamic place of worship.
    Mosque = 2103,

    /// A place of worship in the Shinto religion.
    ShintoShrine = 2104,

    /// A Jewish place of worship.
    Synagogue = 2105,

    // ---------------------------------------------------------------------------------------------
    // Services (2200-2399)
    // ---------------------------------------------------------------------------------------------

    /// A business that rents aircraft.
    AircraftRentalService = 2200,

    /// A membership-based association or organization.
    AssociationOrOrganization = 2201,

    /// A person who practices astrology.
    Astrologer = 2202,

    /// A shop providing hair cutting and styling for men.
    BarberShop = 2203,

    /// A professional who provides beauty treatments.
    Beautician = 2204,

    /// A facility offering beauty and cosmetic services.
    BeautySalon = 2205,

    /// A service providing body art such as tattoos or piercings.
    BodyArtService = 2206,

    /// A service providing food for events and gatherings.
    CateringService = 2207,

    /// A burial ground for the deceased.
    Cemetery = 2208,

    /// A business providing professional driver services.
    ChauffeurService = 2209,

    /// An agency providing child care services.
    ChildCareAgency = 2210,

    /// A professional who provides expert advice.
    Consultant = 2211,

    /// A service for delivering packages and mail.
    CourierService = 2212,

    /// A professional who installs and maintains electrical systems.
    Electrician = 2213,

    /// A business that matches job seekers with employers.
    EmploymentAgency = 2214,

    /// A person or business that sells and arranges flowers.
    Florist = 2215,

    /// A service that delivers food from restaurants.
    FoodDelivery = 2216,

    /// A service providing foot care and treatment.
    FootCare = 2217,

    /// A business that handles funeral arrangements and services.
    FuneralHome = 2218,

    /// A service providing hair care and styling.
    HairCare = 2219,

    /// A salon providing hair care and styling services.
    HairSalon = 2220,

    /// An agency providing insurance services.
    InsuranceAgency = 2221,

    /// A service for cleaning clothes and textiles.
    Laundry = 2222,

    /// A legal professional who practices law.
    Lawyer = 2223,

    /// A professional who installs and repairs locks.
    Locksmith = 2224,

    /// A professional who provides makeup and cosmetic services.
    MakeupArtist = 2225,

    /// A business providing marketing consulting services.
    MarketingConsultant = 2226,

    /// A company that provides relocation and moving services.
    MovingCompany = 2227,

    /// A salon providing nail care and manicure services.
    NailSalon = 2228,

    /// An organization operating for purposes other than profit.
    NonProfitOrganization = 2229,

    /// A professional who paints buildings and structures.
    Painter = 2230,

    /// A business that boards pets while owners are away.
    PetBoardingService = 2231,

    /// A business providing pet care services.
    PetCare = 2232,

    /// A professional who installs and repairs plumbing systems.
    Plumber = 2233,

    /// A person who claims to have supernatural abilities for divination.
    Psychic = 2234,

    /// An agency that deals with buying and selling real estate.
    RealEstateAgency = 2235,

    /// A contractor specializing in roof installation and repair.
    RoofingContractor = 2236,

    /// A general service provider not covered by a more specific type.
    Service = 2237,

    /// A business providing package shipping services.
    ShippingService = 2238,

    /// A facility for storing goods and belongings.
    Storage = 2239,

    /// A service that organizes summer camps for children.
    SummerCampOrganizer = 2240,

    /// A professional who makes and repairs clothing.
    Tailor = 2241,

    /// A company providing telecommunications services.
    TelecommunicationsServiceProvider = 2242,

    /// An agency that organizes tours and travel experiences.
    TourAgency = 2243,

    /// A center providing information and services to tourists.
    TouristInformationCenter = 2244,

    /// An agency that arranges travel and accommodations.
    TravelAgency = 2245,

    /// A professional providing medical care for animals.
    VeterinaryCare = 2246,

    // ---------------------------------------------------------------------------------------------
    // Shopping (2400-2599)
    // ---------------------------------------------------------------------------------------------

    /// A grocery store specializing in Asian products.
    AsianGroceryStore = 2400,

    /// A store selling automotive parts and accessories.
    AutoPartsStore = 2401,

    /// A store specializing in bicycles and cycling equipment.
    BicycleStore = 2402,

    /// A store selling books and reading materials.
    BookStore = 2403,

    /// A store that sells building and construction materials.
    BuildingMaterialsStore = 2404,

    /// A shop that sells fresh meat.
    ButcherShop = 2405,

    /// A store selling mobile phones and accessories.
    CellPhoneStore = 2406,

    /// A store selling clothing and apparel.
    ClothingStore = 2407,

    /// A small retail store with everyday items.
    ConvenienceStore = 2408,

    /// A store that sells cosmetics and beauty products.
    CosmeticsStore = 2409,

    /// A large retail store selling various goods across multiple departments.
    DepartmentStore = 2410,

    /// A store offering merchandise at reduced prices.
    DiscountStore = 2411,

    /// A supermarket focused on discounted prices.
    DiscountSupermarket = 2412,

    /// A store specializing in electronic devices and equipment.
    ElectronicsStore = 2413,

    /// A market where farmers sell produce and goods directly to consumers.
    FarmersMarket = 2414,

    /// A market where vendors sell secondhand and inexpensive goods.
    FleaMarket = 2415,

    /// A store selling food and grocery items.
    FoodStore = 2416,

    /// A store selling furniture and home furnishings.
    FurnitureStore = 2417,

    /// A store that sells plants and gardening supplies.
    GardenCenter = 2418,

    /// A store carrying a broad range of everyday goods.
    GeneralStore = 2419,

    /// A store specializing in gifts and novelty items.
    GiftShop = 2420,

    /// A store selling food and household supplies.
    GroceryStore = 2421,

    /// A store selling tools and hardware supplies.
    HardwareStore = 2422,

    /// A store specializing in health foods and natural products.
    HealthFoodStore = 2423,

    /// A store selling household items and home decor.
    HomeGoodsStore = 2424,

    /// A store selling supplies for home improvement and renovation.
    HomeImprovementStore = 2425,

    /// A very large store combining a supermarket and a department store.
    Hypermarket = 2426,

    /// A store selling jewelry and precious accessories.
    JewelryStore = 2427,

    /// A store selling alcoholic beverages.
    LiquorStore = 2428,

    /// A place for buying and selling goods, often outdoors.
    Market = 2429,

    /// A store selling pet supplies and accessories.
    PetStore = 2430,

    /// A store specializing in footwear.
    ShoeStore = 2431,

    /// A large complex containing multiple retail stores.
    ShoppingMall = 2432,

    /// A store selling sports equipment and athletic gear.
    SportingGoodsStore = 2433,

    /// A store that sells athletic clothing and footwear.
    SportswearStore = 2434,

    /// A retail establishment selling goods or services.
    Store = 2435,

    /// A large self-service store selling groceries and household goods.
    Supermarket = 2436,

    /// A store that sells tea and tea-related goods.
    TeaStore = 2437,

    /// A store that sells secondhand goods, often for charity.
    ThriftStore = 2438,

    /// A store that sells toys.
    ToyStore = 2439,

    /// A large store selling goods in bulk quantities.
    WarehouseStore = 2440,

    /// A business that sells goods in large quantities to retailers.
    Wholesaler = 2441,

    /// A store that sells women's clothing.
    WomensClothingStore = 2442,

    // ---------------------------------------------------------------------------------------------
    // Sports (2600-2699)
    // ---------------------------------------------------------------------------------------------

    /// A large venue for sports events and competitions.
    Arena = 2600,

    /// A field or area designated for athletic activities.
    AthleticField = 2601,

    /// A service providing fishing trips on a chartered boat.
    FishingCharter = 2602,

    /// A pier used for fishing.
    FishingPier = 2603,

    /// A small body of water stocked with fish for recreational fishing.
    FishingPond = 2604,

    /// A facility offering exercise equipment and fitness classes.
    FitnessCenter = 2605,

    /// A course designed for playing golf.
    GolfCourse = 2606,

    /// A facility with exercise equipment for physical fitness.
    Gym = 2607,

    /// A rink for ice skating activities.
    IceSkatingRink = 2608,

    /// An indoor facility for playing or practicing golf.
    IndoorGolfCourse = 2609,

    /// An area designed for children's recreational activities.
    Playground = 2610,

    /// A track or course for racing.
    RaceCourse = 2611,

    /// A mountain resort facility for skiing and winter sports.
    SkiResort = 2612,

    /// A location for various sports activities.
    SportsActivityLocation = 2613,

    /// An organization for sports enthusiasts and athletes.
    SportsClub = 2614,

    /// A service providing sports instruction and training.
    SportsCoaching = 2615,

    /// A facility with multiple sports venues and amenities.
    SportsComplex = 2616,

    /// A school focused on athletic training.
    SportsSchool = 2617,

    /// A large venue for sporting events with spectator seating.
    Stadium = 2618,

    /// A facility for swimming and aquatic activities.
    SwimmingPool = 2619,

    /// A court for playing tennis.
    TennisCourt = 2620,

    // ---------------------------------------------------------------------------------------------
    // Transportation (2700-2799)
    // ---------------------------------------------------------------------------------------------

    /// A facility for aircraft takeoffs and landings.
    Airport = 2700,

    /// A small airport or landing strip for aircraft.
    Airstrip = 2701,

    /// A station for picking up and returning shared bicycles.
    BikeSharingStation = 2702,

    /// A structure spanning a river, road, or other obstacle.
    Bridge = 2703,

    /// A facility for bus transportation services.
    BusStation = 2704,

    /// A designated stopping point for buses.
    BusStop = 2705,

    /// A business operating ferry transportation.
    FerryService = 2706,

    /// A terminal for ferry boat services.
    FerryTerminal = 2707,

    /// A landing and takeoff facility for helicopters.
    Heliport = 2708,

    /// A large airport serving international flights.
    InternationalAirport = 2709,

    /// A station for light rail transit systems.
    LightRailStation = 2710,

    /// A facility where commuters can park and use public transit.
    ParkAndRide = 2711,

    /// A station for subway or underground rail systems.
    SubwayStation = 2712,

    /// A business providing taxi transportation.
    TaxiService = 2713,

    /// A designated area where taxis wait for passengers.
    TaxiStand = 2714,

    /// A station where road tolls are collected.
    TollStation = 2715,

    /// A facility for passenger and freight train services.
    TrainStation = 2716,

    /// An office that sells train tickets.
    TrainTicketOffice = 2717,

    /// A stop for tram or streetcar services.
    TramStop = 2718,

    /// A facility for public transportation vehicle maintenance and storage.
    TransitDepot = 2719,

    /// A facility serving various forms of public transit.
    TransitStation = 2720,

    /// A stop for public transit services.
    TransitStop = 2721,

    /// A business providing general transportation services.
    TransportationService = 2722,

    /// A facility providing services for truck drivers and cargo transport.
    TruckStop = 2723,

    //   ______      __    __          ____
    //  /_  __/___ _/ /_  / /__       / __ )
    //   / / / __ `/ __ \/ / _ \     / __  |
    //  / / / /_/ / /_/ / /  __/    / /_/ /
    // /_/  \__,_/_.___/_/\___/    /_____/
    //
    // <https://developers.google.com/maps/documentation/places/web-service/place-types#table-b>

    // ---------------------------------------------------------------------------------------------
    // Table B (10000-10999)
    // ---------------------------------------------------------------------------------------------

    /// A third-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_3")]
    #[strum(serialize = "administrative_area_level_3")]
    AdministrativeAreaLevel3 = 10_000,

    /// A fourth-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_4")]
    #[strum(serialize = "administrative_area_level_4")]
    AdministrativeAreaLevel4 = 10_001,

    /// A fifth-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_5")]
    #[strum(serialize = "administrative_area_level_5")]
    AdministrativeAreaLevel5 = 10_002,

    /// A sixth-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_6")]
    #[strum(serialize = "administrative_area_level_6")]
    AdministrativeAreaLevel6 = 10_003,

    /// A seventh-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_7")]
    #[strum(serialize = "administrative_area_level_7")]
    AdministrativeAreaLevel7 = 10_004,

    /// A group of islands or a sea containing many islands.
    Archipelago = 10_005,

    /// A commonly-used alternative name for an entity.
    ColloquialArea = 10_006,

    /// A large landmass or geographical division.
    Continent = 10_007,

    /// A business establishment or commercial entity.
    Establishment = 10_008,

    /// A place related to financial services.
    Finance = 10_009,

    /// A place related to food services.
    Food = 10_010,

    /// A contractor that provides general construction services.
    GeneralContractor = 10_011,

    /// A geographical coordinate that has been reverse geocoded.
    Geocode = 10_012,

    /// A place related to health services.
    Health = 10_013,

    /// A major intersection, usually of two major roads.
    Intersection = 10_014,

    /// A notable or historically significant place.
    Landmark = 10_015,

    /// A geographical feature that occurs naturally.
    NaturalFeature = 10_016,

    /// A named residential or commercial area within a locality.
    Neighborhood = 10_017,

    /// A religious site or building for worship.
    PlaceOfWorship = 10_018,

    /// An encoded location reference derived from latitude and longitude.
    PlusCode = 10_019,

    /// A notable location or attraction.
    PointOfInterest = 10_020,

    /// A political entity or administrative division.
    Political = 10_021,

    /// A prefix component of a postal code.
    PostalCodePrefix = 10_022,

    /// A suffix component of a postal code.
    PostalCodeSuffix = 10_023,

    /// A grouping of geographic areas used for mailing addresses.
    PostalTown = 10_024,

    /// A named location, usually a building or collection of buildings.
    Premise = 10_025,

    /// A named route such as a highway or street.
    Route = 10_026,

    /// A precise street address designation.
    StreetAddress = 10_027,

    /// A first-order civil entity below a locality.
    Sublocality = 10_028,

    /// A first-level subdivision within a sublocality.
    #[serde(rename = "sublocality_level_1")]
    #[strum(serialize = "sublocality_level_1")]
    SublocalityLevel1 = 10_029,

    /// A second-level subdivision within a sublocality.
    #[serde(rename = "sublocality_level_2")]
    #[strum(serialize = "sublocality_level_2")]
    SublocalityLevel2 = 10_030,

    /// A third-level subdivision within a sublocality.
    #[serde(rename = "sublocality_level_3")]
    #[strum(serialize = "sublocality_level_3")]
    SublocalityLevel3 = 10_031,

    /// A fourth-level subdivision within a sublocality.
    #[serde(rename = "sublocality_level_4")]
    #[strum(serialize = "sublocality_level_4")]
    SublocalityLevel4 = 10_032,

    /// A fifth-level subdivision within a sublocality.
    #[serde(rename = "sublocality_level_5")]
    #[strum(serialize = "sublocality_level_5")]
    SublocalityLevel5 = 10_033,

    /// An addressable entity below the premise level, such as an apartment or suite.
    Subpremise = 10_034,

    /// A public square or plaza in a town or city.
    TownSquare = 10_035,

    // ---------------------------------------------------------------------------------------------
    // New and undocumented types
    // ---------------------------------------------------------------------------------------------

    /// A place type that this crate does not recognize.
    ///
    /// Google adds new place types over time, so responses can contain types that were unknown when
    /// this version of the crate was published. Those types deserialize into this variant instead
    /// of failing.
    #[serde(other)]
    Unknown = 65_535,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl PlaceType {
    //   ______      __    __          ___
    //  /_  __/___ _/ /_  / /__       /   |
    //   / / / __ `/ __ \/ / _ \     / /| |
    //  / / / /_/ / /_/ / /  __/    / ___ |
    // /_/  \__,_/_.___/_/\___/    /_/  |_|
    //
    // <https://developers.google.com/maps/documentation/places/web-service/place-types#table-a>

    /// Returns whether this place type belongs to Table A.
    ///
    /// [Table A](https://developers.google.com/maps/documentation/places/web-service/place-types#table-a)
    /// lists the types that are used in the following ways:
    ///
    /// - As part of a response from Place Details (New), Nearby Search (New), and
    ///   Text Search (New). The request must specify at least one of the `places.types` or
    ///   `places.primaryType` fields in the field mask. The values in Table A are then used to
    ///   populate those fields.
    ///
    /// - As part of a Nearby Search (New) request, used as the value of the `includedTypes`,
    ///   `excludedTypes`, `includedPrimaryTypes`, and `excludedPrimaryTypes` parameter. The values
    ///   in Table B are then used to populate those fields.
    ///
    /// - As part of a Text Search (New) request, used as the value of the `includedType` parameter.
    ///
    /// - As part of a Autocomplete (New) request, use as the values to the `includedPrimaryTypes`
    ///   parameter.
    ///
    /// - As part of a Autocomplete (New) response.
    #[must_use]
    pub const fn is_table_a(self) -> bool {
        (self as u16) < 10_000
    }

    /// Returns whether this place type is in the Automotive category.
    ///
    /// Automotive places include car dealers, gas stations, parking facilities, and other
    /// vehicle-related services.
    #[must_use]
    pub const fn is_automotive(self) -> bool {
        let discriminant = self as u16;
        discriminant <= 99
    }

    /// Returns whether this place type is in the Business category.
    ///
    /// Business places include corporate offices, farms, and ranches.
    #[must_use]
    pub const fn is_business(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 100 && discriminant <= 199
    }

    /// Returns whether this place type is in the Culture category.
    ///
    /// Culture places include art galleries, museums, monuments, and other cultural landmarks and
    /// artistic venues.
    #[must_use]
    pub const fn is_culture(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 200 && discriminant <= 299
    }

    /// Returns whether this place type is in the Education category.
    ///
    /// Education places include schools, libraries, universities, and other learning and academic
    /// institutions.
    #[must_use]
    pub const fn is_education(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 300 && discriminant <= 399
    }

    /// Returns whether this place type is in the Entertainment and Recreation category.
    ///
    /// Entertainment places include parks, theaters, casinos, sports venues, and recreational
    /// facilities for leisure activities.
    #[must_use]
    pub const fn is_entertainment_and_recreation(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 400 && discriminant <= 599
    }

    /// Returns whether this place type is in the Facilities category.
    ///
    /// Facilities include public bathrooms, stables, and other utility or service facilities.
    #[must_use]
    pub const fn is_facility(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 600 && discriminant <= 699
    }

    /// Returns whether this place type is in the Finance category.
    ///
    /// Finance places include banks, ATMs, and accounting services.
    #[must_use]
    pub const fn is_finance(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 700 && discriminant <= 799
    }

    /// Returns whether this place type is in the Food and Drink category.
    ///
    /// Food and drink places include restaurants, cafes, bars, and any establishment serving food
    /// or beverages.
    #[must_use]
    pub const fn is_food_and_drink(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 800 && discriminant <= 1_499
    }

    /// Returns whether this place type is in the Geographical Areas category.
    ///
    /// Geographical areas include countries, localities, postal codes, and administrative regions.
    #[must_use]
    pub const fn is_geographical_area(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 1_500 && discriminant <= 1_599
    }

    /// Returns whether this place type is in the Government category.
    ///
    /// Government places include city halls, police stations, fire stations, and other civic and
    /// government facilities.
    #[must_use]
    pub const fn is_government(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 1_600 && discriminant <= 1_699
    }

    /// Returns whether this place type is in the Health and Wellness category.
    ///
    /// Health and wellness places include hospitals, clinics, pharmacies, spas, and medical or
    /// wellness service providers.
    #[must_use]
    pub const fn is_health_and_wellness(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 1_700 && discriminant <= 1_799
    }

    /// Returns whether this place type is in the Housing category.
    ///
    /// Housing places include apartment buildings, condominium complexes, and residential
    /// developments.
    #[must_use]
    pub const fn is_housing(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 1_800 && discriminant <= 1_899
    }

    /// Returns whether this place type is in the Lodging category.
    ///
    /// Lodging places include hotels, motels, bed and breakfasts, and other temporary accommodation
    /// facilities.
    #[must_use]
    pub const fn is_lodging(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 1_900 && discriminant <= 1_999
    }

    /// Returns whether this place type is in the Natural Features category.
    ///
    /// Natural features include beaches, parks, and other naturally occurring geographical
    /// landmarks.
    #[must_use]
    pub const fn is_natural_feature(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 2_000 && discriminant <= 2_099
    }

    /// Returns whether this place type is in the Places of Worship category.
    ///
    /// Places of worship include churches, mosques, synagogues, temples, and other religious
    /// buildings and sites.
    #[must_use]
    pub const fn is_place_of_worship(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 2_100 && discriminant <= 2_199
    }

    /// Returns whether this place type is in the Services category.
    ///
    /// Services include hair salons, repair services, professional services, and other businesses
    /// providing specialized services.
    #[must_use]
    pub const fn is_service(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 2_200 && discriminant <= 2_399
    }

    /// Returns whether this place type is in the Shopping category.
    ///
    /// Shopping places include stores, markets, malls, and retail establishments selling goods and
    /// merchandise.
    #[must_use]
    pub const fn is_shopping(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 2_400 && discriminant <= 2_599
    }

    /// Returns whether this place type is in the Sports category.
    ///
    /// Sports places include gyms, stadiums, golf courses, and facilities for athletic activities
    /// and sports competitions.
    #[must_use]
    pub const fn is_sports(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 2_600 && discriminant <= 2_699
    }

    /// Returns whether this place type is in the Transportation category.
    ///
    /// Transportation places include airports, train stations, bus stops, and other facilities for
    /// moving people and goods.
    #[must_use]
    pub const fn is_transportation(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 2_700 && discriminant <= 2_799
    }

    //   ______      __    __          ____
    //  /_  __/___ _/ /_  / /__       / __ )
    //   / / / __ `/ __ \/ / _ \     / __  |
    //  / / / /_/ / /_/ / /  __/    / /_/ /
    // /_/  \__,_/_.___/_/\___/    /_____/
    //
    // <https://developers.google.com/maps/documentation/places/web-service/place-types#table-b>

    /// Returns whether this place type belongs to Table B.
    ///
    /// [Table B](https://developers.google.com/maps/documentation/places/web-service/place-types#table-b)
    /// lists additional place type values which may also be returned as part of a
    /// Place Details (New), Nearby Search (New), Text Search (New), and Autocomplete (New)
    /// response. The request must specify at least one of the `places.types` or
    /// `places.primaryType` fields in the field mask. Values from Table B may NOT be used as part
    /// of a request, except as the values to the `includedPrimaryTypes` parameter for a
    /// Autocomplete (New) request.
    #[must_use]
    pub const fn is_table_b(self) -> bool {
        (self as u16) >= 10_000 && (self as u16) < 65_535
    }
}