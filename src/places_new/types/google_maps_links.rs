// -------------------------------------------------------------------------------------------------
//
/// Links to trigger different Google Maps actions for a place.
///
/// Google Maps links provide direct URLs that launch specific Google Maps functionality for a
/// place, such as getting directions, viewing the place page, writing reviews, or viewing photos.
/// These links are formatted to work across different Google Maps interfaces and platforms.
///
/// All links are pre-formatted and ready to use in web browsers or mobile applications that
/// support Google Maps integration.
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
pub struct GoogleMapsLinks {
    /// A link to show directions to the place using default travel mode.
    ///
    /// Opens Google Maps with directions to this place from the user's current location. Uses the
    /// default travel mode (typically driving) and only populates the destination location in the
    /// directions interface.
    #[serde(rename = "directionsUri")]
    #[getset(get = "pub")]
    pub directions: String,

    /// A direct link to show this place on Google Maps.
    ///
    /// Opens the main Google Maps view centered on this place, showing the place marker and basic
    /// information overlay. Used for general place viewing and exploration.
    #[serde(rename = "placeUri")]
    #[getset(get = "pub")]
    pub place: String,

    /// A link to write a review for this place on Google Maps.
    ///
    /// Opens the Google Maps review interface allowing users to submit ratings and written reviews
    /// for this place. Requires user authentication with Google account.
    #[serde(rename = "writeAReviewUri")]
    #[getset(get = "pub")]
    pub write_a_review: String,

    /// A link to show reviews of this place on Google Maps.
    ///
    /// Opens the Google Maps interface displaying existing user reviews and ratings for this place.
    /// Provides read-only access to community feedback and experiences.
    #[serde(rename = "reviewsUri")]
    #[getset(get = "pub")]
    pub reviews: String,

    /// A link to show photos of this place on Google Maps.
    ///
    /// Opens the Google Maps photo gallery view for this place, displaying user-contributed and
    /// business photos. Provides visual context and additional place information.
    #[serde(rename = "photosUri")]
    #[getset(get = "pub")]
    pub photos: String,
}