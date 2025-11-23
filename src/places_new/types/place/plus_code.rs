// -------------------------------------------------------------------------------------------------
//
/// Plus code ([http://plus.codes](http://plus.codes)) is a location reference with two formats: 
/// global code defining a 14mx14m (1/8000th of a degree) or smaller rectangle, and compound code,
/// replacing the prefix with a reference location.
/// 
/// Plus codes provide a simple way to encode locations that can be shared and decoded without
/// requiring traditional street addresses. They are particularly useful in areas where traditional
/// addressing systems are not well established.
#[derive(
    //std
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
    // getset
    getset::Getters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[serde(rename_all = "camelCase")]
pub struct PlusCode {
    /// Place's global (full) code, such as "9FWM33GV+HQ", representing an 1/8000 by 1/8000 degree
    /// area (~14 by 14 meters).
    /// 
    /// The global code is a complete plus code that can be used anywhere in the world
    /// without additional context. It provides precise location identification to within
    /// approximately 14 square meters.
    #[serde(default)]
    #[getset(get = "pub")]
    pub global_code: Option<String>,

    /// Place's compound code, such as "33GV+HQ, Ramberg, Norway", containing the suffix of the
    /// global code and replacing the prefix with a formatted name of a reference entity.
    /// 
    /// The compound code is a shortened version that replaces the area prefix with a human-readable
    /// location name, making it easier to communicate while maintaining the precision of the full
    /// plus code.
    #[serde(default)]
    #[getset(get = "pub")]
    pub compound_code: Option<String>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl PlusCode {
    /// Returns whether this `PlusCode` has a global code.
    #[must_use]
    pub const fn has_global_code(&self) -> bool {
        self.global_code.is_some()
    }

    /// Returns whether this `PlusCode` has a compound code.
    #[must_use]
    pub const fn has_compound_code(&self) -> bool {
        self.compound_code.is_some()
    }

    /// Returns whether this `PlusCode` has any code information.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.global_code.is_none() && self.compound_code.is_none()
    }
}