#![allow(clippy::ref_option)]

// Imports

use crate::places_new::AuthorAttribution;
use crate::places_new::place_photos::image::Response;
use crate::places_new::place_photos::PhotoRequest;
use url::Url;

// -------------------------------------------------------------------------------------------------
//
/// URI to the photo of a place.
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
pub struct PhotoImage {
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
    pub name: Option<String>,

    /// The actual image file bytes.
    ///
    /// Contains the complete image data in its original format (JPEG, PNG, or WebP). These bytes
    /// can be:
    /// - Written directly to a file
    /// - Passed to image processing libraries
    /// - Sent over network connections
    /// - Cached in local storage
    ///
    /// The image format can be determined by checking the first few bytes (magic numbers) or by
    /// using image detection libraries like the `image` crate.
    ///
    /// Access via `Deref` or `AsRef<[u8]>` allows using this struct anywhere `&[u8]` is expected.
    #[getset(get = "pub")]
    pub bytes: Vec<u8>,

    /// The image's width, in pixels.
    ///
    /// The maximum width that this photo can be displayed at while maintaining quality. Used to
    /// determine appropriate display sizes and avoid upscaling beyond the original photo
    /// resolution.
    #[getset(get_copy = "pub")]
    pub width_px: Option<u32>,

    /// The image's height, in pixels.
    ///
    /// The maximum height that this photo can be displayed at while maintaining quality. Used to
    /// determine appropriate display sizes and avoid upscaling beyond the original photo
    /// resolution.
    #[getset(get_copy = "pub")]
    pub height_px: Option<u32>,

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

impl PhotoImage {
    /// Creates a new `PhotoImage` with just image bytes.
    ///
    /// Creates a minimal photo image containing only the raw bytes. All metadata fields are set to
    /// `None` or empty. Use this when you have raw image data but no associated metadata.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let bytes = std::fs::read("photo.jpg")?;
    /// let photo = PhotoImage::new(bytes);
    /// ```
    #[must_use]
    pub const fn new(bytes: Vec<u8>) -> Self {
        Self {
            name: None,
            bytes,
            width_px: None,
            height_px: None,
            author_attributions: Vec::new(),
            flag_content_uri: None,
            google_maps_uri: None,
        }
    }

    /// Creates a `PhotoImage` from an API response, optionally preserving metadata.
    ///
    /// Combines the image bytes from the API response with metadata from the original
    /// `PhotoRequest`. This preserves information about dimensions, attributions, and links that
    /// were present in the photo request.
    ///
    /// If no `PhotoRequest` is provided, creates a minimal `PhotoImage` with only the bytes.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo = PhotoImage::from_response(response, Some(photo_request));
    /// // Metadata is preserved!
    /// println!("Dimensions: {:?}", photo.dimension_description());
    /// ```
    #[must_use]
    pub fn from_response(response: Response, request: Option<PhotoRequest>) -> Self {
        if let Some(request) = request {
            Self {
                name: Some(request.name),
                bytes: response.0,
                width_px: request.width_px,
                height_px: request.height_px,
                author_attributions: request.author_attributions,
                flag_content_uri: request.flag_content_uri,
                google_maps_uri: request.google_maps_uri,
            }
        } else {
            response.into()
        }
    }

    /// Returns the aspect ratio of the photo (width / height).
    ///
    /// Used for layout calculations, responsive design, and determining appropriate display
    /// containers. Helps maintain photo proportions when resizing.
    #[must_use]
    pub fn aspect_ratio(&self) -> Option<f64> {
        if let (Some(width_px), Some(height_px)) = (self.width_px, self.height_px) {
            if height_px > 0 {
                Some(f64::from(width_px) / f64::from(height_px))
            } else {
                Some(1.0) // Default to square if height is invalid
            }
        } else {
            None
        }
    }

    /// Returns whether this is a landscape-oriented photo.
    ///
    /// Used for layout decisions and responsive design, helping applications choose appropriate
    /// display styles for horizontally-oriented images.
    #[must_use]
    pub const fn is_landscape(&self) -> Option<bool> {
        if let (Some(width_px), Some(height_px)) = (self.width_px, self.height_px) {
            Some(width_px > height_px)
        } else {
            None
        }
    }

    /// Returns whether this is a portrait-oriented photo.
    ///
    /// Used for layout decisions and responsive design, helping applications choose appropriate
    /// display styles for vertically-oriented images.
    #[must_use]
    pub const fn is_portrait(&self) -> Option<bool> {
        if let (Some(width_px), Some(height_px)) = (self.width_px, self.height_px) {
            Some(height_px > width_px)
        } else {
            None
        }
    }

    /// Returns whether this is a square photo.
    ///
    /// Used to identify photos with equal dimensions, which may have special layout considerations
    /// or display treatments in user interfaces.
    #[must_use]
    pub const fn is_square(&self) -> Option<bool> {
        if let (Some(width_px), Some(height_px)) = (self.width_px, self.height_px) {
            Some(width_px == height_px)
        } else {
            None
        }
    }

    /// Returns the total pixel count of the photo.
    ///
    /// Used to assess photo quality, determine storage requirements, or compare relative photo
    /// sizes when multiple photos are available.
    #[must_use]
    pub fn pixel_count(&self) -> Option<i64> {
        if let (Some(width_px), Some(height_px)) = (self.width_px, self.height_px) {
            Some(i64::from(width_px) * i64::from(height_px))
        } else {
            None
        }
    }

    /// Returns whether this photo has high resolution (> 1 megapixel).
    ///
    /// Used to categorize photo quality and determine whether the photo is suitable for
    /// high-quality displays or printing applications.
    #[must_use]
    pub fn is_high_resolution(&self) -> Option<bool> {
        self.pixel_count().map(|pixel_count| pixel_count > 1_000_000)
    }

    /// Calculates dimensions that fit within the specified maximum size while preserving aspect
    /// ratio.
    ///
    /// Used for responsive image display, thumbnail generation, and ensuring photos fit within
    /// specific UI constraints while maintaining their original proportions.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn fit_within(&self, max_width: u32, max_height: u32) -> Option<(u32, u32)> {
        if let (
            Some(width_px),
            Some(height_px),
            Some(aspect_ratio)
        ) = (
            self.width_px,
            self.height_px,
            self.aspect_ratio()
        ) {
            if width_px <= max_width && height_px <= max_height {
                return Some((width_px, height_px));
            }

            let width_constrained = (f64::from(max_height) * aspect_ratio) as u32;
            let height_constrained = (f64::from(max_width) / aspect_ratio) as u32;

            if width_constrained <= max_width {
                Some((width_constrained, max_height))
            } else {
                Some((max_width, height_constrained))
            }
        } else {
            None
        }
    }

