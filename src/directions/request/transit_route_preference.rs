/// Specifies preferences for transit routes.
///
/// [Transit Route Preferences](https://developers.google.com/maps/documentation/directions/intro#optional-parameters)
///
/// Using this parameter, you can bias the options returned, rather than
/// accepting the default best route chosen by the API. This parameter may only
/// be specified for transit directions, and only if the request includes an API
/// key or a Google Maps Platform Premium Plan client ID.

#[derive(Clone, Debug)]
pub enum TransitRoutePreference {
    /// Specifies that the calculated route should prefer a limited number of
    /// transfers.
    FewerTransfers,
    /// Specifies that the calculated route should prefer limited amounts of
    /// walking.
    LessWalking,
} // enum

impl TransitRoutePreference {
    /// Converts a `TransitRoutePreference` enum to a `String` that contains a
    /// pretty source-code-style [transit route preference](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitRoutePreference)
    /// code for debugging.
    pub fn source_code_print(&self) -> String {
        match self {
            TransitRoutePreference::FewerTransfers => String::from("TransitRoutePreference::FewerTransfers"),
            TransitRoutePreference::LessWalking => String::from("TransitRoutePreference::LessWalking"),
        } // match
    } // fn
} // impl

impl From<&TransitRoutePreference> for String {
    /// Converts a `TransitRoutePreference` enum to a `String` that contains a
    /// [transit route preference](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitRoutePreferences)
    /// code.
    fn from(transit_route_preference: &TransitRoutePreference) -> String {
        match transit_route_preference {
            TransitRoutePreference::FewerTransfers => String::from("fewer_transfers"),
            TransitRoutePreference::LessWalking => String::from("less_walking"),
        } // match
    } // fn
} // impl

impl From<String> for TransitRoutePreference {
    /// Gets a `TransitRoutePreference` enum from a `String` that contains a
    /// valid [transit route preference](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitRoutePreference)
    /// code.
    fn from(transit_route_preference: String) -> TransitRoutePreference {
        match transit_route_preference.as_ref() {
            "fewer_transfers" => TransitRoutePreference::FewerTransfers,
            "less_walking" => TransitRoutePreference::LessWalking,
            _ => panic!("'{}' is not a valid transit route preference code. Valid codes are 'fewer_transfers' and 'less_walking'.", transit_route_preference),
        } // match
    } // fn
} // impl