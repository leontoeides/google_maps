use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

// -------------------------------------------------------------------------------------------------
//
/// Represents a field that can be requested from the Places API Text Search.
///
/// Used with field masks to specify which data to include in API responses. Each field triggers
/// specific SKU charges. Only request the fields you need to optimize costs and response size.
#[derive(
    // std
    Clone,
    Copy,
    Debug,
    Default,
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
#[repr(u8)]
pub enum Field {
    // Text Search Essentials ID Only SKU ----------------------------------------------------------
    
    /// Data provider attributions. Triggers ID Only SKU.
    #[serde(rename = "places.attributions")]
    PlacesAttributions = 0,
    
    /// Unique place identifier. Triggers ID Only SKU.
    #[serde(rename = "places.id")]
    #[default] PlacesId = 1,
    
    /// Place resource name (format: `places/PLACE_ID`). Triggers ID Only SKU.
    /// 
    /// Note: Use `PlacesDisplayName` for the text name of the place.
    #[serde(rename = "places.name")]
    PlacesName = 2,
    
    /// Token for accessing the next page of results. Triggers ID Only SKU.
    #[serde(rename = "nextPageToken")]
    NextPageToken = 3,
    
    /// Indicates if the place has moved. Triggers ID Only SKU.
    #[serde(rename = "places.movedPlace")]
    PlacesMovedPlace = 4,
    
    /// New place ID if the place has moved. Triggers ID Only SKU.
    #[serde(rename = "places.movedPlaceId")]
    PlacesMovedPlaceId = 5,
    
    // Text Search Pro SKU -------------------------------------------------------------------------
    
    /// Accessibility options available at the place. Triggers Pro SKU.
    #[serde(rename = "places.accessibilityOptions")]
    PlacesAccessibilityOptions = 6,
    
    /// Structured address components (street, city, state, etc.). Triggers Pro SKU.
    #[serde(rename = "places.addressComponents")]
    PlacesAddressComponents = 7,
    
    /// Relational info about location (landmarks, containing areas). Triggers Pro SKU.
    /// 
    /// Generally available in India, experimental elsewhere.
    #[serde(rename = "places.addressDescriptor")]
    PlacesAddressDescriptor = 8,
    
    /// Address in adr microformat. Triggers Pro SKU.
    #[serde(rename = "places.adrFormatAddress")]
    PlacesAdrFormatAddress = 9,
    
    /// Whether the place is operational, closed, etc. Triggers Pro SKU.
    #[serde(rename = "places.businessStatus")]
    PlacesBusinessStatus = 10,
    
    /// Places that contain this place (e.g., mall containing a store). Triggers Pro SKU.
    #[serde(rename = "places.containingPlaces")]
    PlacesContainingPlaces = 11,
    
    /// Human-readable display name. Triggers Pro SKU.
    #[serde(rename = "places.displayName")]
    PlacesDisplayName = 12,
    
    /// Human-readable address. Triggers Pro SKU.
    #[serde(rename = "places.formattedAddress")]
    PlacesFormattedAddress = 13,
    
    /// Links to Google Maps. Triggers Pro SKU.
    #[serde(rename = "places.googleMapsLinks")]
    PlacesGoogleMapsLinks = 14,
    
    /// Google Maps URI for the place. Triggers Pro SKU.
    #[serde(rename = "places.googleMapsUri")]
    PlacesGoogleMapsUri = 15,
    
    /// Background color for the place icon. Triggers Pro SKU.
    #[serde(rename = "places.iconBackgroundColor")]
    PlacesIconBackgroundColor = 16,
    
    /// Base URI for the place icon mask. Triggers Pro SKU.
    #[serde(rename = "places.iconMaskBaseUri")]
    PlacesIconMaskBaseUri = 17,
    
    /// Geographic coordinates (latitude/longitude). Triggers Pro SKU.
    #[serde(rename = "places.location")]
    PlacesLocation = 18,
    
    /// Photos of the place. Triggers Pro SKU.
    #[serde(rename = "places.photos")]
    PlacesPhotos = 19,
    
    /// Plus code location identifier. Triggers Pro SKU.
    #[serde(rename = "places.plusCode")]
    PlacesPlusCode = 20,
    
    /// Postal address in structured format. Triggers Pro SKU.
    #[serde(rename = "places.postalAddress")]
    PlacesPostalAddress = 21,
    
