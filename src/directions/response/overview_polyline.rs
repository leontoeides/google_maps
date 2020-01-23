use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverviewPolyline {

    pub points: String,

} // struct