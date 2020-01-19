//! Resources (enums, structs) for the client process the Geolocation API
//! response from the Google Cloud server.

use crate::{
    bounds::Bounds,
    geocoding::location_type::LocationType,
    latlng::LatLng,
    place_type::PlaceType,
}; // use
use serde::{Serialize, Deserialize};

/// The response from the Google Maps Geolocation API will be stored in this
/// structure.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    /// When the geocoder returns a status code other than `OK`, there may be an
    /// additional `error_message` field within the Geocoding response object.
    /// This field contains more detailed information about the reasons behind
    /// the given status code.
    ///
    /// *Note: This field is not guaranteed to be always present, and its
    /// content is subject to change.*
    pub error_message: Option<String>,
    /// When the geocoder returns results, it places them within a results
    /// array. Even if the geocoder returns no results (such as if the address
    /// doesn't exist) it still returns an empty results array.
    pub results: Vec<Geocoding>,
    /// The `status` field within the Geocoding response object contains the
    /// status of the request, and may contain debugging information to help you
    /// track down why geocoding is not working.
    pub status: Status,
} // struct

/// When the geocoder returns results, it places them within a results array.
/// Even if the geocoder returns no results (such as if the address doesn't
/// exist) it still returns an empty results array.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Geocoding {
    /// Array containing the separate components applicable to this address.
    pub address_components: Vec<AddressComponent>,
    /// A string containing the human-readable address of this location.
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
    pub formatted_address: String,
    /// Contains the geocoded latitude/longitude, recommended viewport for
    /// displaying the returned result, the bounding box, and other additional
    /// data. See the `Geometry` struct declaration.
    pub geometry: Geometry,
    /// Indicates that the geocoder did not return an exact match for the
    /// original request, though it was able to match part of the requested
    /// address. You may wish to examine the original request for misspellings
    /// and/or an incomplete address.
    ///
    /// Partial matches most often occur for street addresses that do not exist
    /// within the locality you pass in the request. Partial matches may also be
    /// returned when a request matches two or more locations in the same
    /// locality. For example, "21 Henr St, Bristol, UK" will return a partial
    /// match for both Henry Street and Henrietta Street. Note that if a request
    /// includes a misspelled address component, the geocoding service may
    /// suggest an alternative address. Suggestions triggered in this way will
    /// also be marked as a partial match.
    pub partial_match: Option<bool>,
    /// A unique identifier that can be used with other Google APIs. For
    /// example, you can use the place_id in a [Places
    /// API](https://developers.google.com/places/web-service/details) request
    /// to get details of a local business, such as phone number, opening hours,
    /// user reviews, and more. See the [place ID
    /// overview](https://developers.google.com/places/place-id).
    pub place_id: String,
    /// (See [Open Location
    /// Code](https://en.wikipedia.org/wiki/Open_Location_Code) and [plus
    /// codes](https://plus.codes/)) is an encoded location reference. Plus
    /// codes can be used as a replacement for street addresses in places where
    /// they do not exist (where buildings are not numbered or streets are not
    /// named.)
    pub plus_code: Option<PlusCode>,
    /// Array indicates the `type` of the returned result. This array contains a
    /// set of zero or more tags identifying the type of feature returned in the
    /// result. For example, a geocode of "Chicago" returns "locality" which
    /// indicates that "Chicago" is a city, and also returns "political" which
    /// indicates it is a political entity.
    pub types: Vec<PlaceType>,
} // struct

