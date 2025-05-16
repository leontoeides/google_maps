use crate::address_validation::{AddressComponent, PostalAddress};
use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Details of the post-processed address. Post-processing includes correcting
/// misspelled parts of the address, replacing incorrect parts, and inferring
/// missing parts.
///
/// The `address` property provides formatting for the processed address
/// provided in the request, along with component-level summaries of the
/// address, including misspelled parts of the address, replaced incorrect
/// parts, and inferred missing parts.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, CopyGetters, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// The post-processed address, formatted as a single-line address following
    /// the address formatting rules of the region where the address is located.
    ///
    /// Note: the format of this address may not match the format of the address
    /// in the `postalAddress` field. For example, the `postalAddress` always
    /// represents the country as a 2 letter `regionCode`, such as "US" or "NZ".
    /// By contrast, this field uses a longer form of the country name, such as
    /// "USA" or "New Zealand".
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub formatted_address: String,

    /// The post-processed address represented as a postal address.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub postal_address: PostalAddress,

    /// Unordered list. The individual address components of the formatted and
    /// corrected address, along with validation information. This provides
    /// information on the validation status of the individual components.
    ///
    /// Address components are not ordered in a particular way. Do not make any
    /// assumptions on the ordering of the address components in the list.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub address_components: Vec<AddressComponent>,

    /// The types of components that were expected to be present in a correctly
    /// formatted mailing address but were not found in the input AND could not
    /// be inferred. Components of this type are not present in
    /// `formattedAddress`, `postalAddress`, or `addressComponents`. An example
    /// might be `['street_number', 'route']` for an input like "Boulder,
    /// Colorado, 80301, USA". The list of possible types can be found
    /// [here](https://developers.google.com/maps/documentation/geocoding/requests-geocoding#Types).
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub missing_component_types: Vec<String>,

    /// The types of the components that are present in the `addressComponents`
    /// but could not be confirmed to be correct. This field is provided for the
    /// sake of convenience: its contents are equivalent to iterating through
    /// the `addressComponents` to find the types of all the components where
    /// the
    /// [confirmationLevel](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#AddressComponent.FIELDS.confirmation_level)
    /// is not [CONFIRMED](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#ConfirmationLevel.ENUM_VALUES.CONFIRMED)
    /// or the inferred flag is not set to `true`. The list of possible types
    /// can be found [here](https://developers.google.com/maps/documentation/geocoding/requests-geocoding#Types).
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub unconfirmed_component_types: Vec<String>,

    /// Any tokens in the input that could not be resolved. This might be an
    /// input that was not recognized as a valid part of an address. For
    /// example, for an input such as "Parcel 0000123123 & 0000456456 Str #
    /// Guthrie Center IA 50115 US", the unresolved tokens might look like
    /// `["Parcel", "0000123123", "&", "0000456456"]`.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub unresolved_tokens: Vec<String>,
} // struct Address