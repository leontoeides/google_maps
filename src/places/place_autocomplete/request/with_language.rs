use crate::places::place_autocomplete::request::Request;
use crate::types::Language;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Adds the language parameter to the Place API _Place Autocomplete_ query.
    ///
    /// ## Arguments
    ///
    /// * `language` ‧ The language in which to return results.
    ///
    ///     * See the list of supported languages. Google often updates the
    ///       supported languages, so this list may not be exhaustive.
    ///
    ///     * If `language` is not supplied, the API attempts to use the
    ///       preferred language as specified in the `Accept-Language` header.
    ///
    ///     * The API does its best to provide a street address that is readable
    ///       for both the user and locals. To achieve that goal, it returns
    ///       street addresses in the local language, transliterated to a script
    ///       readable by the user if necessary, observing the preferred language.
    ///       All other addresses are returned in the preferred language. Address
    ///       components are all returned in the same language, which is chosen
    ///       from the first component.
    ///
    ///     * If a name is not available in the preferred language, the API uses
    ///       the closest match.
    ///
    ///     * The preferred language has a small influence on the set of results
    ///       that the API chooses to return, and the order in which they are
    ///       returned. The geocoder interprets abbreviations differently
    ///       depending on language, such as the abbreviations for street types,
    ///       or synonyms that may be valid in one language but not in another.
    ///       For example, _utca_ and _tér_ are synonyms for street in Hungarian.

    pub fn with_language(
        &'a mut self,
        language: impl Into<Language>
    ) -> &'a mut Self {
        // Set language in Request struct.
        self.language = Some(language.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
