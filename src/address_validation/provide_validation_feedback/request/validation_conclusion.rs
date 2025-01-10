use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, EnumIter, AsRefStr, Display};

// -----------------------------------------------------------------------------
//
/// The possible final outcomes of the sequence of address validation requests
/// needed to validate an address.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, EnumString, EnumIter, AsRefStr, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum ValidationConclusion {
    /// This value is unused. If the
    /// `ProvideValidationFeedbackRequest.conclusion` field is set to
    /// `VALIDATION_CONCLUSION_UNSPECIFIED`, an `INVALID_ARGUMENT` error will be
    /// returned.
    ValidationConclusionUnspecified = 0,
    /// The version of the address returned by the Address Validation API was
    /// used for the transaction.
    ValidatedVersionUsed = 1,
    /// The version of the address provided by the user was used for the
    /// transaction
    UserVersionUsed = 2,
    /// A version of the address that was entered after the last validation
    /// attempt but that was not re-validated was used for the transaction.
    UnvalidatedVersionUsed = 3,
    /// The transaction was abandoned and the address was not used.
    #[default] Unused = 4,
} // enum ValidationConclusion