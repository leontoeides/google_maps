//! **Look in this module for documentation on building your _Time Zone API_
//! query**. In particular, look at the _Request_ struct for examples of the
//! builder pattern. This module contains the tools (enums, structs, methods)
//! for building your Google Maps Platform request.

mod build;
#[cfg(feature = "enable-reqwest")]
mod execute;
#[cfg(feature = "enable-reqwest")]
mod get;
mod new;
mod query_url;
mod with_language;

use chrono::{DateTime, Utc};
use crate::{client::GoogleMapsClient, language::Language, latlng::LatLng};

// -----------------------------------------------------------------------------
//
/// **Look at this `Request` struct for documentation on how to build your _Time
/// Zone API_ query**. The methods implemented for this struct are what's used
/// to build your request.

#[derive(Debug)]
pub struct Request<'a> {

    // Required parameters:
    // --------------------

    /// This structure contains the application's API key and other
    /// user-definable settings such as "maximum retries."
    client: &'a GoogleMapsClient,

    /// The location to look up.
    location: LatLng,

    /// Specifies the desired time. The Time Zone API uses the `time` to
    /// determine whether or not Daylight Savings should be applied, based on
    /// the time zone of the `location`. Note that the API does not take
    /// historical time zones into account. That is, if you specify a past
    /// `time`, the API does not take into account the possibility that the
    /// `location` was previously in a different time zone.
    timestamp: DateTime<Utc>,

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

} // struct