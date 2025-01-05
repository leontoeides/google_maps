//! Contains the `AutocompleteType` enum and its associated traits. It is used
//! to restrict results to certain place types.

// -----------------------------------------------------------------------------

use crate::error::Error as GoogleMapsError;
use crate::places::Error as PlaceAutocompleteError;
use phf::phf_map;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// -----------------------------------------------------------------------------

/// You may restrict results from a Place Autocomplete request to be of a
/// certain type by passing a [types
/// parameter](https://developers.google.com/maps/documentation/places/web-service/autocomplete#types).
/// The parameter specifies a type or a type collection, as listed in the
/// supported types below. If nothing is specified, all types are returned. In
/// general only a single type is allowed. The exception is that you can safely
/// mix the geocode and establishment types, but note that this will have the
/// same effect as specifying no types.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum AutocompleteType {
    /// Instructs the Place Autocomplete service to return only geocoding
    /// results, rather than business results. Generally, you use this request
    /// to disambiguate results where the location specified may be
    /// indeterminate.
    Geocode = 0,
    /// Instructs the Place Autocomplete service to return only geocoding
    /// results with a precise address. Generally, you use this request when you
    /// know the user will be looking for a fully specified address.
    /// indeterminate.
    #[default]
    Address = 1,
    /// Instructs the Place Autocomplete service to return only business
    /// results.
    Establishment = 2,
    /// Type collection instructs the Places service to return any result
    /// matching the following types:
    /// * `locality`
    /// * `sublocality`
    /// * `postal_code`
    /// * `country`
    /// * `administrative_area_level_1`
    /// * `administrative_area_level_2`
    Regions = 3,
    /// Type collection instructs the Places service to return results that
    /// match `locality` or `administrative_area_level_3`.
    Cities = 4,
} // enum

// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for AutocompleteType {
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

impl Serialize for AutocompleteType {
    /// Manual implementation of `Serialize` for `serde`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(std::convert::Into::<&str>::into(self))
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&Self> for AutocompleteType {
    /// Converts a borrowed `&AutocompleteType` enum into an owned
    /// `AutocompleteType` enum by cloning it.
    fn from(autocomplete_type: &Self) -> Self {
        autocomplete_type.clone()
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&AutocompleteType> for &str {
    /// Converts a `AutocompleteType` enum to a `String` that contains a
    /// [autocomplete
    /// type](https://developers.google.com/maps/documentation/places/web-service/autocomplete#types)
    /// code.
    fn from(autocomplete_type: &AutocompleteType) -> Self {
        match autocomplete_type {
            AutocompleteType::Geocode => "geocode",
            AutocompleteType::Address => "address",
            AutocompleteType::Establishment => "establishment",
            AutocompleteType::Regions => "(regions)",
            AutocompleteType::Cities => "(cities)",
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::fmt::Display for AutocompleteType {
    /// Converts a `AutocompleteType` enum to a `String` that contains a
    /// [autocomplete
    /// type](https://developers.google.com/maps/documentation/places/web-service/autocomplete#types)
    /// code.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::convert::Into::<&str>::into(self))
    } // fmt
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<&AutocompleteType> for String {
    /// Converts a `AutocompleteType` enum to a `String` that contains a
    /// [autocomplete
    /// type](https://developers.google.com/maps/documentation/places/web-service/autocomplete#types)
    /// code.
    fn from(autocomplete_type: &AutocompleteType) -> Self {
        std::convert::Into::<&str>::into(autocomplete_type).to_string()
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

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<&str> for AutocompleteType {
    // Error definitions are contained in the
    // `google_maps\src\places\place_autocomplete\error.rs` module.
    type Error = GoogleMapsError;
    /// Gets a `AutocompleteType` enum from a `String` that contains a valid
    /// [autocomplete
    /// type](https://developers.google.com/maps/documentation/places/web-service/autocomplete#types)
    /// code.
    fn try_from(autocomplete_code: &str) -> Result<Self, Self::Error> {
        Ok(AUTOCOMPLETE_TYPES_BY_CODE
            .get(autocomplete_code)
            .cloned()
            .ok_or_else(|| {
                PlaceAutocompleteError::InvalidAutocompleteType(autocomplete_code.to_string())
            })?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for AutocompleteType {
    // Error definitions are contained in the
    // `google_maps\src\places\place_autocomplete\error.rs` module.
    type Err = GoogleMapsError;
    /// Gets a `AutocompleteType` enum from a `String` that contains a valid
    /// [autocomplete
    /// type](https://developers.google.com/maps/documentation/places/web-service/autocomplete#types)
    /// code.
    fn from_str(autocomplete_code: &str) -> Result<Self, Self::Err> {
        Ok(AUTOCOMPLETE_TYPES_BY_CODE
            .get(autocomplete_code)
            .cloned()
            .ok_or_else(|| {
                PlaceAutocompleteError::InvalidAutocompleteType(autocomplete_code.to_string())
            })?)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl AutocompleteType {
    /// Formats a `AutocompleteType` enum into a string that is presentable to
    /// the end user.
    #[must_use]
    pub const fn display(&self) -> &str {
        match self {
            Self::Geocode => "Geocode",
            Self::Address => "Address",
            Self::Establishment => "Establishment",
            Self::Regions => "Regions",
            Self::Cities => "Cities",
        } // match
    } // fn
} // impl
