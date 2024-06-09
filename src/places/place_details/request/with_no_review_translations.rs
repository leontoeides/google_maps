use crate::places::place_details::request::Request;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Specifies whether Google should provide translations of user reviews in
    /// the Places API _Place Details_ response.
    ///
    /// ## Arguments
    ///
    /// * `no_translations` ‧ Specify `true` to disable translation of reviews;
    ///   specify `false` to enable translation of reviews. Reviews are returned
    ///   in their original language.
    ///
    /// If omitted, or passed with no value, translation of reviews is enabled.
    /// If the `language` parameter was specified in the request, use the
    /// specified language as the preferred language for translation. If
    /// `language` is omitted, the API attempts to use the `Accept-Language`
    /// header as the preferred language.

    pub fn with_no_review_translations(
        &'a mut self,
        no_translations: impl Into<bool>
    ) -> &'a mut Self {
        // Set translations setting in Request struct.
        self.reviews_no_translations = Some(no_translations.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
