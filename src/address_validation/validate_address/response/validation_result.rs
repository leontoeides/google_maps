use crate::address_validation::{
    Address,
    AddressMetadata,
    Geocode,
    UspsData,
    Verdict,
};
use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// The response to an address validation request.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, CopyGetters, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    /// Overall verdict flags.
    ///
    /// The verdict property summarizes the results of the address validation
    /// and should be the first property to evaluate when building address
    /// checking logic. The property can return a variety of fields, depending
    /// on the quality of the output address.
    ///
    /// Use the `verdict` property to get a baseline summary of the quality of
    /// the address.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub verdict: Verdict,

    /// Information about the address itself as opposed to the geocode.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub address: Address,

    /// Information about the location and place that the address geocoded to.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub geocode: Geocode,

    /// Other information relevant to deliverability. `metadata` is not
    /// guaranteed to be fully populated for every address sent to the Address
    /// Validation API.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub metadata: Option<AddressMetadata>,

    /// Extra deliverability flags provided by USPS. Only provided in region
    /// `US` and `PR`.
    ///
    /// This property provides useful information for United States postal
    /// addresses. However, it's not guaranteed to be fully populated for every
    /// address validated by the service. For that reason, you shouldn't rely on
    /// this property as the sole means to validate addresses, but instead check
    /// the verdict and address as well.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub usps_data: Option<UspsData>,

    /// Preview: This feature is in Preview (pre-GA). Pre-GA products and
    /// features might have limited support, and changes to pre-GA products and
    /// features might not be compatible with other pre-GA versions. Pre-GA
    /// Offerings are covered by the [Google Maps Platform Service Specific
    /// Terms](https://cloud.google.com/maps-platform/terms/maps-service-terms).
    /// For more information, see the [launch stage
    /// descriptions](https://developers.google.com/maps/launch-stages).
    ///
    /// The address translated to English.
    ///
    /// Translated addresses are not reusable as API input. The service provides
    /// them so that the user can use their native language to confirm or deny
    /// the validation of the originally-provided address.
    ///
    /// If part of the address doesn't have an English translation, the service
    /// returns that part in an alternate language that uses a Latin script. See
    /// [here](https://developers.google.com/maps/documentation/address-validation/convert-addresses-english)
    /// for an explanation of how the alternate language is selected. If part of
    /// the address doesn't have any translations or transliterations in a
    /// language that uses a Latin script, the service returns that part in the
    /// local language associated with the address.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub english_latin_address: Option<Address>,
} // struct ValidationResult