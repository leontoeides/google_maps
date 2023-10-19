//! Resources (enums, structs) for processing the _Places API_ _Place Details_
//! response from the Google Maps Platform. Look in here for more information
//! about the data returned from Google's server and how to parse it with your
//! program.

// -----------------------------------------------------------------------------

use crate::places::status::Status;
use crate::places::Place;
use serde::{Deserialize, Serialize};

/// The response from the Google Maps Places API _Place Details_ request
/// will be stored in this structure.
///
/// See [Place Details response](https://developers.google.com/maps/documentation/places/web-service/details#PlaceDetailsResponses)
/// for more information

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// May contain a set of attributions about this listing which must be
    /// displayed to the user (some listings may not have attribution).
    pub html_attributions: Vec<String>,

    /// Contains the detailed information about the place requested.
    ///
    /// See [Place](https://developers.google.com/maps/documentation/places/web-service/details#Place)
    /// for more information.
    pub result: Option<Place>,

    /// Contains the status of the request, and may contain debugging
    /// information to help you track down why the request failed.
    ///
    /// See [PlacesDetailsStatus](https://developers.google.com/maps/documentation/places/web-service/details#PlacesDetailsStatus)
    /// for more information.
    pub status: Status,

    /// When the service returns additional information about the request
    /// specification, there may be an additional `info_messages` field within
    /// the response object. This field is only returned for successful
    /// requests. It may not always be returned, and its content is subject to
    /// change.
    pub info_messages: Option<Vec<String>>,
} // struct

// -----------------------------------------------------------------------------

impl std::str::FromStr for Response {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Places API _Place Details_ JSON response into a
    /// usable `Response` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    } // fn from_str
} // impl FromStr
