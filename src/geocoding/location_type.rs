use serde::{Serialize, Deserialize};

/// Stores additional data about the specified location.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LocationType {

    /// Indicates that the returned result is approximate.
    #[serde(alias = "APPROXIMATE")]
    Approximate,

    /// Indicates that the returned result is the geometric center of a result
    /// such as a polyline (for example, a street) or polygon (region).
    #[serde(alias = "GEOMETRIC_CENTER")]
    GeometricCenter,

    /// Indicates that the returned result reflects an approximation (usually on
    /// a road) interpolated between two precise points (such as intersections).
    /// Interpolated results are generally returned when rooftop geocodes are
    /// unavailable for a street address.
    #[serde(alias = "RANGE_INTERPOLATED")]
    RangeInterpolated,

    /// Indicates that the returned result is a precise geocode for which we
    /// have location information accurate down to street address precision.
    #[serde(alias = "ROOFTOP")]
    RoofTop,

} // enum

impl From<&LocationType> for String {
    /// Converts a `LocationType` enum to a `String` that contains a [location
    /// type](https://developers.google.com/maps/documentation/geocoding/intro#Results) code.
    fn from(location_type: &LocationType) -> String {
        match location_type {
            LocationType::Approximate => String::from("APPROXIMATE"),
            LocationType::GeometricCenter => String::from("GEOMETRIC_CENTER"),
            LocationType::RangeInterpolated => String::from("RANGE_INTERPOLATED"),
            LocationType::RoofTop => String::from("ROOFTOP"),
        } // match
    } // fn
} // impl

impl From<String> for LocationType {
    /// Gets a `LocationType` enum from a `String` that contains a supported
    /// [location
    /// type](https://developers.google.com/maps/documentation/geocoding/intro#Results)
    /// code.
    fn from(location_type: String) -> LocationType {
        match location_type.as_ref() {
            "APPROXIMATE" => LocationType::Approximate,
            "GEOMETRIC_CENTER" => LocationType::GeometricCenter,
            "RANGE_INTERPOLATED" => LocationType::RangeInterpolated,
            "ROOFTOP" => LocationType::RoofTop,
            _ => panic!("'{}' is not a known location type code. \
                Tip: The location type code must be in uppercase. \
                For a list of supported location types see \
                https://developers.google.com/maps/documentation/geocoding/intro#Results",
                location_type),
        } // match
    } // fn
} // impl