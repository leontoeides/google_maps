//! **Look in this module for documentation on building your _Elevation API_
//! query**. In particular, look at the _Request_ struct for examples of the
//! builder pattern. This module contains the tools (enums, structs, methods)
//! for building your Google Maps Platform request.

mod build;
mod end_point;
mod for_positional_request;
mod for_sampled_path_request;
pub mod locations;
mod new;
mod query_string;
mod validatable;

#[cfg(feature = "reqwest")]
mod execute;

#[cfg(feature = "reqwest")]
mod get;

// -----------------------------------------------------------------------------

use crate::elevation::Locations;

// -----------------------------------------------------------------------------
//
/// **Look at this `Request` struct for documentation on how to build your
/// _Elevation API_ query**. The methods implemented for this struct are what's
/// used to build your request.
#[derive(Debug)]
pub struct Request<'a> {
    // Required parameters:
    // --------------------
    /// This structure contains the application's API key and other
    /// user-definable settings such as "maximum retries."
    client: &'a crate::client::Client,

    // Positional Requests:
    // --------------------
    /// Defines the location(s) on the earth from which to return elevation
    /// data. This parameter takes either a single location as a
    /// latitude/longitude pair, multiple latitude/longitude pairs, or an
    /// encoded polyline.
    locations: Option<Locations>,

    // Sampled Path Requests:
    // ----------------------
    /// Defines a path on the earth for which to return elevation data. This
    /// parameter defines a set of two or more ordered latitude/longitude
    /// pairs defining a path along the surface of the earth. This parameter
    /// must be used in conjunction with the `samples` parameter described
    /// below.
    path: Option<Locations>,

    /// Specifies the number of sample points along a path for which to return
    /// elevation data. The samples parameter divides the given path into an
    /// ordered set of equidistant points along the path.
    samples: Option<u8>,
} // struct
