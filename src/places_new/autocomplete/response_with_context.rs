use crate::places_new::autocomplete::{Response, Request, Suggestion};

// -------------------------------------------------------------------------------------------------
//
/// Response from the Google Maps Places API (New) Autocomplete service. Includes the original
/// request as context for session continuation.
///
/// Contains the autocomplete suggestions from the API along with the original request used to
/// generate them. This enables easy session continuation where subsequent requests reuse the same
/// session token and parameters, which is important for Google's billing model and relevance
/// scoring.
///
/// # `Response` vs. `ResponseWithContext`
///
/// * `Response` - Just the API data. For simple queries or when request context isn't needed.
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
    /// The autocomplete suggestions returned by the API.
    ///
    /// Contains the list of place predictions matching the input, along with metadata like match
    /// highlights and structured formatting.
    #[getset(get = "pub")]
    pub response: Response,

    /// The original request used to generate this response.
    ///
    /// Store this to maintain session continuity across multiple autocomplete calls. The session
    /// token and all other parameters (location bias, language, etc.) are preserved for the next
    /// request.
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

    /// Continue the session with new input.
    ///
    /// Creates a new request that reuses the same session token and all other parameters from the
    /// original request, only updating the input text. This maintains session continuity for
    /// Google's billing and ensures consistent behavior across the interaction.
    #[must_use]
    pub fn continue_with(self, new_input: impl Into<String>) -> Request {
        let mut next_request = self.request;
        next_request.set_input(new_input.into());
        next_request
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
    /// Use this when you're done with session continuation and only need the final results.
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
    type Output = Suggestion;

    /// Indexes into the suggestions list.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn index(&self, index: usize) -> &Self::Output {
        &self.response.suggestions[index]
    }
}

impl std::ops::IndexMut<usize> for ResponseWithContext {
    /// Mutably indexes into the suggestions list.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.response.suggestions[index]
    }
}

impl AsRef<[Suggestion]> for ResponseWithContext {
    /// Returns the suggestions as a slice.
    ///
    /// Allows passing the response to functions expecting `&[Suggestion]`.
    fn as_ref(&self) -> &[Suggestion] {
        &self.response.suggestions
    }
}

impl AsMut<[Suggestion]> for ResponseWithContext {
    /// Returns the suggestions as a mutable slice.
    ///
    /// Allows passing the response to functions expecting `&mut [Suggestion]`.
    fn as_mut(&mut self) -> &mut [Suggestion] {
        &mut self.response.suggestions
    }
}

impl IntoIterator for ResponseWithContext {
    type Item = Suggestion;
    type IntoIter = std::vec::IntoIter<Suggestion>;

    /// Converts the response into an iterator over suggestions.
    ///
    /// Consumes the response and allows iteration over the owned suggestions.
    fn into_iter(self) -> Self::IntoIter {
        self.response.suggestions.into_iter()
    }
}

impl<'a> IntoIterator for &'a ResponseWithContext {
    type Item = &'a Suggestion;
    type IntoIter = std::slice::Iter<'a, Suggestion>;

    /// Creates an iterator over borrowed suggestions.
    ///
    /// Allows iteration over the suggestions without consuming the response.
    fn into_iter(self) -> Self::IntoIter {
        self.response.suggestions.iter()
    }
}

impl<'a> IntoIterator for &'a mut ResponseWithContext {
    type Item = &'a mut Suggestion;
    type IntoIter = std::slice::IterMut<'a, Suggestion>;

    /// Creates a mutable iterator over suggestions.
    ///
    /// Allows mutable iteration over the suggestions without consuming the response.
    fn into_iter(self) -> Self::IntoIter {
        self.response.suggestions.iter_mut()
    }
}

impl ResponseWithContext {
    /// Returns an iterator over the suggestions.
    ///
    /// This is equivalent to calling `response.suggestions.iter()` or `(&response).into_iter()`.
    pub fn iter(&self) -> std::slice::Iter<'_, Suggestion> {
        self.response.suggestions.iter()
    }

    /// Returns a mutable iterator over the suggestions.
    ///
    /// This is equivalent to calling `response.suggestions.iter_mut()`.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Suggestion> {
        self.response.suggestions.iter_mut()
    }
}