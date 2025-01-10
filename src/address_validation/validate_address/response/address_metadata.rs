use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// The metadata for the address. `metadata` is not guaranteed to be fully
/// populated for every address sent to the Address Validation API.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, CopyGetters, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct AddressMetadata {
    /// Indicates that this is the address of a business. If unset, indicates
    /// that the value is unknown.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub business: bool,

    /// Indicates that the address of a PO box. If unset, indicates that the
    /// value is unknown.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub po_box: bool,

    /// Indicates that this is the address of a residence. If unset, indicates
    /// that the value is unknown.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub residential: bool,
} // struct AddressMetadata