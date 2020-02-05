//! Resources (enums, structs, methods) for the client to build a Geocoding API
//! request for the Google Cloud server. Forward geocoding converts an address
//! to a latitude/longitude.

mod build;
mod execute;
mod get;
mod new;
mod validate;
mod with_address;
mod with_bounds;
mod with_component;
mod with_language;
mod with_region;
pub mod component;
pub mod country;

use crate::{
    bounds::Bounds,
    geocoding::forward::component::Component,
    client_settings::ClientSettings,
    language::Language,
    region::Region,
}; // use

/// Use this structure's methods to build your Forward Geocoding API request.
/// Latitude/longitude lookup from address.

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ForwardRequest<'a> {

    // Required parameters:
    // --------------------

    /// This structure contains the application's API key and other
    /// user-definable settings such as "maximum retries."
    client_settings: &'a mut ClientSettings,

    // Optional parameters:
    // --------------------

    /// The street address that you want to geocode, in the format used by the
    /// national postal service of the country concerned. Additional address
    //elements such as business names and unit, suite or floor numbers should be
    /// avoided. Please refer to [the
    /// FAQ](https://developers.google.com/maps/faq#geocoder_queryformat) for
    /// additional guidance.
    address: Option<String>,

    /// The bounding box of the viewport within which to bias geocode results
    /// more prominently. This parameter will only influence, not fully
    /// restrict, results from the geocoder. (For more information see [Viewport
    /// Biasing](https://developers.google.com/maps/documentation/geocoding/intro#Viewports).)
    bounds: Option<Bounds>,

    /// A components filter with elements. The components filter is also
    /// accepted as an optional parameter if an `address` is provided. Each
    /// element in the components filter fully restricts the results from the
    /// geocoder. See more information about [component
    /// filtering](https://developers.google.com/maps/documentation/geocoding/intro#ComponentFiltering).
    components: Option<Vec<Component>>,

    /// The language in which to return results.
    language: Option<Language>,

    /// The region code, specified as a ccTLD ("top-level domain") two-character
    /// value. This parameter will only influence, not fully restrict, results
    /// from the geocoder. (For more information see [Region
    /// Biasing](https://developers.google.com/maps/documentation/geocoding/intro#RegionCodes)
    /// below.)
    region: Option<Region>,

    // Internal use only:
    // ------------------

    /// Query string that is to be submitted to the Google Cloud Maps Platform.
    query: Option<String>,

    /// Has the request been validated?
    validated: bool,

} // struct