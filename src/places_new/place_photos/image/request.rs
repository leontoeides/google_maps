#![allow(clippy::ref_option, reason = "for the getset crate")]

use crate::places_new::place_photos::PhotoRequest;

// -------------------------------------------------------------------------------------------------
//
/// Request for the Google Maps Places API (New) Place Photos service that returns photo images.
///
/// Used to fetch photo images from Google's servers, downloading the actual image bytes. The
/// response contains a temporary URL that can be used to display or download the photo.
///
/// You must specify at least one dimension constraint (`max_width_px` or `max_height_px`) to
/// control the image size. Google will scale the image while maintaining aspect ratio.
///
/// # Example
///
/// ```rust,ignore
/// let photo_uri = client.place_photos_uri(&photo_info)?
///     .max_width_px(800)
///     .max_height_px(600)
///     .skip_http_redirect(true)
///     .execute()
///     .await?;
///
/// println!("Photo URL: {}", photo_uri.uri());
/// ```
#[derive(
    //std
    Clone,
    Debug,
    // serde
    serde::Serialize,
    // getset
    getset::Getters,
    getset::CopyGetters,
    getset::MutGetters,
    getset::Setters,
    // other
    bon::Builder
)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_field_names, reason = "disambiguate photo request information from request")]
pub struct Request<'c> {
    /// The Google Maps API client.
    ///
    /// The `Client` structure contains the application's API key and other user-definable settings
    /// such as "maximum retries," and most importantly the
    /// [reqwest](https://crates.io/crates/reqwest) client itself.
    #[serde(skip_deserializing, skip_serializing)]
    pub(crate) client: &'c crate::Client,

    /// Caches the `PhotoRequest`. Used to build the `PhotoImage` when the response is received from
    /// Google Maps.
    #[getset(get = "pub")]
    #[serde(skip_deserializing, skip_serializing)]
    pub(crate) photo_request: PhotoRequest,

    /// The image's maximum width, in pixels.
    ///
    /// * If the image is smaller than the values specified, the original image will be returned.
    ///
    /// * If the image is larger in either dimension, it will be scaled to match the smaller of the
    ///   two dimensions, restricted to its original aspect ratio.
    ///
    /// Both the `max_height_px` and `max_width_px` properties accept an integer between `1` and
    /// `4_800`.
    ///
    /// You must specify either `max_height_px`, or `max_width_px`, or both.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[getset(get_copy = "pub")]
    pub max_width_px: Option<u32>,

    /// The image's maximum height, in pixels.
    ///
    /// * If the image is smaller than the values specified, the original image will be returned.
    ///
    /// * If the image is larger in either dimension, it will be scaled to match the smaller of the
    ///   two dimensions, restricted to its original aspect ratio.
    ///
    /// Both the `max_height_px` and `max_width_px` properties accept an integer between `1` and
    /// `4_800`.
    ///
    /// You must specify either `max_height_px`, or `max_width_px`, or both.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[getset(get_copy = "pub")]
    pub max_height_px: Option<u32>,

    /// If `false` (default), make an HTTP redirect to the image to return the image. If `true`,
    /// skip the redirect and return a JSON response containing the image details.
    ///
    /// When `true`, the response will be a `PhotoImage` struct containing the photo URL. When
    /// `false`, the response will be the actual image bytes (handled by a different endpoint).
    #[builder(default = true)]
    #[getset(get_copy = "pub")]
    pub skip_http_redirect: bool,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

#[cfg(feature = "reqwest")]
impl Request<'_> {
    /// Executes the place photo image request.
    ///
    /// Sends the configured request to the Google Maps API and returns a `PhotoImage` response
    /// containing the photo URL and metadata merged from the original photo request.
    ///
    /// This method is available on both the builder (via `.build().execute()` shorthand) and on
    /// `Request` directly when constructing requests manually.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The request validation fails (missing or invalid dimensions)
    /// - The network request fails
    /// - The API returns an error response
    pub async fn execute(
        self
    ) -> Result<crate::places_new::place_photos::PhotoImage, crate::Error> {
        let bytes = self.client.get_binary_request(&self).await?;
        let response = crate::places_new::place_photos::image::Response(bytes);
        let photo_image = crate::places_new::place_photos::PhotoImage::from_response(
            response,
            Some(self.photo_request)
        );
        Ok(photo_image)
    }
}

