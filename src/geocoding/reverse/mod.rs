//! **Look in this module for documentation on building your _Geocoding API_
//! reverse query**. In particular, look at the _ReverseRequest_ struct for
//! examples of the builder pattern. This module contains the tools (enums,
//! structs, methods) for building your Google Maps Platform request. Reverse
//! geocoding converts latitude & longitude coordinates to a street address.

mod build;
#[cfg(feature = "enable-reqwest")]
mod execute;
#[cfg(feature = "enable-reqwest")]
mod get;
mod new;
mod query_url;
mod with_language;
mod with_location_types;
mod with_result_types;

// -----------------------------------------------------------------------------

use crate::client::GoogleMapsClient;
use crate::types::{Language, LatLng, LocationType, PlaceType};

// -----------------------------------------------------------------------------
//
/// **Look at this `Request` struct for documentation on how to build your
/// _Geocoding API_ query**. The methods implemented for this struct are what's
/// used to build your request. Reverse geocoding looks up a street address
/// from latitude & longitude coorindates.

#[derive(Debug)]
pub struct ReverseRequest<'a> {
    // Required parameters:
    // --------------------
    /// This structure contains the application's API key and other
    /// user-definable settings such as "maximum retries."
    client: &'a GoogleMapsClient,

    /// The latitude and longitude values specifying the location for which you
    /// wish to obtain the closest, human-readable address.
    latlng: LatLng,

    // Optional parameters:
    // --------------------
    /// The language in which to return results.
    language: Option<Language>,

    /// A filter of one or more location types. If the parameter contains
    /// multiple location types, the API returns all addresses that match any of
    /// the types. A note about processing: The `location_type` parameter does
    /// not restrict the search to the specified location type(s). Rather, the
    /// `location_type` acts as a post-search filter: the API fetches all
    /// results for the specified `latlng`, then discards those results that do
    /// not match the specified location type(s).
    location_types: Option<Vec<LocationType>>,

    /// A filter of one or more address types. If the parameter contains
    /// multiple address types, the API returns all addresses that match any of
    /// the types. A note about processing: The `result_type` parameter does not
    /// restrict the search to the specified address type(s). Rather, the
    /// `result_type` acts as a post-search filter: the API fetches all results
    /// or the specified `latlng`, then discards those results that do not match
    /// the specified address type(s).
    result_types: Option<Vec<PlaceType>>,

    // Internal use only:
    // ------------------
    /// Query string that is to be submitted to the Google Cloud Maps Platform.
    query: Option<String>,
} // impl
