use crate::places_new::Place;
use crate::places_new::text_search::{Response, Request};

// -------------------------------------------------------------------------------------------------
//
/// Response from the Google Maps Places API (New) Text Search service. Includes the original
/// request as context for session continuation.
///
/// Contains the text search results from the API along with the original request used to generate
/// them. This enables easy session continuation where subsequent requests reuse the same page token
/// and parameters.
///
/// # `ResponseWithContext` vs. `ResponseWithContext`
///
/// * `ResponseWithContext` - Just the API data. For simple queries or when request context isn't needed.
/// * `ResponseWithContext` - Includes original request. Preserves pagination tokens, session state
///   for follow-up requests.
#[derive(
    //std
    Clone,
    Debug,
    // serde
    serde::Serialize,
    // getset
    getset::Getters,
    getset::MutGetters,
    getset::Setters,
)]
pub struct ResponseWithContext {
    /// The text search results returned by the API.
    ///
    /// Contains the list of text search results matching the input.
    #[getset(get = "pub")]
    pub response: Response,

    /// The original request used to generate this response.
    ///
    /// Store this to maintain session continuity across multiple text search pagination calls. The
    /// page token and all other parameters are preserved for the next request.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub request: Request,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl ResponseWithContext {
    /// Creates a new response with its originating request context.
    ///
    /// Pairs the API response with the request that generated it, enabling session continuation and
    /// parameter reuse for subsequent calls.
    #[must_use]
    pub const fn new(response: Response, request: Request) -> Self {
        Self { response, request }
    }

    /// Consumes `self` and returns the response and request as a tuple:
    /// * `0` · Google Maps API's response
    /// * `1` · The original request (or context)
    ///
    /// Useful when you need owned access to both components, such as when moving them into
    /// different data structures or passing them to functions that take ownership.
    #[must_use]
    pub fn into_parts(self) -> (Response, Request) {
        (self.response, self.request)
    }

    /// Consumes `self` and returns just the response.
    ///
    /// Use this when you're done with session continuation and only need the final text search
    /// results.
    #[must_use]
    pub fn into_response(self) -> Response {
        self.response
    }

    /// Consumes `self` and returns just the request.
    ///
    /// Use this when you want to continue the session but don't need the current response data
    /// anymore.
    #[must_use]
    pub fn into_request(self) -> Request {
        self.request
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::ops::Deref for ResponseWithContext {
    type Target = Response;

    /// Dereferences to the inner response.
    ///
    /// Allows transparent access to response fields and methods without explicitly accessing the
    /// `response` field.
    fn deref(&self) -> &Self::Target {
        &self.response
    }
}

impl std::ops::DerefMut for ResponseWithContext {
    /// Mutably dereferences to the inner response.
    ///
    /// Allows transparent mutable access to response fields without explicitly accessing the
    /// `response` field.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.response
    }
}

impl std::ops::Index<usize> for ResponseWithContext {
    type Output = Place;

    /// Indexes into the places list.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn index(&self, index: usize) -> &Self::Output {
        &self.response.places[index]
    }
}

impl std::ops::IndexMut<usize> for ResponseWithContext {
    /// Mutably indexes into the places list.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.response.places[index]
    }
}

impl AsRef<[Place]> for ResponseWithContext {
    /// Returns the places as a slice.
    ///
    /// Allows passing the response to functions expecting `&[Place]`.
    fn as_ref(&self) -> &[Place] {
        &self.response.places
    }
}

impl AsMut<[Place]> for ResponseWithContext {
    /// Returns the places as a mutable slice.
    ///
    /// Allows passing the response to functions expecting `&mut [Place]`.
    fn as_mut(&mut self) -> &mut [Place] {
        &mut self.response.places
    }
}

impl IntoIterator for ResponseWithContext {
    type Item = Place;
    type IntoIter = std::vec::IntoIter<Place>;

    /// Converts the response into an iterator over places.
    ///
    /// Consumes the response and allows iteration over the owned places.
    fn into_iter(self) -> Self::IntoIter {
        self.response.places.into_iter()
    }
}

impl<'a> IntoIterator for &'a ResponseWithContext {
    type Item = &'a Place;
    type IntoIter = std::slice::Iter<'a, Place>;

    /// Creates an iterator over borrowed places.
    ///
    /// Allows iteration over the places without consuming the response.
    fn into_iter(self) -> Self::IntoIter {
        self.response.places.iter()
    }
}

impl<'a> IntoIterator for &'a mut ResponseWithContext {
    type Item = &'a mut Place;
    type IntoIter = std::slice::IterMut<'a, Place>;

    /// Creates a mutable iterator over places.
    ///
    /// Allows mutable iteration over the places without consuming the response.
    fn into_iter(self) -> Self::IntoIter {
        self.response.places.iter_mut()
    }
}

impl ResponseWithContext {
    /// Returns an iterator over the places.
    ///
    /// This is equivalent to calling `response.places.iter()` or `(&response).into_iter()`.
    pub fn iter(&self) -> std::slice::Iter<'_, Place> {
        self.response.places.iter()
    }

    /// Returns a mutable iterator over the places.
    ///
    /// This is equivalent to calling `response.places.iter_mut()`.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Place> {
        self.response.places.iter_mut()
    }
}