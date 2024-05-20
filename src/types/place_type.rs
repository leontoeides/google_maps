//! Contains the `PlaceType` enum and its associated traits. It specifies the
//! types or categories of a place. For example, a returned place could be a
//! "country" (as in a nation) or it could be a "shopping mall."

use crate::error::Error as GoogleMapsError;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------

/// This specifies the types or categories of a place. For example, a returned
/// location could be a "country" (as in a nation) or it could be a "shopping
/// mall." Also, a requested place could be a "locality" (a city) or a
/// "`street_address`" This type helps define the data that is being returned or
/// sought. See [Place
/// Types](https://developers.google.com/places/web-service/supported_types)
/// for more information.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
#[non_exhaustive]
pub enum PlaceType {
    // [Table 1: Place types](https://developers.google.com/places/web-service/supported_types#table1)
    // The types that are supported for place searches, and can be returned with Place details results, and as part of autocomplete place predictions.
    Accounting = 0,
    /// Indicates an airport.
    Airport = 1,
    AmusementPark = 2,
    Aquarium = 3,
    ArtGallery = 4,
    Atm = 5,
    Bakery = 6,
    Bank = 7,
    Bar = 8,
    BeautySalon = 9,
    BicycleStore = 10,
    BookStore = 11,
    BowlingAlley = 12,
    BusStation = 13,
    Cafe = 14,
    Campground = 15,
    CarDealer = 16,
    CarRental = 17,
    CarRepair = 18,
    CarWash = 19,
    Casino = 20,
    Cemetery = 21,
    Church = 22,
    CityHall = 23,
    ClothingStore = 24,
    ConvenienceStore = 25,
    Courthouse = 26,
    Dentist = 27,
    DepartmentStore = 28,
    Doctor = 29,
    DrugStore = 30,
    Electrician = 31,
    ElectronicsStore = 32,
    Embassy = 33,
    FireStation = 34,
    Florist = 35,
    FuneralHome = 36,
    FurnitureStore = 37,
    GasStation = 38,
    GroceryOrSupermarket = 39,
    Gym = 40,
    HairCare = 41,
    HardwareStore = 42,
    HinduTemple = 43,
    HomeGoodsStore = 44,
    Hospital = 45,
    InsuranceAgency = 46,
    JewelryStore = 47,
    Laundry = 48,
    Lawyer = 49,
    Library = 50,
    LightRailStation = 51,
    LiquorStore = 52,
    LocalGovernmentOffice = 53,
    Locksmith = 54,
    Lodging = 55,
    MealDelivery = 56,
    MealTakeaway = 57,
    Mosque = 58,
    MovieRental = 59,
    MovieTheater = 60,
    MovingCompany = 61,
    Museum = 62,
    NightClub = 63,
    Painter = 64,
    /// Indicates a named park.
    Park = 65,
    Parking = 66,
    PetStore = 67,
    Pharmacy = 68,
    Physiotherapist = 69,
    Plumber = 70,
    PlusCode = 71,
    Police = 72,
    PostOffice = 73,
    PrimarySchool = 74,
    RealEstateAgency = 75,
    Restaurant = 76,
    RoofingContractor = 77,
    RvPark = 78,
    School = 79,
    SecondarySchool = 80,
    ShoeStore = 81,
    ShoppingMall = 82,
    Spa = 83,
    Stadium = 84,
    Storage = 85,
    Store = 86,
    SubwayStation = 87,
    Supermarket = 88,
    Synagogue = 89,
    TaxiStand = 90,
    TouristAttraction = 91,
    TrainStation = 92,
    TransitStation = 93,
    TravelAgency = 94,
    University = 95,
    VeterinaryCare = 96,
    Zoo = 97,
    // [Table 2: Additional types returned by the Places service](https://developers.google.com/places/web-service/supported_types#table2)
    // Additional types that can be returned with Place details results, and as part of autocomplete place predictions.
    // Note: The types below are *not supported* in the `type` filter of a place search.
    /// Indicates a first-order civil entity below the country level. Within the
    /// United States, these administrative levels are states. Not all nations
    /// exhibit these administrative levels. In most cases,
    /// `AdministrativeAreaLevel1` short names will closely match ISO 3166-2
    /// subdivisions and other widely circulated lists; however this is not
    /// guaranteed as our geocoding results are based on a variety of signals
    /// and location data.
    AdministrativeAreaLevel1 = 98,
    /// Indicates a second-order civil entity below the country level. Within
    /// the United States, these administrative levels are counties. Not all
    /// nations exhibit these administrative levels.
    AdministrativeAreaLevel2 = 99,
    /// Indicates a third-order civil entity below the country level. This type
    /// indicates a minor civil division. Not all nations exhibit these
    /// administrative levels.
    AdministrativeAreaLevel3 = 100,
    /// Indicates a fourth-order civil entity below the country level. This type
    /// indicates a minor civil division. Not all nations exhibit these
    /// administrative levels.
    AdministrativeAreaLevel4 = 101,
    /// Indicates a fifth-order civil entity below the country level. This type
    /// indicates a minor civil division. Not all nations exhibit these
    /// administrative levels.
    AdministrativeAreaLevel5 = 102,
    Archipelago = 103,
    /// Indicates a commonly-used alternative name for the entity.
    ColloquialArea = 104,
    Continent = 105,
    /// Indicates the national political entity, and is typically the highest
    /// order type returned by the Geocoder.
    Country = 106,
    Establishment = 107,
    Finance = 108,
    Floor = 109,
    Food = 110,
    GeneralContractor = 111,
    Geocode = 112,
    Health = 113,
    /// Indicates a major intersection, usually of two major roads.
    Intersection = 114,
    /// Indicates an incorporated city or town political entity.
    #[default]
    Locality = 115,
    /// Indicates a prominent natural feature.
    NaturalFeature = 116,
    /// Indicates a named neighborhood.
    Neighborhood = 117,
    PlaceOfWorship = 118,
    /// Indicates a named point of interest. Typically, these "POI"s are
    /// prominent local entities that don't easily fit in another category, such
    /// as "Empire State Building" or "Eiffel Tower".
    PointOfInterest = 119,
    /// Indicates a political entity. Usually, this type indicates a polygon of
    /// some civil administration.
    Political = 120,
    PostBox = 121,
    /// Indicates a postal code as used to address postal mail within the
    /// country.
    PostalCode = 122,
    PostalCodePrefix = 123,
    PostalCodeSuffix = 124,
    PostalTown = 125,
    /// Indicates a named location, usually a building or collection of
    /// buildings with a common name.
    Premise = 126,
    Room = 127,
    /// Indicates a named route (such as "US 101").
    Route = 128,
    /// Indicates a precise street address.
    StreetAddress = 129,
    StreetNumber = 130,
    /// Indicates a first-order civil entity below a locality. For some
    /// locations may receive one of the additional types: `SublocalityLevel1`
    /// to `SublocalityLevel5`. Each sublocality level is a civil entity. Larger
    /// numbers indicate a smaller geographic area.
    Sublocality = 131,
    SublocalityLevel1 = 132,
    SublocalityLevel2 = 133,
    SublocalityLevel3 = 134,
    SublocalityLevel4 = 135,
    SublocalityLevel5 = 136,
    /// Indicates a first-order entity below a named location, usually a
    /// singular building within a collection of buildings with a common name.
    Subpremise = 137,
    TownSquare = 138,
    // [Table 3: Types supported in place autocomplete requests](https://developers.google.com/places/web-service/supported_types#table3)
    // Types you can use in place autocomplete requests.
    // #[serde(alias = "geocode")]
    // Geocode,
    Address = 139,
    // #[serde(alias = "establishment")]
    // Establishment,
    Regions = 140,
    Cities = 141,
    Landmark = 142,
    /// If the place type is not recognized by
    /// [serde](https://crates.io/crates/serde) when reading data from
    /// Google it will be assigned to this `Other` variant.
    ///
    /// As new types are added to Google Maps, they must also be added to this
    /// crate. However, in the meantime, the `Other` catch-all variant allows
    /// `serde` to read data from Google without producing an error until the
    /// new variant added to this `enum`.
    Other = 143,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for PlaceType {
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

impl Serialize for PlaceType {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Self> for PlaceType {
    /// Converts a borrowed `&PlaceType` enum into an owned `PlaceType` enum
    /// by copying it.
    fn from(place_type: &Self) -> Self {
        *place_type
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&PlaceType> for &str {
    /// Converts a `PlaceType` enum to a `String` that contains a [place
    /// type](https://developers.google.com/places/web-service/supported_types)
    /// code.
    fn from(place_type: &PlaceType) -> Self {
        match place_type {
            PlaceType::Accounting => "accounting",
            PlaceType::Airport => "airport",
            PlaceType::AmusementPark => "amusement_park",
            PlaceType::Aquarium => "aquarium",
            PlaceType::ArtGallery => "art_gallery",
            PlaceType::Atm => "atm",
            PlaceType::Bakery => "bakery",
            PlaceType::Bank => "bank",
            PlaceType::Bar => "bar",
            PlaceType::BeautySalon => "beauty_salon",
            PlaceType::BicycleStore => "bicycle_store",
            PlaceType::BookStore => "book_store",
            PlaceType::BowlingAlley => "bowling_alley",
            PlaceType::BusStation => "bus_station",
            PlaceType::Cafe => "cafe",
            PlaceType::Campground => "campground",
            PlaceType::CarDealer => "car_dealer",
            PlaceType::CarRental => "car_rental",
            PlaceType::CarRepair => "car_repair",
            PlaceType::CarWash => "car_wash",
            PlaceType::Casino => "casino",
            PlaceType::Cemetery => "cemetery",
            PlaceType::Church => "church",
            PlaceType::CityHall => "city_hall",
            PlaceType::ClothingStore => "clothing_store",
            PlaceType::ConvenienceStore => "convenience_store",
            PlaceType::Courthouse => "courthouse",
            PlaceType::Dentist => "dentist",
            PlaceType::DepartmentStore => "department_store",
            PlaceType::Doctor => "doctor",
            PlaceType::DrugStore => "drugstore",
            PlaceType::Electrician => "electrician",
            PlaceType::ElectronicsStore => "electronics_store",
            PlaceType::Embassy => "embassy",
            PlaceType::FireStation => "fire_station",
            PlaceType::Florist => "florist",
            PlaceType::FuneralHome => "funeral_home",
            PlaceType::FurnitureStore => "furniture_store",
            PlaceType::GasStation => "gas_station",
            PlaceType::GroceryOrSupermarket => "grocery_or_supermarket",
            PlaceType::Gym => "gym",
            PlaceType::HairCare => "hair_care",
            PlaceType::HardwareStore => "hardware_store",
            PlaceType::HinduTemple => "hindu_temple",
            PlaceType::HomeGoodsStore => "home_goods_store",
            PlaceType::Hospital => "hospital",
            PlaceType::InsuranceAgency => "insurance_agency",
            PlaceType::JewelryStore => "jewelry_store",
            PlaceType::Laundry => "laundry",
            PlaceType::Lawyer => "lawyer",
            PlaceType::Library => "library",
            PlaceType::LightRailStation => "light_rail_station",
            PlaceType::LiquorStore => "liquor_store",
            PlaceType::LocalGovernmentOffice => "local_government_office",
            PlaceType::Locksmith => "locksmith",
            PlaceType::Lodging => "lodging",
            PlaceType::MealDelivery => "meal_delivery",
            PlaceType::MealTakeaway => "meal_takeaway",
            PlaceType::Mosque => "mosque",
            PlaceType::MovieRental => "movie_rental",
            PlaceType::MovieTheater => "movie_theater",
            PlaceType::MovingCompany => "moving_company",
            PlaceType::Museum => "museum",
            PlaceType::NightClub => "night_club",
            PlaceType::Painter => "painter",
            PlaceType::Park => "park",
            PlaceType::Parking => "parking",
            PlaceType::PetStore => "pet_store",
            PlaceType::Pharmacy => "pharmacy",
            PlaceType::Physiotherapist => "physiotherapist",
            PlaceType::Plumber => "plumber",
            PlaceType::PlusCode => "plus_code",
            PlaceType::Police => "police",
            PlaceType::PostOffice => "post_office",
            PlaceType::PrimarySchool => "primary_school",
            PlaceType::RealEstateAgency => "real_estate_agency",
            PlaceType::Restaurant => "restaurant",
            PlaceType::RoofingContractor => "roofing_contractor",
            PlaceType::RvPark => "rv_park",
            PlaceType::School => "school",
            PlaceType::SecondarySchool => "secondary_school",
            PlaceType::ShoeStore => "shoe_store",
            PlaceType::ShoppingMall => "shopping_mall",
            PlaceType::Spa => "spa",
            PlaceType::Stadium => "stadium",
            PlaceType::Storage => "storage",
            PlaceType::Store => "store",
            PlaceType::SubwayStation => "subway_station",
            PlaceType::Supermarket => "supermarket",
            PlaceType::Synagogue => "synagogue",
            PlaceType::TaxiStand => "taxi_stand",
            PlaceType::TouristAttraction => "tourist_attraction",
            PlaceType::TrainStation => "train_station",
            PlaceType::TransitStation => "transit_station",
            PlaceType::TravelAgency => "travel_agency",
            PlaceType::University => "university",
            PlaceType::VeterinaryCare => "veterinary_care",
            PlaceType::Zoo => "zoo",
            PlaceType::AdministrativeAreaLevel1 => "administrative_area_level_1",
            PlaceType::AdministrativeAreaLevel2 => "administrative_area_level_2",
            PlaceType::AdministrativeAreaLevel3 => "administrative_area_level_3",
            PlaceType::AdministrativeAreaLevel4 => "administrative_area_level_4",
            PlaceType::AdministrativeAreaLevel5 => "administrative_area_level_5",
            PlaceType::Archipelago => "archipelago",
            PlaceType::ColloquialArea => "colloquial_area",
            PlaceType::Continent => "continent",
            PlaceType::Country => "country",
            PlaceType::Establishment => "establishment",
            PlaceType::Finance => "finance",
            PlaceType::Floor => "floor",
            PlaceType::Food => "food",
            PlaceType::GeneralContractor => "general_contractor",
            PlaceType::Geocode => "geocode",
            PlaceType::Health => "health",
            PlaceType::Intersection => "intersection",
            PlaceType::Locality => "locality",
            PlaceType::NaturalFeature => "natural_feature",
            PlaceType::Neighborhood => "neighborhood",
            PlaceType::PlaceOfWorship => "place_of_worship",
            PlaceType::PointOfInterest => "point_of_interest",
            PlaceType::Political => "political",
            PlaceType::PostBox => "post_box",
            PlaceType::PostalCode => "postal_code",
            PlaceType::PostalCodePrefix => "postal_code_prefix",
            PlaceType::PostalCodeSuffix => "postal_code_suffix",
            PlaceType::PostalTown => "postal_town",
            PlaceType::Premise => "premise",
            PlaceType::Room => "room",
            PlaceType::Route => "route",
            PlaceType::StreetAddress => "street_address",
            PlaceType::StreetNumber => "street_number",
            PlaceType::Sublocality => "sublocality",
            PlaceType::SublocalityLevel1 => "sublocality_level_1",
            PlaceType::SublocalityLevel2 => "sublocality_level_2",
            PlaceType::SublocalityLevel3 => "sublocality_level_3",
            PlaceType::SublocalityLevel4 => "sublocality_level_4",
            PlaceType::SublocalityLevel5 => "sublocality_level_5",
            PlaceType::Subpremise => "subpremise",
            PlaceType::TownSquare => "town_square",
            PlaceType::Address => "address",
            PlaceType::Regions => "regions",
            PlaceType::Cities => "cities",
            PlaceType::Landmark => "landmark",
            PlaceType::Other => "other",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for PlaceType {
    /// Converts a `PlaceType` enum to a `String` that contains a [place
    /// type](https://developers.google.com/places/web-service/supported_types)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&PlaceType> for String {
    /// Converts a `PlaceType` enum to a `String` that contains a [place
    /// type](https://developers.google.com/places/web-service/supported_types)
    /// code.
    fn from(place_type: &PlaceType) -> Self {
        std::convert::Into::<&str>::into(place_type).to_string()
    } // fn
} // impl

// -----------------------------------------------------------------------------

static PLACE_TYPES_BY_CODE: phf::Map<&'static str, PlaceType> = phf_map! {
    "accounting" => PlaceType::Accounting,
    "airport" => PlaceType::Airport,
    "amusement_park" => PlaceType::AmusementPark,
    "aquarium" => PlaceType::Aquarium,
    "art_gallery" => PlaceType::ArtGallery,
    "atm" => PlaceType::Atm,
    "bakery" => PlaceType::Bakery,
    "bank" => PlaceType::Bank,
    "bar" => PlaceType::Bar,
    "beauty_salon" => PlaceType::BeautySalon,
    "bicycle_store" => PlaceType::BicycleStore,
    "book_store" => PlaceType::BookStore,
    "bowling_alley" => PlaceType::BowlingAlley,
    "bus_station" => PlaceType::BusStation,
    "cafe" => PlaceType::Cafe,
    "campground" => PlaceType::Campground,
    "car_dealer" => PlaceType::CarDealer,
    "car_rental" => PlaceType::CarRental,
    "car_repair" => PlaceType::CarRepair,
    "car_wash" => PlaceType::CarWash,
    "casino" => PlaceType::Casino,
    "cemetery" => PlaceType::Cemetery,
    "church" => PlaceType::Church,
    "city_hall" => PlaceType::CityHall,
    "clothing_store" => PlaceType::ClothingStore,
    "convenience_store" => PlaceType::ConvenienceStore,
    "courthouse" => PlaceType::Courthouse,
    "dentist" => PlaceType::Dentist,
    "department_store" => PlaceType::DepartmentStore,
    "doctor" => PlaceType::Doctor,
    "drugstore" => PlaceType::DrugStore,
    "electrician" => PlaceType::Electrician,
    "electronics_store" => PlaceType::ElectronicsStore,
    "embassy" => PlaceType::Embassy,
    "fire_station" => PlaceType::FireStation,
    "florist" => PlaceType::Florist,
    "funeral_home" => PlaceType::FuneralHome,
    "furniture_store" => PlaceType::FurnitureStore,
    "gas_station" => PlaceType::GasStation,
    "grocery_or_supermarket" => PlaceType::GroceryOrSupermarket,
    "gym" => PlaceType::Gym,
    "hair_care" => PlaceType::HairCare,
    "hardware_store" => PlaceType::HardwareStore,
    "hindu_temple" => PlaceType::HinduTemple,
    "home_goods_store" => PlaceType::HomeGoodsStore,
    "hospital" => PlaceType::Hospital,
    "insurance_agency" => PlaceType::InsuranceAgency,
    "jewelry_store" => PlaceType::JewelryStore,
    "laundry" => PlaceType::Laundry,
    "lawyer" => PlaceType::Lawyer,
    "library" => PlaceType::Library,
    "light_rail_station" => PlaceType::LightRailStation,
    "liquor_store" => PlaceType::LiquorStore,
    "local_government_office" => PlaceType::LocalGovernmentOffice,
    "locksmith" => PlaceType::Locksmith,
    "lodging" => PlaceType::Lodging,
    "meal_delivery" => PlaceType::MealDelivery,
    "meal_takeaway" => PlaceType::MealTakeaway,
    "mosque" => PlaceType::Mosque,
    "movie_rental" => PlaceType::MovieRental,
    "movie_theater" => PlaceType::MovieTheater,
    "moving_company" => PlaceType::MovingCompany,
    "museum" => PlaceType::Museum,
    "night_club" => PlaceType::NightClub,
    "painter" => PlaceType::Painter,
    "park" => PlaceType::Park,
    "parking" => PlaceType::Parking,
    "pet_store" => PlaceType::PetStore,
    "pharmacy" => PlaceType::Pharmacy,
    "physiotherapist" => PlaceType::Physiotherapist,
    "plumber" => PlaceType::Plumber,
    "plus_code" => PlaceType::PlusCode,
    "police" => PlaceType::Police,
    "post_office" => PlaceType::PostOffice,
    "primary_school" => PlaceType::PrimarySchool,
    "real_estate_agency" => PlaceType::RealEstateAgency,
    "restaurant" => PlaceType::Restaurant,
    "roofing_contractor" => PlaceType::RoofingContractor,
    "rv_park" => PlaceType::RvPark,
    "school" => PlaceType::School,
    "secondary_school" => PlaceType::SecondarySchool,
    "shoe_store" => PlaceType::ShoeStore,
    "shopping_mall" => PlaceType::ShoppingMall,
    "spa" => PlaceType::Spa,
    "stadium" => PlaceType::Stadium,
    "storage" => PlaceType::Storage,
    "store" => PlaceType::Store,
    "subway_station" => PlaceType::SubwayStation,
    "supermarket" => PlaceType::Supermarket,
    "synagogue" => PlaceType::Synagogue,
    "taxi_stand" => PlaceType::TaxiStand,
    "tourist_attraction" => PlaceType::TouristAttraction,
    "train_station" => PlaceType::TrainStation,
    "transit_station" => PlaceType::TransitStation,
    "travel_agency" => PlaceType::TravelAgency,
    "university" => PlaceType::University,
    "veterinary_care" => PlaceType::VeterinaryCare,
    "zoo" => PlaceType::Zoo,
    "administrative_area_level_1" => PlaceType::AdministrativeAreaLevel1,
    "administrative_area_level_2" => PlaceType::AdministrativeAreaLevel2,
    "administrative_area_level_3" => PlaceType::AdministrativeAreaLevel3,
    "administrative_area_level_4" => PlaceType::AdministrativeAreaLevel4,
    "administrative_area_level_5" => PlaceType::AdministrativeAreaLevel5,
    "archipelago" => PlaceType::Archipelago,
    "colloquial_area" => PlaceType::ColloquialArea,
    "continent" => PlaceType::Continent,
    "country" => PlaceType::Country,
    "establishment" => PlaceType::Establishment,
    "finance" => PlaceType::Finance,
    "floor" => PlaceType::Floor,
    "food" => PlaceType::Food,
    "general_contractor" => PlaceType::GeneralContractor,
    "geocode" => PlaceType::Geocode,
    "health" => PlaceType::Health,
    "intersection" => PlaceType::Intersection,
    "locality" => PlaceType::Locality,
    "natural_feature" => PlaceType::NaturalFeature,
    "neighborhood" => PlaceType::Neighborhood,
    "place_of_worship" => PlaceType::PlaceOfWorship,
    "point_of_interest" => PlaceType::PointOfInterest,
    "political" => PlaceType::Political,
    "post_box" => PlaceType::PostBox,
    "postal_code" => PlaceType::PostalCode,
    "postal_code_prefix" => PlaceType::PostalCodePrefix,
    "postal_code_suffix" => PlaceType::PostalCodeSuffix,
    "postal_town" => PlaceType::PostalTown,
    "premise" => PlaceType::Premise,
    "room" => PlaceType::Room,
    "route" => PlaceType::Route,
    "street_address" => PlaceType::StreetAddress,
    "street_number" => PlaceType::StreetNumber,
    "sublocality" => PlaceType::Sublocality,
    "sublocality_level_1" => PlaceType::SublocalityLevel1,
    "sublocality_level_2" => PlaceType::SublocalityLevel2,
    "sublocality_level_3" => PlaceType::SublocalityLevel3,
    "sublocality_level_4" => PlaceType::SublocalityLevel4,
    "sublocality_level_5" => PlaceType::SublocalityLevel5,
    "subpremise" => PlaceType::Subpremise,
    "town_square" => PlaceType::TownSquare,
    "address" => PlaceType::Address,
    "regions" => PlaceType::Regions,
    "cities" => PlaceType::Cities,
    "landmark" => PlaceType::Landmark,
    "other" => PlaceType::Other,
};

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for PlaceType {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = GoogleMapsError;
    /// Gets a `PlaceType` enum from a `String` that contains a supported [place
    /// type](https://developers.google.com/places/web-service/supported_types)
    /// code.
    fn try_from(place_type_code: &str) -> Result<Self, Self::Error> {
        Ok(PLACE_TYPES_BY_CODE
            .get(place_type_code)
            .copied()
            .unwrap_or(Self::Other))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for PlaceType {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Err = GoogleMapsError;
    /// Gets a `PlaceType` enum from a `String` that contains a supported [place
    /// type](https://developers.google.com/places/web-service/supported_types)
    /// code.
    fn from_str(place_type_code: &str) -> Result<Self, Self::Err> {
        Ok(PLACE_TYPES_BY_CODE
            .get(place_type_code)
            .copied()
            .unwrap_or(Self::Other))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl PlaceType {
    /// Formats a `PlaceType` enum into a string that is presentable to the end
    /// user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::Accounting => "Accounting",
            Self::Airport => "Airport",
            Self::AmusementPark => "Amusement Park",
            Self::Aquarium => "Aquarium",
            Self::ArtGallery => "Art Gallery",
            Self::Atm => "ATM",
            Self::Bakery => "Bakery",
            Self::Bank => "Bank",
            Self::Bar => "Bar",
            Self::BeautySalon => "Beauty Salon",
            Self::BicycleStore => "Bicycle Store",
            Self::BookStore => "Book Store",
            Self::BowlingAlley => "Bowling Alley",
            Self::BusStation => "Bus Station",
            Self::Cafe => "Café",
            Self::Campground => "Campground",
            Self::CarDealer => "Car Dealer",
            Self::CarRental => "Car Rental",
            Self::CarRepair => "Car Repair",
            Self::CarWash => "Car Wash",
            Self::Casino => "Casino",
            Self::Cemetery => "Cemetery",
            Self::Church => "Church",
            Self::CityHall => "City Hall",
            Self::ClothingStore => "Clothing Store",
            Self::ConvenienceStore => "Convenience Store",
            Self::Courthouse => "Courthouse",
            Self::Dentist => "Dentist",
            Self::DepartmentStore => "Department Store",
            Self::Doctor => "Doctor",
            Self::DrugStore => "Drug Store",
            Self::Electrician => "Electrician",
            Self::ElectronicsStore => "Electronics Store",
            Self::Embassy => "Embassy",
            Self::FireStation => "Fire Station",
            Self::Florist => "Florist",
            Self::FuneralHome => "Funeral Home",
            Self::FurnitureStore => "Furniture Store",
            Self::GasStation => "Gas Station",
            Self::GroceryOrSupermarket => "Grocery or Supermarket",
            Self::Gym => "Gym",
            Self::HairCare => "Hair Care",
            Self::HardwareStore => "Hardware Store",
            Self::HinduTemple => "Hindu Temple",
            Self::HomeGoodsStore => "Home Goods Store",
            Self::Hospital => "Hospital",
            Self::InsuranceAgency => "Insurance Agency",
            Self::JewelryStore => "Jewelry Store",
            Self::Laundry => "Laundry",
            Self::Lawyer => "Lawyer",
            Self::Library => "Library",
            Self::LightRailStation => "Light Rail Station",
            Self::LiquorStore => "Liquor Store",
            Self::LocalGovernmentOffice => "Local Government Office",
            Self::Locksmith => "Locksmith",
            Self::Lodging => "Lodging",
            Self::MealDelivery => "Meal Delivery",
            Self::MealTakeaway => "Meal Takeaway",
            Self::Mosque => "Mosque",
            Self::MovieRental => "Movie Rental",
            Self::MovieTheater => "Movie Theater",
            Self::MovingCompany => "Moving Company",
            Self::Museum => "Museum",
            Self::NightClub => "Night Club",
            Self::Painter => "Painter",
            Self::Park => "Park",
            Self::Parking => "Parking",
            Self::PetStore => "Pet Store",
            Self::Pharmacy => "Pharmacy",
            Self::Physiotherapist => "Physiotherapist",
            Self::Plumber => "Plumber",
            Self::PlusCode => "Plus Code",
            Self::Police => "Police",
            Self::PostOffice => "Post Office",
            Self::PrimarySchool => "Primary School",
            Self::RealEstateAgency => "Real Estate Agency",
            Self::Restaurant => "Restaurant",
            Self::RoofingContractor => "Roofing Contractor",
            Self::RvPark => "RV Park",
            Self::School => "School",
            Self::SecondarySchool => "Secondary School",
            Self::ShoeStore => "Shoe Store",
            Self::ShoppingMall => "Shopping Mall",
            Self::Spa => "Spa",
            Self::Stadium => "Stadium",
            Self::Storage => "Storage",
            Self::Store => "Store",
            Self::SubwayStation => "Subway Station",
            Self::Supermarket => "Supermarket",
            Self::Synagogue => "Synagogue",
            Self::TaxiStand => "Taxi Stand",
            Self::TouristAttraction => "Tourist Attraction",
            Self::TrainStation => "Train Station",
            Self::TransitStation => "Transit Station",
            Self::TravelAgency => "Travel Agency",
            Self::University => "University",
            Self::VeterinaryCare => "Veterinary Care",
            Self::Zoo => "Zoo",
            Self::AdministrativeAreaLevel1 => "Administrative Area Level 1",
            Self::AdministrativeAreaLevel2 => "Administrative Area Level 2",
            Self::AdministrativeAreaLevel3 => "Administrative Area Level 3",
            Self::AdministrativeAreaLevel4 => "Administrative Area Level 4",
            Self::AdministrativeAreaLevel5 => "Administrative Area Level 5",
            Self::Archipelago => "Archipelago",
            Self::ColloquialArea => "Colloquial Area",
            Self::Continent => "Continent",
            Self::Country => "Country",
            Self::Establishment => "Establishment",
            Self::Finance => "Finance",
            Self::Floor => "Floor",
            Self::Food => "Food",
            Self::GeneralContractor => "General Contractor",
            Self::Geocode => "Geocode",
            Self::Health => "Health",
            Self::Intersection => "Intersection",
            Self::Locality => "Locality",
            Self::NaturalFeature => "Natural Feature",
            Self::Neighborhood => "Neighborhood",
            Self::PlaceOfWorship => "Place of Worship",
            Self::PointOfInterest => "Point of Interest",
            Self::Political => "Political",
            Self::PostBox => "Post Box",
            Self::PostalCode => "Postal Code",
            Self::PostalCodePrefix => "Postal Code Prefix",
            Self::PostalCodeSuffix => "Postal Code Suffix",
            Self::PostalTown => "Postal Town",
            Self::Premise => "Premise",
            Self::Room => "Room",
            Self::Route => "Route",
            Self::StreetAddress => "Street Address",
            Self::StreetNumber => "Street Number",
            Self::Sublocality => "Sublocality",
            Self::SublocalityLevel1 => "Sublocality Level 1",
            Self::SublocalityLevel2 => "Sublocality Level 2",
            Self::SublocalityLevel3 => "Sublocality Level 3",
            Self::SublocalityLevel4 => "Sublocality Level 4",
            Self::SublocalityLevel5 => "Sublocality Level 5",
            Self::Subpremise => "Subpremise",
            Self::TownSquare => "Town Square",
            Self::Address => "Address",
            Self::Regions => "Regions",
            Self::Cities => "Cities",
            Self::Landmark => "Landmark",
            Self::Other => "Other",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl PlaceType {
    /// A helper function that converts a `Vec<PlaceType>` (i.e. an array of
    /// `PlaceType` enum) to a `String` that contains a comma-delimited list of
    /// [place
    /// types](https://developers.google.com/places/web-service/supported_types)
    /// codes.
    pub fn vec_to_csv(place_types: &[Self]) -> String {
        place_types
            .iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join(",")
    } // fn
} // impl
