#![allow(clippy::ref_option, reason = "for the getset crate")]

use crate::places_new::Place;

// -------------------------------------------------------------------------------------------------
//
/// Response from the Google Maps Places API (New) Text Search service.
///
/// Contains a list of places matching the search query and an optional token for retrieving the
/// next page of results. The list of places is not guaranteed to be consistent across identical
/// requests.
///
/// # `Response` vs. `ResponseWithContext`
///
/// * `Response` - Just the API data. For simple queries or when request context isn't needed.
/// * `ResponseWithContext` - Includes original request. Preserves pagination tokens, session state
///   for follow-up requests.
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
pub struct Response {
    /// Places matching the search query.
    ///
    /// Each place contains the fields specified in the request's field mask. The list may be empty
    /// if no places match the query.
    #[serde(default)]
    #[getset(get = "pub")]
    pub(crate) places: Vec<Place>,

    /// Token for accessing the next page of results.
    ///
    /// Present when there are more results available beyond the current page. Pass this value as
    /// `page_token` in a subsequent request to retrieve the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[getset(get = "pub")]
    pub(crate) next_page_token: Option<String>,

    /// Error returned from the Google Maps API server.
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
        Self { places, next_page_token: None, error: None }
    }

    /// Creates an empty `Response` with no places.
    ///
    /// Use this to represent cases where no places were found or when initializing before an API
    /// call.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            places: Vec::new(),
            next_page_token: None,
            error: None
        }
    }

    /// Returns the number of places in the response.
    ///
    /// The total count of both place and query predictions combined.
    #[must_use]
    pub fn len(&self) -> usize {
        self.places.len()
    }

    /// Checks if the response contains no places.
    ///
    /// Returns `true` if no place or query predictions were returned, indicating that no relevant
    /// results were found for the input.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.places.is_empty()
    }

    /// Returns whether additional pages of results are available.
    ///
    /// When a text search returns more than 20 results, Google provides a pagination token to
    /// retrieve the next page. This method checks if such a token exists, indicating that more
    /// results can be fetched using `next_text_search()`.
    ///
    /// Use this before calling `next_text_search()` to avoid errors from attempting to paginate
    /// past the last page of results. Note that page tokens expire after a short time, so
    /// pagination should be done promptly.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use google_maps::places_new::Field;
    ///
    /// let response = google_maps_client
    ///     .text_search("restaurants in Tokyo")
    ///     .field_mask([Field::PlacesDisplayName, Field::NextPageToken])
    ///     .execute()
    ///     .await?;
    ///
    /// if response.has_next_page() {
    ///     let next_page = google_maps_client
    ///         .next_text_search(response)
    ///         .await?;
    /// }
    /// ```
    ///
    /// Collecting all pages:
    ///
    /// ```rust,no_run
    /// use google_maps::places_new::Field;
    ///
    /// let mut all_places = Vec::new();
    /// let mut response = google_maps_client
    ///     .text_search("coffee shops")
    ///     .field_mask([Field::PlacesDisplayName, Field::NextPageToken])
    ///     .execute()
    ///     .await?;
    ///
    /// all_places.extend(response.places.clone());
    ///
    /// while response.has_next_page() {
    ///     response = google_maps_client.next_text_search(response).await?;
    ///     all_places.extend(response.places.clone());
    /// }
    ///
    /// println!("Found {} total places", all_places.len());
    /// ```
    #[must_use]
    pub const fn has_next_page(&self) -> bool {
        self.next_page_token.is_some()
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