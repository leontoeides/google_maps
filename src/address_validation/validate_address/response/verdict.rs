use crate::address_validation::Granularity;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// High level overview of the address validation result and geocode.
///
/// The `verdict` property summarizes the results of the address validation and
/// should be the first property to evaluate when building address checking
/// logic. The property can return a variety of fields, depending on the quality
/// of the output address.
///
/// [Completeness of the address](https://developers.google.com/maps/documentation/address-validation/understand-response#completeness_of_the_address)
///
/// The verdict returns the `addressComplete` property as a signal for a
/// high-quality address, which means specifically that it has no missing,
/// unresolved, or unexpected components:
///
/// ```json
/// "verdict": {
///     "inputGranularity": "PREMISE",
///     "validationGranularity": "PREMISE",
///     "geocodeGranularity": "PREMISE",
///     "addressComplete": true
/// }
/// ```
///
/// When the address has missing, unresolved, or unexpected components, the
/// field is set to `false`.
///
/// * Note: When the `addressComplete` property does not appear in the verdict
///   upon manual inspection, its value is false. Depending on your programming
///   language, you would either query the verdict for the presence of the
///   `addressComplete` property or query the property directly. See [Address
///   quality](https://developers.google.com/maps/documentation/address-validation/understand-response#address_quality)
///   below for an example.
///
/// [Address quality](https://developers.google.com/maps/documentation/address-validation/understand-response#address_quality)
///
/// A number of possible fields indicate either problems with address
/// components, or adjustments to them, such as inferred or missing address
/// components. For example, the following `verdict` property indicates an address
/// with unconfirmed components and a missing `addressComplete` field:
///
/// ```json
/// "verdict": {
///     "inputGranularity": "PREMISE",
///     "validationGranularity": "OTHER",
///     "geocodeGranularity": "OTHER",
///     "hasUnconfirmedComponents": true,
///     "hasInferredComponents": true
/// }
/// ```
///
/// ## Key Point
///
/// * Use the `verdict` property to get a baseline summary of the quality of the
///   address.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, CopyGetters, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct Verdict {
    /// The granularity of the `input` address. This is the result of parsing
    /// the input address and does not give any validation signals.
    ///
    /// For example, if the input address includes a specific apartment number,
    /// then the `inputGranularity` here will be `SUB_PREMISE`. If we cannot
    /// match the apartment number in the databases or the apartment number is
    /// invalid, the `validationGranularity` will likely be `PREMISE` or below.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub input_granularity: Granularity,

    /// The granularity level that the API can fully `validate` the address to.
    /// For example, an `validationGranularity` of `PREMISE` indicates all
    /// address components at the level of `PREMISE` or more coarse can be
    /// validated.
    ///
    /// Per address component validation result can be found in
    /// [google.maps.addressvalidation.v1.Address.address_components](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#Address.FIELDS.address_components).
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub validation_granularity: Granularity,

    /// Information about the granularity of the `geocode`. This can be
    /// understood as the semantic meaning of how coarse or fine the geocoded
    /// location is.
    ///
    /// This can differ from the `validationGranularity` above occasionally. For
    /// example, our database might record the existence of an apartment number
    /// but do not have a precise location for the apartment within a big
    /// apartment complex. In that case, the `validationGranularity` will be
    /// `SUB_PREMISE` but the `geocodeGranularity` will be `PREMISE`.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub geocode_granularity: Granularity,

    /// The address is considered complete if there are no unresolved tokens, no
    /// unexpected or missing address components. If unset, indicates that the
    /// value is `false`. See
    /// [missingComponentTypes](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#Address.FIELDS.missing_component_types),
    /// [unresolvedTokens](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#Address.FIELDS.unresolved_tokens)
    /// or [unexpected](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#AddressComponent.FIELDS.unexpected)
    /// fields for more details.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub address_complete: bool,

    /// At least one address component cannot be categorized or validated, see
    /// [google.maps.addressvalidation.v1.Address.address_components](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#Address.FIELDS.address_components)
    /// for details.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub has_unconfirmed_components: bool,

    /// At least one address component was inferred (added) that wasn't in the
    /// input, see
    /// [google.maps.addressvalidation.v1.Address.address_components](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#Address.FIELDS.address_components)
    /// or details.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub has_inferred_components: bool,

    /// At least one address component was replaced, see
    /// [google.maps.addressvalidation.v1.Address.address_components](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#Address.FIELDS.address_components)
    /// for details.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub has_replaced_components: bool,
} // struct Verdict