    /// Primary type of the place. Triggers Pro SKU.
    #[serde(rename = "places.primaryType")]
    PlacesPrimaryType = 22,
    
    /// Localized display name for the primary type. Triggers Pro SKU.
    #[serde(rename = "places.primaryTypeDisplayName")]
    PlacesPrimaryTypeDisplayName = 23,
    
    /// Whether this is a service area business without a physical location. Triggers Pro SKU.
    #[serde(rename = "places.pureServiceAreaBusiness")]
    PlacesPureServiceAreaBusiness = 24,
    
    /// Shortened version of the formatted address. Triggers Pro SKU.
    #[serde(rename = "places.shortFormattedAddress")]
    PlacesShortFormattedAddress = 25,
    
    /// URI for searching this place. Triggers Pro SKU.
    #[serde(rename = "places.searchUri")]
    PlacesSearchUri = 26,
    
    /// Sub-destinations within this place. Triggers Pro SKU.
    #[serde(rename = "places.subDestinations")]
    PlacesSubDestinations = 27,
    
    /// Array of place types. Triggers Pro SKU.
    #[serde(rename = "places.types")]
    PlacesTypes = 28,
    
    /// UTC offset in minutes. Triggers Pro SKU.
    #[serde(rename = "places.utcOffsetMinutes")]
    PlacesUtcOffsetMinutes = 29,
    
    /// Viewport for displaying the place on a map. Triggers Pro SKU.
    #[serde(rename = "places.viewport")]
    PlacesViewport = 30,
    
    // Text Search Enterprise SKU ------------------------------------------------------------------
    
    /// Current opening hours. Triggers Enterprise SKU.
    #[serde(rename = "places.currentOpeningHours")]
    PlacesCurrentOpeningHours = 31,
    
    /// Secondary opening hours (e.g., drive-through, kitchen). Triggers Enterprise SKU.
    #[serde(rename = "places.currentSecondaryOpeningHours")]
    PlacesCurrentSecondaryOpeningHours = 32,
    
    /// International phone number with country code. Triggers Enterprise SKU.
    #[serde(rename = "places.internationalPhoneNumber")]
    PlacesInternationalPhoneNumber = 33,
    
    /// Phone number formatted for the place's country. Triggers Enterprise SKU.
    #[serde(rename = "places.nationalPhoneNumber")]
    PlacesNationalPhoneNumber = 34,
    
    /// Price level (e.g., $, $$, $$$). Triggers Enterprise SKU.
    #[serde(rename = "places.priceLevel")]
    PlacesPriceLevel = 35,
    
    /// Price range with currency. Triggers Enterprise SKU.
    #[serde(rename = "places.priceRange")]
    PlacesPriceRange = 36,
    
    /// User rating (0.0-5.0). Triggers Enterprise SKU.
    #[serde(rename = "places.rating")]
    PlacesRating = 37,
    
    /// Regular opening hours. Triggers Enterprise SKU.
    #[serde(rename = "places.regularOpeningHours")]
    PlacesRegularOpeningHours = 38,
    
    /// Secondary regular opening hours. Triggers Enterprise SKU.
    #[serde(rename = "places.regularSecondaryOpeningHours")]
    PlacesRegularSecondaryOpeningHours = 39,
    
    /// Total number of user ratings. Triggers Enterprise SKU.
    #[serde(rename = "places.userRatingCount")]
    PlacesUserRatingCount = 40,
    
    /// Website URI. Triggers Enterprise SKU.
    #[serde(rename = "places.websiteUri")]
    PlacesWebsiteUri = 41,
    
    // Text Search Enterprise + Atmosphere SKU -----------------------------------------------------
    
    /// Whether dogs are allowed. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.allowsDogs")]
    PlacesAllowsDogs = 42,
    
    /// Whether curbside pickup is available. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.curbsidePickup")]
    PlacesCurbsidePickup = 43,
    
    /// Whether delivery is available. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.delivery")]
    PlacesDelivery = 44,
    
    /// Whether dine-in is available. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.dineIn")]
    PlacesDineIn = 45,
    
    /// Editorial summary of the place. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.editorialSummary")]
    PlacesEditorialSummary = 46,
    
    /// Summary of EV charging amenities. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.evChargeAmenitySummary")]
    PlacesEvChargeAmenitySummary = 47,
    
