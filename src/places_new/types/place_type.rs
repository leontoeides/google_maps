use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

// -------------------------------------------------------------------------------------------------
//
// Last updated: 2025-08-31
//
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
    Copy,
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
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
    // Automotive (0-9)
    // ---------------------------------------------------------------------------------------------

    /// A business that sells cars.
    CarDealer = 0,

    /// A business that rents cars for short-term use.
    CarRental = 1,

    /// A business that provides automotive repair services.
    CarRepair = 2,

    /// A facility for washing vehicles.
    CarWash = 3,

    /// A station for charging electric vehicles.
    ElectricVehicleChargingStation = 4,

    /// A facility that sells fuel for motor vehicles.
    GasStation = 5,

    /// A facility for parking vehicles.
    Parking = 6,

    /// A roadside facility for travelers to rest.
    RestStop = 7,

    // ---------------------------------------------------------------------------------------------
    // Business (10-19)
    // ---------------------------------------------------------------------------------------------

    /// A building or office space designated for business activities.
    CorporateOffice = 10,

    /// An agricultural facility for growing crops or raising livestock.
    Farm = 11,

    /// A large farm for raising horses, cattle, or other livestock.
    Ranch = 12,

    // ---------------------------------------------------------------------------------------------
    // Culture (20-29)
    // ---------------------------------------------------------------------------------------------

    /// A facility for displaying works of art.
    ArtGallery = 20,

    /// A workspace for creating art.
    ArtStudio = 21,

    /// A large room or hall for public gatherings or performances.
    Auditorium = 22,

    /// A site of cultural or historical significance.
    CulturalLandmark = 23,

    /// A place of historical importance.
    HistoricalPlace = 24,

    /// A structure built to commemorate a person or event.
    Monument = 25,

    /// An institution for preserving and exhibiting objects of cultural importance.
    Museum = 26,

    /// A venue for theatrical performances.
    PerformingArtsTheater = 27,

    /// A three-dimensional work of art.
    Sculpture = 28,

    // ---------------------------------------------------------------------------------------------
    // Education (30-39)
    // ---------------------------------------------------------------------------------------------

    /// A facility containing books and other materials for reading and study.
    Library = 30,

    /// An educational institution for young children before primary school.
    Preschool = 31,

    /// An elementary school for children in early grades.
    PrimarySchool = 32,

    /// An educational institution for children and adolescents.
    School = 33,

    /// A high school for adolescents.
    SecondarySchool = 34,

    /// An institution of higher education.
    University = 35,

    // ---------------------------------------------------------------------------------------------
    // Entertainment and Recreation (40-89)
    // ---------------------------------------------------------------------------------------------

    /// A facility offering adventure sports activities.
    AdventureSportsCenter = 40,

    /// An outdoor venue for performances with tiered seating.
    Amphitheatre = 41,

    /// A facility providing various forms of entertainment.
    AmusementCenter = 42,

    /// A park with rides and attractions for entertainment.
    AmusementPark = 43,

    /// A facility for displaying marine life.
    Aquarium = 44,

    /// A hall for hosting banquets and large dining events.
    BanquetHall = 45,

    /// An area designated for outdoor cooking and grilling.
    BarbecueArea = 46,

    /// A garden featuring a wide variety of plants for scientific study and display.
    BotanicalGarden = 47,

    /// A facility for the sport of bowling.
    BowlingAlley = 48,

    /// A facility for gambling and gaming.
    Casino = 49,

    /// A recreational facility for children's camping activities.
    ChildrensCamp = 50,

    /// A venue for comedy performances.
    ComedyClub = 51,

    /// A facility serving the local community for various activities.
    CommunityCenter = 52,

    /// A venue designed for musical performances.
    ConcertHall = 53,

    /// A facility for large meetings, conferences, and exhibitions.
    ConventionCenter = 54,

    /// A facility dedicated to cultural activities and events.
    CulturalCenter = 55,

    /// A park designed for bicycle riding and cycling activities.
    CyclingPark = 56,

    /// A venue for dancing and dance events.
    DanceHall = 57,

    /// A park specifically designed for dogs to exercise and play.
    DogPark = 58,

    /// A facility for hosting events and gatherings.
    EventVenue = 59,

    /// A large rotating wheel ride at an amusement park.
    FerrisWheel = 60,

    /// A cultivated area for growing flowers, plants, or vegetables.
    Garden = 61,

    /// An area designated for hiking and walking trails.
    HikingArea = 62,

    /// A site of historical significance.
    HistoricalLandmark = 63,

    /// A facility providing internet access to customers.
    InternetCafe = 64,

    /// A venue for karaoke entertainment.
    Karaoke = 65,

    /// A harbor facility for boats and yachts.
    Marina = 66,

    /// A business that rents movies for home viewing.
    MovieRental = 67,

    /// A venue for showing films.
    MovieTheater = 68,

    /// A park protected by the national government.
    NationalPark = 69,

    /// An entertainment venue operating primarily at night.
    NightClub = 70,

    /// A platform offering panoramic views of the surrounding area.
    ObservationDeck = 71,

    /// An area designated for off-road vehicle activities.
    OffRoadingArea = 72,

    /// A venue for opera performances.
    OperaHouse = 73,

    /// A public area of land for recreation and enjoyment.
    Park = 74,

    /// A concert hall specifically for philharmonic orchestras.
    PhilharmonicHall = 75,

    /// An area designated for picnicking.
    PicnicGround = 76,

    /// A facility for astronomical observation and education.
    Planetarium = 77,

    /// An open public square or marketplace.
    Plaza = 78,

    /// A ride at an amusement park featuring a track with steep slopes and sharp turns.
    RollerCoaster = 79,

    /// A park designed for skateboarding activities.
    SkateboardPark = 80,

    /// A park protected by state government.
    StatePark = 81,

    /// A place of interest for tourists.
    TouristAttraction = 82,

    /// A facility with coin-operated video games.
    VideoArcade = 83,

    /// A facility providing information and services to tourists.
    VisitorCenter = 84,

    /// A recreational facility with water attractions.
    WaterPark = 85,

    /// A facility for hosting wedding ceremonies and receptions.
    WeddingVenue = 86,

    /// A park for observing wildlife in natural habitats.
    WildlifePark = 87,

    /// A protected area for wildlife conservation.
    WildlifeRefuge = 88,

    /// A facility for displaying animals for public viewing.
    Zoo = 89,

    // ---------------------------------------------------------------------------------------------
    // Facilities (90-99)
    // ---------------------------------------------------------------------------------------------

    /// A facility for public bathing.
    PublicBath = 90,

    /// A public restroom facility.
    PublicBathroom = 91,

    /// A building for housing horses or livestock.
    Stable = 92,

    // ---------------------------------------------------------------------------------------------
    // Finance (100-109)
    // ---------------------------------------------------------------------------------------------

    /// A business providing accounting and bookkeeping services.
    Accounting = 100,

    /// An automated teller machine for banking transactions.
    Atm = 101,

    /// A financial institution that accepts deposits and makes loans.
    Bank = 102,

    // ---------------------------------------------------------------------------------------------
    // Food and Drink (110-179)
    // ---------------------------------------------------------------------------------------------

    /// A shop specializing in acai bowls and related products.
    AcaiShop = 110,

    /// A restaurant serving Afghan cuisine.
    AfghaniRestaurant = 111,

    /// A restaurant serving African cuisine.
    AfricanRestaurant = 112,

    /// A restaurant serving American cuisine.
    AmericanRestaurant = 113,

    /// A restaurant serving Asian cuisine.
    AsianRestaurant = 114,

    /// A shop specializing in bagels.
    BagelShop = 115,

    /// A shop that bakes and sells bread, cakes, and pastries.
    Bakery = 116,

    /// An establishment serving alcoholic beverages.
    Bar = 117,

    /// A restaurant that combines bar and grill services.
    BarAndGrill = 118,

    /// A restaurant specializing in barbecued food.
    BarbecueRestaurant = 119,

    /// A restaurant serving Brazilian cuisine.
    BrazilianRestaurant = 120,

    /// A restaurant serving breakfast meals.
    BreakfastRestaurant = 121,

    /// A restaurant serving brunch meals.
    BrunchRestaurant = 122,

    /// A restaurant offering self-service dining from a variety of dishes.
    BuffetRestaurant = 123,

    /// A casual establishment serving coffee, light meals, and snacks.
    Cafe = 124,

    /// A dining facility typically found in institutions like schools or offices.
    Cafeteria = 125,

    /// A shop selling confections and candy.
    CandyStore = 126,

    /// A cafe where customers can interact with cats.
    CatCafe = 127,

    /// A restaurant serving Chinese cuisine.
    ChineseRestaurant = 128,

    /// A facility that manufactures chocolate products.
    ChocolateFactory = 129,

    /// A shop specializing in chocolate products.
    ChocolateShop = 130,

    /// An establishment primarily serving coffee and coffee-based beverages.
    CoffeeShop = 131,

    /// A shop selling confections and sweets.
    Confectionery = 132,

    /// A shop selling sliced meats, cheeses, and prepared foods.
    Deli = 133,

    /// A restaurant specializing in desserts.
    DessertRestaurant = 134,

    /// A shop specializing in desserts and sweets.
    DessertShop = 135,

    /// A casual restaurant, typically serving American fare.
    Diner = 136,

    /// A cafe where customers can interact with dogs.
    DogCafe = 137,

    /// A shop specializing in donuts and related pastries.
    DonutShop = 138,

    /// A restaurant offering quick service meals.
    FastFoodRestaurant = 139,

    /// An upscale restaurant offering high-quality cuisine and service.
    FineDiningRestaurant = 140,

    /// An area with multiple food vendors and shared seating.
    FoodCourt = 141,

    /// A restaurant serving French cuisine.
    FrenchRestaurant = 142,

    /// A restaurant serving Greek cuisine.
    GreekRestaurant = 143,

    /// A restaurant specializing in hamburgers.
    HamburgerRestaurant = 144,

    /// A shop selling ice cream and frozen treats.
    IceCreamShop = 145,

    /// A restaurant serving Indian cuisine.
    IndianRestaurant = 146,

    /// A restaurant serving Indonesian cuisine.
    IndonesianRestaurant = 147,

    /// A restaurant serving Italian cuisine.
    ItalianRestaurant = 148,

    /// A restaurant serving Japanese cuisine.
    JapaneseRestaurant = 149,

    /// A shop specializing in fresh juices and smoothies.
    JuiceShop = 150,

    /// A restaurant serving Korean cuisine.
    KoreanRestaurant = 151,

    /// A restaurant serving Lebanese cuisine.
    LebaneseRestaurant = 152,

    /// A service that delivers prepared meals.
    MealDelivery = 153,

    /// A restaurant offering meals for takeaway.
    MealTakeaway = 154,

    /// A restaurant serving Mediterranean cuisine.
    MediterraneanRestaurant = 155,

    /// A restaurant serving Mexican cuisine.
    MexicanRestaurant = 156,

    /// A restaurant serving Middle Eastern cuisine.
    MiddleEasternRestaurant = 157,

    /// A restaurant specializing in pizza.
    PizzaRestaurant = 158,

    /// A tavern or bar serving food and alcoholic beverages.
    Pub = 159,

    /// A restaurant specializing in ramen noodle soup.
    RamenRestaurant = 160,

    /// A business serving prepared food and beverages to customers.
    Restaurant = 161,

    /// A shop specializing in sandwiches.
    SandwichShop = 162,

    /// A restaurant serving seafood.
    SeafoodRestaurant = 163,

    /// A restaurant serving Spanish cuisine.
    SpanishRestaurant = 164,

    /// A restaurant specializing in steak and other grilled meats.
    SteakHouse = 165,

    /// A restaurant specializing in sushi.
    SushiRestaurant = 166,

    /// An establishment serving tea and light refreshments.
    TeaHouse = 167,

    /// A restaurant serving Thai cuisine.
    ThaiRestaurant = 168,

    /// A restaurant serving Turkish cuisine.
    TurkishRestaurant = 169,

    /// A restaurant serving vegan food.
    VeganRestaurant = 170,

    /// A restaurant serving vegetarian food.
    VegetarianRestaurant = 171,

    /// A restaurant serving Vietnamese cuisine.
    VietnameseRestaurant = 172,

    /// A bar specializing in wine.
    WineBar = 173,

    // ---------------------------------------------------------------------------------------------
    // Geographical Areas (180-189)
    // ---------------------------------------------------------------------------------------------

    /// A first-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_1")]
    AdministrativeAreaLevel1 = 180,

    /// A second-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_2")]
    AdministrativeAreaLevel2 = 181,

    /// A sovereign nation.
    Country = 182,

    /// An incorporated city or town.
    Locality = 183,

    /// A postal code area.
    PostalCode = 184,

    /// A district defined for school administrative purposes.
    SchoolDistrict = 185,

    // ---------------------------------------------------------------------------------------------
    // Government (190-199)
    // ---------------------------------------------------------------------------------------------

    /// The administrative center of a city or town.
    CityHall = 190,

    /// A building housing judicial courts.
    Courthouse = 191,

    /// The diplomatic mission of one country in another.
    Embassy = 192,

    /// A facility for firefighting services.
    FireStation = 193,

    /// A building housing government offices.
    GovernmentOffice = 194,

    /// A local government administrative office.
    LocalGovernmentOffice = 195,

    /// A local police station (specific to Japan).
    NeighborhoodPoliceStation = 196,

    /// A law enforcement facility.
    Police = 197,

    /// A facility for postal and mailing services.
    PostOffice = 198,

    // ---------------------------------------------------------------------------------------------
    // Health and Wellness (200-219)
    // ---------------------------------------------------------------------------------------------

    /// A healthcare practitioner specializing in spinal adjustment.
    Chiropractor = 200,

    /// A clinic specializing in dental care.
    DentalClinic = 201,

    /// A healthcare professional specializing in dental care.
    Dentist = 202,

    /// A medical practitioner.
    Doctor = 203,

    /// A retail pharmacy.
    Drugstore = 204,

    /// A medical facility for patient care.
    Hospital = 205,

    /// A facility offering therapeutic massage services.
    Massage = 206,

    /// A facility for medical testing and analysis.
    MedicalLab = 207,

    /// A facility dispensing prescription medications.
    Pharmacy = 208,

    /// A healthcare professional specializing in physical therapy.
    Physiotherapist = 209,

    /// A facility offering sauna services.
    Sauna = 210,

    /// A clinic specializing in skin care treatments.
    SkinCareClinic = 211,

    /// A facility offering health and wellness treatments.
    Spa = 212,

    /// A facility offering tanning services.
    TanningStudio = 213,

    /// A facility focused on health and wellness services.
    WellnessCenter = 214,

    /// A studio offering yoga classes and instruction.
    YogaStudio = 215,

    // ---------------------------------------------------------------------------------------------
    // Housing (220-229)
    // ---------------------------------------------------------------------------------------------

    /// A residential building divided into individual apartments.
    ApartmentBuilding = 220,

    /// A group of apartment buildings managed together.
    ApartmentComplex = 221,

    /// A residential complex of individually owned units.
    CondominiumComplex = 222,

    /// A residential development with multiple housing units.
    HousingComplex = 223,

    // ---------------------------------------------------------------------------------------------
    // Lodging (230-249)
    // ---------------------------------------------------------------------------------------------

    /// A small lodging establishment offering overnight accommodation and breakfast.
    BedAndBreakfast = 230,

    /// An affordable Japanese-style inn.
    BudgetJapaneseInn = 231,

    /// A recreational area for camping with facilities.
    Campground = 232,

    /// A small cabin for camping and outdoor recreation.
    CampingCabin = 233,

    /// A small house, typically in a rural area.
    Cottage = 234,

    /// A hotel designed for extended stays.
    ExtendedStayHotel = 235,

    /// A working farm offering accommodation to visitors.
    Farmstay = 236,

    /// A private home offering accommodation to travelers.
    GuestHouse = 237,

    /// A budget accommodation with shared facilities.
    Hostel = 238,

    /// An establishment providing lodging and services for travelers.
    Hotel = 239,

    /// A small hotel or lodging establishment.
    Inn = 240,

    /// A traditional Japanese inn.
    JapaneseInn = 241,

    /// A place that provides temporary accommodation.
    Lodging = 242,

    /// A park designed for mobile homes.
    MobileHomePark = 243,

    /// A roadside hotel for motorists.
    Motel = 244,

    /// A private room in someone's home for guests.
    PrivateGuestRoom = 245,

    /// A hotel at a resort destination.
    ResortHotel = 246,

    /// A park for recreational vehicles.
    RvPark = 247,

    // ---------------------------------------------------------------------------------------------
    // Natural Features (250-259)
    // ---------------------------------------------------------------------------------------------

    /// A sandy or pebbly shore along a body of water.
    Beach = 250,

    // ---------------------------------------------------------------------------------------------
    // Places of Worship (260-269)
    // ---------------------------------------------------------------------------------------------

    /// A Christian place of worship.
    Church = 260,

    /// A Hindu place of worship.
    HinduTemple = 261,

    /// An Islamic place of worship.
    Mosque = 262,

    /// A Jewish place of worship.
    Synagogue = 263,

    // ---------------------------------------------------------------------------------------------
    // Services (270-309)
    // ---------------------------------------------------------------------------------------------

    /// A person who practices astrology.
    Astrologer = 270,

    /// A shop providing hair cutting and styling for men.
    BarberShop = 271,

    /// A professional who provides beauty treatments.
    Beautician = 272,

    /// A facility offering beauty and cosmetic services.
    BeautySalon = 273,

    /// A service providing body art such as tattoos or piercings.
    BodyArtService = 274,

    /// A service providing food for events and gatherings.
    CateringService = 275,

    /// A burial ground for the deceased.
    Cemetery = 276,

    /// An agency providing child care services.
    ChildCareAgency = 277,

    /// A professional who provides expert advice.
    Consultant = 278,

    /// A service for delivering packages and mail.
    CourierService = 279,

    /// A professional who installs and maintains electrical systems.
    Electrician = 280,

    /// A person or business that sells and arranges flowers.
    Florist = 281,

    /// A service that delivers food from restaurants.
    FoodDelivery = 282,

    /// A service providing foot care and treatment.
    FootCare = 283,

    /// A business that handles funeral arrangements and services.
    FuneralHome = 284,

    /// A service providing hair care and styling.
    HairCare = 285,

    /// A salon providing hair care and styling services.
    HairSalon = 286,

    /// An agency providing insurance services.
    InsuranceAgency = 287,

    /// A service for cleaning clothes and textiles.
    Laundry = 288,

    /// A legal professional who practices law.
    Lawyer = 289,

    /// A professional who installs and repairs locks.
    Locksmith = 290,

    /// A professional who provides makeup and cosmetic services.
    MakeupArtist = 291,

    /// A company that provides relocation and moving services.
    MovingCompany = 292,

    /// A salon providing nail care and manicure services.
    NailSalon = 293,

    /// A professional who paints buildings and structures.
    Painter = 294,

    /// A professional who installs and repairs plumbing systems.
    Plumber = 295,

    /// A person who claims to have supernatural abilities for divination.
    Psychic = 296,

    /// An agency that deals with buying and selling real estate.
    RealEstateAgency = 297,

    /// A contractor specializing in roof installation and repair.
    RoofingContractor = 298,

    /// A facility for storing goods and belongings.
    Storage = 299,

    /// A service that organizes summer camps for children.
    SummerCampOrganizer = 300,

    /// A professional who makes and repairs clothing.
    Tailor = 301,

    /// A company providing telecommunications services.
    TelecommunicationsServiceProvider = 302,

    /// An agency that organizes tours and travel experiences.
    TourAgency = 303,

    /// A center providing information and services to tourists.
    TouristInformationCenter = 304,

    /// An agency that arranges travel and accommodations.
    TravelAgency = 305,

    /// A professional providing medical care for animals.
    VeterinaryCare = 306,

    // ---------------------------------------------------------------------------------------------
    // Shopping (310-339)
    // ---------------------------------------------------------------------------------------------

    /// A grocery store specializing in Asian products.
    AsianGroceryStore = 310,

    /// A store selling automotive parts and accessories.
    AutoPartsStore = 311,

    /// A store specializing in bicycles and cycling equipment.
    BicycleStore = 312,

    /// A store selling books and reading materials.
    BookStore = 313,

    /// A shop that sells fresh meat.
    ButcherShop = 314,

    /// A store selling mobile phones and accessories.
    CellPhoneStore = 315,

    /// A store selling clothing and apparel.
    ClothingStore = 316,

    /// A small retail store with everyday items.
    ConvenienceStore = 317,

    /// A large retail store selling various goods across multiple departments.
    DepartmentStore = 318,

    /// A store offering merchandise at reduced prices.
    DiscountStore = 319,

    /// A store specializing in electronic devices and equipment.
    ElectronicsStore = 320,

    /// A store selling food and grocery items.
    FoodStore = 321,

    /// A store selling furniture and home furnishings.
    FurnitureStore = 322,

    /// A store specializing in gifts and novelty items.
    GiftShop = 323,

    /// A store selling food and household supplies.
    GroceryStore = 324,

    /// A store selling tools and hardware supplies.
    HardwareStore = 325,

    /// A store selling household items and home decor.
    HomeGoodsStore = 326,

    /// A store selling supplies for home improvement and renovation.
    HomeImprovementStore = 327,

    /// A store selling jewelry and precious accessories.
    JewelryStore = 328,

    /// A store selling alcoholic beverages.
    LiquorStore = 329,

    /// A place for buying and selling goods, often outdoors.
    Market = 330,

    /// A store selling pet supplies and accessories.
    PetStore = 331,

    /// A store specializing in footwear.
    ShoeStore = 332,

    /// A large complex containing multiple retail stores.
    ShoppingMall = 333,

    /// A store selling sports equipment and athletic gear.
    SportingGoodsStore = 334,

    /// A retail establishment selling goods or services.
    Store = 335,

    /// A large self-service store selling groceries and household goods.
    Supermarket = 336,

    /// A large store selling goods in bulk quantities.
    WarehouseStore = 337,

    /// A business that sells goods in large quantities to retailers.
    Wholesaler = 338,

    // ---------------------------------------------------------------------------------------------
    // Sports (340-359)
    // ---------------------------------------------------------------------------------------------

    /// A large venue for sports events and competitions.
    Arena = 340,

    /// A field or area designated for athletic activities.
    AthleticField = 341,

    /// A service providing fishing trips on a chartered boat.
    FishingCharter = 342,

    /// A small body of water stocked with fish for recreational fishing.
    FishingPond = 343,

    /// A facility offering exercise equipment and fitness classes.
    FitnessCenter = 344,

    /// A course designed for playing golf.
    GolfCourse = 345,

    /// A facility with exercise equipment for physical fitness.
    Gym = 346,

    /// A rink for ice skating activities.
    IceSkatingRink = 347,

    /// An area designed for children's recreational activities.
    Playground = 348,

    /// A mountain resort facility for skiing and winter sports.
    SkiResort = 349,

    /// A location for various sports activities.
    SportsActivityLocation = 350,

    /// An organization for sports enthusiasts and athletes.
    SportsClub = 351,

    /// A service providing sports instruction and training.
    SportsCoaching = 352,

    /// A facility with multiple sports venues and amenities.
    SportsComplex = 353,

    /// A large venue for sporting events with spectator seating.
    Stadium = 354,

    /// A facility for swimming and aquatic activities.
    SwimmingPool = 355,

    // ---------------------------------------------------------------------------------------------
    // Transportation (360-379)
    // ---------------------------------------------------------------------------------------------

    /// A facility for aircraft takeoffs and landings.
    Airport = 360,

    /// A small airport or landing strip for aircraft.
    Airstrip = 361,

    /// A facility for bus transportation services.
    BusStation = 362,

    /// A designated stopping point for buses.
    BusStop = 363,

    /// A terminal for ferry boat services.
    FerryTerminal = 364,

    /// A landing and takeoff facility for helicopters.
    Heliport = 365,

    /// A large airport serving international flights.
    InternationalAirport = 366,

    /// A station for light rail transit systems.
    LightRailStation = 367,

    /// A facility where commuters can park and use public transit.
    ParkAndRide = 368,

    /// A station for subway or underground rail systems.
    SubwayStation = 369,

    /// A designated area where taxis wait for passengers.
    TaxiStand = 370,

    /// A facility for passenger and freight train services.
    TrainStation = 371,

    /// A facility for public transportation vehicle maintenance and storage.
    TransitDepot = 372,

    /// A facility serving various forms of public transit.
    TransitStation = 373,

    /// A facility providing services for truck drivers and cargo transport.
    TruckStop = 374,

    //   ______      __    __          ____
    //  /_  __/___ _/ /_  / /__       / __ )
    //   / / / __ `/ __ \/ / _ \     / __  |
    //  / / / /_/ / /_/ / /  __/    / /_/ /
    // /_/  \__,_/_.___/_/\___/    /_____/
    //
    // <https://developers.google.com/maps/documentation/places/web-service/place-types#table-b>

    /// A third-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_3")]
    AdministrativeAreaLevel3 = 500,

    /// A fourth-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_4")]
    AdministrativeAreaLevel4 = 501,

    /// A fifth-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_5")]
    AdministrativeAreaLevel5 = 502,

    /// A sixth-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_6")]
    AdministrativeAreaLevel6 = 503,

    /// A seventh-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_8")]
    AdministrativeAreaLevel7 = 504,

    /// A group of islands or a sea containing many islands.
    Archipelago = 505,

    /// A commonly-used alternative name for an entity.
    ColloquialArea = 506,

    /// A large landmass or geographical division.
    Continent = 507,

    /// A business establishment or commercial entity.
    Establishment = 508,

    /// A place related to financial services.
    Finance = 509,

    /// A place related to food services.
    Food = 510,

    /// A contractor that provides general construction services.
    GeneralContractor = 511,

    /// A geographical coordinate that has been reverse geocoded.
    Geocode = 512,

    /// A place related to health services.
    Health = 513,

    /// A major intersection, usually of two major roads.
    Intersection = 514,

    /// A notable or historically significant place.
    Landmark = 515,

    /// A geographical feature that occurs naturally.
    NaturalFeature = 516,

    /// A named residential or commercial area within a locality.
    Neighborhood = 517,

    /// A religious site or building for worship.
    PlaceOfWorship = 518,

    /// An encoded location reference derived from latitude and longitude.
    PlusCode = 519,

    /// A notable location or attraction.
    PointOfInterest = 520,

    /// A political entity or administrative division.
    Political = 521,

    /// A prefix component of a postal code.
    PostalCodePrefix = 522,

    /// A suffix component of a postal code.
    PostalCodeSuffix = 523,

    /// A grouping of geographic areas used for mailing addresses.
    PostalTown = 524,

    /// A named location, usually a building or collection of buildings.
    Premise = 525,

    /// A named route such as a highway or street.
    Route = 526,

    /// A precise street address designation.
    StreetAddress = 527,

    /// A first-order civil entity below a locality.
    Sublocality = 528,

    /// A first-level subdivision within a sublocality.
    #[serde(rename = "sublocality_level_1")]
    SublocalityLevel1 = 529,

    /// A second-level subdivision within a sublocality.
    #[serde(rename = "sublocality_level_2")]
    SublocalityLevel2 = 530,

    /// A third-level subdivision within a sublocality.
    #[serde(rename = "sublocality_level_3")]
    SublocalityLevel3 = 531,

    /// A fourth-level subdivision within a sublocality.
    #[serde(rename = "sublocality_level_4")]
    SublocalityLevel4 = 532,

    /// A fifth-level subdivision within a sublocality.
    #[serde(rename = "sublocality_level_5")]
    SublocalityLevel5 = 533,

    /// An addressable entity below the premise level, such as an apartment or suite.
    Subpremise = 534,

    /// A public square or plaza in a town or city.
    TownSquare = 535,
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
        (self as u16) < 500
    }

    /// Returns whether this place type is in the Automotive category.
    ///
    /// Automotive places include car dealers, gas stations, parking facilities, and other
    /// vehicle-related services.
    #[must_use]
    pub const fn is_automotive(self) -> bool {
        let discriminant = self as u16;
        discriminant <= 9
    }

    /// Returns whether this place type is in the Business category.
    ///
    /// Business places include corporate offices, farms, and ranches.
    #[must_use]
    pub const fn is_business(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 10 && discriminant <= 19
    }

    /// Returns whether this place type is in the Culture category.
    ///
    /// Culture places include art galleries, museums, monuments, and other cultural landmarks and
    /// artistic venues.
    #[must_use]
    pub const fn is_culture(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 20 && discriminant <= 29
    }

    /// Returns whether this place type is in the Education category.
    ///
    /// Education places include schools, libraries, universities, and other learning and academic
    /// institutions.
    #[must_use]
    pub const fn is_education(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 30 && discriminant <= 39
    }

    /// Returns whether this place type is in the Entertainment and Recreation category.
    ///
    /// Entertainment places include parks, theaters, casinos, sports venues, and recreational
    /// facilities for leisure activities.
    #[must_use]
    pub const fn is_entertainment_and_recreation(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 40 && discriminant <= 89
    }

    /// Returns whether this place type is in the Facilities category.
    ///
    /// Facilities include public bathrooms, stables, and other utility or service facilities.
    #[must_use]
    pub const fn is_facility(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 90 && discriminant <= 99
    }

    /// Returns whether this place type is in the Finance category.
    ///
    /// Finance places include banks, ATMs, and accounting services.
    #[must_use]
    pub const fn is_finance(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 100 && discriminant <= 109
    }

    /// Returns whether this place type is in the Food and Drink category.
    ///
    /// Food and drink places include restaurants, cafes, bars, and any establishment serving food
    /// or beverages.
    #[must_use]
    pub const fn is_food_and_drink(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 110 && discriminant <= 179
    }

    /// Returns whether this place type is in the Geographical Areas category.
    ///
    /// Geographical areas include countries, localities, postal codes, and administrative regions.
    #[must_use]
    pub const fn is_geographical_area(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 180 && discriminant <= 189
    }

    /// Returns whether this place type is in the Government category.
    ///
    /// Government places include city halls, police stations, fire stations, and other civic and
    /// government facilities.
    #[must_use]
    pub const fn is_government(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 190 && discriminant <= 199
    }

    /// Returns whether this place type is in the Health and Wellness category.
    ///
    /// Health and wellness places include hospitals, clinics, pharmacies, spas, and medical or
    /// wellness service providers.
    #[must_use]
    pub const fn is_health_and_wellness(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 200 && discriminant <= 219
    }

    /// Returns whether this place type is in the Housing category.
    ///
    /// Housing places include apartment buildings, condominium complexes, and residential
    /// developments.
    #[must_use]
    pub const fn is_housing(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 220 && discriminant <= 229
    }

    /// Returns whether this place type is in the Lodging category.
    ///
    /// Lodging places include hotels, motels, bed and breakfasts, and other temporary accommodation
    /// facilities.
    #[must_use]
    pub const fn is_lodging(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 230 && discriminant <= 249
    }

    /// Returns whether this place type is in the Natural Features category.
    ///
    /// Natural features include beaches, parks, and other naturally occurring geographical
    /// landmarks.
    #[must_use]
    pub const fn is_natural_feature(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 250 && discriminant <= 259
    }

    /// Returns whether this place type is in the Places of Worship category.
    ///
    /// Places of worship include churches, mosques, synagogues, temples, and other religious
    /// buildings and sites.
    #[must_use]
    pub const fn is_place_of_worship(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 260 && discriminant <= 269
    }

    /// Returns whether this place type is in the Services category.
    ///
    /// Services include hair salons, repair services, professional services, and other businesses
    /// providing specialized services.
    #[must_use]
    pub const fn is_service(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 270 && discriminant <= 309
    }

    /// Returns whether this place type is in the Shopping category.
    ///
    /// Shopping places include stores, markets, malls, and retail establishments selling goods and
    /// merchandise.
    #[must_use]
    pub const fn is_shopping(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 310 && discriminant <= 339
    }

    /// Returns whether this place type is in the Sports category.
    ///
    /// Sports places include gyms, stadiums, golf courses, and facilities for athletic activities
    /// and sports competitions.
    #[must_use]
    pub const fn is_sports(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 340 && discriminant <= 359
    }

    /// Returns whether this place type is in the Transportation category.
    ///
    /// Transportation places include airports, train stations, bus stops, and other facilities for
    /// moving people and goods.
    #[must_use]
    pub const fn is_transportation(self) -> bool {
        let discriminant = self as u16;
        discriminant >= 360 && discriminant <= 379
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
        (self as u16) >= 500
    }
}