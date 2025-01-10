use bon::Builder;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Preview: This feature is in Preview (pre-GA). Pre-GA products and features
/// might have limited support, and changes to pre-GA products and features
/// might not be compatible with other pre-GA versions. Pre-GA Offerings are
/// covered by the [Google Maps Platform Service Specific
/// Terms](https://cloud.google.com/maps-platform/terms/maps-service-terms).
/// For more information, see the launch stage descriptions.
///
/// Enables the Address Validation API to include additional information in the
/// response.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, Builder, CopyGetters, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct LanguageOptions {
    /// Preview: Return a
    /// [google.maps.addressvalidation.v1.Address](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#Address)
    /// in English. See
    /// [google.maps.addressvalidation.v1.ValidationResult.english_latin_address](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#ValidationResult.FIELDS.english_latin_address) for details.
    #[builder(into)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub return_english_latin_address: bool,
} // struct LanguageOptions