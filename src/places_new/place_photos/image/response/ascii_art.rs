use crate::places_new::place_photos::PhotoImage;
use image::DynamicImage;
use std::num::NonZeroU32;

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl PhotoImage {
/// Loads the photo bytes into a dynamic image representation.
    ///
    /// Decodes the raw image bytes into a `DynamicImage`, which represents a matrix of pixels
    /// convertible to and from RGBA format. This allows for further image processing, manipulation,
    /// or analysis using the `image` crate's API.
    ///
    /// Useful when you need the actual pixel data rather than ASCII art, such as for:
    /// - Custom image transformations (resizing, filtering, etc.)
    /// - Pixel-level analysis
    /// - Integration with other libraries that expect `DynamicImage`
    ///
    /// More variants adhering to RGBA conversion principles may be added in the future for
    /// additional image format support.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo = client.place_photos_image(&place)?.execute().await?;
    /// let img = photo.dynamic_image()?;
    ///
    /// // Example: Get image dimensions
    /// println!("Width: {}, Height: {}", img.width(), img.height());
    ///
    /// // Example: Convert to grayscale
    /// let gray = img.grayscale();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the image bytes cannot be decoded (invalid image format).
    pub fn dynamic_image(self) -> Result<DynamicImage, crate::Error> {
        self.try_into()
    }
    /// Displays the photo as colored ANSI art in the terminal.
    ///
    /// Converts the image bytes into ANSI art suitable for terminal display. The output uses ANSI
    /// truecolor escape codes for colored display (falls back to 16-color ANSI on terminals that
    /// don't support truecolor).
    ///
    /// This creates colored terminal art, not pure ASCII art - it uses ANSI color codes!
    ///
    /// For true ASCII art (characters only, no colors), use `display_ascii()`.
    ///
    /// # Parameters
    ///
    /// * `width` - Target width in characters. Height is calculated automatically to maintain
    ///   aspect ratio. Common values: 80 (standard terminal), 120 (wide), 40 (small preview).
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Download a photo and display it as ASCII art
    /// let photo = client.place_photos_image(&place)?
    /// .max_width_px(1024)
    /// .execute()
    /// .await?;
    ///
    /// // Display in terminal (80 chars wide)
    /// println!("{}", photo.display_ansi(std::num::NonZeroU32::new(80).unwrap())?);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The image bytes cannot be decoded (invalid image format)
    /// - The image conversion fails
    pub fn display_ansi(
        self,
        width: NonZeroU32
    ) -> Result<String, crate::Error> {
        self.display_with_options(
            width,
            true, // colored
            None, // use default charset
        )
    }
    /// Displays the photo as pure ASCII art (grayscale, no colors).
    ///
    /// Uses character density for shading without any ANSI color codes. This is true ASCII art -
    /// just characters, no escape codes. Works in any terminal, even ones that don't support
    /// colors.
    ///
    /// Similar to `display_ansi()` but without colors. Useful for:
    /// - Terminals that don't support color
    /// - Situations where color might be distracting
    /// - Retro/minimalist aesthetics
    /// - Logging or text-based outputs
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo = client.place_photos_image(&place)?.execute().await?;
    /// println!("{}", photo.display_ascii(std::num::NonZeroU32::new(80).unwrap())?);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the image cannot be decoded or converted.
    pub fn display_ascii(
        self,
        width: NonZeroU32
    ) -> Result<String, crate::Error> {
        self.display_with_options(
            width,
            false, // grayscale
            None, // use default charset
        )
    }
    /// Displays the photo as ASCII art with custom options.
    ///
    /// Provides full control over the ASCII art conversion including colors, character sets, and
    /// sizing. Use this for advanced customization.
    ///
    /// # Parameters
    ///
    /// * `width` - Target width in characters
    /// * `colored` - Whether to use ANSI color codes (truecolor if supported, 16-color fallback)
    /// * `charset` - Optional custom character set from transparent to opaque. If `None`, uses
    ///   the default charset optimized for photo conversion.
    ///
    /// # Character Sets
    ///
    /// The charset should be ordered from transparent/lightest to opaque/darkest. Examples:
    /// - Default (good for photos): ` .:-=+*#%@`
    /// - Dense: ` ░▒▓█`
    /// - Block: ` ▖▗▘▝▀▄▐▌▞▚▟▙▛▜▀▄█`
    /// - Simple: ` .,;!#@`
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo = client.place_photos_image(&place)?.execute().await?;
    ///
    /// // Custom block characters, colored
    /// let ascii = photo.display_with_options(
    /// std::num::NonZeroU32::new(100).unwrap(),
    /// true,
    /// Some(" ░▒▓█"),
    /// )?;
    /// println!("{}", ascii);
    ///
    /// // Dense ASCII, grayscale
    /// let ascii = photo.display_with_options(
    /// std::num::NonZeroU32::new(80).unwrap(),
    /// false,
    /// Some(" .:-=+*#%@"),
    /// )?;
    /// println!("{}", ascii);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the image cannot be decoded or converted.
    pub fn display_with_options(
        self,
        width: NonZeroU32,
        colored: bool,
        charset: Option<&str>,
    ) -> Result<String, crate::Error> {
        // Load image from bytes using the image crate
        let img = self.dynamic_image()?;

        // Build config for artem
        let mut config = artem::config::ConfigBuilder::new();
        let mut config = config.target_size(width);

        // Set color mode
        config = if colored {
            config.color(true)
        } else {
            config.color(false)
        };

        // Set custom charset if provided
        if let Some(chars) = charset {
            config = config.characters(chars.to_string());
        }

        // Convert image to ASCII art
        let ascii_art = artem::convert(img, &config.build());

        Ok(ascii_art)
    }
    /// Returns a small ASCII art thumbnail preview.
    ///
    /// Generates a compact colored ASCII representation suitable for quick previews or lists.
    /// Always uses color and a width of 40 characters.
    ///
    /// Perfect for:
    /// - Quick photo previews in terminal UIs
    /// - Listing multiple photos compactly
    /// - Thumbnails in CLI tools
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Show thumbnails of all restaurant photos
    /// for place in places {
    /// let photo = client.place_photos_image(&place)?.execute().await?;
    /// println!("📷 {}", place.display_name());
    /// println!("{}", photo.display_ascii_thumbnail()?);
    /// println!();
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the image cannot be decoded or converted.
    #[allow(clippy::missing_panics_doc, reason = "guaranteed not to panic")]
    pub fn display_ascii_thumbnail(self) -> Result<String, crate::Error> {
        self.display_ansi(std::num::NonZeroU32::new(40).unwrap())
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl TryFrom<PhotoImage> for DynamicImage {
    type Error = crate::Error;

    /// Attempts to convert a `PhotoImage` into a `DynamicImage`.
    ///
    /// This consumes the `PhotoImage` and decodes its bytes into a `DynamicImage`.
    ///
    /// # Errors
    ///
    /// Returns an error if the image bytes cannot be decoded (invalid image format).
    fn try_from(value: PhotoImage) -> Result<Self, Self::Error> {
        image::load_from_memory(&value.bytes)
            .map_err(|error| crate::Error::from(crate::places_new::place_photos::Error::ImageDecodeError {
                message: error.to_string(),
            }))
    }
}

impl std::fmt::Display for PhotoImage {
    /// Formats the photo as ASCII art when printed.
    ///
    /// Uses default settings: 80 characters wide, colored. This allows you to simply `println!`
    /// a photo and see ASCII art.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let photo = client.place_photos_image(&place)?.execute().await?;
    /// println!("{}", photo); // Automatically displays as ASCII art!
    /// ```
    ///
    /// Note: If the image cannot be converted, displays an error message instead.
    #[allow(clippy::missing_panics_doc, reason = "guaranteed not to panic")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.clone().display_ascii(NonZeroU32::new(80).unwrap()) {
            Ok(ascii) => write!(f, "{ascii}"),
            Err(e) => write!(f, "[Error displaying ASCII art: {e}]"),
        }
    }
}