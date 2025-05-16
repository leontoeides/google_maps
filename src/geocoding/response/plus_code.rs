use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// (See [Open Location Code](https://en.wikipedia.org/wiki/Open_Location_Code)
/// and [plus codes](https://plus.codes/)) is an encoded location reference,
/// derived from latitude and longitude coordinates, that represents an area:
/// 1/8000th of a degree by 1/8000th of a degree (about 14m x 14m at the
/// equator) or smaller. Plus codes can be used as a replacement for street
/// addresses in places where they do not exist (where buildings are not
/// numbered or streets are not named).
///
/// Typically, both the global code and compound code are returned. However, if
/// the result is in a remote location (for example, an ocean or desert) only
/// the global code may be returned.
///
/// # Notes
///
/// * This differs from `google_maps::address_validation::PlusCode` in that it
///   uses `camelCase` for serialization & deserialization.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PlusCode {
    /// A 4 character area code and 6 character or longer local code
    /// (`849VCWC8+R9`).
    pub global_code: String,

    /// A 6 character or longer local code with an explicit location (`CWC8+R9,
    /// Mountain View, CA, USA`).
    #[serde(default)]
    pub compound_code: Option<String>,
} // struct

// -----------------------------------------------------------------------------

#[cfg(feature = "address_validation")]
impl std::convert::From<crate::address_validation::PlusCode> for PlusCode {
    fn from(plus_code: crate::address_validation::PlusCode) -> Self {
        Self {
            global_code: plus_code.global_code,
            compound_code: plus_code.compound_code,
        } // PlusCode
    } // fn
} // impl