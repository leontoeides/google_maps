use crate::places_new::{LocalizedText, ContentBlock};

// -------------------------------------------------------------------------------------------------
//
/// The summary of amenities near an EV charging station.
///
/// EV charge amenity summaries provide information about facilities and services available
/// near electric vehicle charging stations. This helps EV drivers plan charging stops by
/// understanding what amenities they can access while their vehicle charges.
///
/// This information only applies to places with type `electric_vehicle_charging_station` and
/// provides AI-generated summaries of nearby options for food, shopping, and other services.
#[derive(
    //std
    Clone,
    Debug,
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
pub struct EvChargeAmenitySummary {
    /// An overview of the available amenities.
    ///
    /// Provides a general summary of amenities and services available near the charging station.
    /// This overview is guaranteed to be provided and gives users a quick understanding of what
    /// facilities they can expect while charging.
    #[getset(get = "pub")]
    pub overview: ContentBlock,

    /// A summary of nearby coffee options.
    ///
    /// Contains information about coffee shops, cafes, and other beverage options available
    /// near the charging station. Useful for drivers who want to grab refreshments during
    /// their charging session.
    #[serde(default)]
    #[getset(get = "pub")]
    pub coffee: Option<ContentBlock>,

    /// A summary of nearby restaurant options.
    ///
    /// Provides information about dining establishments near the charging station, helping
    /// EV drivers find meal options during longer charging sessions or trip stops.
    #[serde(default)]
    #[getset(get = "pub")]
    pub restaurant: Option<ContentBlock>,

    /// A summary of nearby store options.
    ///
    /// Contains information about retail stores, convenience stores, and shopping options
    /// available near the charging station. Useful for drivers who need supplies or want
    /// to run errands while charging.
    #[serde(default)]
    #[getset(get = "pub")]
    pub store: Option<ContentBlock>,

    /// A link where users can flag a problem with the summary.
    ///
    /// Provides a URL where users can report issues with the AI-generated amenity summaries,
    /// enabling quality control and improvement of the information provided to EV drivers.
    #[getset(get = "pub")]
    pub flag_content_uri: url::Url,

    /// The AI disclosure message indicating the content source.
    ///
    /// Contains localized text such as "Summarized with Gemini" to inform users that the
    /// amenity summaries were generated using AI technology. Appears in the language
    /// specified in the request when available.
    #[getset(get = "pub")]
    pub disclosure_text: LocalizedText,
}