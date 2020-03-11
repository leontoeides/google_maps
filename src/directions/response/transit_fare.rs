use crate::directions::response::transit_currency::TransitCurrency;
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

/// If present, contains the total fare (that is, the total ticket costs) on
/// this route. This property is only returned for transit requests and only for
/// routes where fare information is available for all transit legs.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TransitFare {

    /// An [ISO 4217 currency code](https://en.wikipedia.org/wiki/ISO_4217)
    /// indicating the currency that the amount is expressed in.
    pub currency: TransitCurrency,

    /// The total fare amount, formatted in the requested language.
    pub text: String,

    /// The total fare amount, in the currency specified above.
    pub value: Decimal,

} // struct