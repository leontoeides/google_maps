/// Used to specify features that calculated routes should avoid.
///
/// [Restrictions](https://developers.google.com/maps/documentation/directions/intro#Restrictions)
/// ==============================================================================================
///
/// Directions may be calculated that adhere to certain restrictions.
/// Restrictions are indicated by use of the `avoid` parameter, and an argument to
/// that parameter indicating the restriction to avoid. The following
/// restrictions are supported:
///
/// * Tolls
/// * Highways
/// * Ferries
///
/// It's possible to request a route that avoids any combination of tolls,
/// highways and ferries by passing both restrictions to the avoid parameter.
/// For example: `avoid=tolls|highways|ferries`.
///
/// Note: the addition of restrictions does not preclude routes that include the
/// restricted feature; it simply biases the result to more favorable routes.

#[derive(Clone, Debug)]
pub enum Avoid {
    /// Instructs the Directions service to avoid toll roads where possible.
    Tolls,
    /// Instructs the Directions service to avoid highways where possible.
    Highways,
    /// Instructs the Directions service to avoid ferries where possible.
    Ferries,
    /// Instructs the Directions service to avoid indoor steps for walking and
    /// transit directions. Only requests that include an API key or a Google
    /// Maps Platform Premium Plan client ID will receive indoor steps by
    /// default.
    Indoor,
} // enum

impl Avoid {
    /// Converts an `Avoid` enum to a `String` that contains a pretty
    /// source-code-style [restrictions](https://developers.google.com/maps/documentation/directions/intro#Restrictions)
    /// code for debugging.
    pub fn source_code_print(&self) -> String {
        match self {
            Avoid::Tolls => String::from("Avoid::Tolls"),
            Avoid::Highways => String::from("Avoid::Highways"),
            Avoid::Ferries => String::from("Avoid::Ferries"),
            Avoid::Indoor => String::from("Avoid::Indoor"),
        } // match
    } // fn
} // impl

impl From<&Avoid> for String {
    /// Converts an `Avoid` enum to a `String` that contains a
    /// [restrictions](https://developers.google.com/maps/documentation/directions/intro#Restrictions)
    /// code.
    fn from(avoid: &Avoid) -> String {
        match avoid {
            Avoid::Tolls => String::from("tolls"),
            Avoid::Highways => String::from("highways"),
            Avoid::Ferries => String::from("ferries"),
            Avoid::Indoor => String::from("indoor"),
        } // match
    } // fn
} // impl

impl From<String> for Avoid {
    /// Gets an `Avoid` enum from a `String` that contains a valid
    /// [restrictions](https://developers.google.com/maps/documentation/directions/intro#Restrictions)
    /// code.
    fn from(avoid: String) -> Avoid {
        match avoid.as_ref() {
            "tolls" => Avoid::Tolls,
            "highways" => Avoid::Highways,
            "ferries" => Avoid::Ferries,
            "indoor" => Avoid::Indoor,
            _ => panic!("'{}' is not a valid restrictions code. Valid codes are 'tolls', 'highways', 'ferries' and 'indoor'.", avoid),
        } // match
    } // fn
} // impl