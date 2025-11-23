#![allow(clippy::ref_option, reason = "for the getset crate")]

use crate::places_new::Place;

// -------------------------------------------------------------------------------------------------
//
/// Response from the Google Maps Places API (New) Place Details service.
#[derive(
    // std
    Clone,
    Debug,
    // serde
    serde::Deserialize,
    serde::Serialize,
    // getset
    getset::Getters,
    getset::MutGetters,
    getset::Setters,
)]
#[serde(rename_all = "camelCase")]
pub struct Response(Place);

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Response {
    /// Creates a new `Response`.
    #[must_use]
    pub const fn new(place: Place) -> Self {
        Self(place)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::ops::Deref for Response {
    type Target = Place;

    /// Dereferences to the underlying places vector.
    ///
    /// Allows using the response directly as a `Place` for operations like iteration,
    /// indexing, and length checks.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Response {
    /// Mutably dereferences to the underlying places vector.
    ///
    /// Allows mutable access to the places list.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<Place> for Response {
    /// Returns the places as a slice.
    ///
    /// Allows passing the response to functions expecting `&[Place]`.
    fn as_ref(&self) -> &Place {
        &self.0
    }
}

impl AsMut<Place> for Response {
    /// Returns the places as a mutable slice.
    ///
    /// Allows passing the response to functions expecting `&mut [Place]`.
    fn as_mut(&mut self) -> &mut Place {
        &mut self.0
    }
}

impl std::convert::From<Response> for Result<Response, crate::Error> {
    /// Converts a Google Maps API `Response` into a `Result<Response, Error>` by examining the
    /// `status` field inside of the response.
    ///
    /// If the status indicates a success, then an `Ok(response)` will be returned. If the status
    /// indicates an error, then an `Err(error)` will be returned.
    fn from(response: Response) -> Self {
        Ok(response)
    }
}