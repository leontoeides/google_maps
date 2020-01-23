use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Polyline {

    pub points: String,

} // struct