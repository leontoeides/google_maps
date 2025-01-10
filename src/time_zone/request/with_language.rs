impl crate::time_zone::Request<'_> {
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
    #[must_use] pub fn with_language(
        mut self,
        language: impl Into<crate::types::Language>
    ) -> Self {
        // Set language in Request struct.
        self.language = Some(language.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl