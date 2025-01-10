impl crate::places::place_details::Request<'_> {
    /// Specifies the sort order of user reviews in the Places API _Place
    /// Details_ response.
    ///
    /// ## Arguments
    ///
    /// * `sort_order` ‧ The sorting method to use when returning reviews. Can
    ///   be set to `most_relevant` (default) or newest.
    ///
    ///     * For `most_relevant` (default), reviews are sorted by relevance;
    ///       the service will bias the results to return reviews originally
    ///       written in the preferred language.
    ///
    ///     * For `newest`, reviews are sorted in chronological order; the
    ///       preferred language does not affect the sort order.
    ///
    /// Google recommends that you display how the reviews are being sorted to
    /// the end user.
    #[must_use] pub fn with_reviews_sort(
        mut self,
        sort_order: impl Into<crate::places::place_details::SortOrder>
    ) -> Self {
        // Set sort order setting in Request struct.
        self.reviews_sort = Some(sort_order.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl