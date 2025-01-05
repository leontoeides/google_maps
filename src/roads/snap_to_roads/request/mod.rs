//! **Look in this module for documentation on building your _Snap To Roads_
//! query**. In particular, look at the _Request_ struct for examples of the
//! builder pattern. This module contains the tools (enums, structs, methods)
//! for building your Google Maps Platform request.

// -----------------------------------------------------------------------------

mod build;
mod end_point;
mod new;
mod query_string;
mod validatable;
mod with_interpolation;

#[cfg(feature = "reqwest")]
mod execute;

#[cfg(feature = "reqwest")]
mod get;

// -----------------------------------------------------------------------------
//
/// **Look at this `Request` struct for documentation on how to build your _Snap
/// To Roads_ query**. The methods implemented for this struct are what's used
/// to build your request.
#[derive(Debug)]
pub struct Request<'r> {
    // Required parameters:
    // --------------------
    /// This structure contains the application's API key and other
    /// user-definable settings such as "maximum retries."
    client: &'r crate::client::Client,

    /// The path to be snapped. The path parameter accepts a list of
    /// latitude/longitude pairs. Latitude and longitude values should be
    /// separated by commas. Coordinates should be separated by the pipe
    /// character: "|". For example:
    /// `path=60.170880,24.942795|60.170879,24.942796|60.170877,24.942796`.
    path: Vec<crate::types::LatLng>,

    // Optional parameters:
    // --------------------
    /// Whether to interpolate a path to include all points forming the full
    /// road-geometry. When `true`, additional interpolated points will also be
    /// returned, resulting in a path that smoothly follows the geometry of the
    /// road, even around corners and through tunnels. Interpolated paths will
    /// most likely contain more points than the original path. Defaults to
    /// `false`.
    interpolate: Option<bool>,
} // struct
