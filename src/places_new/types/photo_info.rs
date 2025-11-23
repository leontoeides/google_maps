#![allow(clippy::ref_option)]

use crate::places_new::AuthorAttribution;
use url::Url;

// -------------------------------------------------------------------------------------------------
//
/// Information about a photo of a place.
///
/// Reedem this structure for a URL using the Place Photos (New) service.
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
#[derive(
    //std
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    // getset
    getset::Getters,
    getset::CopyGetters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[serde(rename_all = "camelCase")]
pub struct PhotoInfo {
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
    #[getset(get = "pub")]
    pub name: String,

    /// The image's width, in pixels.
    ///
    /// The maximum width that this photo can be displayed at while maintaining quality. Used to
    /// determine appropriate display sizes and avoid upscaling beyond the original photo
    /// resolution.
    #[getset(get_copy = "pub")]
    pub width_px: u32,

    /// The image's height, in pixels.
    ///
    /// The maximum height that this photo can be displayed at while maintaining quality. Used to
    /// determine appropriate display sizes and avoid upscaling beyond the original photo
    /// resolution.
    #[getset(get_copy = "pub")]
    pub height_px: u32,

    /// Attribution information for this photo's authors.
    ///
    /// Contains information about who took or contributed this photo, including display names,
    /// profile links, and avatar URLs. This attribution should be displayed when showing the photo
    /// to comply with usage requirements.
    #[serde(default)]
    #[getset(get = "pub")]
    pub author_attributions: Vec<AuthorAttribution>,

    /// A link where users can flag a problem with the photo.
    ///
    /// URL that allows users to report inappropriate content, copyright issues, or other problems
    /// with this photo. Should be provided as a reporting mechanism in applications that display
    /// photos.
    #[serde(default)]
    #[getset(get = "pub")]
    pub flag_content_uri: Option<Url>,

    /// A link to show the photo on Google Maps.
    ///
    /// URL that opens this photo in the Google Maps interface, providing users with additional
    /// context and the full Google Maps photo viewing experience. Useful for "View in Google Maps"
    /// functionality.
    #[serde(default)]
    #[getset(get = "pub")]
    pub google_maps_uri: Option<Url>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl PhotoInfo {
    /// Creates a new `PhotoInfo` with the specified name and dimensions.
    ///
    /// Used to construct a `PhotoInfo` instance with the essential identification and size
    /// information. Additional metadata like attributions and links can be added separately.
    #[must_use]
    pub fn new(name: impl Into<String>, width_px: u32, height_px: u32) -> Self {
        Self {
            name: name.into(),
            width_px,
            height_px,
            author_attributions: Vec::new(),
            flag_content_uri: None,
            google_maps_uri: None,
        }
    }

    /// Returns the aspect ratio of the photo (width / height).
    ///
    /// Used for layout calculations, responsive design, and determining appropriate display
    /// containers. Helps maintain photo proportions when resizing.
    #[must_use]
    pub fn aspect_ratio(&self) -> f64 {
        if self.height_px > 0 {
            f64::from(self.width_px) / f64::from(self.height_px)
        } else {
            1.0 // Default to square if height is invalid
        }
    }

    /// Returns whether this is a landscape-oriented photo.
    ///
    /// Used for layout decisions and responsive design, helping applications choose appropriate
    /// display styles for horizontally-oriented images.
    #[must_use]
    pub const fn is_landscape(&self) -> bool {
        self.width_px > self.height_px
    }

    /// Returns whether this is a portrait-oriented photo.
    ///
    /// Used for layout decisions and responsive design, helping applications choose appropriate
    /// display styles for vertically-oriented images.
    #[must_use]
    pub const fn is_portrait(&self) -> bool {
        self.height_px > self.width_px
    }

    /// Returns whether this is a square photo.
    ///
    /// Used to identify photos with equal dimensions, which may have special layout considerations
    /// or display treatments in user interfaces.
    #[must_use]
    pub const fn is_square(&self) -> bool {
        self.width_px == self.height_px
    }

    /// Returns the total pixel count of the photo.
    ///
    /// Used to assess photo quality, determine storage requirements, or compare relative photo
    /// sizes when multiple photos are available.
    #[must_use]
    pub fn pixel_count(&self) -> i64 {
        i64::from(self.width_px) * i64::from(self.height_px)
    }

    /// Returns whether this photo has high resolution (> 1 megapixel).
    ///
    /// Used to categorize photo quality and determine whether the photo is suitable for
    /// high-quality displays or printing applications.
    #[must_use]
    pub fn is_high_resolution(&self) -> bool {
        self.pixel_count() > 1_000_000
    }

    /// Calculates dimensions that fit within the specified maximum size while preserving aspect
    /// ratio.
    ///
    /// Used for responsive image display, thumbnail generation, and ensuring photos fit within
    /// specific UI constraints while maintaining their original proportions.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn fit_within(&self, max_width: u32, max_height: u32) -> (u32, u32) {
        if self.width_px <= max_width && self.height_px <= max_height {
            return (self.width_px, self.height_px);
        }

        let aspect_ratio = self.aspect_ratio();
        let width_constrained = (f64::from(max_height) * aspect_ratio) as u32;
        let height_constrained = (f64::from(max_width) / aspect_ratio) as u32;

        if width_constrained <= max_width {
            (width_constrained, max_height)
        } else {
            (max_width, height_constrained)
        }
    }

