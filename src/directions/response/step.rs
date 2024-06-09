//! Each element in the `steps` array defines a single step of the calculated
//! directions. A step is the most atomic unit of a direction's route.

use crate::directions::response::{
    directions_distance::DirectionsDistance,
    directions_duration::DirectionsDuration,
    driving_maneuver::DrivingManeuver,
    polyline::Polyline,
    transit_details::TransitDetails
};
use crate::directions::travel_mode::TravelMode;
use crate::types::LatLng;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Each element in the `steps` array defines a single step of the calculated
/// directions. A step is the most atomic unit of a direction's route,
/// containing a single step describing a specific, single instruction on the
/// journey. E.g. "Turn left at W. 4th St." The step not only describes the
/// instruction but also contains distance and duration information relating to
/// how this step relates to the following step. For example, a step denoted as
/// "Merge onto I-80 West" may contain a duration of "37 miles" and
/// "40 minutes," indicating that the next step is 37 miles/40 minutes from this
/// step.
///
/// When using the Directions API to search for transit directions, the steps
/// array will include additional [transit
/// details](https://developers.google.com/maps/documentation/directions/intro#TransitDetails)
/// in the form of a `transit_details` array. If the directions include multiple
/// modes of transportation, detailed directions will be provided for walking or
/// driving steps in an inner `steps` array. For example, a walking step will
/// include directions from the start and end locations: "Walk to Innes Ave &
/// Fitch St". That step will include detailed walking directions for that route
/// in the inner steps array, such as: "Head north-west", "Turn left onto
/// Arelious Walker", and "Turn left onto Innes Ave".

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Step {
    /// The distance covered by this step. This property may be undefined as the
    /// distance may be unknown.
    pub distance: DirectionsDistance,
    /// The typical time required to perform this step in seconds and in text
    /// form. This property may be undefined as the duration may be unknown.
    pub duration: DirectionsDuration,
    /// The ending location of this step.
    pub end_location: LatLng,
    /// Instructions for this step.
    pub html_instructions: Option<String>,
    /// Contains the action to take for the current step (turn left, merge,
    /// straight, etc.). This field is used to determine which icon to display.
    pub maneuver: Option<DrivingManeuver>,
    /// Contains a single `points` object that holds an [encoded polyline](https://developers.google.com/maps/documentation/utilities/polylinealgorithm) representation of the step. This polyline is an approximate (smoothed) path of the step. (Corresponds to `path` in the [Directions.Step interface](https://developers.google.com/maps/documentation/javascript/reference/directions#DirectionsStep).)
    ///
    /// See also: the Google Encoded Polyline encoding & decoding crate called
    /// [polyline](https://crates.io/crates/polyline).
    pub polyline: Polyline,
    /// The starting location of this step.
    pub start_location: LatLng,
    /// Contains detailed directions for walking or driving steps in transit
    /// directions. Substeps are only available when `travel_mode` is set to
    /// "transit". The inner steps array is of the same type as steps.
    #[serde(default)]
    pub steps: Vec<Step>,
    /// Transit-specific details about this step. This property will be
    /// undefined unless the travel mode of this step is `TravelMode::Transit`.
    pub transit_details: Option<TransitDetails>,
    /// The mode of travel used in this step.
    pub travel_mode: TravelMode,
} // struct

// -----------------------------------------------------------------------------

impl Step {
    /// A helper function for destructuring (or serializing) the optional
    /// `maneuver` field. If the `ManeuverType` enum in the step is populated,
    /// this function will return it as a `String`. If the _`ManeuverType`_ enum
    /// is empty, this function will return `None`.
    /// ```rust
    /// let maneuver = step.get_maneuver();
    /// ```

    pub fn get_maneuver(&self) -> Option<String> {
        self.maneuver.as_ref().map(String::from)
    } // fn
} // impl

// -----------------------------------------------------------------------------

#[cfg(all(feature = "polyline", feature = "geo"))]
impl Step {
    /// Attempts to convert a borrowed `&Step` struct to a
    /// `geo_types::geometry::LineString<f64>` struct.
    ///
    /// # Errors
    ///
    /// * Returns an error if the polyline is invalid or if the decoded
    ///   coordinates are out of bounds.
    fn decode_polyline(
        &self,
        precision: u32
    ) -> Result<geo_types::geometry::LineString<f64>, crate::error::Error> {
        self.polyline.decode(precision)
    } // fn
} // impl

// -----------------------------------------------------------------------------

#[cfg(all(feature = "polyline", feature = "geo"))]
impl TryFrom<&Step> for geo_types::geometry::LineString<f64> {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert a borrowed `&Step` struct to a
    /// `geo_types::geometry::LineString<f64>` struct.
    ///
    /// # Notes
    ///
    /// * Uses a hard-coded precision of `5`. For control over the precision,
    ///   use the `decode_polyline` method on the `Step` struct.
    ///
    /// # Errors
    ///
    /// * Returns an error if the polyline is invalid or if the decoded
    ///   coordinates are out of bounds.
    fn try_from(step: &Step) -> Result<Self, Self::Error> {
        step.decode_polyline(5)
    } // fn
} // impl

// -----------------------------------------------------------------------------

#[cfg(all(feature = "polyline", feature = "geo"))]
impl TryFrom<Step> for geo_types::geometry::LineString<f64> {
    // Error definitions are contained in the `google_maps\src\error.rs` module.
    type Error = crate::error::Error;

    /// Attempts to convert an owned `Step` struct into a
    /// `geo_types::geometry::LineString<f64>` struct.
    ///
    /// # Notes
    ///
    /// * Uses a hard-coded precision of `5`. For control over the precision,
    ///   use the `decode_polyline` method on the `Step` struct.
    ///
    /// # Errors
    ///
    /// * Returns an error if the polyline is invalid or if the decoded
    ///   coordinates are out of bounds.
    fn try_from(step: Step) -> Result<Self, Self::Error> {
        step.decode_polyline(5)
    } // fn
} // impl