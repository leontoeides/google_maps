#![allow(clippy::ref_option)]

use crate::places_new::AuthorAttribution;
use crate::places_new::place_photos::{PhotoRequest, uri::response::Response};
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
pub struct PhotoUri {
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

    /// A short-lived URI that can be used to render the photo.
    ///
    /// This is a Google-hosted URL (typically on `googleusercontent.com`) that directly serves the
    /// image file. The image format (JPEG, PNG, WebP) depends on what was originally uploaded and
    /// can be determined from the `Content-Type` header when fetching the image.
    ///
    /// **Important:** This URI is temporary and may expire or become rate-limited after some time.
    /// It's intended for immediate use - fetch and cache the image data itself, not this URI. The
    /// URI should not be stored long-term or hardcoded.
    ///
    /// Use this URI to:
    /// - Download the image bytes with an HTTP GET request
    /// - Pass to image loading libraries
    /// - Embed in `<img>` tags for web display (with appropriate caching)
    ///
    /// > ⚠️ The URI may have rate limiting or expiration. Always fetch the image promptly after
    /// > receiving this response.
    #[getset(get = "pub")]
    pub uri: url::Url,

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

impl PhotoUri {
    /// Creates a new `PhotoUri` with the specified name and URI.
    #[must_use]
    pub fn new(name: impl Into<String>, uri: Url) -> Self {
        Self {
            name: name.into(),
            uri,
            width_px: None,
            height_px: None,
            author_attributions: Vec::new(),
            flag_content_uri: None,
            google_maps_uri: None,
        }
    }

