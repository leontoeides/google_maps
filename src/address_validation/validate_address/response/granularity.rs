use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, EnumIter, AsRefStr, Display};

// -----------------------------------------------------------------------------
//
/// The various granularities that an address or a geocode can have. When used
/// to indicate granularity for an address, these values indicate with how fine
/// a granularity the address identifies a mailing destination. For example, an
/// address such as "123 Main Street, Redwood City, CA, 94061" identifies a
/// `PREMISE` while something like "Redwood City, CA, 94061" identifies a
/// `LOCALITY`. However, if we are unable to find a geocode for "123 Main
/// Street" in Redwood City, the geocode returned might be of `LOCALITY`
/// granularity even though the address is more granular.
///
/// Address granularity refers to the level of detail used in determining the
/// specificity of an address or geocode. Address specificity in the
/// `validationGranularity` response is a key signal for whether or not an
/// address is deliverable.
///
/// The verdict property returns these granularity signals:
///
/// * `inputGranularity` — Describes the level of detail captured from the
///   address sent to the Address Validation API. The level of address detail in
///   the request influences the level of address detail in the validation
///   response. For example, an address with an inputGranularity below `PREMISE`
///   below level does not typically result in a `validationGranularity` to a
///   `PREMISE` level.
///
/// * `validationGranularity` — The granularity level that the Address
///   Validation API can fully validate the address to. In most cases, a
///   granularity level of `PREMISE` or `SUB_PREMISE` indicates a quality
///   address that is likely deliverable.
///
/// * `geocodeGranularity` — Describes the level of detail of the geocode
///   associated with the address. For example, Google records might indicate
///   the existence of an apartment number, but not a precise location for that
///   particular apartment within a large apartment complex. In that case, the
///   `validationGranularity` is `SUB_PREMISE` but the `geocodeGranularity` is
//    `PREMISE`.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, EnumString, EnumIter, AsRefStr, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum Granularity {
    /// Default value. This value is unused.
    #[default] GranularityUnspecified = 0,
    /// Below-building level result, such as an apartment.
    SubPremise = 1,
    /// Building-level result.
    Premise = 2,
    /// A geocode that approximates the building-level location of the address.
    PremiseProximity = 3,
    /// The address or geocode indicates a block. Only used in regions which
    /// have block-level addressing, such as Japan.
    Block = 4,
    /// The geocode or address is granular to route, such as a street, road, or
    /// highway.
    Route = 5,
    /// All other granularities, which are bucketed together since they are not
    /// deliverable.
    Other = 6,
} // enum Granularity