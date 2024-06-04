//! Contains the `Field` enum and its associated traits. It specifies the
//! fields in the place details that should be returned. For example, business
//! status, price level, wheelchair accessible, and so on.

use crate::error::Error as GoogleMapsError;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
///   the same fields. Place Search requests return a subset of the fields that
///   are returned by Place Details requests. If the field you want is not
///   returned by Place Search, you can use Place Search to get a `place_id`,
///    then use that Place ID to make a Place Details request. For more
///   information on the fields that are unavailable in a Place Search request,
///   see [Places API fields
///   support](https://developers.google.com/maps/documentation/places/web-service/place-data-fields#places-api-fields-support).

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
#[non_exhaustive]
pub enum Field {
    // Basic
    AddressComponent = 0,
    AdrAddress = 1,
    BusinessStatus = 2,
    FormattedAddress = 3,
    Geometry = 4,
    Icon = 5,
    IconMaskBaseUri = 6,
    IconBackgroundColor = 8,
    #[default]
    Name = 9,
    Photo = 10,
    PlaceId = 11,
    PlusCode = 12,
    Type = 13,
    Url = 14,
    UtcOffset = 15,
    Vicinity = 16,
    WheelchairAccessibleEntrance = 17,
    // Contact
    CurrentOpeningHours = 18,
    FormattedPhoneNumber = 19,
    InternationalPhoneNumber = 20,
    OpeningHours = 21,
    SecondaryOpeningHours = 22,
    Website = 23,
    // Atmosphere
    CurbsidePickup = 24,
    Delivery = 25,
    DineIn = 26,
    EditorialSummary = 27,
    PriceLevel = 28,
    Rating = 29,
    Reservable = 30,
    Reviews = 31,
    ServesBeer = 32,
    ServesBreakfast = 33,
    ServesBrunch = 34,
    ServesLunch = 35,
    ServesVegetarianFood = 36,
    ServesWine = 37,
    Takeout = 38,
    UserRatingsTotal = 39,
    /// If the field is not recognized by
    /// [serde](https://crates.io/crates/serde) when reading data from
    /// Google it will be assigned to this `Other` variant.
    ///
    /// As new types are added to Google Maps, they must also be added to this
    /// crate. However, in the meantime, the `Other` catch-all variant allows
    /// `serde` to read data from Google without producing an error until the
    /// new variant added to this `enum`.
    Other(String) = 40,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for Field {
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

impl Serialize for Field {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl<'a> std::convert::From<&'a Field> for &'a str {
    /// Converts a `Field` enum to a `String` that contains a
    /// [field](https://developers.google.com/maps/documentation/places/web-service/details#fields)
    /// code.
    fn from(field: &'a Field) -> Self {
        match field {
            // Basic
            Field::AddressComponent => "address_component",
            Field::AdrAddress => "adr_address",
            Field::BusinessStatus => "business_status",
            Field::FormattedAddress => "formatted_address",
            Field::Geometry => "geometry",
            Field::Icon => "icon",
            Field::IconMaskBaseUri => "icon_mask_base_uri",
            Field::IconBackgroundColor => "icon_background_color",
            Field::Name => "name",
            Field::Photo => "photo",
            Field::PlaceId => "place_id",
            Field::PlusCode => "plus_code",
            Field::Type => "type",
            Field::Url => "url",
            Field::UtcOffset => "utc_offset",
            Field::Vicinity => "vicinity",
            Field::WheelchairAccessibleEntrance => "wheelchair_accessible_entrance",
            // Contact
            Field::CurrentOpeningHours => "current_opening_hours",
            Field::FormattedPhoneNumber => "formatted_phone_number",
            Field::InternationalPhoneNumber => "international_phone_number",
            Field::OpeningHours => "opening_hours",
            Field::SecondaryOpeningHours => "secondary_opening_hours",
            Field::Website => "website",
            // Atmosphere
            Field::CurbsidePickup => "curbside_pickup",
            Field::Delivery => "delivery",
            Field::DineIn => "dine_in",
            Field::EditorialSummary => "editorial_summary",
            Field::PriceLevel => "price_level",
            Field::Rating => "rating",
            Field::Reservable => "reservable",
            Field::Reviews => "reviews",
            Field::ServesBeer => "serves_beer",
            Field::ServesBreakfast => "serves_breakfast",
            Field::ServesBrunch => "serves_brunch",
            Field::ServesLunch => "serves_lunch",
            Field::ServesVegetarianFood => "serves_vegetarian_food",
            Field::ServesWine => "serves_wine",
            Field::Takeout => "takeout",
            Field::UserRatingsTotal => "user_ratings_total",
            Field::Other(string) => string,
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for Field {
    /// Converts a `Field` enum to a `String` that contains a
    /// [field](https://developers.google.com/maps/documentation/places/web-service/details#fields)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Self> for Field {
    /// Converts a borrowed `&Field` enum into an owned `Field` enum by cloning
    /// it.
    fn from(field: &Self) -> Self {
        field.clone()
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Field> for String {
    /// Converts a `Field` enum to a `String` that contains a
    /// [field](https://developers.google.com/maps/documentation/places/web-service/details#fields)
    /// code.
    fn from(field: &Field) -> Self {
        std::convert::Into::<&str>::into(field).to_string()
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

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for Field {
    // Error definitions are contained in the `google_maps\src\places\error.rs` module.
    type Error = GoogleMapsError;
    /// Gets a `Field` enum from a `String` that contains a supported
    /// [field](https://developers.google.com/maps/documentation/places/web-service/details#fields)
    /// code.
    fn try_from(field_code: &str) -> Result<Self, Self::Error> {
        Ok(FIELD_TYPES_BY_CODE
            .get(field_code)
            .cloned()
            .unwrap_or_else(|| Self::Other(field_code.to_string())))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for Field {
    // Error definitions are contained in the `google_maps\src\places\error.rs` module.
    type Err = GoogleMapsError;
    /// Gets a `Field` enum from a `String` that contains a supported
    /// [field](https://developers.google.com/maps/documentation/places/web-service/details#fields)
    /// code.
    fn from_str(field_code: &str) -> Result<Self, Self::Err> {
        Ok(FIELD_TYPES_BY_CODE
            .get(field_code)
            .cloned()
            .unwrap_or_else(|| Self::Other(field_code.to_string())))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl Field {
    /// Formats a `Field` enum into a string that is presentable to the end
    /// user.
    #[must_use]
    pub fn display(&self) -> &str {
        match self {
            Self::AddressComponent => "Address Component",
            Self::AdrAddress => "adr Address",
            Self::BusinessStatus => "Business Status",
            Self::FormattedAddress => "Formatted Address",
            Self::Geometry => "Geometry",
            Self::Icon => "Icon",
            Self::IconMaskBaseUri => "Icon Mask Base URI",
            Self::IconBackgroundColor => "Icon Background Color",
            Self::Name => "Name",
            Self::Photo => "Photo",
            Self::PlaceId => "Place ID",
            Self::PlusCode => "Plus Code",
            Self::Type => "Type",
            Self::Url => "URL",
            Self::UtcOffset => "UTC Offset",
            Self::Vicinity => "Vicinity",
            Self::WheelchairAccessibleEntrance => "Wheelchair Accessible Entrance",
            // Contact
            Self::CurrentOpeningHours => "Current Opening Hours",
            Self::FormattedPhoneNumber => "Formatted Phone Number",
            Self::InternationalPhoneNumber => "International Phone Number",
            Self::OpeningHours => "Opening Hours",
            Self::SecondaryOpeningHours => "Secondary Opening Hours",
            Self::Website => "Website",
            // Atmosphere
            Self::CurbsidePickup => "Curbside Pickup",
            Self::Delivery => "Delivery",
            Self::DineIn => "Dine In",
            Self::EditorialSummary => "Editorial Summary",
            Self::PriceLevel => "Price Level",
            Self::Rating => "Rating",
            Self::Reservable => "Reservable",
            Self::Reviews => "Reviews",
            Self::ServesBeer => "Serves Beer",
            Self::ServesBreakfast => "Serves Breakfast",
            Self::ServesBrunch => "Serves Brunch",
            Self::ServesLunch => "Serves Lunch",
            Self::ServesVegetarianFood => "Serves Vegetarian Food",
            Self::ServesWine => "Serves Wine",
            Self::Takeout => "Takeout",
            Self::UserRatingsTotal => "User Ratings Total",
            Self::Other(string) => string,
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl Field {
    /// A helper function that converts a `Vec<Field>` (i.e. an array of
    /// `Field` enum) to a `String` that contains a comma-delimited list of
    /// [field](https://developers.google.com/maps/documentation/places/web-service/details#fields)
    /// codes.
    pub fn vec_to_csv(fields: &[Self]) -> String {
        fields
            .iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join(",")
    } // fn
} // impl
