mod error_object;
mod error;

use crate::{
    geolocation::response::error_object::ErrorObject,
    latlng::LatLng,
}; // use
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Response {
    pub location: Option<LatLng>,
    pub accuracy: Option<f64>,
    pub error: Option<ErrorObject>,
} // struct