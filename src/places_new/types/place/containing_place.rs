// -------------------------------------------------------------------------------------------------
//
/// Information about a place that contains this place.
///
/// Containing places provide hierarchical context about where a place is situated within larger
/// geographic or administrative boundaries. This helps establish location relationships and
/// enables navigation between different levels of geographic organization.
///
/// For example, a restaurant might be contained within a shopping mall, which is contained within
/// a city, which is contained within a state or province.
#[derive(
    //std
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
    // getset
    getset::Getters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[serde(rename_all = "camelCase")]
pub struct ContainingPlace {
    /// The resource name of the containing place.
    ///
    /// Provides a stable identifier for the containing place in the format `places/{place_id}`
    /// that can be used in subsequent API calls to retrieve detailed information about the
    /// containing location.
    #[serde(default)]
    #[getset(get = "pub")]
    pub name: String,

    /// The place ID of the containing place.
    ///
    /// A unique identifier that can be used with other Google Places API endpoints to fetch
    /// comprehensive details about the containing place, including its own containing places
    /// for building complete hierarchical context.
    #[serde(rename = "id", default)]
    #[getset(get = "pub")]
    pub place_id: String,
}