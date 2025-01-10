use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, EnumIter, AsRefStr, Display};

// -----------------------------------------------------------------------------
//
/// The different possible values for confirmation levels.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, EnumString, EnumIter, AsRefStr, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum ConfirmationLevel {
    /// Default value. This value is unused.
    #[default] ConfirmationLevelUnspecified = 0,

    /// We were able to verify that this component exists and makes sense in the
    /// context of the rest of the address.
    Confirmed = 1,

    /// This component could not be confirmed, but it is plausible that it
    /// exists. For example, a street number within a known valid range of
    /// numbers on a street where specific house numbers are not known.
    UnconfirmedButPlausible = 2,

    /// This component was not confirmed and is likely to be wrong. For example,
    /// a neighborhood that does not fit the rest of the address.
    UnconfirmedAndSuspicious = 3,
} // enum ConfirmationLevel