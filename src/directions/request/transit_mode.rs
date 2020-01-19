/// Specifies one or more preferred modes of transit.
///
/// [Transit Mode](https://developers.google.com/maps/documentation/directions/intro#optional-parameters)
///
/// This parameter may only be specified for transit directions, and only if the
/// request includes an API key or a Google Maps Platform Premium Plan client
/// ID.

#[derive(Clone, Debug)]
pub enum TransitMode {
    /// Specifies bus as a preferred mode of transit.
    Bus,
    /// Specifies rail as a preferred mode of transit.
    Rail,
    /// Specifies subway as a preferred mode of transit.
    Subway,
    /// Specifies train as a preferred mode of transit.
    Train,
    /// Specifies tram as a preferred mode of transit.
    Tram,
} // enum

impl TransitMode {
    /// Converts a `TransitMode` enum to a `String` that contains a pretty
    /// source-code-style [transit mode](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitMode)
    /// code for debugging.
    pub fn source_code_print(&self) -> String {
        match self {
            TransitMode::Bus => String::from("TransitMode::Bus"),
            TransitMode::Rail => String::from("TransitMode::Rail"),
            TransitMode::Subway => String::from("TransitMode::Subway"),
            TransitMode::Train => String::from("TransitMode::Train"),
            TransitMode::Tram => String::from("TransitMode::Tram"),
        } // match
    } // fn
} // impl

impl From<&TransitMode> for String {
    /// Converts a `TransitMode` enum to a `String` that contains a
    /// [transit mode](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitMode)
    /// code.
    fn from(transit_mode: &TransitMode) -> String {
        match transit_mode {
            TransitMode::Bus => String::from("bus"),
            TransitMode::Rail => String::from("rail"),
            TransitMode::Subway => String::from("subway"),
            TransitMode::Train => String::from("train"),
            TransitMode::Tram => String::from("tram"),
        } // match
    } // fn
} // impl

impl From<String> for TransitMode {
    /// Gets a `TransitMode` enum from a `String` that contains a valid
    /// transit mode](https://developers.google.com/maps/documentation/javascript/reference/directions#TransitMode)
    /// code.
    fn from(transit_mode: String) -> TransitMode {
        match transit_mode.as_ref() {
            "bus" => TransitMode::Bus,
            "rail" => TransitMode::Rail,
            "subway" => TransitMode::Subway,
            "train" => TransitMode::Train,
            "tram" => TransitMode::Tram,
            _ => panic!("'{}' is not a valid transit mode code. Valid codes are 'bus', 'rail', 'subway', 'train', and 'tram'.", transit_mode),
        } // match
    } // fn
} // impl