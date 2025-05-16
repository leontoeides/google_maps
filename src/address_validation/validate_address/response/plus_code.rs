use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Plus code (<http://plus.codes>) is a location reference with two formats:
/// global code defining a 14mx14m (1/8000th of a degree) or smaller rectangle,
/// and compound code, replacing the prefix with a reference location.
///
/// # Notes
///
/// * This differs from `google_maps::address_validation::PlusCode` in that this
///   uses `camelCase` for serialization & deserialization.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, CopyGetters, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct PlusCode {
    /// Place's global (full) code, such as "9FWM33GV+HQ", representing an
    /// 1/8000 by 1/8000 degree area (~14 by 14 meters).
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub global_code: String,

    /// Place's compound code, such as "33GV+HQ, Ramberg, Norway", containing
    /// the suffix of the global code and replacing the prefix with a formatted
    /// name of a reference entity.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub compound_code: Option<String>,
} // struct

// -----------------------------------------------------------------------------

#[cfg(feature = "geocoding")]
impl std::convert::From<crate::geocoding::PlusCode> for PlusCode {
    fn from(plus_code: crate::geocoding::PlusCode) -> Self {
        Self {
            global_code: plus_code.global_code,
            compound_code: plus_code.compound_code
        } // PlusCode
    } // fn
} // impl