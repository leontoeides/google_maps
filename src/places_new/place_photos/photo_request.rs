use crate::places_new::{AuthorAttribution, PhotoInfo, Place};
use url::Url;

// -------------------------------------------------------------------------------------------------
//
/// A photo request that's used by the Place Photos (New) service.
///
/// This is not typically used by end-users. It's an intermediary type instantiated by a photo
/// `name` in string format or by a `PhotoInfo` struct.
///
/// This struct contains metadata about photos including dimensions, attribution information, and
/// links for reporting issues or viewing the photo in Google Maps.
///
/// > 🛑 Caution: You cannot cache a photo name. Also, the name can expire. Ensure you always get
/// > the name from a response to a request to
/// > [Place Details (New)](https://developers.google.com/maps/documentation/places/web-service/place-details),
/// > [Nearby Search (New)](https://developers.google.com/maps/documentation/places/web-service/nearby-search),
/// > or [Text Search (New)](https://developers.google.com/maps/documentation/places/web-service/text-search).
/// > For more info, see the caching restrictions in Section 3.2.3(b)(No Caching) of the
/// > [Google Maps Platform Terms of Service](https://cloud.google.com/maps-platform/terms).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PhotoRequest {
    /// Identifier for this place photo.
    ///
    /// A reference representing this place photo which may be used to look up this place photo
    /// again, also called the API "resource" name. This name can be used with the Photo Media API
    /// to fetch the actual photo content at various sizes.
    ///
    /// > 🛑 Caution: You cannot cache a photo name. Also, the name can expire. Ensure you always
    /// > get the name from a response to a request to
    /// > [Place Details (New)](https://developers.google.com/maps/documentation/places/web-service/place-details),
    /// > [Nearby Search (New)](https://developers.google.com/maps/documentation/places/web-service/nearby-search),
    /// > or [Text Search (New)](https://developers.google.com/maps/documentation/places/web-service/text-search).
    /// > For more info, see the caching restrictions in Section 3.2.3(b)(No Caching) of the
    /// > [Google Maps Platform Terms of Service](https://cloud.google.com/maps-platform/terms).
    pub name: String,

    /// The image's width, in pixels.
    ///
    /// The maximum width that this photo can be displayed at while maintaining quality. Used to
    /// determine appropriate display sizes and avoid upscaling beyond the original photo
    /// resolution.
    pub width_px: Option<u32>,

    /// The image's height, in pixels.
    ///
    /// The maximum height that this photo can be displayed at while maintaining quality. Used to
    /// determine appropriate display sizes and avoid upscaling beyond the original photo
    /// resolution.
    pub height_px: Option<u32>,

    /// Attribution information for this photo's authors.
    ///
    /// Contains information about who took or contributed this photo, including display names,
    /// profile links, and avatar URLs. This attribution should be displayed when showing the photo
    /// to comply with usage requirements.
    pub author_attributions: Vec<AuthorAttribution>,

    /// A link where users can flag a problem with the photo.
    ///
    /// URL that allows users to report inappropriate content, copyright issues, or other problems
    /// with this photo. Should be provided as a reporting mechanism in applications that display
    /// photos.
    pub flag_content_uri: Option<Url>,

    /// A link to show the photo on Google Maps.
    ///
    /// URL that opens this photo in the Google Maps interface, providing users with additional
    /// context and the full Google Maps photo viewing experience. Useful for "View in Google Maps"
    /// functionality.
    pub google_maps_uri: Option<Url>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl PhotoRequest {
    /// Creates a new photo request with only a photo name.
    ///
    /// All metadata fields are initialized to `None` or empty. This is the minimum required to
    /// fetch a photo from the Place Photos API.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = PhotoRequest::new("places/ChIJ.../photos/ABC");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self::from(name.into())
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl From<String> for PhotoRequest {
    /// Creates a `PhotoRequest` from an owned `String`.
    ///
    /// The photo name is moved into the request. All metadata fields are set to `None` since only
    /// the photo name is provided. Metadata can be populated later if the photo is obtained from a
    /// place response.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = PhotoRequest::from("places/ChIJ.../photos/ABC".to_string());
    /// ```
    fn from(name: String) -> Self {
        Self {
            name,
            width_px: None,
            height_px: None,
            author_attributions: Vec::new(),
            flag_content_uri: None,
            google_maps_uri: None,
        }
    }
}

impl From<&str> for PhotoRequest {
    /// Creates a `PhotoRequest` from a string slice.
    ///
    /// The photo name is copied into the request. All metadata fields are set to `None` since only
    /// the photo name is provided. Metadata can be populated later if the photo is obtained from a
    /// place response.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let request = PhotoRequest::from("places/ChIJ.../photos/ABC");
    /// ```
    fn from(name: &str) -> Self {
        Self::from(name.to_string())
    }
}

impl From<&String> for PhotoRequest {
    /// Creates a `PhotoRequest` from a reference to a `String`.
    ///
    /// The photo name is cloned into the request. All metadata fields are set to `None` since only
    /// the photo name is provided. Metadata can be populated later if the photo is obtained from a
    /// place response.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let name = "places/ChIJ.../photos/ABC".to_string();
    /// let request = PhotoRequest::from(&name);
    /// ```
    fn from(name: &String) -> Self {
        Self::from(name.to_string())
    }
}

impl<'a> From<std::borrow::Cow<'a, str>> for PhotoRequest {
    /// Creates a `PhotoRequest` from a `Cow<str>`.
    ///
    /// The photo name is converted to an owned string. All metadata fields are set to `None` since
    /// only the photo name is provided. Metadata can be populated later if the photo is obtained
    /// from a place response.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use std::borrow::Cow;
    /// let name = Cow::Borrowed("places/ChIJ.../photos/ABC");
    /// let request = PhotoRequest::from(name);
    /// ```
    fn from(name: std::borrow::Cow<'a, str>) -> Self {
        Self::from(name.to_string())
    }
}

impl From<Box<str>> for PhotoRequest {
    /// Creates a `PhotoRequest` from a boxed string slice.
    ///
    /// The photo name is converted to an owned string. All metadata fields are set to `None` since
    /// only the photo name is provided. Metadata can be populated later if the photo is obtained
    /// from a place response.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let name: Box<str> = "places/ChIJ.../photos/ABC".into();
    /// let request = PhotoRequest::from(name);
    /// ```
    fn from(name: Box<str>) -> Self {
        Self::from(name.to_string())
    }
}

impl std::fmt::Display for PhotoRequest {
    /// Formats the photo request showing the name and available metadata.
    ///
    /// Displays the photo name along with any available dimensions and attribution count. Useful
    /// for logging and debugging.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PhotoRequest {{ name: {}", self.name)?;

        if let (Some(w), Some(h)) = (self.width_px, self.height_px) {
            write!(f, ", dimensions: {w}×{h}")?;
        }

        if !self.author_attributions.is_empty() {
            write!(f, ", attributions: {}", self.author_attributions.len())?;
        }

        write!(f, " }}")
    }
}

