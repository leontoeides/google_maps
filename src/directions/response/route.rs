//! A single route containing a set of legs.

use crate::{
    bounds::Bounds,
    directions::response::{
        leg::Leg, overview_polyline::OverviewPolyline, transit_fare::TransitFare,
    }, // crate::directions::response
}; // use
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// A single route containing a set of legs in a
/// [Response](https://developers.google.com/maps/documentation/javascript/reference/directions#DirectionsResult).
/// Note that though this object is "JSON-like," it is not strictly JSON, as it
/// directly and indirectly includes `LatLng` objects.

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Route {
    /// The bounds for this route.
    pub bounds: Bounds,
    /// Copyrights text to be displayed for this route.
    pub copyrights: String,
    /// If present, contains the total fare (that is, the total ticket costs) on
    /// this route. This property is only returned for transit requests and only
    /// for routes where fare information is available for all transit legs.
    pub fare: Option<TransitFare>,
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

impl Route {

    /// A helper function for destructuring (or serializing) the `summary`
    /// field. If the _summary_ text is populated, this function will return the
    /// _summary_ text in the `String` format. If the _summary_ text is empty,
    /// this function will return `None`.
    /// ```rust
    /// let summary = route.get_summary();
    /// ```

    pub fn get_summary(&self) -> Option<&String> {
        match &*self.summary {
            "" => None,
            _ => Some(&self.summary),
        } // match
    } // fn

    /// A helper function for destructuring (or serializing) the optional `fare`
    /// field. If the _fare_ struct is populated, this function will return the
    /// _currency_ code in the `String` format. If the _fare_ struct is empty,
    /// this function will return `None`.
    /// ```rust
    /// let fare_currency = route.get_fare_currency();
    /// ```

    pub fn get_fare_currency(&self) -> Option<String> {
        self.fare.as_ref().map(|fare| fare.currency.to_string())
    } // fn

    /// A helper function for destructuring (or serializing) the optional `fare`
    /// field. If the _fare_ struct is populated, this function will return the
    /// _value_ `f64`. If the _fare_ struct is empty, this function will return
    /// `None`.
    /// ```rust
    /// let fare_value = route.get_fare_value();
    /// ```

    pub fn get_fare_value(&self) -> Option<Decimal> {
        self.fare.as_ref().map(|fare| fare.value)
    } // fn

    /// A helper function for destructuring (or serializing) the optional `fare`
    /// field. If the _fare_ struct is populated, this function will return the
    /// _text_ `String`. If the _fare_ struct is empty, this function will
    /// return `None`.
    /// ```rust
    /// let fare_text = route.get_fare_text();
    /// ```

    pub fn get_fare_text(&self) -> Option<&String> {
        self.fare.as_ref().map(|fare| &fare.text)
    } // fn

    /// A helper function for destructuring (or serializing) the `warnings`
    /// field. If the _warnings_ `Vec` is populated, this function will return
    /// the warnings as a `String` in pipe-separated format. If the _warnings_
    /// Vec is empty, this function will return `None`.
    /// ```rust
    /// let warnings = route.get_warning();
    /// ```

    pub fn get_warnings(&self) -> Option<String> {
        if self.warnings.is_empty() {
            None
        } else {
            Some(self.warnings.join("|"))
        } // if
    } // fn

    /// A helper function for destructuring (or serializing) the
    /// `waypoint_order` field. If the _waypoint_order_ `Vec` is populated, this
    /// function will return the waypoint order as a `String` in CSV format. If
    /// the _waypoint_order_ Vec is empty, this function will return `None`.
    /// ```rust
    /// let waypoint_order = route.get_fare_text();
    /// ```

    pub fn get_waypoint_order(&self) -> Option<String> {
        if self.waypoint_order.is_empty() {
            None
        } else {
            Some(self.waypoint_order.iter().map(|integer| integer.to_string()).collect::<Vec<String>>().join("|"))
        } // if
    } // fn

} // impl