    /// EV charging options and availability. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.evChargeOptions")]
    PlacesEvChargeOptions = 48,
    
    /// Fuel options for gas stations. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.fuelOptions")]
    PlacesFuelOptions = 49,
    
    /// AI-generated summary of the place. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.generativeSummary")]
    PlacesGenerativeSummary = 50,
    
    /// Whether the place is good for children. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.goodForChildren")]
    PlacesGoodForChildren = 51,
    
    /// Whether the place is good for groups. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.goodForGroups")]
    PlacesGoodForGroups = 52,
    
    /// Whether the place is good for watching sports. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.goodForWatchingSports")]
    PlacesGoodForWatchingSports = 53,
    
    /// Whether the place has live music. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.liveMusic")]
    PlacesLiveMusic = 54,
    
    /// Whether menu for children is available. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.menuForChildren")]
    PlacesMenuForChildren = 55,
    
    /// Summary of the neighborhood. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.neighborhoodSummary")]
    PlacesNeighborhoodSummary = 56,
    
    /// Parking options available. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.parkingOptions")]
    PlacesParkingOptions = 57,
    
    /// Payment options accepted. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.paymentOptions")]
    PlacesPaymentOptions = 58,
    
    /// Whether outdoor seating is available. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.outdoorSeating")]
    PlacesOutdoorSeating = 59,
    
    /// Whether reservations are possible. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.reservable")]
    PlacesReservable = 60,
    
    /// Whether restrooms are available. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.restroom")]
    PlacesRestroom = 61,
    
    /// User reviews of the place. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.reviews")]
    PlacesReviews = 62,
    
    /// Summary of user reviews. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.reviewSummary")]
    PlacesReviewSummary = 63,
    
    /// Routing summaries for Text Search and Nearby Search. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "routingSummaries")]
    RoutingSummaries = 64,
    
    /// Whether beer is served. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.servesBeer")]
    PlacesServesBeer = 65,
    
    /// Whether breakfast is served. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.servesBreakfast")]
    PlacesServesBreakfast = 66,
    
    /// Whether brunch is served. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.servesBrunch")]
    PlacesServesBrunch = 67,
    
    /// Whether cocktails are served. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.servesCocktails")]
    PlacesServesCocktails = 68,
    
    /// Whether coffee is served. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.servesCoffee")]
    PlacesServesCoffee = 69,
    
    /// Whether dessert is served. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.servesDessert")]
    PlacesServesDessert = 70,
    
    /// Whether dinner is served. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.servesDinner")]
    PlacesServesDinner = 71,
    
    /// Whether lunch is served. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.servesLunch")]
    PlacesServesLunch = 72,
    
    /// Whether vegetarian food is served. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.servesVegetarianFood")]
    PlacesServesVegetarianFood = 73,
    
    /// Whether wine is served. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.servesWine")]
    PlacesServesWine = 74,
    