#[cfg(feature = "reqwest")]
impl<S: request_builder::State> RequestBuilder<'_, S> {
    /// Executes the place photo image request.
    ///
    /// Builds the request and sends it to the Google Maps API, returning the parsed photo image
    /// response. This method both completes the builder and executes the HTTP request in one step.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The request validation fails (missing or invalid dimensions)
    /// - The network request fails
    /// - The API returns an error response
    pub async fn execute(self) -> Result<crate::places_new::place_photos::PhotoImage, crate::Error>
    where
        S: request_builder::IsComplete,
    {
        let request = self.build();
        let bytes = request.client.get_binary_request(&request).await?;
        let response = crate::places_new::place_photos::image::Response(bytes);
        let photo_image = crate::places_new::place_photos::PhotoImage::from_response(
            response,
            Some(request.photo_request)
        );
        Ok(photo_image)
    }
}

impl crate::client::Client {
    /// Downloads a place photo's image bytes directly from Google's servers.
    ///
    /// The Google Maps Places Place Photos (New) API can return the actual image file rather than
    /// just a URL. This method accepts various input types - photo names, `PhotoInfo` structs, or
    /// entire `Place` objects - and returns a builder for configuring size constraints.
    ///
    /// Unlike `place_photos_uri()` which returns a temporary URL, this method returns the actual
    /// image bytes immediately. Use this when you need the image data right away for processing, or
    /// displaying.
    ///
    /// You must specify at least one dimension (`max_width_px` or `max_height_px`) to control the
    /// image size. Google scales the image while maintaining aspect ratio.
    ///
    /// # What You Get Back
    ///
    /// Returns a `PhotoImage` containing:
    /// - The actual image file bytes (JPEG, PNG, or WebP format)
    /// - Photo metadata (dimensions, attributions, links) if provided
    /// - Helper methods for format detection, size display, and orientation
    ///
    /// # When to Use This vs `place_photos_uri()`
    ///
    /// **Use `place_photos_image()` when:**
    /// - You're processing the image (resizing, filtering, etc.)
    /// - You want to cache the actual image data
    /// - You need offline access to the image
    ///
    /// **Use `place_photos_uri()` when:**
    /// - You just need a URL for an `<img>` tag
    /// - You're passing the URL to a frontend/mobile app
    /// - You want the client to handle the download
    /// - You need multiple sizes of the same photo
    ///
    /// # Input Types Accepted
    ///
    /// * **Photo name strings** - `"places/ChIJ.../photos/ABC"` - When you just have the name
    /// * **`PhotoInfo` references** - `&photo_info` - Preserves all metadata (dimensions, attributions)
    /// * **`Place` references** - `&place` - Automatically uses the first photo from the place
    ///
    /// # Examples
    ///
    /// **Download and save images for furtherance & display from nearby search:**
    ///
    /// ```rust,ignore
    /// // Find nearby restaurants and download their photos
    /// let places = client.nearby_search((37.7749, -122.4194, 5000.0))?
    ///     .field_mask(FieldMask::specific([Field::PlacesPhotos, Field::PlacesDisplayName]))
    ///     .included_primary_types(vec![PlaceType::Restaurant])
    ///     .execute()
    ///     .await?;
    ///
    /// for place in places {
    ///     // Download the actual image bytes
    ///     let photo = client.place_photos_image(&place)?
    ///         .max_width_px(1024)
    ///         .execute()
    ///         .await?;
    ///
    ///     // Save to disk
    ///     let filename = format!("{}.jpg", place.display_text().unwrap());
    ///     std::fs::write(&filename, photo.bytes())?;
    ///     println!("Saved {} ({} bytes)", filename, photo.size_bytes());
    /// }
    /// ```
    ///
    /// **Process the image bytes:**
    ///
    /// ```rust,ignore
    /// let photo = client.place_photos_image(&place)?
    ///     .max_width_px(1024)
    ///     .execute()
    ///     .await?;
    ///
    /// // Use with image processing library
    /// let img = image::load_from_memory(&photo)?;
    /// let thumbnail = img.thumbnail(200, 200);
    /// thumbnail.save("thumbnail.jpg")?;
    ///
    /// // Or just access the bytes directly
    /// let bytes: &[u8] = &photo; // Via Deref
    /// let bytes: &[u8] = photo.as_ref(); // Via AsRef
    /// ```
    ///
    /// **From a photo name string:**
    ///
    /// ```rust,ignore
    /// // When you just have the name from storage or cache
    /// let photo = client.place_photos_image("places/ChIJ.../photos/ABC")?
    ///     .max_width_px(400)
    ///     .execute()
    ///     .await?;
    ///
    /// std::fs::write("photo.jpg", &photo)?;
    /// ```
    ///
    /// # Dimension Constraints
    ///
    /// You must specify at least one dimension:
    ///
    /// ```rust,ignore
    /// // Width only - height scales proportionally
    /// .max_width_px(800)
    ///
    /// // Height only - width scales proportionally
    /// .max_height_px(600)
    ///
    /// // Both - scales to fit within box, maintaining aspect ratio
    /// .max_width_px(800)
    /// .max_height_px(600)
    /// ```
    ///
    /// Valid range: 1-4800 pixels.
    ///
    /// # Important Notes
    ///
    /// * **Photo names expire** - Always get fresh photo names from recent Place Details, Nearby
    ///   Search, or Text Search responses. Never cache or hardcode photo names.
    ///
    /// * **Field mask required** - When fetching places, include `photos` in your field mask or
    ///   you won't get any photo data.
    ///
    /// * **Bandwidth usage** - This downloads the full image immediately. For large images or many
    ///   photos, consider using `place_photos_uri()` instead to get URLs that clients can download
    ///   on-demand.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The place has no photos (when passing a `Place`)
    /// - The photo name is invalid or expired
    /// - Dimension constraints are missing or out of range (1-4800)
    /// - Network request fails
    /// - API returns an error response
    pub fn place_photos_image<P>(
        &self,
        photo_request: P,
    ) -> Result<
        RequestBuilder<
            '_,
            crate::places_new::place_photos::image::request::request_builder::SetSkipHttpRedirect<
                crate::places_new::place_photos::image::request::request_builder::SetPhotoRequest<
                    crate::places_new::place_photos::image::request::request_builder::SetClient
                >
            >
        >,
        crate::Error,
    >
    where
        P: TryInto<PhotoRequest>,
        P::Error: Into<crate::Error>,
    {
        let photo_request = photo_request
            .try_into()
            .map_err(Into::into)?;

        Ok(Request::builder()
            .client(self)
            .photo_request(photo_request)
            .skip_http_redirect(false))
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

#[cfg(feature = "reqwest")]
use crate::request_rate::api::Api;

/// Defines the Google Maps Places API HTTP endpoint for requests.
///
/// This trait returns information needed to make HTTP GET requests to the Place Photos API
/// endpoint. The photo name is appended to the base service URL to construct the full endpoint.
impl crate::traits::EndPoint for &Request<'_> {
    fn service_url() -> &'static str {
        ""
    }

