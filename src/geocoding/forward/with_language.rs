use crate::{geocoding::forward::ForwardRequest, types::Language};

impl<'a> ForwardRequest<'a> {
    /// Specifies the language in which to return results.
    ///
    /// ## Arguments
    ///
    /// * `language` - The language in which to return results.
    ///
    /// ## Description
    ///
    /// * See the [list of supported
    ///   languages](https://developers.google.com/maps/faq#languagesupport).
    ///   Google often updates the supported languages, so this list may not be
    ///   exhaustive.
    ///
    /// * If `language` is not supplied, the geocoder attempts to use the
    ///   preferred language as specified in the `Accept-Language` header, or
    ///   the native language of the domain from which the request is sent.
    ///
    /// * The geocoder does its best to provide a street address that is
    ///   readable for both the user and locals. To achieve that goal, it
    ///   returns street addresses in the local language, transliterated to a
    ///   script readable by the user if necessary, observing the preferred
    ///   language. All other addresses are returned in the preferred language.
    ///   Address components are all returned in the same language, which is
    ///   chosen from the first component.
    ///
    /// * If a name is not available in the preferred language, the geocoder
    ///   uses the closest match.
    ///
    /// * The preferred language has a small influence on the set of results
    ///   that the API chooses to return, and the order in which they are
    ///   returned. The geocoder interprets abbreviations differently depending
    ///   on language, such as the abbreviations for street types, or synonyms
    ///   that may be valid in one language but not in another. For example,
    ///   _utca_ and _tér_ are synonyms for street and square respectively in
    ///   Hungarian.
    ///
    /// ## Example
    ///
    /// * Set language for result:
    /// ```rust
    /// .with_language(Language::French)
    /// ```

    pub fn with_language(
        &'a mut self,
        language: impl Into<Language>
    ) -> &'a mut Self {
        // Set language in ForwardRequest struct.
        self.language = Some(language.into());
        // Return modified ForwardRequest struct to caller.
        self
    } // fn
} // impl
