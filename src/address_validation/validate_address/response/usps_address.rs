use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// USPS representation of a US address.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, CopyGetters, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct UspsAddress {
    /// First address line.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub first_address_line: String,

    /// Firm name.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub firm: Option<String>,

    /// Second address line.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub second_address_line: Option<String>,

    /// Puerto Rican urbanization name.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub urbanization: Option<String>,

    /// City + state + postal code.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub city_state_zip_address_line: String,

    /// City name.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub city: String,

    /// 2 letter state code.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub state: String,

    /// Postal code e.g. 10009.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub zip_code: Option<String>,

    /// 4-digit postal code extension e.g. 5023.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub zip_code_extension: Option<String>,
} // struct UspsAddress