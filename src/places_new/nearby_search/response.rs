#![allow(clippy::ref_option, reason = "for the getset crate")]

use crate::places_new::Place;

// -------------------------------------------------------------------------------------------------
//
/// Response from the Google Maps Places API (New) Nearby Search service.
///
/// Contains a list of places matching the search criteria. The list of places is not guaranteed to
/// be consistent across identical requests due to factors like place data updates and
/// ranking variations.
///
/// This struct can be used like a `Vec<Place>` through `Deref`, allowing natural iteration and
/// indexing: `response[0]`, `response.iter()`, etc.
#[derive(
    // std
    Clone,
    Debug,
    Default,
    // serde
    serde::Deserialize,
    serde::Serialize,
    // getset
    getset::Getters,
    getset::MutGetters,
    getset::Setters,
)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Places matching the search criteria.
    ///
    /// Each place contains the fields specified in the request's field mask. The list may be empty
    /// if no places match the criteria or if the search area contains no relevant places.
    #[serde(default)]
    #[getset(get = "pub")]
    places: Vec<Place>,

    /// Error returned from the Google Maps API server.
    ///
    /// When present, indicates the request failed. The places list may be empty or incomplete when
    /// an error occurs.
    #[serde(default)]
    #[getset(get = "pub")]
    error: Option<crate::places_new::GoogleApiError>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Response {
    /// Creates a new `Response` with places.
    ///
    /// Use this when constructing responses from API data or for testing.
    #[must_use]
    pub const fn new(places: Vec<Place>) -> Self {
        Self {
            places,
            error: None,
        }
    }

    /// Creates an empty `Response` with no places.
    ///
    /// Use this to represent cases where no places were found or when initializing before an API
    /// call.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            places: Vec::new(),
            error: None,
        }
    }

    /// Returns the number of places in the response.
    ///
    /// Returns 0 if no places matched the search criteria.
    #[must_use]
    pub fn len(&self) -> usize {
        self.places.len()
    }

    /// Checks if the response contains no places.
    ///
    /// Returns `true` if no places were returned, which could mean either no places matched the
    /// criteria or an error occurred. Check the `error` field to distinguish between these cases.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.places.is_empty()
    }

    /// Checks if the response contains an error.
    ///
    /// Returns `true` if the API returned an error. When an error is present, the places list may
    /// be incomplete or empty.
    #[must_use]
    pub const fn has_error(&self) -> bool {
        self.error.is_some()
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::ops::Deref for Response {
    type Target = Vec<Place>;

    /// Dereferences to the underlying places vector.
    ///
    /// Allows using the response directly as a `Vec<Place>` for operations like iteration,
    /// indexing, and length checks.
    fn deref(&self) -> &Self::Target {
        &self.places
    }
}

impl std::ops::DerefMut for Response {
    /// Mutably dereferences to the underlying places vector.
    ///
    /// Allows mutable access to the places list.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.places
    }
}

impl std::ops::Index<usize> for Response {
    type Output = Place;

    /// Indexes into the places list.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn index(&self, index: usize) -> &Self::Output {
        &self.places[index]
    }
}

impl std::ops::IndexMut<usize> for Response {
    /// Mutably indexes into the places list.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.places[index]
    }
}

impl AsRef<[Place]> for Response {
    /// Returns the places as a slice.
    ///
    /// Allows passing the response to functions expecting `&[Place]`.
    fn as_ref(&self) -> &[Place] {
        &self.places
    }
}

impl AsMut<[Place]> for Response {
    /// Returns the places as a mutable slice.
    ///
    /// Allows passing the response to functions expecting `&mut [Place]`.
    fn as_mut(&mut self) -> &mut [Place] {
        &mut self.places
    }
}

impl IntoIterator for Response {
    type Item = Place;
    type IntoIter = std::vec::IntoIter<Place>;

    /// Converts the response into an iterator over places.
    ///
    /// Consumes the response and allows iteration over the owned places.
    fn into_iter(self) -> Self::IntoIter {
        self.places.into_iter()
    }
}

impl<'a> IntoIterator for &'a Response {
    type Item = &'a Place;
    type IntoIter = std::slice::Iter<'a, Place>;

    /// Creates an iterator over borrowed places.
    ///
    /// Allows iteration over the places without consuming the response.
    fn into_iter(self) -> Self::IntoIter {
        self.places.iter()
    }
}

impl<'a> IntoIterator for &'a mut Response {
    type Item = &'a mut Place;
    type IntoIter = std::slice::IterMut<'a, Place>;

    /// Creates a mutable iterator over places.
    ///
    /// Allows mutable iteration over the places without consuming the response.
    fn into_iter(self) -> Self::IntoIter {
        self.places.iter_mut()
    }
}

impl Response {
    /// Returns an iterator over the places.
    ///
    /// This is equivalent to calling `response.places.iter()` or `(&response).into_iter()`.
    pub fn iter(&self) -> std::slice::Iter<'_, Place> {
        self.places.iter()
    }

    /// Returns a mutable iterator over the places.
    ///
    /// This is equivalent to calling `response.places.iter_mut()`.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Place> {
        self.places.iter_mut()
    }
}

impl std::convert::From<Response> for Result<Response, crate::Error> {
    /// Converts a Google Maps API `Response` into a `Result<Response, Error>` by examining the
    /// `status` field inside of the response.
    ///
    /// If the status indicates a success, then an `Ok(response)` will be returned. If the status
    /// indicates an error, then an `Err(error)` will be returned.
    fn from(response: Response) -> Self {
        match response.error {
            Some(error) => Err(crate::places_new::Error::from(error).into()),
            None => Ok(response),
        }
    }
}