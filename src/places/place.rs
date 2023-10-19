//! Attributes describing a place. Not all attributes will be available for all
//! place types.

use crate::places::{
    BusinessStatus, PlaceEditorialSummary, PlaceOpeningHours, PlacePhoto, PlaceReview,
};
use crate::types::{AddressComponent, Geometry, PlaceType};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Attributes describing a place. Not all attributes will be available for all
/// place types.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Place {
    /// An array containing the separate components applicable to this address.
    ///
    /// See [AddressComponent](https://developers.google.com/maps/documentation/places/web-service/search-text#AddressComponent)
    /// for more information.
    pub address_components: Option<Vec<AddressComponent>>,

    /// A representation of the place's address in the
    /// [adr microformat](http://microformats.org/wiki/adr).
    pub adr_address: Option<String>,

    /// Indicates the operational status of the place, if it is a business. If
    /// no data exists, `business_status` is not returned.
    ///
    /// The allowed values include: `OPERATIONAL`, `CLOSED_TEMPORARILY`, and
    /// `CLOSED_PERMANENTLY`
    pub business_status: Option<BusinessStatus>,

    /// Specifies if the business supports curbside pickup.
    pub curbside_pickup: Option<bool>,

    /// Contains the hours of operation for the next seven days (including
    /// today). The time period starts at midnight on the date of the request
    /// and ends at 11:59 pm six days later. This field includes the
    /// `special_days` subfield of all hours, set for dates that have
    /// exceptional hours.
    ///
    /// See [PlaceOpeningHours](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceOpeningHours)
    /// for more information.
    pub current_opening_hours: Option<PlaceOpeningHours>,

    /// Specifies if the business supports delivery.
    pub delivery: Option<bool>,

    /// Specifies if the business supports indoor or outdoor seating options.
    pub dine_in: Option<bool>,

    /// Contains a summary of the place. A summary is comprised of a textual
    /// overview, and also includes the language code for these if applicable.
    /// Summary text must be presented as-is and can not be modified or altered.
    ///
    /// See [PlaceEditorialSummary](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceEditorialSummary)
    /// for more information.
    pub editorial_summary: Option<PlaceEditorialSummary>,

    /// A string containing the human-readable address of this place.
    ///
    /// Often this address is equivalent to the postal address. Note that some
    /// countries, such as the United Kingdom, do not allow distribution of true
    /// postal addresses due to licensing restrictions.
    ///
    /// The formatted address is logically composed of one or more address
    /// components. For example, the address "111 8th Avenue, New York, NY"
    /// consists of the following components: "111" (the street number), "8th
    /// Avenue" (the route), "New York" (the city) and "NY" (the US state).
    ///
    /// Do not parse the formatted address programmatically. Instead you should
    /// use the individual address components, which the API response includes
    /// in addition to the formatted address field.
    pub formatted_address: Option<String>,

    /// Contains the place's phone number in its
    /// [local format](http://en.wikipedia.org/wiki/Local_conventions_for_writing_telephone_numbers).
    pub formatted_phone_number: Option<String>,

    /// Contains the location and viewport for the location.
    ///
    /// See [Geometry](https://developers.google.com/maps/documentation/places/web-service/search-text#Geometry) for more information.
    pub geometry: Option<Geometry>,

    /// Contains the URL of a suggested icon which may be displayed to the user
    /// when indicating this result on a map.
    pub icon: Option<String>,

    /// Contains the default HEX color code for the place's category.
    pub icon_background_color: Option<String>,

    /// Contains the URL of a recommended icon, minus the `.svg` or `.png` file
    /// type extension.
    pub icon_mask_base_uri: Option<String>,

    /// Contains the place's phone number in international format. International
    /// format includes the country code, and is prefixed with the plus, +,
    /// sign. For example, the international_phone_number for Google's Sydney,
    /// Australia office is `+61 2 9374 4000`.
    pub international_phone_number: Option<String>,

    /// Contains the human-readable name for the returned result. For
    /// `establishment` results, this is usually the canonicalized business
    /// name.
    pub name: Option<String>,

    /// Contains the regular hours of operation.
    ///
    /// See [PlaceOpeningHours](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceOpeningHours)
    /// for more information.
    pub opening_hours: Option<PlaceOpeningHours>,

    /// An array of photo objects, each containing a reference to an image. A
    /// request may return up to ten photos. More information about place photos
    /// and how you can use the images in your application can be found in the
    /// [Place Photos](https://developers.google.com/maps/documentation/places/web-service/photos)
    /// documentation.
    ///
    /// See [PlacePhoto](https://developers.google.com/maps/documentation/places/web-service/search-text#PlacePhoto)
    /// for more information.
    pub photos: Option<Vec<PlacePhoto>>,

    /// A textual identifier that uniquely identifies a place. To retrieve
    /// information about the place, pass this identifier in the `place_id`
    /// field of a Places API request. For more information about place IDs, see
    /// the [place ID overview](https://developers.google.com/maps/documentation/places/web-service/place-id).
    pub place_id: Option<String>,

    /// The price level of the place, on a scale of 0 to 4. The exact amount
    /// indicated by a specific value will vary from region to region. Price
    /// levels are interpreted as follows:
    ///
    /// * 0 Free
    /// * 1 Inexpensive
    /// * 2 Moderate
    /// * 3 Expensive
    /// * 4 Very Expensive
    pub price_level: Option<u8>,

    /// Contains the place's rating, from 1.0 to 5.0, based on aggregated user
    /// reviews.
    pub rating: Option<Decimal>,

    /// Specifies if the place supports reservations.
    pub reservable: Option<bool>,

    /// A JSON array of up to five reviews. By default, the reviews are sorted
    /// in order of relevance. Use the `reviews_sort` request parameter to
    /// control sorting.
    ///
    /// * For `most_relevant` (default), reviews are sorted by relevance; the
    /// service will bias the results to return reviews originally written in
    /// the preferred language.
    ///
    /// * For `newest`, reviews are sorted in chronological order; the preferred
    /// language does not affect the sort order.
    ///
    /// Google recommends indicating to users whether results are ordered by
    /// `most_relevant` or `newest`.
    ///
    /// See [PlaceReview](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceReview)
    /// for more information.
    pub reviews: Option<Vec<PlaceReview>>,

    /// Contains an array of entries for the next seven days including
    /// information about secondary hours of a business. Secondary hours are
    /// different from a business's main hours. For example, a restaurant can
    /// specify drive through hours or delivery hours as its secondary hours.
    /// This field populates the type subfield, which draws from a predefined
    /// list of opening hours types (such as `DRIVE_THROUGH`, `PICKUP`, or
    /// `TAKEOUT`) based on the types of the place. This field includes the
    /// `special_days` subfield of all hours, set for dates that have
    /// exceptional hours.
    ///
    /// See [PlaceOpeningHours](https://developers.google.com/maps/documentation/places/web-service/search-text#PlaceOpeningHours)
    /// for more information.
    pub secondary_opening_hours: Option<PlaceOpeningHours>,

    /// Specifies if the place serves beer.
    pub serves_beer: Option<bool>,

    /// Specifies if the place serves breakfast.
    pub serves_breakfast: Option<bool>,

    /// Specifies if the place serves brunch.
    pub serves_brunch: Option<bool>,

    /// Specifies if the place serves dinner.
    pub serves_dinner: Option<bool>,

    /// Specifies if the place serves lunch.
    pub serves_lunch: Option<bool>,

    /// Specifies if the place serves vegetarian food.
    pub serves_vegetarian_food: Option<bool>,

    /// Specifies if the place serves wine.
    pub serves_wine: Option<bool>,

    /// Specifies if the business supports takeout.
    pub takeout: Option<bool>,

    /// Contains an array of feature types describing the given result. See the
    /// list of [supported types](https://developers.google.com/maps/documentation/places/web-service/supported_types#table2).
    pub types: Option<Vec<PlaceType>>,

    /// Contains the URL of the official Google page for this place. This will
    /// be the Google-owned page that contains the best available information
    /// about the place. Applications must link to or embed this page on any
    /// screen that shows detailed results about the place to the user.
    pub url: Option<String>,

    /// The total number of reviews, with or without text, for this place.
    pub user_ratings_total: Option<u64>,

    /// Contains the number of minutes this place’s current timezone is offset
    /// from UTC. For example, for places in Sydney, Australia during daylight
    /// saving time this would be 660 (+11 hours from UTC), and for places in
    /// California outside of daylight saving time this would be -480 (-8 hours
    /// from UTC).
    pub utc_offset: Option<i32>,

    /// For establishment (`types:["establishment", ...]`) results only, the
    /// `vicinity` field contains a simplified address for the place, including
    /// the street name, street number, and locality, but not the
    /// province/state, postal code, or country.
    ///
    /// For all other results, the `vicinity` field contains the name of the
    /// narrowest political (`types:["political", ...]`) feature that is present
    /// in the address of the result.
    ///
    /// This content is meant to be read as-is. Do not programmatically parse
    /// the formatted address.
    pub vicinity: Option<String>,

    /// The authoritative website for this place, such as a business' homepage.
    pub website: Option<String>,

    /// Specifies if the place has an entrance that is wheelchair-accessible.
    pub wheelchair_accessible_entrance: Option<bool>,
} // struct Place

/// ----------------------------------------------------------------------------

impl std::str::FromStr for Place {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Places API JSON response into a usable `Place`
    /// struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    } // fn from_str
} // impl FromStr
