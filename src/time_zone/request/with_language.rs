use crate::time_zone::request::Request;
use crate::types::Language;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Adds the language parameter to the Time Zone API query.
    ///
    /// ## Arguments
    ///
    /// * `language` ‧ The language that Google's response should be presented
    ///   in.
    ///
    /// ## Example
    ///
    /// * Set Google's response to the French language:
    /// ```rust
    /// .with_language(Language::French)
    /// ```

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
