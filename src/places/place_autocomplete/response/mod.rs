//! Resources (enums, structs) for processing the _Places API_ _Place
//! Autocomplete_ response from the Google Maps Platform. Look in here for more
//! information about the data returned from Google's server and how to parse it
//! with your program.

pub mod matched_substring;
pub mod prediction;
pub mod status;
pub mod structured_format;
pub mod term;

// -----------------------------------------------------------------------------

use crate::places::place_autocomplete::response::{
    prediction::Prediction,
    status::Status,
}; // crate::places::place_autocomplete::response
use serde::{Deserialize, Serialize};

/// The response from the Google Maps Places API _Place Autocomplete_ request
/// will be stored in this structure.
///
/// [Place Autocomplete Response](https://developers.google.com/maps/documentation/places/web-service/autocomplete#place_autocomplete_responses)
/// ------------------------------------------------------------------------------------------------
///
/// Of particular interest within the results are the `place_id` elements, which
/// can be used to request more specific details about the place via a separate
/// query. See [Place Details](https://developers.google.com/maps/documentation/places/web-service/details#PlaceDetailsRequests)
/// requests.

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// Contains an array of predictions.
    ///
    /// See [PlaceAutocompletePrediction](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlaceAutocompletePrediction)
    /// for more information.
    #[serde(alias = "predictions")]
    pub predictions: Vec<Prediction>,

    /// Contains the status of the request, and may contain debugging
    /// information to help you track down why the request failed.
    ///
    /// See [PlacesAutocompleteStatus](https://developers.google.com/maps/documentation/places/web-service/autocomplete#PlacesAutocompleteStatus)
    /// for more information.
    #[serde(alias = "status")]
    pub status: Status,

    /// When the service returns a status code other than `OK<`, there may be an
    /// additional `error_message` field within the response object. This field
    /// contains more detailed information about thereasons behind the given
    /// status code. This field is not always returned, and its content is
    /// subject to change.
    #[serde(alias = "error_message")]
    pub error_message: Option<String>,

    /// When the service returns additional information about the request
    /// specification, there may be an additional `info_messages` field within
    /// the response object. This field is only returned for successful
    /// requests. It may not always be returned, and its content is subject to
    /// change.
    #[serde(alias = "info_messages")]
    pub info_messages: Option<Vec<String>>,
} // struct

impl std::str::FromStr for Response {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Places API _Place Autocomplete_ JSON `String`
    /// response into a usable `Response` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    } // fn from_str
} // impl FromStr