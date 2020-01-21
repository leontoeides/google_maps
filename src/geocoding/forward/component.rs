use crate::geocoding::forward::country::Country;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Component {

    // Filter results:
    // -----------------------

    /// Country matches a country name or a two letter [ISO
    /// 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1) country code. The API
    /// follows the ISO standard for defining countries, and the filtering works
    /// best when using the corresponding ISO code of the country.
    Country(Country),

    /// Matches `postal_code` and `postal_code_prefix`.
    PostalCode(String),

    // Bias results:
    // --------------------------

    /// Matches all the `administrative_area` levels.
    AdministrativeArea(String),

    /// Matches against `locality` and `sublocality` types.
    Locality(String),

    /// Matches the long or short name of a route.
    Route(String),

} // enum

impl From<&Component> for String {
    /// Converts a `Component` struct to a `String` that contains a
    /// component:value pair for filtering results.
    fn from(component: &Component) -> String {
        match component {
            Component::AdministrativeArea(administrative_area) => format!("administrative_area:{}", administrative_area),
            Component::Country(country) => format!("country:{}", String::from(country)),
            Component::PostalCode(postal_code) => format!("postal_code:{}", postal_code),
            Component::Locality(locality) => format!("locality:{}", locality),
            Component::Route(route) => format!("route:{}", route),
        } // match
    } // fn
} // impl