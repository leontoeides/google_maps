use crate::{
    bounds::Bounds,
    directions::response::{
        fare::Fare,
        leg::Leg,
        overview_polyline::OverviewPolyline,
    }, // directions::response
}; // use
use serde::{Serialize, Deserialize};

/// A single route containing a set of legs in a
/// [Response](https://developers.google.com/maps/documentation/javascript/reference/directions#DirectionsResult).
/// Note that though this object is "JSON-like," it is not strictly JSON, as it
/// directly and indirectly includes `LatLng` objects.

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Route {

    /// The bounds for this route.
    pub bounds: Bounds,

    /// Copyrights text to be displayed for this route.
    pub copyrights: String,

    /// If present, contains the total fare (that is, the total ticket costs) on
    /// this route. This property is only returned for transit requests and only
    /// for routes where fare information is available for all transit legs.
    pub fare: Option<Fare>,

    /// An array of `Legs`, each of which contains information about the steps
    /// of which it is composed. There will be one leg for each stopover
    /// waypoint or destination specified. So a route with no stopover waypoints
    /// will contain one `Leg` and a route with one stopover waypoint will
    /// contain two.
    pub legs: Vec<Leg>,

    /// An [encoded polyline representation](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)
    /// of the route. This polyline is an approximate (smoothed) path of the
    /// resulting directions.
    pub overview_polyline: OverviewPolyline,

    /// Contains a short textual description for the route, suitable for naming
    /// and disambiguating the route from alternatives.
    pub summary: String,

    /// Contains an array of warnings to be displayed when showing these
    /// directions. You must handle and display these warnings yourself.
    pub warnings: Vec<String>,

    /// If `optimizeWaypoints` was set to `true`, this field will contain the
    /// re-ordered permutation of the input waypoints. For example, if the input
    /// was:
    /// ```
    ///     Origin: Los Angeles
    ///     Waypoints: Dallas, Bangor, Phoenix
    ///     Destination: New York
    /// ```
    /// and the optimized output was ordered as follows:
    /// ```
    ///     Origin: Los Angeles
    ///     Waypoints: Phoenix, Dallas, Bangor
    ///     Destination: New York
    /// ```
    /// then this field will be an Array containing the values [2, 0, 1]. Note
    /// that the numbering of waypoints is zero-based.
    ///
    /// If any of the input waypoints has `stopover` set to `false`, this field
    /// will be empty, since route optimization is not available for such
    /// queries.
    pub waypoint_order: Vec<u8>,

} // struct