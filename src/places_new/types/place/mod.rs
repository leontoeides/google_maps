// -------------------------------------------------------------------------------------------------
//
// Structures

pub mod accessibility_options;
pub mod address_component;
pub mod attribution;
pub mod containing_place;
pub mod ev_charge_amenity_summary;
pub mod generative_summary;
pub mod neighborhood_summary;
pub mod opening_hours;
pub mod parking_options;
pub mod payment_options;
pub mod period;
pub mod plus_code;
pub mod point;
pub mod review_summary;
pub mod secondary_hours_type;
pub mod special_day;
pub mod sub_destination;

// -------------------------------------------------------------------------------------------------
//
// Enumerations

pub mod business_status;

// -------------------------------------------------------------------------------------------------

use crate::places_new::{
    AccessibilityOptions,
    AddressComponent,
    AddressDescriptor,
    Attribution,
    BusinessStatus,
    ContainingPlace,
    EvChargeAmenitySummary,
    EvChargeOptions,
    FuelOptions,
    GenerativeSummary,
    LatLng,
    LocalizedText,
    NeighborhoodSummary,
    OpeningHours,
    ParkingOptions,
    PaymentOptions,
    PhotoInfo,
    PlaceType,
    PlusCode,
    PostalAddress,
    PriceLevel,
    PriceRange,
    Review,
    ReviewSummary,
    SubDestination,
    Viewport,
    TimeZone
};
use icu_locale::Locale;
use url::Url;

