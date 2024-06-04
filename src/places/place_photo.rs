//! A photo of a Place. The photo can be accesed via the
/// [Place Photo](https://developers.google.com/places/web-service/photos) API
/// using a URL.
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// A photo of a Place. The photo can be accesed via the
/// [Place Photo](https://developers.google.com/places/web-service/photos) API
/// using an url in the following pattern:
///
/// ```
/// https://maps.googleapis.com/maps/api/place/photo?maxwidth=400&photo_reference=photo_reference&key=YOUR_API_KEY
/// ```
///
/// See [Place Photos](https://developers.google.com/places/web-service/photos) for more information.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PlacePhoto {
    /// The height of the photo.
    pub height: u16,
    /// The HTML attributions for the photo.
    #[serde(default)]
    pub html_attributions: Vec<String>,
    /// A string used to identify the photo when you perform a Photo request.
    pub photo_reference: String,
    /// The width of the photo.
    pub width: u16,
} // struct PlacePhoto

// -----------------------------------------------------------------------------

impl std::str::FromStr for PlacePhoto {
    type Err = serde_json::error::Error;
    /// Parse a Google Maps Places API JSON response into a usable
    /// `PlacePhoto` struct.
    fn from_str(s: &str) -> Result<Self, serde_json::error::Error> {
        serde_json::from_str(s)
    } // fn from_str
} // impl FromStr