/// Contains the separate components applicable to this address.
///
/// Note the following facts about the address_components array:
///
/// * The array of address components may contain more components than the
/// `formatted_address`.
///
/// * The array does not necessarily include all the political entities that
/// contain an address, apart from those included in the `formatted_address`. To
/// retrieve all the political entities that contain a specific address, you
/// should use reverse geocoding, passing the latitude/longitude of the address
/// as a parameter to the request.
///
/// * The format of the response is not guaranteed to remain the same between
/// requests. In particular, the number of `address_components` varies based on
/// the address requested and can change over time for the same address. A
/// component can change position in the array. The type of the component can
/// change. A particular component may be missing in a later response.
///
/// To handle the array of components, you should parse the response and select
/// appropriate values via expressions. See the guide to [parsing a
/// response](https://developers.google.com/maps/documentation/geocoding/web-service-best-practices#Parsing).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddressComponent {
    /// The full text description or name of the address component as returned
    /// by the Geocoder.
    pub long_name: String,
    /// An abbreviated textual name for the address component, if available. For
    /// example, an address component for the state of Alaska may have a
    /// `long_name` of "Alaska" and a `short_name` of "AK" using the 2-letter
    /// postal abbreviation.
    pub short_name: String,
    /// An array indicating the type of the address component. See the list of
    /// [supported
    /// types](https://developers.google.com/places/web-service/supported_types).
    pub types: Vec<PlaceType>,
} // struct

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Geometry {
    /// Contains the geocoded latitude, longitude value. For normal address
    /// lookups, this field is typically the most important.
    pub location: LatLng,
    /// Stores additional data about the specified location.
    pub location_type: LocationType,
    /// Contains the recommended viewport for displaying the returned result,
    /// specified as two latitude/longitude values defining the southwest and
    /// northeast corner of the viewport bounding box. Generally the viewport is
    /// used to frame a result when displaying it to a user.
    pub viewport: Bounds,
    /// Stores the bounding box which can fully contain the returned result.
    /// Note that these bounds may not match the recommended viewport. (For
    /// example, San Francisco includes the [Farallon
    /// islands](https://en.wikipedia.org/wiki/Farallon_Islands), which are
    /// technically part of the city, but probably should not be returned in the
    /// viewport.)
    pub bounds: Option<Bounds>,
} // struct

/// (See [Open Location Code](https://en.wikipedia.org/wiki/Open_Location_Code)
/// and [plus codes](https://plus.codes/)) is an encoded location reference,
/// derived from latitude and longitude coordinates, that represents an area:
/// 1/8000th of a degree by 1/8000th of a degree (about 14m x 14m at the
/// equator) or smaller. Plus codes can be used as a replacement for street
/// addresses in places where they do not exist (where buildings are not
/// numbered or streets are not named).
///
/// Typically, both the global code and compound code are returned. However, if
/// the result is in a remote location (for example, an ocean or desert) only
/// the global code may be returned.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlusCode {
    /// A 4 character area code and 6 character or longer local code
    /// (`849VCWC8+R9`).
    global_code: String,
    /// A 6 character or longer local code with an explicit location (`CWC8+R9,
    /// Mountain View, CA, USA`).
    compound_code: Option<String>,
} // struct

/// Indicates the status of the response.
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Status {
    /// Generally indicates one of the following:
    /// * The query (`address`, `components` or `latlng`) is missing.
    /// * An invalid `result_type` or `location_type` was given.
    #[serde(alias = "INVALID_REQUEST")]
    InvalidRequest,
    /// Indicates that no errors occurred; the address was successfully parsed
    /// and at least one geocode was returned.
    #[serde(alias = "OK")]
    Ok,
    /// Indicates any of the following:
    /// * The API key is missing or invalid.
    /// * Billing has not been enabled on your account.
    /// * A self-imposed usage cap has been exceeded.
    /// * The provided method of payment is no longer valid (for example, a
    /// credit card has expired).
    ///
    /// See the [Maps
    /// FAQ](https://developers.google.com/maps/faq#over-limit-key-error) to
    /// learn how to fix this.
    #[serde(alias = "OVER_DAILY_LIMIT")]
    OverDailyLimit,
    /// Indicates the requestor has exceeded quota.
    #[serde(alias = "OVER_QUERY_LIMIT")]
    OverQueryLimit,
    /// Indicates that the API did not complete the request. Confirm that the
    /// request was sent over HTTPS instead of HTTP.
    #[serde(alias = "REQUEST_DENIED")]
    RequestDenied,
    /// Indicates that the request could not be processed due to a server error.
    /// The request may succeed if you try again.
    #[serde(alias = "UNKNOWN_ERROR")]
    UnknownError,
    /// Indicates that the geocode was successful but returned no results. This
    /// may occur if the geocoder was passed a non-existent `address`. This may
    /// also occur if the geocoder was passed a `latlng` in a remote location.
    #[serde(alias = "ZERO_RESULTS")]
    ZeroResults,
} // struct