    /// Returns whether the photo has valid dimensions.
    ///
    /// Used to validate photo metadata before displaying or processing images, ensuring both width
    /// and height are positive values.
    #[must_use]
    pub const fn has_valid_dimensions(&self) -> Option<bool> {
        if let (Some(width_px), Some(height_px)) = (self.width_px, self.height_px) {
            Some(width_px > 0 && height_px > 0)
        } else {
            None
        }
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
    pub fn dimension_description(&self) -> Option<String> {
        if let (
            Some(width_px),
            Some(height_px),
            Some(has_valid_dimensions),
            Some(pixel_count),
            Some(is_landscape),
            Some(is_portrait)
        ) = (
            self.width_px,
            self.height_px,
            self.has_valid_dimensions(),
            self.pixel_count(),
            self.is_landscape(),
            self.is_portrait()
        ) {
            if has_valid_dimensions {
                let megapixels = pixel_count as f64 / 1_000_000.0;

                let orientation = if is_landscape {
                    "landscape"
                } else if is_portrait {
                    "portrait"
                } else {
                    "square"
                };

                if megapixels >= 1.0 {
                    Some(format!("{width_px} × {height_px} ({megapixels:.1}MP, {orientation})"))
                } else {
                    Some(format!("{width_px} × {height_px} ({orientation})"))
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns a CSS-compatible aspect ratio string.
    ///
    /// Provides aspect ratio in CSS format (e.g., "16/9", "4/3") for use in web applications,
    /// responsive design, and CSS aspect-ratio properties.
    #[must_use]
    pub fn css_aspect_ratio(&self) -> Option<String> {
        if let (
            Some(width_px),
            Some(height_px),
            Some(has_valid_dimensions)
        ) = (
            self.width_px,
            self.height_px,
            self.has_valid_dimensions()
        ) {
            if has_valid_dimensions {
                // Find GCD to simplify the ratio
                fn gcd(a: u32, b: u32) -> u32 {
                    if b == 0 { a } else { gcd(b, a % b) }
                }

                let divisor = gcd(width_px, height_px);
                let simplified_width = width_px / divisor;
                let simplified_height = height_px / divisor;

                Some(format!("{simplified_width}/{simplified_height}"))
            } else {
                Some("1/1".to_string())
            }
        } else {
            None
        }
    }

    /// Estimates the appropriate thumbnail size for this photo.
    ///
    /// Returns suggested thumbnail dimensions based on the photo's aspect ratio and orientation,
    /// useful for generating consistent thumbnail displays.
    #[must_use]
    pub fn suggested_thumbnail_size(&self, thumbnail_target_size: u32) -> Option<(u32, u32)> {
        if let (
            Some(is_landscape),
            Some(is_portrait)
        ) = (
            self.is_landscape(),
            self.is_portrait()
        ) {
            if is_landscape {
                self.fit_within(thumbnail_target_size * 2, thumbnail_target_size)
            } else if is_portrait {
                self.fit_within(thumbnail_target_size, thumbnail_target_size * 2)
            } else {
                Some((thumbnail_target_size, thumbnail_target_size))
            }
        } else {
            None
        }
    }

    /// Returns the size of the image data in bytes.
    ///
    /// Useful for determining storage requirements, bandwidth usage, or comparing relative file
    /// sizes of different photos.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo = client.place_photo_image(&place)?.execute().await?;
    /// println!("Downloaded {} KB", photo.size_bytes() / 1024);
    /// ```
    #[must_use]
    pub fn size_bytes(&self) -> usize {
        self.bytes.len()
    }

    /// Returns the size of the image in a human-readable format.
    ///
    /// Formats the byte size with appropriate units (KB, MB) for display in user interfaces.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo = client.place_photo_image(&place)?.execute().await?;
    /// println!("File size: {}", photo.display_size()); // "245.3 KB"
    /// ```
    #[must_use]
    pub fn display_size(&self) -> String {
        let size = self.size_bytes() as f64;

        if size < 1024.0 {
            format!("{size} bytes")
        } else if size < 1024.0 * 1024.0 {
            format!("{:.1} KB", size / 1024.0)
        } else {
            format!("{:.1} MB", size / (1024.0 * 1024.0))
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl From<Response> for PhotoImage {
    /// Converts an API `Response` into a `PhotoImage`.
    ///
    /// Creates a minimal photo image with just the image bytes. All metadata fields are `None` or
    /// empty since no `PhotoRequest` metadata is available to preserve.
    fn from(response: Response) -> Self {
        Self {
            name: None,
            bytes: response.0,
            width_px: None,
            height_px: None,
            author_attributions: Vec::default(),
            flag_content_uri: None,
            google_maps_uri: None,
        }
    }
}

impl std::ops::Deref for PhotoImage {
    type Target = Vec<u8>;

    /// Dereferences to the underlying image bytes.
    ///
    /// Allows using `PhotoImage` anywhere a `&Vec<u8>` or `&[u8]` is expected. You can call
    /// slice methods directly on a `PhotoImage` or pass it to functions expecting byte slices.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo = client.place_photo_image(&place)?.execute().await?;
    ///
    /// // Can use Vec/slice methods directly
    /// println!("First byte: {}", photo[0]);
    /// println!("Length: {}", photo.len());
    ///
    /// // Can pass to functions expecting &[u8]
    /// std::fs::write("photo.jpg", &*photo)?;
    /// ```
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl AsRef<[u8]> for PhotoImage {
    /// Returns a reference to the underlying bytes.
    ///
    /// Allows passing `PhotoImage` to any function that accepts `AsRef<[u8]>`, such as file I/O
    /// operations or image processing libraries.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo = client.place_photo_image(&place)?.execute().await?;
    /// std::fs::write("photo.jpg", photo.as_ref())?;
    /// image::load_from_memory(photo.as_ref())?;
    /// ```
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}