    /// Returns whether the photo has valid dimensions.
    ///
    /// Used to validate photo metadata before displaying or processing images, ensuring both width
    /// and height are positive values.
    #[must_use]
    pub const fn has_valid_dimensions(&self) -> bool {
        self.width_px > 0 && self.height_px > 0
    }

    /// Returns whether this photo has author attribution information.
    ///
    /// Used to determine if attribution should be displayed and whether proper credit information
    /// is available for the photo contributors.
    #[must_use]
    pub fn has_attribution(&self) -> bool {
        !self.author_attributions.is_empty()
    }

    /// Returns whether this photo has moderation links available.
    ///
    /// Used to determine if reporting functionality should be enabled for this photo, allowing
    /// users to flag inappropriate or problematic content.
    #[must_use]
    pub const fn has_moderation_links(&self) -> bool {
        self.flag_content_uri.is_some()
    }

    /// Returns whether this photo has Google Maps integration.
    ///
    /// Used to determine if "View in Google Maps" functionality is available for this photo,
    /// providing enhanced viewing options for users.
    #[must_use]
    pub const fn has_google_maps_link(&self) -> bool {
        self.google_maps_uri.is_some()
    }

    /// Gets a display-friendly description of the photo dimensions.
    ///
    /// Provides a human-readable description of the photo size suitable for displaying in user
    /// interfaces, tooltips, or metadata views.
    #[must_use]
    pub fn dimension_description(&self) -> String {
        if !self.has_valid_dimensions() {
            return "Invalid dimensions".to_string();
        }

        let megapixels = self.pixel_count() as f64 / 1_000_000.0;

        let orientation = if self.is_landscape() {
            "landscape"
        } else if self.is_portrait() {
            "portrait" 
        } else {
            "square"
        };

        if megapixels >= 1.0 {
            format!("{} × {} ({:.1}MP, {})", self.width_px, self.height_px, megapixels, orientation)
        } else {
            format!("{} × {} ({})", self.width_px, self.height_px, orientation)
        }
    }

    /// Returns a CSS-compatible aspect ratio string.
    ///
    /// Provides aspect ratio in CSS format (e.g., "16/9", "4/3") for use in web applications,
    /// responsive design, and CSS aspect-ratio properties.
    #[must_use]
    pub fn css_aspect_ratio(&self) -> String {
        // Find GCD to simplify the ratio
        fn gcd(a: u32, b: u32) -> u32 {
            if b == 0 { a } else { gcd(b, a % b) }
        }

        if !self.has_valid_dimensions() {
            return "1/1".to_string();
        }

        let divisor = gcd(self.width_px, self.height_px);
        let simplified_width = self.width_px / divisor;
        let simplified_height = self.height_px / divisor;

        format!("{simplified_width}/{simplified_height}")
    }

