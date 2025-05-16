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

use crate::places::Error;
use crate::places::place_autocomplete::response::{prediction::Prediction, status::Status};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
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
    #[serde(default)]
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
    #[serde(default)]
    pub info_messages: Vec<String>,
} // struct

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<String> for Response {
    type Error = serde_json::Error;
    /// Convert a Google Maps API [JSON](https://en.wikipedia.org/wiki/JSON)
    /// `String` response into a `Response` struct.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        serde_json::from_slice(&s.into_bytes())
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for Response {
    type Err = serde_json::Error;
    /// Converts a Google Maps API [JSON](https://en.wikipedia.org/wiki/JSON)
    /// `&str` response into a `Response` struct.
    ///
    /// # Notes
    ///
    /// * It's recommended to use the implemented `TryFrom` trait instead.
    ///
    /// * The [serde_json](https://crates.io/crates/simd-json)'s `from_str`
    ///   function implementation is unsafe and it's `from_slice` function
    ///   requires a mutable reference. Therefore this trait clones the `&str`
    ///   into a `String` to give `from_slice` mutable access to the string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.to_string().into_bytes();
        serde_json::from_slice(&bytes)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<Response> for Result<Response, crate::places::Error> {
    /// Converts a Google Maps API `Response` into a `Result<Response, Error>`
    /// by examining the `status` field inside of the response.
    ///
    /// If the status indicates a success, then an `Ok(response)` will be
    /// returned. If the status indicates an error, then an `Err(error)` will be
    /// returned.
    fn from(response: Response) -> Self {
        match response.status {
            Status::Ok => Ok(response),
            Status::InvalidRequest => Err(Error::InvalidRequest),
            Status::OverQueryLimit => Err(Error::OverQueryLimit),
            Status::RequestDenied => Err(Error::RequestDenied),
            Status::UnknownError => Err(Error::UnknownError),
            Status::ZeroResults => Err(Error::ZeroResults),
        } // match
    } // fn
} // impl