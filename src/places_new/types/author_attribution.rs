use url::Url;

// -------------------------------------------------------------------------------------------------
//
/// Information about the author of the UGC data.
/// 
/// Used in [Review](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Review)
/// and [Photo](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Photo).
/// This struct provides attribution information for user-generated content to ensure
/// proper crediting of content creators.
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
pub struct AuthorAttribution {
    /// Name of the author of the
    /// [Photo](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Photo) or
    /// [Review](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Review).
    #[serde(default)]
    #[getset(get = "pub")]
    pub display_name: Option<String>,

    /// URI of the author of the
    /// [Photo](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Photo) or
    /// [Review](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Review).
    #[serde(default)]
    #[getset(get = "pub")]
    pub uri: Option<Url>,

    /// Profile photo URI of the author of the
    /// [Photo](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Photo) or
    /// [Review](https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places#Review).
    #[serde(default)]
    #[getset(get = "pub")]
    pub photo_uri: Option<Url>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl AuthorAttribution {
    /// Returns whether this attribution has a display name.
    ///
    /// This is the display name that should be shown when attributing the content to its author.
    #[must_use]
    pub const fn has_display_name(&self) -> bool {
        self.display_name.is_some()
    }

    /// Returns whether this attribution has a profile URI.
    ///
    /// This link can be used to view more information about the author, such as their Google
    /// profile or other authored content.
    #[must_use]
    pub const fn has_uri(&self) -> bool {
        self.uri.is_some()
    }

    /// Returns whether this attribution has a photo URI.
    ///
    /// This link provides access to the author's profile image for display alongside their
    /// attributed content.
    #[must_use]
    pub const fn has_photo_uri(&self) -> bool {
        self.photo_uri.is_some()
    }

    /// Returns whether this attribution is completely empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.display_name.is_none() && self.uri.is_none() && self.photo_uri.is_none()
    }
}