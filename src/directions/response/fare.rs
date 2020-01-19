use serde::{Serialize, Deserialize};

/// If present, contains the total fare (that is, the total ticket costs) on
/// this route. This property is only returned for transit requests and only for
/// routes where fare information is available for all transit legs.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fare {

    /// An [ISO 4217 currency code](https://en.wikipedia.org/wiki/ISO_4217)
    /// indicating the currency that the amount is expressed in.
    pub currency: String,

    /// The total fare amount, formatted in the requested language.
    pub text: String,

    /// The total fare amount, in the currency specified above.
    pub value: f32,

} // struct