    /// Whether takeout is available. Triggers Enterprise + Atmosphere SKU.
    #[serde(rename = "places.takeout")]
    PlacesTakeout = 75,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Field {
    /// Returns whether this field triggers the [Text Search Essentials ID Only
    /// SKU](https://developers.google.com/maps/billing-and-pricing/sku-details#text-search-id-only-ess-sku).
    ///
    /// The ID Only SKU is the most basic pricing tier and includes fields like place IDs,
    /// attributions, and pagination tokens.
    ///
    /// Use this to check if a field falls under the cheapest billing category before building
    /// requests.
    #[must_use]
    pub fn is_text_search_essentials_id_only(&self) -> bool {
        [
            Self::PlacesAttributions,
            Self::PlacesId,
            Self::PlacesName,
            Self::NextPageToken,
            Self::PlacesMovedPlace,
            Self::PlacesMovedPlaceId
        ].contains(self)
    }

    /// Returns whether this field triggers the [Text Search Pro
    /// SKU](https://developers.google.com/maps/billing-and-pricing/sku-details#text-search-pro-sku).
    ///
    /// The Pro SKU includes richer place data like addresses, photos, location coordinates, and
    /// place types. This is the mid-tier pricing level.
    ///
    /// Use this to identify when requesting a field will incur Pro SKU charges instead of the
    /// cheaper ID Only SKU.
    #[must_use]
    pub fn is_text_search_pro(&self) -> bool {
        [
            Self::PlacesAccessibilityOptions,
            Self::PlacesAddressComponents,
            Self::PlacesAddressDescriptor,
            Self::PlacesAdrFormatAddress,
            Self::PlacesBusinessStatus,
            Self::PlacesContainingPlaces,
            Self::PlacesDisplayName,
            Self::PlacesFormattedAddress,
            Self::PlacesGoogleMapsLinks,
            Self::PlacesGoogleMapsUri,
            Self::PlacesIconBackgroundColor,
            Self::PlacesIconMaskBaseUri,
            Self::PlacesLocation,
            Self::PlacesPhotos,
            Self::PlacesPlusCode,
            Self::PlacesPostalAddress,
            Self::PlacesPrimaryType,
            Self::PlacesPrimaryTypeDisplayName,
            Self::PlacesPureServiceAreaBusiness,
            Self::PlacesShortFormattedAddress,
            Self::PlacesSearchUri,
            Self::PlacesSubDestinations,
            Self::PlacesTypes,
            Self::PlacesUtcOffsetMinutes,
            Self::PlacesViewport
        ].contains(self)
    }

    /// Returns whether this field triggers the [Text Search Enterprise
    /// SKU](https://developers.google.com/maps/billing-and-pricing/sku-details#text-search-ent-sku).
    ///
    /// The Enterprise SKU includes business-critical data like contact information, opening hours,
    /// ratings, and pricing. This is a higher-tier pricing level.
    ///
    /// Use this to determine if requesting a field will incur Enterprise SKU charges, which are
    /// more expensive than Pro or ID Only SKUs.
    #[must_use]
    pub fn is_text_search_enterprise(&self) -> bool {
        [
            Self::PlacesCurrentOpeningHours,
            Self::PlacesCurrentSecondaryOpeningHours,
            Self::PlacesInternationalPhoneNumber,
            Self::PlacesNationalPhoneNumber,
            Self::PlacesPriceLevel,
            Self::PlacesPriceRange,
            Self::PlacesRating,
            Self::PlacesRegularOpeningHours,
            Self::PlacesRegularSecondaryOpeningHours,
            Self::PlacesUserRatingCount,
            Self::PlacesWebsiteUri
        ].contains(self)
    }

    /// Returns whether this field triggers the [Text Search Enterprise + Atmosphere
    /// SKU](https://developers.google.com/maps/billing-and-pricing/sku-details#text-search-ent-plus-sku).
    ///
    /// The Enterprise + Atmosphere SKU is the highest pricing tier and includes detailed place
    /// attributes like reviews, amenities, ambiance data, and AI-generated summaries.
    ///
    /// Use this to identify the most expensive fields before adding them to requests, helping you
    /// control API costs by only requesting atmosphere data when truly needed.
    #[must_use]
    pub fn is_text_search_enterprise_and_atmosphere(&self) -> bool {
        [
            Self::PlacesAllowsDogs,
            Self::PlacesCurbsidePickup,
            Self::PlacesDelivery,
            Self::PlacesDineIn,
            Self::PlacesEditorialSummary,
            Self::PlacesEvChargeAmenitySummary,
            Self::PlacesEvChargeOptions,
            Self::PlacesFuelOptions,
            Self::PlacesGenerativeSummary,
            Self::PlacesGoodForChildren,
            Self::PlacesGoodForGroups,
            Self::PlacesGoodForWatchingSports,
            Self::PlacesLiveMusic,
            Self::PlacesMenuForChildren,
            Self::PlacesNeighborhoodSummary,
            Self::PlacesParkingOptions,
            Self::PlacesPaymentOptions,
            Self::PlacesOutdoorSeating,
            Self::PlacesReservable,
            Self::PlacesRestroom,
            Self::PlacesReviews,
            Self::PlacesReviewSummary,
            Self::RoutingSummaries,
            Self::PlacesServesBeer,
            Self::PlacesServesBreakfast,
            Self::PlacesServesBrunch,
            Self::PlacesServesCocktails,
            Self::PlacesServesCoffee,
            Self::PlacesServesDessert,
            Self::PlacesServesDinner,
            Self::PlacesServesLunch,
            Self::PlacesServesVegetarianFood,
            Self::PlacesServesWine,
            Self::PlacesTakeout
        ].contains(self)
    }
}