/// Specifies the unit system to use when displaying results.
///
/// [Unit System](https://developers.google.com/maps/documentation/directions/intro#UnitSystems)
/// ==============================================================================================
///
/// Directions results contain `text` within `distance` fields that may be
/// displayed to the user to indicate the distance of a particular "step" of the
/// route. By default, this text uses the unit system of the origin's country or
/// region.
///
/// For example, a route from "Chicago, IL" to "Toronto, ONT" will display
/// results in miles, while the reverse route will display results in
/// kilometers. You may override this unit system by setting one explicitly
/// within the request's `units` parameter, passing one of the following values:
///
/// Note: this unit system setting only affects the `text` displayed within
/// `distance` fields. The `distance` fields also contain values which are
/// always expressed in meters.

#[derive(Clone, Debug)]
pub enum UnitSystem {
    /// Specifies that distances in the response should be expressed in imperial
    /// units, miles and feet.
    Imperial,
    /// Specifies that distances in the response should be expressed in metric
    /// units, using kilometres and metres.
    Metric,
} // enum

impl From<&UnitSystem> for String {
    /// Converts a `UnitSystem` enum to a `String` that contains a [unit system](https://developers.google.com/maps/documentation/javascript/reference/directions#UnitSystem) code.
    fn from(units: &UnitSystem) -> String {
        match units {
            UnitSystem::Imperial => String::from("imperial"),
            UnitSystem::Metric => String::from("metric"),
        } // match
    } // fn
} // impl

impl From<String> for UnitSystem {
    /// Gets a `UnitSystem` enum from a `String` that contains a valid [unit system](https://developers.google.com/maps/documentation/javascript/reference/directions#UnitSystem) code.
    fn from(units: String) -> UnitSystem {
        match units.as_ref() {
            "imperial" => UnitSystem::Imperial,
            "metric" => UnitSystem::Metric,
            _ => panic!("'{}' is not a valid unit system code. Valid codes are 'metric', and 'imperial'.", units),
        } // match
    } // fn
} // impl