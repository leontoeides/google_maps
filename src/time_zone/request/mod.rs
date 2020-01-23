//! Resources (enums, structs, methods) for the client to build a Time Zone API
//! request for the Google Cloud server.

mod build;
mod execute;
mod get;
mod new;
mod with_language;

use crate::{
    language::Language,
    latlng::LatLng,
}; // use
use time::PrimitiveDateTime;

/// Use this structure's methods to build your Time Zone API request.
#[derive(Clone, Debug)]
pub struct Request {

    // Required parameters:
    // --------------------

    /// Your application's API key. This key identifies your application for
    /// purposes of quota management. Learn how to
    /// [get a key](https://developers.google.com/maps/documentation/timezone/get-api-key).
    key: String,

    /// The location to look up.
    location: LatLng,

    /// Specifies the desired time. The Time Zone API uses the `time` to
    /// determine whether or not Daylight Savings should be applied, based on
    /// the time zone of the `location`. Note that the API does not take
    /// historical time zones into account. That is, if you specify a past
    /// `time`, the API does not take into account the possibility that the
    /// `location` was previously in a different time zone.
    time: PrimitiveDateTime,

    // Optional parameters:
    // --------------------

    /// The language in which to return results. See the
    /// [list of supported domain languages](https://developers.google.com/maps/faq#languagesupport).
    /// Note that we often update supported languages so this list may not be
    /// exhaustive. Defaults to `Language::English`.
    language: Option<Language>,

    // Internal use only:
    // ------------------

    /// Query string that is to be submitted to the Google Cloud Maps Platform.
    query: Option<String>,

    /// Has the request been validated?
    validated: bool,

} // struct