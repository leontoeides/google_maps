//! Contains the `AutocompleteType` enum and its associated traits. It is used
//! to restrict results to certain place types.

// -----------------------------------------------------------------------------

use crate::places::place_autocomplete::error::Error;
use phf::phf_map;
use serde::{Deserialize, Serialize, Deserializer};

// -----------------------------------------------------------------------------

/// You may restrict results from a Place Autocomplete request to be of a
/// certain type by passing a [types
/// parameter](https://developers.google.com/maps/documentation/places/web-service/autocomplete#types).
/// The parameter specifies a type or a type collection, as listed in the
/// supported types below. If nothing is specified, all types are returned. In
/// general only a single type is allowed. The exception is that you can safely
/// mix the geocode and establishment types, but note that this will have the
/// same effect as specifying no types.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AutocompleteType {
    /// Instructs the Place Autocomplete service to return only geocoding
    /// results, rather than business results. Generally, you use this request
    /// to disambiguate results where the location specified may be
    /// indeterminate.
    #[serde(alias = "geocode")]
    Geocode,
    /// Instructs the Place Autocomplete service to return only geocoding
    /// results with a precise address. Generally, you use this request when you
    /// know the user will be looking for a fully specified address.
    /// indeterminate.
    #[serde(alias = "address")]
    Address,
    /// Instructs the Place Autocomplete service to return only business
    /// results.
    #[serde(alias = "establishment")]
    Establishment,
    /// Type collection instructs the Places service to return any result
    /// matching the following types:
    /// * `locality`
    /// * `sublocality`
    /// * `postal_code`
    /// * `country`
    /// * `administrative_area_level_1`
    /// * `administrative_area_level_2`
    #[serde(alias = "(regions)")]
    Regions,
    /// Type collection instructs the Places service to return results that
    /// match `locality` or `administrative_area_level_3`.
    #[serde(alias = "(cities)")]
    Cities,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for AutocompleteType {
    /// Manual implementation of `Deserialize` for `serde`. This will take
    /// advantage of the `phf`-powered `TryFrom` implementation for this type.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        match AutocompleteType::try_from(string.as_str()) {
            Ok(variant) => Ok(variant),
            Err(error) => Err(serde::de::Error::custom(error.to_string()))
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&AutocompleteType> for String {
    /// Converts a `AutocompleteType` enum to a `String` that contains a
    /// [autocomplete
    /// type](https://developers.google.com/maps/documentation/places/web-service/autocomplete#types)
    /// code.
    fn from(autocomplete_type: &AutocompleteType) -> String {
        match autocomplete_type {
            AutocompleteType::Geocode => String::from("geocode"),
            AutocompleteType::Address => String::from("address"),
            AutocompleteType::Establishment => String::from("establishment"),
            AutocompleteType::Regions => String::from("(regions)"),
            AutocompleteType::Cities => String::from("(cities)"),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

static AUTOCOMPLETE_TYPES_BY_CODE: phf::Map<&'static str, AutocompleteType> = phf_map! {
    "geocode" => AutocompleteType::Geocode,
    "address" => AutocompleteType::Address,
    "establishment" => AutocompleteType::Establishment,
    "(regions)" => AutocompleteType::Regions,
    "regions" => AutocompleteType::Regions,
    "(cities)" => AutocompleteType::Cities,
    "cities" => AutocompleteType::Cities,
};

impl std::convert::TryFrom<&str> for AutocompleteType {
    // Error definitions are contained in the
    // `google_maps\src\places\place_autocomplete\error.rs` module.
    type Error = crate::places::place_autocomplete::error::Error;
    /// Gets a `AutocompleteType` enum from a `String` that contains a valid
    /// [autocomplete
    /// type](https://developers.google.com/maps/documentation/places/web-service/autocomplete#types)
    /// code.
    fn try_from(autocomplete_code: &str) -> Result<AutocompleteType, Error> {
        AUTOCOMPLETE_TYPES_BY_CODE
            .get(autocomplete_code)
            .cloned()
            .ok_or_else(|| Error::InvalidAutocompleteType(autocomplete_code.to_string()))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::default::Default for AutocompleteType {
    /// Returns a reasonable default variant for the `AutocompleteType` enum
    /// type.
    fn default() -> Self {
        AutocompleteType::Address
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for AutocompleteType {
    /// Formats a `AutocompleteType` enum into a string that is presentable to
    /// the end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AutocompleteType::Geocode => write!(f, "Geocode"),
            AutocompleteType::Address => write!(f, "Address"),
            AutocompleteType::Establishment => write!(f, "Establishment"),
            AutocompleteType::Regions => write!(f, "Regions"),
            AutocompleteType::Cities => write!(f, "Cities"),
        } // match
    } // fn
} // impl