use crate::address_validation::{ComponentName, ConfirmationLevel};
use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Represents an address component, such as a street, city, or state.
///
/// The `addressComponent` property is a subcomponent of address which provides
/// a detailed listing of the elements—or components—of the address that the
/// Address Validation API has processed. The API identifies each component
/// field provides by its name, type, and confirmation level.
///
/// ## Key Point
///
/// * Use the `address` property to obtain address formatting. Use its
///   subcomponent `addressComponent` for investigating errors and providing
///   information about those errors to your customers. However, don't build
///   your integration to rely on the specific confirmation level for address
///   components. See [Build your
///   validation](https://developers.google.com/maps/documentation/address-validation/build-validation-logic)
///   logic for integration guidance.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, CopyGetters, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct AddressComponent {
    /// The name for this component.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub component_name: ComponentName,

    /// The type of the address component. See [Table 2: Additional types
    /// returned by the Places service](https://developers.google.com/places/web-service/supported_types#table2)
    /// for a list of possible types.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub component_type: String,

    /// Indicates the level of certainty that we have that the component is
    /// correct.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub confirmation_level: ConfirmationLevel,

    /// Indicates that the component was not part of the input, but we inferred
    /// it for the address location and believe it should be provided for a
    /// complete address.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub inferred: bool,

    /// Indicates a correction to a misspelling in the component name. The API
    /// does not always flag changes from one spelling variant to another, such
    /// as when changing "centre" to "center". It also does not always flag
    /// common misspellings, such as when changing "Amphitheater Pkwy" to
    /// "Amphitheatre Pkwy".
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub spell_corrected: bool,

    /// Indicates the name of the component was replaced with a completely
    /// different one, for example a wrong postal code being replaced with one
    /// that is correct for the address. This is not a cosmetic change, the
    /// input component has been changed to a different one.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub replaced: bool,

    /// Indicates an address component that is not expected to be present in a
    /// postal address for the given region. We have retained it only because it
    /// was part of the input.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub unexpected: bool,
} // struct AddressComponent