// -------------------------------------------------------------------------------------------------
//
/// All the information representing a Place.
///
/// Place is the comprehensive data structure containing all available information about a location,
/// business, or point of interest from Google's Places API. This includes basic identification,
/// contact information, operational details, user reviews, photos, and various specialized
/// information like fuel options or EV charging capabilities.
#[derive(
    //std
    Clone,
    Debug,
    // getset
    getset::Getters,
    getset::CopyGetters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    /// This Place's resource name, in `places/{place_id}` format.
    ///
    /// Provides the canonical resource identifier that can be used in API calls. This is distinct
    /// from the display name and serves as the stable reference for API operations.
    #[serde(default)]
    #[getset(get = "pub")]
    pub name: Option<String>,

    /// The unique identifier of a place.
    ///
    /// A stable, opaque identifier that uniquely identifies this place across Google's systems.
    /// Can be used to retrieve this same place in future API calls.
    #[getset(get = "pub")]
    pub id: Option<String>,

    /// The localized name of the place.
    ///
    /// Human-readable name suitable as a short description, such as "Google Sydney", "Starbucks",
    /// or "Pyrmont". This is what users typically see as the place name.
    #[serde(default)]
    #[getset(get = "pub")]
    pub display_name: Option<LocalizedText>,

    /// A set of type tags for this result.
    ///
    /// Contains type classifications like "political", "locality", "restaurant", etc. Multiple
    /// types may apply to a single place to provide comprehensive categorization.
    #[serde(default)]
    #[getset(get = "pub")]
    pub types: Vec<PlaceType>,

    /// The primary type of the given result.
    ///
    /// The most specific and relevant type from the Places API supported types. A place can only
    /// have a single primary type, such as "restaurant", "cafe", or "airport".
    #[serde(default)]
    #[getset(get = "pub")]
    pub primary_type: Option<PlaceType>,

    /// The display name of the primary type.
    ///
    /// Localized, human-readable name of the primary type, suitable for display in user interfaces.
    /// For example, "Restaurant" or "Coffee Shop".
    #[serde(default)]
    #[getset(get = "pub")]
    pub primary_type_display_name: Option<LocalizedText>,

    /// A human-readable phone number for the place, in national format.
    ///
    /// Phone number formatted according to national conventions, suitable for display to users
    /// in the place's country. For example: "(02) 9374 4000" in Australia.
    #[serde(default)]
    #[getset(get = "pub")]
    pub national_phone_number: Option<String>,

    /// A human-readable phone number for the place, in international format.
    ///
    /// Phone number formatted for international dialing, including country code. For example:
    /// "+61 2 9374 4000" for an Australian number.
    #[serde(default)]
    #[getset(get = "pub")]
    pub international_phone_number: Option<String>,

    /// A full, human-readable address for this place.
    ///
    /// Complete address formatted according to local conventions, suitable for display or
    /// use in mapping applications.
    #[serde(default)]
    #[getset(get = "pub")]
    pub formatted_address: Option<String>,

    /// A short, human-readable address for this place.
    ///
    /// Abbreviated address format for space-constrained displays while maintaining
    /// sufficient detail for identification.
    #[serde(default)]
    #[getset(get = "pub")]
    pub short_formatted_address: Option<String>,

    /// The address in postal address format.
    ///
    /// Structured postal address information following international postal addressing standards.
    #[serde(default)]
    #[getset(get = "pub")]
    pub postal_address: Option<PostalAddress>,

    /// Repeated components for each locality level.
    ///
    /// Structured address components broken down by administrative levels and types.
    /// The format and number of components can vary based on location and may change over time.
    #[serde(default)]
    #[getset(get = "pub")]
    pub address_components: Vec<AddressComponent>,

    /// Plus code of the place location lat/long.
    ///
    /// Open Location Code (Plus Code) providing a precise location reference that can be
    /// used globally without requiring a traditional address.
    #[serde(default)]
    #[getset(get = "pub")]
    pub plus_code: Option<PlusCode>,

    /// The position of this place.
    ///
    /// Geographic coordinates (latitude and longitude) pinpointing the exact location of this place.
    #[serde(default)]
    #[getset(get = "pub")]
    pub location: Option<LatLng>,

    /// A viewport suitable for displaying the place on an average-sized map.
    ///
    /// Recommended bounds for displaying this place on a map. This is not the physical boundary
    /// or service area of the business, but rather the optimal map view.
    #[serde(default)]
    #[getset(get = "pub")]
    pub viewport: Option<Viewport>,

    /// A rating between 1.0 and 5.0, based on user reviews.
    ///
    /// Overall user rating for this place derived from aggregated user reviews and feedback.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub rating: Option<f64>,

    /// A URL providing more information about this place.
    ///
    /// Link to the place's page on Google Maps for additional details and functionality.
    #[serde(default)]
    #[getset(get = "pub")]
    pub google_maps_uri: Option<Url>,

    /// The authoritative website for this place.
    ///
    /// Official website URL for the business or location, typically the business's homepage
    /// rather than a chain's general website.
    #[serde(default)]
    #[getset(get = "pub")]
    pub website_uri: Option<Url>,

    /// List of reviews about this place, sorted by relevance.
    ///
    /// User reviews and ratings for this place. Maximum of 5 reviews can be returned,
    /// prioritized by relevance and usefulness.
    #[serde(default)]
    #[getset(get = "pub")]
    pub reviews: Vec<Review>,

    /// The regular hours of operation.
    ///
    /// Standard weekly operating hours for this place. Always open places are represented
    /// with specific patterns as documented in the Places API.
    #[serde(default)]
    #[getset(get = "pub")]
    pub regular_opening_hours: Option<OpeningHours>,

    /// IANA Time Zone Database time zone.
    ///
    /// Time zone identifier for this place's location, such as `America/New_York` or
    /// `Europe/London`.
    #[serde(default)]
    #[getset(get = "pub")]
    pub time_zone: Option<TimeZone>,

    /// Information about photos of this place.
    ///
    /// Photo references and metadata for images associated with this place. Maximum of 10 photos
    /// can be returned, including both user-contributed and business photos.
    #[serde(default)]
    #[getset(get = "pub")]
    pub photos: Vec<PhotoInfo>,

    /// The place's address in adr microformat.
    ///
    /// Address formatted according to the adr microformat specification for structured data markup.
    #[serde(default)]
    #[getset(get = "pub")]
    pub adr_format_address: Option<String>,

    /// The business status for the place.
    ///
    /// Current operational status indicating whether the business is open, closed temporarily,
    /// or permanently closed.
    #[serde(default)]
    #[getset(get = "pub")]
    pub business_status: Option<BusinessStatus>,

    /// Price level of the place.
    ///
    /// General price range indicator for the place, typically used for restaurants and similar
    /// businesses to indicate relative cost expectations.
    #[serde(default)]
    #[getset(get = "pub")]
    pub price_level: Option<PriceLevel>,

    /// A set of data provider that must be shown with this result.
    ///
    /// Attribution information for data sources that contributed information about this place.
    /// These attributions must be displayed according to the provider's requirements.
    #[serde(default)]
    #[getset(get = "pub")]
    pub attributions: Vec<Attribution>,

    /// A truncated URL to an icon mask.
    ///
    /// Base URL for retrieving icon graphics for this place type. Append file extensions
    /// like ".svg" or ".png" to get specific icon formats.
    #[serde(default)]
    #[getset(get = "pub")]
    pub icon_mask_base_uri: Option<Url>,

    /// Background color for `icon_mask` in hex format.
    ///
    /// Hexadecimal color code (e.g., "#909CE1") for the background color to use with the icon mask.
    #[serde(default)]
    #[getset(get = "pub")]
    pub icon_background_color: Option<String>,

    /// The hours of operation for the next seven days including today.
    ///
    /// Current operating hours covering the next week, including any special hours or exceptions.
    /// This field includes special days with exceptional hours.
    #[serde(default)]
    #[getset(get = "pub")]
    pub current_opening_hours: Option<OpeningHours>,

    /// Contains an array of entries for the next seven days secondary hours.
    ///
    /// Secondary operating hours (such as drive-through, pickup, or takeout hours) that differ
    /// from the main business hours. Covers the next seven days including special exceptions.
    #[serde(default)]
    #[getset(get = "pub")]
    pub current_secondary_opening_hours: Vec<OpeningHours>,

    /// Contains an array of entries for regular secondary hours.
    ///
    /// Regular secondary operating hours for services like drive-through, pickup, or takeout
    /// that operate on different schedules than the main business hours.
    #[serde(default)]
    #[getset(get = "pub")]
    pub regular_secondary_opening_hours: Vec<OpeningHours>,

    /// Contains a summary of the place.
    ///
    /// AI-generated textual overview and summary of the place, including language information.
    /// This content must be presented as-is and cannot be modified.
    #[serde(default)]
    #[getset(get = "pub")]
    pub editorial_summary: Option<LocalizedText>,

    /// Payment options the place accepts.
    ///
    /// Information about accepted payment methods including cards, cash, and NFC payments.
    #[serde(default)]
    #[getset(get = "pub")]
    pub payment_options: Option<PaymentOptions>,

    /// Options of parking provided by the place.
    ///
    /// Information about available parking options including free/paid parking, street parking,
    /// garage parking, and valet services.
    #[serde(default)]
    #[getset(get = "pub")]
    pub parking_options: Option<ParkingOptions>,

    /// A list of sub-destinations related to the place.
    ///
    /// Specific locations within larger places (like terminals in an airport or parking lots
    /// in a stadium) that provide more granular destination options.
    #[serde(default)]
    #[getset(get = "pub")]
    pub sub_destinations: Vec<SubDestination>,

    /// The most recent information about fuel options in a gas station.
    ///
    /// Current fuel pricing and availability information for gas stations, updated regularly
    /// to reflect current market conditions.
    #[serde(default)]
    #[getset(get = "pub")]
    pub fuel_options: Option<FuelOptions>,

    /// Information of ev charging options.
    ///
    /// Details about electric vehicle charging capabilities including connector types,
    /// charge rates, and availability information.
    #[serde(default)]
    #[getset(get = "pub")]
    pub ev_charge_options: Option<EvChargeOptions>,

    /// AI-generated summary of the place.
    ///
    /// Comprehensive AI-generated overview providing insights and key information about the place
    /// using Google's Gemini AI technology.
    #[serde(default)]
    #[getset(get = "pub")]
    pub generative_summary: Option<GenerativeSummary>,

    /// List of places in which the current place is located.
    ///
    /// Hierarchical containment information showing larger geographic or administrative areas
    /// that contain this place, enabling navigation up the location hierarchy.
    #[serde(default)]
    #[getset(get = "pub")]
    pub containing_places: Vec<ContainingPlace>,

    /// The address descriptor of the place.
    ///
    /// Rich contextual address information using landmarks and areas to provide detailed
    /// location descriptions beyond traditional addresses.
    #[serde(default)]
    #[getset(get = "pub")]
    pub address_descriptor: Option<AddressDescriptor>,

    /// The price range associated with a Place.
    ///
    /// Cost expectations and pricing information for places like restaurants, helping users
    /// understand expected spending levels.
    #[serde(default)]
    #[getset(get = "pub")]
    pub price_range: Option<PriceRange>,

    /// AI-generated summary of the place using user reviews.
    ///
    /// Synthesized insights from user reviews highlighting common themes and experiences
    /// mentioned by customers.
    #[serde(default)]
    #[getset(get = "pub")]
    pub review_summary: Option<ReviewSummary>,

    /// The summary of amenities near the EV charging station.
    ///
    /// Information about nearby facilities and services available while using EV charging stations,
    /// only applicable to electric vehicle charging locations.
    #[serde(default)]
    #[getset(get = "pub")]
    pub ev_charge_amenity_summary: Option<EvChargeAmenitySummary>,

    /// A summary of points of interest near the place.
    ///
    /// AI-generated overview of the surrounding neighborhood including notable places,
    /// attractions, and area characteristics.
    #[serde(default)]
    #[getset(get = "pub")]
    pub neighborhood_summary: Option<NeighborhoodSummary>,

    // Optional fields based on protobuf definition
    /// Number of minutes this place's timezone is currently offset from UTC.
    ///
    /// UTC offset in minutes to handle timezones with fractional hour offsets. Positive values
    /// indicate time zones ahead of UTC, negative values indicate time zones behind UTC.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub utc_offset_minutes: Option<i32>,

    /// The total number of reviews for this place.
    ///
    /// Total count of all user reviews and ratings, including those that may only contain
    /// ratings without written text.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub user_rating_count: Option<i32>,

    /// Specifies if the business supports takeout.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub takeout: Option<bool>,

    /// Specifies if the business supports delivery.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub delivery: Option<bool>,

    /// Specifies if the business supports indoor or outdoor seating options.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub dine_in: Option<bool>,

    /// Specifies if the business supports curbside pickup.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub curbside_pickup: Option<bool>,

    /// Specifies if the place supports reservations.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub reservable: Option<bool>,

    /// Specifies if the place serves breakfast.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub serves_breakfast: Option<bool>,

    /// Specifies if the place serves lunch.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub serves_lunch: Option<bool>,

    /// Specifies if the place serves dinner.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub serves_dinner: Option<bool>,

    /// Specifies if the place serves beer.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub serves_beer: Option<bool>,

    /// Specifies if the place serves wine.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub serves_wine: Option<bool>,

    /// Specifies if the place serves brunch.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub serves_brunch: Option<bool>,

    /// Specifies if the place serves vegetarian food.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub serves_vegetarian_food: Option<bool>,

    /// Place provides outdoor seating.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub outdoor_seating: Option<bool>,

    /// Place provides live music.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub live_music: Option<bool>,

    /// Place has a children's menu.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub menu_for_children: Option<bool>,

    /// Place serves cocktails.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub serves_cocktails: Option<bool>,

    /// Place serves dessert.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub serves_dessert: Option<bool>,

    /// Place serves coffee.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub serves_coffee: Option<bool>,

    /// Place is good for children.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub good_for_children: Option<bool>,

    /// Place allows dogs.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub allows_dogs: Option<bool>,

    /// Place has restroom.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub restroom: Option<bool>,

    /// Place accommodates groups.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub good_for_groups: Option<bool>,

    /// Place is suitable for watching sports.
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub good_for_watching_sports: Option<bool>,

    /// Information about the accessibility options a place offers.
    ///
    /// Details about wheelchair accessibility and other accommodation features available
    /// at this location.
    #[serde(default)]
    #[getset(get = "pub")]
    pub accessibility_options: Option<AccessibilityOptions>,

    /// Indicates whether the place is a pure service area business.
    ///
    /// Pure service area businesses visit or deliver to customers directly but do not serve
    /// customers at their business address (e.g., cleaning services, plumbers).
    #[serde(default)]
    #[getset(get_copy = "pub")]
    pub pure_service_area_business: Option<bool>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Place {
    /// Returns whether this place has a display name.
    ///
    /// Used to determine if the place has user-friendly text that can be shown in interfaces or
    /// location descriptions.
    #[must_use]
    pub const fn has_display_name(&self) -> bool {
        self.display_name.is_some()
    }

    /// Gets the display name text if available.
    ///
    /// Returns the human-readable place name for display purposes. Used when showing place
    /// information to users in applications and maps.
    #[must_use]
    pub fn display_text(&self) -> Option<&str> {
        self.display_name.as_ref().map(LocalizedText::text).map(String::as_str)
    }

    /// Gets the language code of the display name if available.
    ///
    /// Returns the BCP-47 language code indicating what language the display name is in. Useful for
    /// internationalization and proper text rendering.
    #[must_use]
    pub fn display_language(&self) -> Option<&Locale> {
        self.display_name.as_ref().map(LocalizedText::language)
    }

    /// Returns whether this place is currently operational.
    ///
    /// Returns `None` if business status information is not available. Used to identify places that
    /// are open for business, as opposed to temporarily or permanently closed establishments.
    #[must_use]
    pub fn is_operational(&self) -> Option<bool> {
        self.business_status.as_ref().map(|status| *status == BusinessStatus::Operational)
    }

    /// Returns whether this place is temporarily closed.
    ///
    /// Returns `None` if business status information is not available. Used to identify places that
    /// are closed temporarily, useful for displaying appropriate status information to users.
    #[must_use]
    pub fn is_temporarily_closed(&self) -> Option<bool> {
        self.business_status.as_ref().map(|status| *status == BusinessStatus::ClosedTemporarily)
    }

    /// Returns whether this place is permanently closed.
    ///
    /// Returns `None` if business status information is not available. Used to identify places that
    /// are no longer in operation, which may need special handling in user interfaces.
    #[must_use]
    pub fn is_permanently_closed(&self) -> Option<bool> {
        self.business_status.as_ref().map(|status| *status == BusinessStatus::ClosedPermanently)
    }

    /// Returns whether this place has user reviews.
    ///
    /// Used to determine if review information should be displayed in user interfaces and whether
    /// review-based features are available.
    #[must_use]
    pub fn has_reviews(&self) -> bool {
        !self.reviews.is_empty()
    }

    /// Returns the number of reviews available.
    ///
    /// Used to understand the depth of review information available for this place. Note that this
    /// returns the number of detailed reviews, not the total review count.
    #[must_use]
    pub fn review_count(&self) -> usize {
        self.reviews.len()
    }

    /// Returns whether this place has photos.
    ///
    /// Used to determine if photo gallery functionality should be available for this place in user
    /// interfaces.
    #[must_use]
    pub fn has_photos(&self) -> bool {
        !self.photos.is_empty()
    }

    /// Returns the number of photos available.
    ///
    /// Used to understand the richness of visual content available for this place.
    #[must_use]
    pub fn photo_count(&self) -> usize {
        self.photos.len()
    }

    /// Returns whether this place has a user rating.
    ///
    /// Used to determine if rating information should be displayed and whether rating-based
    /// filtering or sorting is meaningful.
    #[must_use]
    pub const fn has_rating(&self) -> bool {
        self.rating.is_some()
    }

    /// Returns whether this place has a high rating.
    ///
    /// Returns `None` if rating information is not available. Used to identify well-rated places
    /// for highlighting in user interfaces or recommendation systems. Uses a threshold of 4.0
    /// stars.
    #[must_use]
    pub fn has_high_rating(&self) -> Option<bool> {
        self.rating.map(|r| r >= 4.0)
    }

    /// Returns whether this place is a restaurant or food establishment.
    ///
    /// Returns `None` if type information is not available. Used to determine if food-related
    /// features and information should be displayed for this place.
    #[must_use]
    pub fn is_food_establishment(&self) -> Option<bool> {
        let has_food_type = self.types.iter().any(|t| {
            matches!(t,
                PlaceType::Restaurant | PlaceType::Food | PlaceType::MealTakeaway |
                PlaceType::MealDelivery | PlaceType::Cafe | PlaceType::Bakery |
                PlaceType::Bar | PlaceType::NightClub
            )
        });

        let primary_is_food = self.primary_type.as_ref().is_some_and(|pt| {
            matches!(pt,
                PlaceType::Restaurant | PlaceType::Food | PlaceType::Cafe
            )
        });

        if has_food_type || primary_is_food {
            Some(true)
        } else if !self.types.is_empty() || self.primary_type.is_some() {
            Some(false)
        } else {
            None
        }
    }

    /// Returns whether this place offers food delivery.
    ///
    /// Returns `None` if delivery information is not available. Used to identify places that
    /// provide delivery services for food ordering and delivery application integration.
    #[must_use]
    pub fn offers_delivery(&self) -> Option<bool> {
        self.delivery.or_else(|| {
            if self.types.contains(&PlaceType::MealDelivery) {
                Some(true)
            } else {
                None
            }
        })
    }

    /// Returns whether this place offers takeout.
    ///
    /// Returns `None` if takeout information is not available. Used to identify places that provide
    /// takeout services for quick food pickup and ordering applications.
    #[must_use]
    pub fn offers_takeout(&self) -> Option<bool> {
        self.takeout.or_else(|| {
            if self.types.contains(&PlaceType::MealTakeaway) {
                Some(true)
            } else {
                None
            }
        })
    }

    /// Returns whether this place accepts reservations.
    ///
    /// Returns `None` if reservation information is not available. Used to determine if reservation
    /// functionality should be offered for this place in booking and dining applications.
    #[must_use]
    pub const fn accepts_reservations(&self) -> Option<bool> {
        self.reservable
    }

    /// Returns whether this place is family-friendly.
    ///
    /// Returns `None` if family-friendliness information is not available. Used to identify places
    /// suitable for families with children, useful for family-oriented filtering and
    /// recommendations.
    #[must_use]
    pub const fn is_family_friendly(&self) -> Option<bool> {
        match (self.good_for_children, self.menu_for_children) {
            (Some(true), _) | (_, Some(true)) => Some(true),
            (Some(false), Some(false)) => Some(false),
            _ => None,
        }
    }

    /// Returns whether this place is pet-friendly.
    ///
    /// Returns `None` if pet policy information is not available. Used to identify places that
    /// allow pets, useful for pet owner travel planning and location filtering.
    #[must_use]
    pub const fn is_pet_friendly(&self) -> Option<bool> {
        self.allows_dogs
    }

    /// Returns whether this place has wheelchair accessibility.
    ///
    /// Returns `None` if accessibility information is not available. Used to identify places with
    /// accessibility features for users with mobility requirements.
    #[must_use]
    pub fn is_wheelchair_accessible(&self) -> Option<bool> {
        self.accessibility_options.as_ref().and_then(|opts| {
            match (opts.wheelchair_accessible_entrance(),
                   opts.wheelchair_accessible_parking()) {
                (Some(true), _) | (_, Some(true)) => Some(true),
                (Some(false), Some(false)) => Some(false),
                _ => None,
            }
        })
    }

    /// Returns whether this place serves alcohol.
    ///
    /// Returns `None` if alcohol service information is not available. Used to identify places that
    /// serve alcoholic beverages, useful for filtering and age-appropriate recommendations.
    #[must_use]
    pub fn serves_alcohol(&self) -> Option<bool> {
        let serves_from_attributes = match (self.serves_beer, self.serves_wine, self.serves_cocktails) {
            (Some(true), _, _) | (_, Some(true), _) | (_, _, Some(true)) => Some(true),
            (Some(false), Some(false), Some(false)) => Some(false),
            _ => None,
        };

        serves_from_attributes.or_else(|| {
            let has_alcohol_type = self.types.iter().any(|t| {
                matches!(t, PlaceType::Bar | PlaceType::NightClub | PlaceType::LiquorStore)
            });
            if has_alcohol_type {
                Some(true)
            } else {
                None
            }
        })
    }

    /// Returns whether this place has parking available.
    ///
    /// Returns `None` if parking information is not available. Used to help users understand
    /// parking availability for trip planning and accessibility considerations.
    #[must_use]
    pub fn has_parking(&self) -> Option<bool> {
        self.parking_options.as_ref().and_then(|opts| {
            let any_parking = opts.free_parking_lot().unwrap_or(false) ||
                opts.paid_parking_lot().unwrap_or(false) ||
                opts.free_street_parking().unwrap_or(false) ||
                opts.paid_street_parking().unwrap_or(false) ||
                opts.free_garage_parking().unwrap_or(false) ||
                opts.paid_garage_parking().unwrap_or(false);

            if any_parking {
                Some(true)
            } else {
                None
            }
        })
    }

    /// Returns whether this place has free parking available.
    ///
    /// Returns `None` if parking information is not available. Used to identify places with free
    /// parking options, valuable for budget-conscious trip planning and urban navigation.
    #[must_use]
    pub fn has_free_parking(&self) -> Option<bool> {
        self.parking_options.as_ref().and_then(|opts| {
            match (opts.free_parking_lot(), opts.free_street_parking(), opts.free_garage_parking()) {
                (Some(true), _, _) | (_, Some(true), _) | (_, _, Some(true)) => Some(true),
                (Some(false), Some(false), Some(false)) => Some(false),
                _ => None,
            }
        })
    }

    /// Returns whether this place serves vegetarian food.
    ///
    /// Returns `None` if vegetarian food information is not available. Used to identify places
    /// suitable for vegetarian diners, useful for dietary filtering and restaurant recommendations.
    #[must_use]
    pub const fn serves_vegetarian(&self) -> Option<bool> {
        self.serves_vegetarian_food
    }

    /// Returns whether this place is suitable for groups.
    ///
    /// Returns `None` if group accommodation information is not available. Used to identify places
    /// that can accommodate larger parties, useful for event planning and group dining
    /// recommendations.
    #[must_use]
    pub const fn accommodates_groups(&self) -> Option<bool> {
        self.good_for_groups
    }

    /// Returns whether this place offers outdoor seating.
    ///
    /// Returns `None` if outdoor seating information is not available. Used to identify places
    /// with outdoor dining or seating options, useful for weather-dependent recommendations and
    /// preference filtering.
    #[must_use]
    pub const fn has_outdoor_seating(&self) -> Option<bool> {
        self.outdoor_seating
    }

    /// Returns whether this place features live entertainment.
    ///
    /// Returns `None` if entertainment information is not available. Used to identify places with
    /// live music or entertainment, useful for nightlife and entertainment venue recommendations.
    #[must_use]
    pub const fn has_live_entertainment(&self) -> Option<bool> {
        self.live_music
    }

    /// Returns whether this place is good for watching sports.
    ///
    /// Returns `None` if sports viewing information is not available. Used to identify sports bars
    /// and venues suitable for watching games, useful for sports-related event planning and
    /// recommendations.
    #[must_use]
    pub const fn is_sports_venue(&self) -> Option<bool> {
        self.good_for_watching_sports
    }

    /// Gets the resource name without the places/ prefix.
    ///
    /// Extracts just the place ID portion from the resource name for use in APIs that expect only
    /// the identifier without the full resource path.
    #[must_use]
    pub fn extract_place_id_from_name(&self) -> Option<&str> {
        self.name.as_ref().and_then(|n| n.strip_prefix("places/"))
    }

    /// Returns whether the resource name is properly formatted.
    ///
    /// Used to validate that the place has a correctly formatted resource name following the
    /// expected `places/{place_id}` pattern.
    #[must_use]
    pub fn has_valid_resource_name(&self) -> bool {
        self.name.as_ref()
            .is_some_and(|n| n.starts_with("places/") && n.len() > "places/".len())
    }

    /// Returns whether this place serves meals throughout the day.
    ///
    /// Returns `None` if meal service information is not available. Used to identify all-day dining
    /// establishments that serve breakfast, lunch, and dinner.
    #[must_use]
    pub const fn serves_all_day_dining(&self) -> Option<bool> {
        match (self.serves_breakfast, self.serves_lunch, self.serves_dinner) {
            (Some(true), Some(true), Some(true)) => Some(true),
            (Some(false), _, _) | (_, Some(false), _) | (_, _, Some(false)) => Some(false),
            _ => None,
        }
    }

    /// Returns whether this place offers brunch.
    ///
    /// Returns `None` if brunch information is not available. Used to identify places suitable for
    /// brunch dining, useful for weekend dining recommendations and meal planning.
    #[must_use]
    pub const fn offers_brunch(&self) -> Option<bool> {
        self.serves_brunch
    }

    /// Returns whether this place serves coffee.
    ///
    /// Returns `None` if coffee service information is not available. Used to identify coffee shops
    /// and cafes, useful for finding places to work, meet, or get caffeinated beverages.
    #[must_use]
    pub fn offers_coffee(&self) -> Option<bool> {
        self.serves_coffee.or_else(|| {
            let has_coffee_type = self.types.iter().any(|t| {
                matches!(t, PlaceType::Cafe | PlaceType::CoffeeShop)
            });
            if has_coffee_type {
                Some(true)
            } else {
                None
            }
        })
    }

    /// Returns whether this place serves desserts.
    ///
    /// Returns `None` if dessert information is not available. Used to identify places with dessert
    /// options, useful for special occasions and sweet treat recommendations.
    #[must_use]
    pub fn serves_desserts(&self) -> Option<bool> {
        self.serves_dessert.or_else(|| {
            let has_dessert_type = self.types.iter().any(|t| {
                matches!(t, PlaceType::Bakery | PlaceType::IceCreamShop)
            });
            if has_dessert_type {
                Some(true)
            } else {
                None
            }
        })
    }

    /// Returns whether this place has restroom facilities.
    ///
    /// Returns `None` if restroom information is not available. Used to identify places with
    /// restroom access, important for accessibility and convenience planning.
    #[must_use]
    pub const fn has_restrooms(&self) -> Option<bool> {
        self.restroom
    }

    /// Returns whether this place supports curbside pickup.
    ///
    /// Returns `None` if curbside pickup information is not available. Used to identify places with
    /// convenient pickup options, useful for contactless service and quick transactions.
    #[must_use]
    pub const fn supports_curbside_pickup(&self) -> Option<bool> {
        self.curbside_pickup
    }

    /// Returns whether this place supports dine-in service.
    ///
    /// Returns `None` if dine-in information is not available. Used to determine if the place
    /// offers on-premises dining, as opposed to delivery or takeout-only establishments.
    #[must_use]
    pub const fn supports_dine_in(&self) -> Option<bool> {
        self.dine_in
    }

    /// Gets a summary of food service options available.
    ///
    /// Returns available service types for food establishments. An empty vector indicates no
    /// service information is available, not that no services are offered.
    #[must_use]
    pub fn food_service_summary(&self) -> Vec<&'static str> {
        let mut services = Vec::new();

        if self.supports_dine_in() == Some(true) { services.push("Dine-in"); }
        if self.offers_takeout() == Some(true) { services.push("Takeout"); }
        if self.offers_delivery() == Some(true) { services.push("Delivery"); }
        if self.supports_curbside_pickup() == Some(true) { services.push("Curbside pickup"); }

        services
    }

    /// Gets a summary of meal types served.
    ///
    /// Returns meal types available at this establishment. An empty vector indicates no meal
    /// information is available, not that no meals are served.
    #[must_use]
    pub fn meal_types_served(&self) -> Vec<&'static str> {
        let mut meals = Vec::new();

        if self.serves_breakfast == Some(true) { meals.push("Breakfast"); }
        if self.offers_brunch() == Some(true) { meals.push("Brunch"); }
        if self.serves_lunch == Some(true) { meals.push("Lunch"); }
        if self.serves_dinner == Some(true) { meals.push("Dinner"); }

        meals
    }

    /// Gets a summary of accessibility features.
    ///
    /// Returns available accessibility options. An empty vector indicates no accessibility
    /// information is available, not that no features are present.
    #[must_use]
    pub fn accessibility_summary(&self) -> Vec<&'static str> {
        let mut features = Vec::new();

        if let Some(opts) = &self.accessibility_options {
            if opts.wheelchair_accessible_entrance() == Some(true) {
                features.push("Wheelchair accessible entrance");
            }
            if opts.wheelchair_accessible_parking() == Some(true) {
                features.push("Wheelchair accessible parking");
            }
            if opts.wheelchair_accessible_restroom() == Some(true) {
                features.push("Wheelchair accessible restroom");
            }
            if opts.wheelchair_accessible_seating() == Some(true) {
                features.push("Wheelchair accessible seating");
            }
        }

        features
    }

    /// Returns whether this place has sub-destinations.
    ///
    /// Used to determine if this place contains other notable locations within it, useful for large
    /// venues like airports, malls, or universities.
    #[must_use]
    pub fn has_sub_destinations(&self) -> bool {
        !self.sub_destinations.is_empty()
    }

    /// Gets the number of sub-destinations.
    ///
    /// Returns the count of internal destinations, useful for understanding the complexity and size
    /// of a venue.
    #[must_use]
    pub fn sub_destination_count(&self) -> usize {
        self.sub_destinations.len()
    }

    /// Returns whether this place has AI-generated content.
    ///
    /// Used to determine if enhanced AI summaries and insights are available for this place,
    /// indicating richer content for user experience.
    #[must_use]
    pub const fn has_ai_content(&self) -> bool {
        self.generative_summary.is_some() ||
        self.review_summary.is_some() ||
        self.editorial_summary.is_some()
    }

    /// Returns whether this place is in a hierarchical location structure.
    ///
    /// Used to determine if location context and navigation up the geographic hierarchy is
    /// available for this place.
    #[must_use]
    pub fn has_location_hierarchy(&self) -> bool {
        !self.containing_places.is_empty()
    }

    /// Gets the top-level containing place.
    ///
    /// Returns the highest-level geographic container for this place, useful for understanding the
    /// broadest location context.
    #[must_use]
    pub fn top_level_container(&self) -> Option<&ContainingPlace> {
        self.containing_places.last()
    }

    /// Returns whether this place has detailed address information.
    ///
    /// Used to determine if rich address context including landmarks and areas is available for
    /// enhanced location understanding.
    #[must_use]
    pub fn has_address_descriptor(&self) -> bool {
        self.address_descriptor
            .as_ref()
            .is_some_and(AddressDescriptor::has_context)
    }

    /// Returns whether this place has pricing information.
    ///
    /// Used to determine if cost expectations and budget planning information is available for this
    /// place.
    #[must_use]
    pub fn has_pricing_info(&self) -> bool {
        self.price_level.is_some() ||
        self.price_range.as_ref()
            .is_some_and(PriceRange::has_upper_bound)
    }

    /// Gets a compact display string for the place.
    ///
    /// Provides a concise place description suitable for mobile interfaces, map overlays, or
    /// space-constrained displays.
    #[must_use]
    pub fn compact_display(&self) -> String {
        let name = self.display_text().unwrap_or("Unknown Place");

        self.rating.map_or_else(
            || name.to_string(),
            |rating| format!("{name} ({rating:.1}★)")
        )
    }

    /// Gets a detailed description including key features.
    ///
    /// Provides comprehensive information suitable for detailed place listings, including name,
    /// rating, key amenities, and service options.
    #[must_use]
    pub fn detailed_description(&self) -> String {
        let name = self.display_text().unwrap_or("Unknown Place");

        let rating_part = self.rating.map(|r| {
            self.user_rating_count.map_or_else(
                || format!(" ({r:.1}★)"),
                |count| format!(" ({r:.1}★, {count} reviews)")
            )
        }).unwrap_or_default();

        let services_part = if self.is_food_establishment() == Some(true) {
            let services = self.food_service_summary();
            if services.is_empty() {
                String::new()
            } else {
                format!(" • {}", services.join(", "))
            }
        } else {
            String::new()
        };

        format!("{name}{rating_part}{services_part}")
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::fmt::Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.compact_display())
    }
}