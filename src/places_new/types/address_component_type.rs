use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

// -------------------------------------------------------------------------------------------------
//
// Last updated: 2025-08-31
//
/// Address component types identify individual pieces within an address structure.
///
/// These types answer "What part of the address is this?" - for example, is this piece the street
/// number, the street name, or the apartment number? Used for parsing and breaking down addresses
/// into their constituent parts.
///
/// # [Address types and address component types](https://developers.google.com/maps/documentation/places/web-service/place-types#address-types)
///
/// The types array in the response indicates the _address type_. Examples of address types include
/// a street address, a country, or a political entity. The `types` array in the `AddressComponent`
/// field indicates the type of each part of the address. Examples include street number or country.
///
/// Addresses may have multiple types. The types may be considered 'tags'. For example, many cities
/// are tagged with the `political` and `locality` types.
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
#[repr(u8)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AddressComponentType {
    /// A precise street address.
    StreetAddress = 0,

    /// A named route (such as "US 101").
    Route = 1,

    /// A major intersection, usually of two major roads.
    Intersection = 2,

    /// A political entity. Usually indicates a polygon of some civil administration.
    Political = 3,

    /// The national political entity, typically the highest order type returned by the Geocoder.
    Country = 4,

    /// A first-order civil entity below the country level (e.g., states in the US).
    #[serde(rename = "administrative_area_level_1")]
    AdministrativeAreaLevel1 = 5,

    /// A second-order civil entity below the country level (e.g., counties in the US).
    #[serde(rename = "administrative_area_level_2")]
    AdministrativeAreaLevel2 = 6,

    /// A third-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_3")]
    AdministrativeAreaLevel3 = 7,

    /// A fourth-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_4")]
    AdministrativeAreaLevel4 = 8,

    /// A fifth-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_5")]
    AdministrativeAreaLevel5 = 9,

    /// A sixth-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_6")]
    AdministrativeAreaLevel6 = 10,

    /// A seventh-order civil entity below the country level.
    #[serde(rename = "administrative_area_level_7")]
    AdministrativeAreaLevel7 = 11,

    /// A commonly-used alternative name for the entity.
    ColloquialArea = 12,

    /// An incorporated city or town political entity.
    Locality = 13,

    /// A first-order civil entity below a locality.
    ///
    /// For some locations may receive one of the additional types: `sublocality_level_1` to
    /// `sublocality_level_5`. Each sublocality level is a civil entity. Larger numbers indicate a
    /// smaller geographic area.
    Sublocality = 14,

    /// A first-order civil entity below a locality.
    #[serde(rename = "sublocality_level_1")]
    SublocalityLevel1 = 15,

    /// A second-order civil entity below a locality.
    #[serde(rename = "sublocality_level_2")]
    SublocalityLevel2 = 16,

    /// A third-order civil entity below a locality.
    #[serde(rename = "sublocality_level_3")]
    SublocalityLevel3 = 17,

    /// A fourth-order civil entity below a locality.
    #[serde(rename = "sublocality_level_4")]
    SublocalityLevel4 = 18,

    /// A fifth-order civil entity below a locality.
    #[serde(rename = "sublocality_level_5")]
    SublocalityLevel5 = 19,

    /// A named neighborhood.
    Neighborhood = 20,

    /// A named location, usually a building or collection of buildings with a common name.
    Premise = 21,

    /// An addressable entity below the premise level, such as an apartment, unit, or suite.
    Subpremise = 22,

    /// An encoded location reference, derived from latitude and longitude.
    PlusCode = 23,

    /// A postal code for mail delivery.
    PostalCode = 24,

    /// A prefix component of a postal code.
    PostalCodePrefix = 25,

    /// A suffix component of a postal code.
    PostalCodeSuffix = 26,

    /// A geographical feature that occurs naturally.
    NaturalFeature = 27,

    /// An airport facility.
    Airport = 28,

    /// A park or recreational area.
    Park = 29,

    /// A notable location or attraction.
    PointOfInterest = 30,

    // In addition to the above, address components may include the types listed below.

    /// The floor of a building address
    Floor = 31,

    /// Typically a place that has not yet been categorized
    Establishment = 32,

    /// A nearby place that is used as a reference, to aid navigation
    Landmark = 33,

    /// A parking lot or parking structure
    Parking = 34,

    /// A specific postal box
    PostBox = 35,

    /// A grouping of geographic areas, such as locality and sublocality, used for mailing addresses in some countries
    PostalTown = 36,

    /// The room of a building address.
    Room = 37,

    /// The precise street number
    StreetNumber = 38,

    /// Undocumented
    BusStation = 39,

    /// Undocumented
    TrainStation = 40,

    /// Undocumented
    TransitStation = 41,
}