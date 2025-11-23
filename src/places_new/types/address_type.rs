use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

// -------------------------------------------------------------------------------------------------
//
// Last updated: 2025-08-31
//
/// Address types classify entire addresses or geographic areas.
///
/// These types answer "What kind of place is this?" - for example, is this a street address, a
/// city, or a state? Used to categorize whole locations rather than individual parts.
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
#[repr(u16)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AddressType {
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

    /// A first-order civil entity below the country level.
    ///
    /// Within the United States, these administrative levels are states. Not all nations exhibit
    /// these administrative levels. In most cases, `administrative_area_level_1` short names will
    /// closely match ISO 3166-2 subdivisions and other widely circulated lists; however this is not
    /// guaranteed as our geocoding results are based on a variety of signals and location data.
    #[serde(rename = "administrative_area_level_1")]
    AdministrativeAreaLevel1 = 5,

    /// A second-order civil entity below the country level.
    ///
    /// Within the United States, these administrative levels are counties. Not all nations exhibit
    /// these administrative levels.
    #[serde(rename = "administrative_area_level_2")]
    AdministrativeAreaLevel2 = 6,

    /// A third-order civil entity below the country level. This type indicates a minor civil
    /// division. Not all nations exhibit these administrative levels.
    #[serde(rename = "administrative_area_level_3")]
    AdministrativeAreaLevel3 = 7,

    /// A fourth-order civil entity below the country level. This type indicates a minor civil
    /// division. Not all nations exhibit these administrative levels.
    #[serde(rename = "administrative_area_level_4")]
    AdministrativeAreaLevel4 = 8,

    /// A fifth-order civil entity below the country level. This type indicates a minor civil
    /// division. Not all nations exhibit these administrative levels.
    #[serde(rename = "administrative_area_level_5")]
    AdministrativeAreaLevel5 = 9,

    /// A sixth-order civil entity below the country level. This type indicates a minor civil
    /// division. Not all nations exhibit these administrative levels.
    #[serde(rename = "administrative_area_level_6")]
    AdministrativeAreaLevel6 = 10,

    /// A seventh-order civil entity below the country level. This type indicates a minor civil
    /// division. Not all nations exhibit these administrative levels.
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
    ///
    /// Plus codes can be used as a replacement for street addresses in places where they do not
    /// exist (where buildings are not numbered or streets are not named). See <https://plus.codes>
    /// for details.
    PlusCode = 23,

    /// A postal code as used to address postal mail within the country.
    PostalCode = 24,

    /// A prominent natural feature.
    NaturalFeature = 25,

    /// An airport.
    Airport = 26,

    /// A named park.
    Park = 27,

    /// A named point of interest. Typically, these "POI"s are prominent local entities that don't
    /// easily fit in another category, such as "Empire State Building" or "Eiffel Tower".
    PointOfInterest = 28,
}