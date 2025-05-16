use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// A wrapper for the name of the component.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, CopyGetters, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct ComponentName {
    /// The name text. For example, "5th Avenue" for a street name or "1253" for
    /// a street number.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub text: String,

    /// The BCP-47 language code. This will not be present if the component name
    /// is not associated with a language, such as a street number.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub language_code: Option<String>,
} // struct ComponentName