//! Resources (enums, structs) for processing the _Places API_ _Text
//! Search_ response from the Google Maps Platform. Look in here for more
//! information about the data returned from Google's server and how to parse it
//! with your program.

// -----------------------------------------------------------------------------

use crate::places::status::Status;
use crate::places::Place;
use serde::{Deserialize, Serialize};

/// The response from the Google Maps Places API _Nearby Search_ request will be
/// stored in this structure.
///
/// See [Nearby Search responses](https://developers.google.com/maps/documentation/places/web-service/search-text#text-search-responses)
/// for more information.

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// May contain a set of attributions about this listing which must be
    /// displayed to the user (some listings may not have attribution).
    #[serde(default)]
    pub html_attributions: Vec<String>,

    /// Contains an array of places.
    ///
    /// * Place Search requests return a subset of the fields that are returned
    ///   by Place Details requests. If the field you want is not returned by
    ///   Place Search, you can use Place Search to get a `place_id`, then use
    ///   that Place ID to make a Place Details request.
    ///
    /// See [Place](https://developers.google.com/maps/documentation/places/web-service/search-text#Place)
    /// for more information.
    #[serde(default)]
    pub results: Vec<Place>,

    /// Contains the status of the request, and may contain debugging
    /// information to help you track down why the request failed.
    ///
    /// See [PlacesSearchStatus](https://developers.google.com/maps/documentation/places/web-service/search-text#PlacesSearchStatus)
    /// for more information.
    pub status: Status,

    /// When the service returns a status code other than `OK<`, there may be an
    /// additional `error_message` field within the response object. This field
    /// contains more detailed information about thereasons behind the given
    /// status code. This field is not always returned, and its content is
    /// subject to change.
    pub error_message: Option<String>,

    /// When the service returns additional information about the request
    /// specification, there may be an additional `info_messages` field within
    /// the response object. This field is only returned for successful
    /// requests. It may not always be returned, and its content is subject to
    /// change.
    #[serde(default)]
    pub info_messages: Vec<String>,

    /// Contains a token that can be used to return up to 20 additional results.
    /// A `next_page_token` will not be returned if there are no additional
    /// results to display. The maximum number of results that can be returned
    /// is 60. There is a short delay between when a `next_page_token` is issued,
    /// and when it will become valid.
    pub next_page_token: Option<String>,
} // struct

// -----------------------------------------------------------------------------

impl std::str::FromStr for Response {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Places API _Nearby Search_ JSON response into a usable
    /// `Response` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    } // fn from_str
} // impl FromStr