    fn output_format() -> std::option::Option<&'static str> {
        None
    }

    #[cfg(feature = "reqwest")]
    fn title() -> &'static str {
        "Places API (New) Place Photo Image"
    }

    #[cfg(feature = "reqwest")]
    fn apis() -> &'static [Api] {
        &[Api::All, Api::PlacesNew, Api::PlacePhoto]
    }
}

#[cfg(feature = "reqwest")]
impl crate::traits::RequestBody for &Request<'_> {
    /// Returns an empty request body.
    ///
    /// The Place Photos API uses GET requests with all parameters in the URL path and query
    /// string, so no request body is needed.
    ///
    /// # Errors
    ///
    /// This method never returns an error.
    fn request_body(&self) -> Result<String, crate::Error> {
        Ok(String::new())
    }
}

#[cfg(feature = "reqwest")]
impl crate::traits::QueryString for &Request<'_> {
    /// Builds the URL query string for the HTTP request.
    ///
    /// Constructs query parameters for dimension constraints and redirect behavior. At least one
    /// of `maxWidthPx` or `maxHeightPx` must be specified.
    ///
    /// # Example Query String
    ///
    /// ```text
    /// maxWidthPx=800&maxHeightPx=600&skipHttpRedirect=true
    /// ```
    fn query_string(&self) -> String {
        let base_url = format!(
            "https://places.googleapis.com/v1/{name}/media?key={key}",
            name = self.photo_request().name,
            key = self.client.key
        );

        let mut parameters = Vec::with_capacity(3);

        if let Some(width) = self.max_width_px {
            parameters.push(format!("maxWidthPx={width}"));
        }

        if let Some(height) = self.max_height_px {
            parameters.push(format!("maxHeightPx={height}"));
        }

        if self.skip_http_redirect {
            parameters.push("skipHttpRedirect=true".to_string());
        }

        if parameters.is_empty() {
            base_url
        } else {
            format!("{base_url}&{query_string}", query_string = parameters.join("&"))
        }
    }
}