    /// Creates a new `PhotoUri` with the Google Maps Place Photos (New) response.
    #[must_use]
    pub fn from_response(response: Response, request: Option<PhotoRequest>) -> Self {
        if let Some(request) = request {
            if response.name == request.name {
                Self {
                    name: response.name,
                    uri: response.photo_uri,
                    width_px: request.width_px,
                    height_px: request.height_px,
                    author_attributions: request.author_attributions,
                    flag_content_uri: request.flag_content_uri,
                    google_maps_uri: request.google_maps_uri,
                }
            } else {
                response.into()
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
    pub fn suggested_thumbnail_size(&self, target_thumbnail_size: u32) -> Option<(u32, u32)> {
        if let (
            Some(is_landscape),
            Some(is_portrait)
        ) = (
            self.is_landscape(),
            self.is_portrait()
        ) {
            if is_landscape {
                self.fit_within(target_thumbnail_size * 2, target_thumbnail_size)
            } else if is_portrait {
                self.fit_within(target_thumbnail_size, target_thumbnail_size * 2)
            } else {
                Some((target_thumbnail_size, target_thumbnail_size))
            }
        } else {
            None
        }
    }

    /// Generates an HTML `<img>` tag for this photo at full size.
    ///
    /// Creates a complete image tag with proper attributes including dimensions, alt text from
    /// attributions, and performance optimizations like lazy loading. The image will display at
    /// its original dimensions if available.
    ///
    /// Use this for:
    /// - Embedding photos in web pages
    /// - Email templates
    /// - HTML reports or documentation
    /// - Server-side rendered content
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo_uri = client.place_photos(&photo_info)?.execute().await?;
    /// let html = photo_uri.to_html_img();
    /// // <img src="https://..." width="4000" height="3000"
    /// //      alt="Photo by John Doe" loading="lazy">
    /// ```
    #[must_use]
    pub fn to_html_img(&self) -> String {
        self.to_html_img_with_options(None, true)
    }

    /// Generates an HTML `<img>` tag for this photo as a thumbnail.
    ///
    /// Creates an image tag optimized for thumbnail display using the suggested thumbnail
    /// dimensions. Includes proper attributes and lazy loading for performance.
    ///
    /// Use this for:
    /// - Photo galleries or grids
    /// - Preview images
    /// - List views with photo thumbnails
    /// - Responsive layouts with smaller images
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo_uri = client.place_photos(&photo_info)?.execute().await?;
    /// let html = photo_uri.to_html_img_thumbnail(200);
    /// // <img src="https://..." width="200" height="150"
    /// //      alt="Photo by John Doe" loading="lazy">
    /// ```
    #[must_use]
    pub fn to_html_img_thumbnail(&self, target_thumbnail_size: u32) -> String {
        let thumbnail_size = self.suggested_thumbnail_size(target_thumbnail_size);
        self.to_html_img_with_options(thumbnail_size, true)
    }

    /// Generates an HTML `<img>` tag with custom options.
    ///
    /// Creates an image tag with full control over dimensions and lazy loading behavior. Use this
    /// when you need specific sizing or loading behavior different from the defaults.
    ///
    /// # Parameters
    ///
    /// * `dimensions` - Optional custom dimensions `(width, height)`. If `None`, uses the photo's
    ///   original dimensions. If specified, these dimensions are used in the HTML attributes.
    ///
    /// * `lazy_load` - Whether to include `loading="lazy"` attribute for performance. Set to
    ///   `true` for better page load performance, `false` if the image should load immediately.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo_uri = client.place_photos(&photo_info)?.execute().await?;
    ///
    /// // Custom size, eager loading
    /// let html = photo_uri.to_html_img_with_options(Some((800, 600)), false);
    ///
    /// // Original size, lazy loading
    /// let html = photo_uri.to_html_img_with_options(None, true);
    /// ```
    #[must_use]
    pub fn to_html_img_with_options(
        &self,
        dimensions: Option<(u32, u32)>,
        lazy_load: bool,
    ) -> String {
        let (width, height) = dimensions
            .or_else(|| self.width_px.and_then(|w| self.height_px.map(|h| (w, h))))
            .unwrap_or((800, 600)); // Default fallback

        // Build alt text from attributions
        let alt_text: Option<String> = if self.author_attributions.is_empty() {
            None
        } else {
            let authors: Vec<String> = self
                .author_attributions
                .iter()
                .filter_map(|attr| attr.display_name().as_ref())
                .map(String::from)
                .collect();

            if authors.len() == 1 {
                Some(format!("Photo by {first_author}", first_author = authors[0]))
            } else if authors.len() == 2 {
                Some(format!(
                    "Photo by {first_author} and {second_author}",
                    first_author = authors[0],
                    second_author = authors[1]
                ))
            } else if authors.len() > 2 {
                Some(format!("Photo by {first_author}, {second_author} and {len} others",
                    first_author = authors[0],
                    second_author = authors[1],
                    len = authors.len() - 2
                ))
            } else {
                None
            }
        };

        let lazy = if lazy_load {
            r#" loading="lazy""#
        } else {
            ""
        };

        alt_text.map_or_else(
            || format!(
                r#"<img src="{uri}" width="{width}" height="{height}"{lazy} />"#,
                uri = self.uri
            ),
            |alt_text| {
                // HTML escape the alt text
                let alt_text = html_escape(&alt_text);

                format!(
                    r#"<img src="{uri}" width="{width}" height="{height}" alt="{alt_text}"{lazy} />"#,
                    uri = self.uri
                )
            }
        )
    }

    /// Generates an HTML `<figure>` element with the photo and attribution caption.
    ///
    /// Creates a semantic HTML figure element containing the image and a caption with proper photo
    /// credits. Use this when you need to display photos with visible attribution.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo_uri = client.place_photos(&photo_info)?.execute().await?;
    /// let html = photo_uri.to_html_figure();
    /// // <figure>
    /// //   <img src="https://..." ... />
    /// //   <figcaption>Photo by John Doe</figcaption>
    /// // </figure>
    /// ```
    #[must_use]
    pub fn to_html_figure(&self) -> String {
        self.to_html_figure_with_options(None, true)
    }

    /// Generates an HTML `<figure>` element with custom options.
    ///
    /// Creates a figure element with full control over image dimensions and loading behavior,
    /// including a caption with photo attribution.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo_uri = client.place_photos(&photo_info)?.execute().await?;
    /// let html = photo_uri.to_html_figure_with_options(Some((400, 300)), false);
    /// ```
    #[must_use]
    pub fn to_html_figure_with_options(
        &self,
        dimensions: Option<(u32, u32)>,
        lazy_load: bool,
    ) -> String {
        let img = self.to_html_img_with_options(dimensions, lazy_load);

        let caption: Option<String> = if self.author_attributions.is_empty() {
            None
        } else {
            let authors: Vec<String> = self
                .author_attributions
                .iter()
                .filter_map(|attr| {
                    if let (
                        Some(name),
                        Some(uri)
                    ) = (
                        attr.display_name().as_ref(),
                        attr.uri()
                    ) {
                        Some(format!(r#"<a href="{}">{}</a>"#, uri, html_escape(name)))
                    } else {
                        attr.display_name().as_ref().map(|name| html_escape(name))
                    }
                })
                .collect();

            if authors.len() == 1 {
                Some(format!("Photo by {first_author}", first_author = authors[0]))
            } else if authors.len() == 2 {
                Some(format!(
                    "Photo by {first_author} and {second_author}",
                    first_author = authors[0],
                    second_author = authors[1]
                ))
            } else if authors.len() > 2 {
                Some(format!("Photo by {first_author}, {second_author} and {len} others",
                    first_author = authors[0],
                    second_author = authors[1],
                    len = authors.len() - 2
                ))
            } else {
                None
            }
        };

        caption.map_or_else(
            || format!("<figure>{img}</figure>"),
            |caption| format!("<figure>{img}<figcaption>{caption}</figcaption></figure>")
        )
    }

    /// Downloads the actual image bytes from the photo URI.
    ///
    /// Makes an HTTP GET request to fetch the image data from Google's servers. The image format
    /// (JPEG, PNG, WebP) can be determined from the response's `Content-Type` header.
    ///
    /// Use this when you need to:
    /// - Save the image to disk
    /// - Process the image data
    /// - Cache the image locally
    /// - Display the image without relying on external URLs
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo_uri = client.place_photos(&photo_info)?
    ///     .max_width_px(800)
    ///     .skip_http_redirect(true)
    ///     .execute()
    ///     .await?;
    ///
    /// // Download the actual image
    /// let image_bytes = photo_uri.download_image(&client).await?;
    ///
    /// // Save to file
    /// std::fs::write("photo.jpg", &image_bytes)?;
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The network request fails
    /// - The server returns a non-200 status
    /// - The response cannot be read
    #[cfg(feature = "reqwest")]
    pub async fn download_image(&self, client: &crate::Client) -> Result<Vec<u8>, crate::Error> {
        let response = client
            .reqwest_client
            .get(self.uri.as_str())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(crate::places_new::place_photos::Error::PhotoDownloadFailed {
                status: response.status().as_u16(),
                url: self.uri.to_string(),
                span: (0, self.uri.to_string().len()).into()
            }.into());
        }

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl From<Response> for PhotoUri {
    /// Converts a `Response` into a `PhotoUri`.
    fn from(response: Response) -> Self {
        Self {
            name: response.name,
            uri: response.photo_uri,
            width_px: None,
            height_px: None,
            author_attributions: Vec::default(),
            flag_content_uri: None,
            google_maps_uri: None,
        }
    }
}

impl From<PhotoUri> for String {
    /// Converts a `PhotoUri` into its URL string.
    ///
    /// Consumes the `PhotoUri` and returns the photo URL as a String. Use this when you just need
    /// the URL and don't need the metadata anymore.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo_uri = client.place_photos(&photo_info)?.execute().await?;
    /// let url_string = String::from(photo_uri);
    /// println!("Photo at: {}", url_string);
    /// ```
    fn from(photo_uri: PhotoUri) -> Self {
        photo_uri.uri.to_string()
    }
}

impl From<&PhotoUri> for String {
    /// Converts a reference to `PhotoUri` into its URL string.
    ///
    /// Clones the URL and returns it as a String without consuming the `PhotoUri`. Use this when
    /// you need the URL but want to keep using the `PhotoUri`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo_uri = client.place_photos(&photo_info)?.execute().await?;
    /// let url_string = String::from(&photo_uri);
    ///
    /// // Can still use photo_uri
    /// println!("Dimensions: {:?}", photo_uri.dimension_description());
    /// ```
    fn from(photo_uri: &PhotoUri) -> Self {
        photo_uri.uri.to_string()
    }
}

impl std::fmt::Display for PhotoUri {
    /// Formats the `PhotoUri` as its URL string.
    ///
    /// When printed or formatted, displays the photo URL. This makes it easy to use `PhotoUri` in
    /// string formatting contexts.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo_uri = client.place_photos(&photo_info)?.execute().await?;
    /// println!("Photo URL: {}", photo_uri);
    /// // Photo URL: https://lh3.googleusercontent.com/...
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uri)
    }
}

impl std::ops::Deref for PhotoUri {
    type Target = url::Url;

    /// Dereferences to the underlying URL.
    ///
    /// Allows using `PhotoUri` anywhere a `&Url` is expected without explicit conversion. You can
    /// call any `Url` methods directly on a `PhotoUri`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo_uri = client.place_photos(&photo_info)?.execute().await?;
    ///
    /// // Can use Url methods directly
    /// println!("Scheme: {}", photo_uri.scheme());
    /// println!("Host: {:?}", photo_uri.host_str());
    /// println!("Path: {}", photo_uri.path());
    ///
    /// // Can pass to functions expecting &Url
    /// some_function(&*photo_uri);
    /// ```
    fn deref(&self) -> &Self::Target {
        &self.uri
    }
}

impl AsRef<url::Url> for PhotoUri {
    /// Returns a reference to the underlying URL.
    ///
    /// Allows passing `PhotoUri` to functions that accept `AsRef<Url>` without explicit conversion.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo_uri = client.place_photos(&photo_info)?.execute().await?;
    /// process_url(photo_uri.as_ref());
    /// ```
    fn as_ref(&self) -> &url::Url {
        &self.uri
    }
}

impl AsRef<str> for PhotoUri {
    /// Returns the URL as a string slice.
    ///
    /// Allows passing `PhotoUri` to functions that accept `AsRef<str>` for the URL.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo_uri = client.place_photos(&photo_info)?.execute().await?;
    /// reqwest::get(photo_uri.as_ref()).await?;
    /// ```
    fn as_ref(&self) -> &str {
        self.uri.as_str()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Private Methods

/// HTML-escapes a string to prevent XSS vulnerabilities.
///
/// Replaces special HTML characters with their entity equivalents.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}