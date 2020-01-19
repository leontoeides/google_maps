use serde::{Serialize, Deserialize};

/// Specifies the mode of transportation.
///
/// [Travel Modes](https://developers.google.com/maps/documentation/directions/intro#TravelModes)
/// =============================================================================================
///
/// Note: Both walking and bicycling directions may sometimes not include
/// clear pedestrian or bicycling paths, so these directions will return
/// `warnings` in the returned result which you must display to the user.

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum TravelMode {
    /// Specifies a driving directions request.
    #[serde(alias = "DRIVING")]
    Driving,
    /// Specifies a driving directions request.
    #[serde(alias = "WALKING")]
    Walking,
    /// Specifies a bicycling directions request.
    #[serde(alias = "BICYCLING")]
    Bicycling,
    /// Specifies a bicycling directions request.
    #[serde(alias = "TRANSIT")]
    Transit,
} // enum

impl TravelMode {
    /// Converts a `TravelMode` enum to a `String` that contains a pretty
    /// source-code-style [travel mode](https://developers.google.com/maps/documentation/javascript/reference/directions#TravelMode)
    /// code for debugging.
    pub fn source_code_print(&self) -> String {
        match self {
            TravelMode::Bicycling => String::from("TravelMode::Bicycling"),
            TravelMode::Driving => String::from("TravelMode::Driving"),
            TravelMode::Transit => String::from("TravelMode::Transit"),
            TravelMode::Walking => String::from("TravelMode::Walking"),
        } // match
    } // fn
} // impl

impl From<&TravelMode> for String {
    /// Converts a `TravelMode` enum to a `String` that contains a
    /// [travel mode](https://developers.google.com/maps/documentation/javascript/reference/directions#TravelMode)
    /// code.
    fn from(travel_mode: &TravelMode) -> String {
        match travel_mode {
            TravelMode::Bicycling => String::from("bicycling"),
            TravelMode::Driving => String::from("driving"),
            TravelMode::Transit => String::from("transit"),
            TravelMode::Walking => String::from("walking"),
        } // match
    } // fn
} // impl

impl From<String> for TravelMode {
    /// Gets a `TravelMode` enum from a `String` that contains a valid
    /// [travel mode](https://developers.google.com/maps/documentation/javascript/reference/directions#TravelMode)
    /// code.
    fn from(travel_mode: String) -> TravelMode {
        match travel_mode.as_ref() {
            "bicycling" => TravelMode::Bicycling,
            "driving" => TravelMode::Driving,
            "transit" => TravelMode::Transit,
            "walking" => TravelMode::Walking,
            _ => panic!("'{}' is not a valid travel mode code. Valid codes are 'bicycling', 'driving', 'transit', and 'walking'.", travel_mode),
        } // match
    } // fn
} // impl