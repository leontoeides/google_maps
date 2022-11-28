//! Contains the `Field` enum and its associated traits. It specifies the
//! fields in the place details that should be returned. For example, business
//! status, price level, wheelchair accessible, and so on.

use crate::places::error::Error;
use phf::phf_map;
use serde::{Deserialize, Serialize, Deserializer};

// -----------------------------------------------------------------------------
//
/// Use the fields parameter to specify a comma-separated list of place data
/// types to return. For example: `fields=formatted_address,name,geometry`. Use
/// a forward slash when specifying compound values. For example:
/// `opening_hours/open_now`.
///
/// Fields are divided into three billing categories: Basic, Contact, and
/// Atmosphere. Basic fields are billed at base rate, and incur no additional
/// charges. Contact and Atmosphere fields are billed at a higher rate. See the
/// [pricing sheet](https://cloud.google.com/maps-platform/pricing/sheet/) for
/// more information. Attributions, `html_attributions`, are always returned
/// with every call, regardless of whether the field has been requested.
///
/// * Caution: Place Search requests and Place Details requests do not return
/// the same fields. Place Search requests return a subset of the fields that
/// are returned by Place Details requests. If the field you want is not
/// returned by Place Search, you can use Place Search to get a `place_id`, then
/// use that Place ID to make a Place Details request. For more information on
/// the fields that are unavailable in a Place Search request, see
/// [Places API fields support](https://developers.google.com/maps/documentation/places/web-service/place-data-fields#places-api-fields-support).

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Field {
    // Basic
    #[serde(alias = "address_component")]
    AddressComponent,
    #[serde(alias = "adr_address")]
    AdrAddress,
    #[serde(alias = "business_status")]
    BusinessStatus,
    #[serde(alias = "formatted_address")]
    FormattedAddress,
    #[serde(alias = "geometry")]
    Geometry,
    #[serde(alias = "icon")]
    Icon,
    #[serde(alias = "icon_mask_base_uri")]
    IconMaskBaseUri,
    #[serde(alias = "icon_background_color")]
    IconBackgroundColor,
    #[serde(alias = "name")]
    Name,
    #[serde(alias = "photo")]
    Photo,
    #[serde(alias = "place_id")]
    PlaceId,
    #[serde(alias = "plus_code")]
    PlusCode,
    #[serde(alias = "type")]
    Type,
    #[serde(alias = "url")]
    Url,
    #[serde(alias = "utc_offset")]
    UtcOffset,
    #[serde(alias = "vicinity")]
    Vicinity,
    #[serde(alias = "wheelchair_accessible_entrance")]
    WheelchairAccessibleEntrance,
    // Contact
    #[serde(alias = "current_opening_hours")]
    CurrentOpeningHours,
    #[serde(alias = "formatted_phone_number")]
    FormattedPhoneNumber,
    #[serde(alias = "international_phone_number")]
    InternationalPhoneNumber,
    #[serde(alias = "opening_hours")]
    OpeningHours,
    #[serde(alias = "secondary_opening_hours")]
    SecondaryOpeningHours,
    #[serde(alias = "website")]
    Website,
    // Atmosphere
    #[serde(alias = "curbside_pickup")]
    CurbsidePickup,
    #[serde(alias = "delivery")]
    Delivery,
    #[serde(alias = "dine_in")]
    DineIn,
    #[serde(alias = "editorial_summary")]
    EditorialSummary,
    #[serde(alias = "price_level")]
    PriceLevel,
    #[serde(alias = "rating")]
    Rating,
    #[serde(alias = "reservable")]
    Reservable,
    #[serde(alias = "reviews")]
    Reviews,
    #[serde(alias = "serves_beer")]
    ServesBeer,
    #[serde(alias = "serves_breakfast")]
    ServesBreakfast,
    #[serde(alias = "serves_brunch")]
    ServesBrunch,
    #[serde(alias = "serves_lunch")]
    ServesLunch,
    #[serde(alias = "serves_vegetarian_food")]
    ServesVegetarianFood,
    #[serde(alias = "serves_wine")]
    ServesWine,
    #[serde(alias = "takeout")]
    Takeout,
    #[serde(alias = "user_ratings_total")]
    UserRatingsTotal,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for Field {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match Field::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string()))
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Field> for String {
    /// Converts a `Field` enum to a `String` that contains a
    /// [field](https://developers.google.com/maps/documentation/places/web-service/details#fields)
    /// code.
    fn from(field: &Field) -> String {
        match field {
            // Basic
            Field::AddressComponent => String::from("address_component"),
            Field::AdrAddress => String::from("adr_address"),
            Field::BusinessStatus => String::from("business_status"),
            Field::FormattedAddress => String::from("formatted_address"),
            Field::Geometry => String::from("geometry"),
            Field::Icon => String::from("icon"),
            Field::IconMaskBaseUri => String::from("icon_mask_base_uri"),
            Field::IconBackgroundColor => String::from("icon_background_color"),
            Field::Name => String::from("name"),
            Field::Photo => String::from("photo"),
            Field::PlaceId => String::from("place_id"),
            Field::PlusCode => String::from("plus_code"),
            Field::Type => String::from("type"),
            Field::Url => String::from("url"),
            Field::UtcOffset => String::from("utc_offset"),
            Field::Vicinity => String::from("vicinity"),
            Field::WheelchairAccessibleEntrance => String::from("wheelchair_accessible_entrance"),
            // Contact
            Field::CurrentOpeningHours => String::from("current_opening_hours"),
            Field::FormattedPhoneNumber => String::from("formatted_phone_number"),
            Field::InternationalPhoneNumber => String::from("international_phone_number"),
            Field::OpeningHours => String::from("opening_hours"),
            Field::SecondaryOpeningHours => String::from("secondary_opening_hours"),
            Field::Website => String::from("website"),
            // Atmosphere
            Field::CurbsidePickup => String::from("curbside_pickup"),
            Field::Delivery => String::from("delivery"),
            Field::DineIn => String::from("dine_in"),
            Field::EditorialSummary => String::from("editorial_summary"),
            Field::PriceLevel => String::from("price_level"),
            Field::Rating => String::from("rating"),
            Field::Reservable => String::from("reservable"),
            Field::Reviews => String::from("reviews"),
            Field::ServesBeer => String::from("serves_beer"),
            Field::ServesBreakfast => String::from("serves_breakfast"),
            Field::ServesBrunch => String::from("serves_brunch"),
            Field::ServesLunch => String::from("serves_lunch"),
            Field::ServesVegetarianFood => String::from("serves_vegetarian_food"),
            Field::ServesWine => String::from("serves_wine"),
            Field::Takeout => String::from("takeout"),
            Field::UserRatingsTotal => String::from("user_ratings_total"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static FIELD_TYPES_BY_CODE: phf::Map<&'static str, Field> = phf_map! {
    // Basic
    "address_component" => Field::AddressComponent,
    "adr_address" => Field::AdrAddress,
    "business_status" => Field::BusinessStatus,
    "formatted_address" => Field::FormattedAddress,
    "geometry" => Field::Geometry,
    "icon" => Field::Icon,
    "icon_mask_base_uri" => Field::IconMaskBaseUri,
    "icon_background_color" => Field::IconBackgroundColor,
    "name" => Field::Name,
    "photo" => Field::Photo,
    "place_id" => Field::PlaceId,
    "plus_code" => Field::PlusCode,
    "type" => Field::Type,
    "url" => Field::Url,
    "utc_offset" => Field::UtcOffset,
    "vicinity" => Field::Vicinity,
    "wheelchair_accessible_entrance" => Field::WheelchairAccessibleEntrance,
    // Contact
    "current_opening_hours" => Field::CurrentOpeningHours,
    "formatted_phone_number" => Field::FormattedPhoneNumber,
    "international_phone_number" => Field::InternationalPhoneNumber,
    "opening_hours" => Field::OpeningHours,
    "secondary_opening_hours" => Field::SecondaryOpeningHours,
    "website" => Field::Website,
    // Atmosphere
    "curbside_pickup" => Field::CurbsidePickup,
    "delivery" => Field::Delivery,
    "dine_in" => Field::DineIn,
    "editorial_summary" => Field::EditorialSummary,
    "price_level" => Field::PriceLevel,
    "rating" => Field::Rating,
    "reservable" => Field::Reservable,
    "reviews" => Field::Reviews,
    "serves_beer" => Field::ServesBeer,
    "serves_breakfast" => Field::ServesBreakfast,
    "serves_brunch" => Field::ServesBrunch,
    "serves_lunch" => Field::ServesLunch,
    "serves_vegetarian_food" => Field::ServesVegetarianFood,
    "serves_wine" => Field::ServesWine,
    "takeout" => Field::Takeout,
    "user_ratings_total" => Field::UserRatingsTotal,
};

impl std::convert::TryFrom<&str> for Field {
    // Error definitions are contained in the `google_maps\src\places\error.rs` module.
    type Error = crate::places::error::Error;
    /// Gets a `Field` enum from a `String` that contains a supported
    /// [field](https://developers.google.com/maps/documentation/places/web-service/details#fields)
    /// code.
    fn try_from(field_code: &str) -> Result<Self, Self::Error> {
        FIELD_TYPES_BY_CODE
            .get(field_code)
            .cloned()
            .ok_or_else(|| Error::InvalidFieldCode(field_code.to_string()))
    } // fn
} // impl

impl std::str::FromStr for Field {
    // Error definitions are contained in the `google_maps\src\places\error.rs` module.
    type Err = crate::places::error::Error;
    /// Gets a `Field` enum from a `String` that contains a supported
    /// [field](https://developers.google.com/maps/documentation/places/web-service/details#fields)
    /// code.
    fn from_str(field_code: &str) -> Result<Self, Self::Err> {
        FIELD_TYPES_BY_CODE
            .get(field_code)
            .cloned()
            .ok_or_else(|| Error::InvalidFieldCode(field_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for Field {
    /// Returns a reasonable default variant for the `Field` enum type.
    fn default() -> Self {
        Field::Name
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for Field {
    /// Formats a `Field` enum into a string that is presentable to the end
    /// user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Field::AddressComponent => write!(f, "Address Component"),
            Field::AdrAddress => write!(f, "adr Address"),
            Field::BusinessStatus => write!(f, "Business Status"),
            Field::FormattedAddress => write!(f, "Formatted Address"),
            Field::Geometry => write!(f, "Geometry"),
            Field::Icon => write!(f, "Icon"),
            Field::IconMaskBaseUri => write!(f, "Icon Mask Base URI"),
            Field::IconBackgroundColor => write!(f, "Icon Background Color"),
            Field::Name => write!(f, "Name"),
            Field::Photo => write!(f, "Photo"),
            Field::PlaceId => write!(f, "Place ID"),
            Field::PlusCode => write!(f, "Plus Code"),
            Field::Type => write!(f, "Type"),
            Field::Url => write!(f, "URL"),
            Field::UtcOffset => write!(f, "UTC Offset"),
            Field::Vicinity => write!(f, "Vicinity"),
            Field::WheelchairAccessibleEntrance => write!(f, "Wheelchair Accessible Entrance"),
            // Contact
            Field::CurrentOpeningHours => write!(f, "Current Opening Hours"),
            Field::FormattedPhoneNumber => write!(f, "Formatted Phone Number"),
            Field::InternationalPhoneNumber => write!(f, "International Phone Number"),
            Field::OpeningHours => write!(f, "Opening Hours"),
            Field::SecondaryOpeningHours => write!(f, "Secondary Opening Hours"),
            Field::Website => write!(f, "Website"),
            // Atmosphere
            Field::CurbsidePickup => write!(f, "Curbside Pickup"),
            Field::Delivery => write!(f, "Delivery"),
            Field::DineIn => write!(f, "Dine In"),
            Field::EditorialSummary => write!(f, "Editorial Summary"),
            Field::PriceLevel => write!(f, "Price Level"),
            Field::Rating => write!(f, "Rating"),
            Field::Reservable => write!(f, "Reservable"),
            Field::Reviews => write!(f, "Reviews"),
            Field::ServesBeer => write!(f, "Serves Beer"),
            Field::ServesBreakfast => write!(f, "Serves Breakfast"),
            Field::ServesBrunch => write!(f, "Serves Brunch"),
            Field::ServesLunch => write!(f, "Serves Lunch"),
            Field::ServesVegetarianFood => write!(f, "Serves Vegetarian Food"),
            Field::ServesWine => write!(f, "Serves Wine"),
            Field::Takeout => write!(f, "Takeout"),
            Field::UserRatingsTotal => write!(f, "User Ratings Total"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl Field {
    /// A helper function that converts a `Vec<Field>` (i.e. an array of
    /// `Field` enum) to a `String` that contains a comma-delimited list of
    /// [field](https://developers.google.com/maps/documentation/places/web-service/details#fields)
    /// codes.
    pub fn vec_to_csv(fields: &[Field]) -> String {
        fields
            .iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join(",")
    } // fn
} // impl