#[cfg(feature = "reqwest")]
impl crate::traits::RequestHeaders for &Request<'_> {
    /// Returns a map of HTTP header names to values.
    ///
    /// These headers will be added to the HTTP request alongside the standard headers like
    /// `X-Goog-Api-Key`.
    fn request_headers(&self) -> reqwest::header::HeaderMap {
        reqwest::header::HeaderMap::new()
    }

    /// Returns whether the `X-Goog-Api-Key` header should be set for this request.
    ///
    /// The Place Photos API requires the API key in the query string, not as a header.
    fn send_x_goog_api_key() -> bool {
        false
    }
}

#[cfg(feature = "reqwest")]
impl crate::traits::Validatable for &Request<'_> {
    /// Validates the request parameters before sending to Google.
    ///
    /// Checks that:
    /// - At least one of `max_width_px` or `max_height_px` is specified
    /// - Both dimensions (if specified) are within the valid range of 1-4800 pixels
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    fn validate(&self) -> Result<(), crate::Error> {
        // At least one dimension must be specified
        if self.max_width_px.is_none() && self.max_height_px.is_none() {
            let debug = "PhotoRequest { max_width_px: None, max_height_px: None }".to_string();

            return Err(crate::places_new::place_photos::Error::MissingPhotoDimensions {
                span: (0..debug.len()).into(),
                debug,
            }.into());
        }

        // Validate max_width_px range if present
        if let Some(width) = self.max_width_px {
            if !(1..=4800).contains(&width) {
                let debug = format!("max_width_px: {width}");

                return Err(crate::places_new::place_photos::Error::InvalidPhotoWidth {
                    width,
                    span: (0..debug.len()).into(),
                    debug,
                }.into());
            }
        }

        // Validate max_height_px range if present
        if let Some(height) = self.max_height_px {
            if !(1..=4800).contains(&height) {
                let debug = format!("max_height_px: {height}");

                return Err(crate::places_new::place_photos::Error::InvalidPhotoHeight {
                    height,
                    span: (0..debug.len()).into(),
                    debug,
                }.into());
            }
        }

        Ok(())
    }
}