impl From<PhotoInfo> for PhotoRequest {
    /// Creates a `PhotoRequest` from a `PhotoInfo`, preserving all metadata.
    ///
    /// This is the preferred way to create a photo request when you have full photo information
    /// from a place response. All metadata including dimensions, attributions, and links are
    /// preserved in the request.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let place = client.place_details("ChIJ...")?.execute().await?;
    /// let photo_info = &place.photos()[0];
    ///
    /// // Convert to request - all metadata is preserved!
    /// let request = PhotoRequest::from(photo_info.clone());
    /// ```
    fn from(photo_info: PhotoInfo) -> Self {
        Self {
            name: photo_info.name,
            width_px: Some(photo_info.width_px),
            height_px: Some(photo_info.height_px),
            author_attributions: photo_info.author_attributions,
            flag_content_uri: photo_info.flag_content_uri,
            google_maps_uri: photo_info.google_maps_uri,
        }
    }
}

impl From<&PhotoInfo> for PhotoRequest {
    /// Creates a `PhotoRequest` from a reference to `PhotoInfo`, preserving all metadata.
    ///
    /// Clones the necessary data from the photo info. This allows you to create a photo request
    /// without consuming the original `PhotoInfo` struct.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let place = client.place_details("ChIJ...")?.execute().await?;
    /// let photo_info = &place.photos()[0];
    ///
    /// // Create request without consuming photo_info
    /// let request = PhotoRequest::from(photo_info);
    ///
    /// // Can still use photo_info here
    /// println!("Photo dimensions: {}x{}", photo_info.width_px, photo_info.height_px);
    /// ```
    fn from(photo_info: &PhotoInfo) -> Self {
        Self {
            name: photo_info.name.clone(),
            width_px: Some(photo_info.width_px),
            height_px: Some(photo_info.height_px),
            author_attributions: photo_info.author_attributions.clone(),
            flag_content_uri: photo_info.flag_content_uri.clone(),
            google_maps_uri: photo_info.google_maps_uri.clone(),
        }
    }
}

impl TryFrom<&Place> for PhotoRequest {
    type Error = crate::places_new::place_photos::Error;

    /// Creates a `PhotoRequest` from the first photo of a `Place`.
    ///
    /// Extracts the first photo from the place's photos array and converts it to a photo request.
    /// This is a convenience for the common case of fetching a place's primary photo.
    ///
    /// # Errors
    ///
    /// Returns an error if the place has no photos.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let place = client.place_details("ChIJ...")?.execute().await?;
    ///
    /// // Get the first photo automatically
    /// let request = PhotoRequest::try_from(&place)?;
    ///
    /// // Equivalent to:
    /// // let request = PhotoRequest::from(&place.photos()[0]);
    /// ```
    fn try_from(place: &Place) -> Result<Self, Self::Error> {
        place
            .photos()
            .first()
            .map(Self::from)
            .ok_or_else(|| crate::places_new::place_photos::Error::PlaceHasNoPhotos {
                place_id: place.id().clone(),
                span: place
                    .id()
                    .as_ref()
                    .map_or((0..0).into(), |place_id| (0..place_id.len()).into())
            })
    }
}

impl TryFrom<Place> for PhotoRequest {
    type Error = crate::places_new::place_photos::Error;

    /// Creates a `PhotoRequest` from the first photo of a `Place`, consuming the place.
    ///
    /// Extracts the first photo from the place's photos array and converts it to a photo request.
    /// This consumes the place, so use `TryFrom<&Place>` if you need to keep the place around.
    ///
    /// # Errors
    ///
    /// Returns an error if the place has no photos.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let place = client.place_details("ChIJ...")?.execute().await?;
    ///
    /// // Move place into the conversion
    /// let request = PhotoRequest::try_from(place)?;
    /// ```
    fn try_from(place: Place) -> Result<Self, Self::Error> {
        place
            .photos()
            .first()
            .map(Self::from)
            .ok_or_else(|| crate::places_new::place_photos::Error::PlaceHasNoPhotos {
                place_id: place.id().clone(),
                span: place
                    .id()
                    .as_ref()
                    .map_or((0..0).into(), |place_id| (0..place_id.len()).into())
            })
    }
}