    /// Estimates the appropriate thumbnail size for this photo.
    ///
    /// Returns suggested thumbnail dimensions based on the photo's aspect ratio and orientation,
    /// useful for generating consistent thumbnail displays.
    #[must_use]
    pub fn suggested_thumbnail_size(&self) -> (u32, u32) {
        const THUMBNAIL_TARGET: u32 = 200;
        
        if self.is_landscape() {
            self.fit_within(THUMBNAIL_TARGET * 2, THUMBNAIL_TARGET)
        } else if self.is_portrait() {
            self.fit_within(THUMBNAIL_TARGET, THUMBNAIL_TARGET * 2)
        } else {
            (THUMBNAIL_TARGET, THUMBNAIL_TARGET)
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialization() {
        let photo = PhotoInfo {
            name: "places/photo123".to_string(),
            width_px: 1920,
            height_px: 1080,
            author_attributions: vec![],
            flag_content_uri: Some("https://example.com/flag".try_into().unwrap()),
            google_maps_uri: Some("https://maps.google.com/photo123".try_into().unwrap()),
        };

        let json = serde_json::to_string(&photo).unwrap();
        assert!(json.contains("places/photo123"));
        assert!(json.contains("1920"));
        assert!(json.contains("1080"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "name": "places/photo456",
            "widthPx": 800,
            "heightPx": 600,
            "authorAttributions": [],
            "flagContentUri": null,
            "googleMapsUri": null
        }"#;

        let photo: PhotoInfo = serde_json::from_str(json).unwrap();
        assert_eq!(photo.name, "places/photo456");
        assert_eq!(photo.width_px, 800);
        assert_eq!(photo.height_px, 600);
        assert!(photo.author_attributions.is_empty());
        assert!(photo.flag_content_uri.is_none());
        assert!(photo.google_maps_uri.is_none());
    }

    #[test]
    fn test_constructor() {
        let photo = PhotoInfo::new("places/test", 1024, 768);
        assert_eq!(photo.name, "places/test");
        assert_eq!(photo.width_px, 1024);
        assert_eq!(photo.height_px, 768);
        assert!(photo.author_attributions.is_empty());
    }

    #[test]
    #[allow(clippy::float_cmp, reason = "test only")]
    fn test_aspect_ratio() {
        let photo = PhotoInfo::new("test", 1920, 1080);
        assert!((photo.aspect_ratio() - 16.0/9.0).abs() < 0.01);

        let square = PhotoInfo::new("square", 500, 500);
        assert!((square.aspect_ratio() - 1.0).abs() < 0.01);

        let invalid = PhotoInfo::new("invalid", 100, 0);
        assert_eq!(invalid.aspect_ratio(), 1.0); // Default for invalid
    }

    #[test]
    fn test_orientation() {
        let landscape = PhotoInfo::new("landscape", 1920, 1080);
        assert!(landscape.is_landscape());
        assert!(!landscape.is_portrait());
        assert!(!landscape.is_square());

        let portrait = PhotoInfo::new("portrait", 1080, 1920);
        assert!(!portrait.is_landscape());
        assert!(portrait.is_portrait());
        assert!(!portrait.is_square());

        let square = PhotoInfo::new("square", 500, 500);
        assert!(!square.is_landscape());
        assert!(!square.is_portrait());
        assert!(square.is_square());
    }

    #[test]
    fn test_pixel_count_and_resolution() {
        let hd = PhotoInfo::new("hd", 1920, 1080);
        assert_eq!(hd.pixel_count(), 2_073_600);
        assert!(hd.is_high_resolution());

        let small = PhotoInfo::new("small", 640, 480);
        assert_eq!(small.pixel_count(), 307_200);
        assert!(!small.is_high_resolution());
    }

    #[test]
    fn test_fit_within() {
        let photo = PhotoInfo::new("test", 1920, 1080);
        
        // No scaling needed
        let (w, h) = photo.fit_within(2000, 1200);
        assert_eq!((w, h), (1920, 1080));

        // Width constrained
        let (w, h) = photo.fit_within(960, 1200);
        assert_eq!((w, h), (960, 540));

        // Height constrained
        let (w, h) = photo.fit_within(2000, 540);
        assert_eq!((w, h), (960, 540));
    }

    #[test]
    fn test_validation() {
        let valid = PhotoInfo::new("valid", 800, 600);
        assert!(valid.has_valid_dimensions());

        let invalid_width = PhotoInfo::new("invalid", 0, 600);
        assert!(!invalid_width.has_valid_dimensions());

        let invalid_height = PhotoInfo::new("invalid", 800, 0);
        assert!(!invalid_height.has_valid_dimensions());
    }

    #[test]
    fn test_feature_detection() {
        let mut photo = PhotoInfo::new("test", 800, 600);
        
        assert!(!photo.has_attribution());
        assert!(!photo.has_moderation_links());
        assert!(!photo.has_google_maps_link());

        photo.author_attributions.push(AuthorAttribution::default());
        photo.flag_content_uri = Some("https://example.com/flag".try_into().unwrap());
        photo.google_maps_uri = Some("https://maps.google.com".try_into().unwrap());

        assert!(photo.has_attribution());
        assert!(photo.has_moderation_links());
        assert!(photo.has_google_maps_link());
    }

    #[test]
    fn test_dimension_description() {
        let hd = PhotoInfo::new("hd", 1920, 1080);
        let desc = hd.dimension_description();
        assert!(desc.contains("1920 × 1080"));
        assert!(desc.contains("2.1MP"));
        assert!(desc.contains("landscape"));

        let small = PhotoInfo::new("small", 640, 480);
        let desc = small.dimension_description();
        assert!(desc.contains("640 × 480"));
        assert!(!desc.contains("MP")); // Under 1MP
        assert!(desc.contains("landscape"));

        let square = PhotoInfo::new("square", 500, 500);
        let desc = square.dimension_description();
        assert!(desc.contains("square"));

        let invalid = PhotoInfo::new("invalid", 0, 100);
        assert_eq!(invalid.dimension_description(), "Invalid dimensions");
    }

    #[test]
    fn test_css_aspect_ratio() {
        let hd = PhotoInfo::new("hd", 1920, 1080);
        assert_eq!(hd.css_aspect_ratio(), "16/9");

        let photo = PhotoInfo::new("4:3", 800, 600);
        assert_eq!(photo.css_aspect_ratio(), "4/3");

        let square = PhotoInfo::new("square", 500, 500);
        assert_eq!(square.css_aspect_ratio(), "1/1");

        let invalid = PhotoInfo::new("invalid", 0, 100);
        assert_eq!(invalid.css_aspect_ratio(), "1/1");
    }

    #[test]
    fn test_suggested_thumbnail_size() {
        let landscape = PhotoInfo::new("landscape", 1920, 1080);
        let (w, h) = landscape.suggested_thumbnail_size();
        assert!(w > h); // Should maintain landscape orientation
        assert!(w <= 400 && h <= 200); // Should fit within thumbnail constraints

        let portrait = PhotoInfo::new("portrait", 1080, 1920);
        let (w, h) = portrait.suggested_thumbnail_size();
        assert!(h > w); // Should maintain portrait orientation

        let square = PhotoInfo::new("square", 1000, 1000);
        let (w, h) = square.suggested_thumbnail_size();
        assert_eq!(w, h); // Should remain square
        assert_eq!(w, 200); // Should be exactly the target size
    }
}