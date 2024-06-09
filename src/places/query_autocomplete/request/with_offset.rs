use crate::places::query_autocomplete::request::Request;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Adds the offset parameter to the Place API _Query Autocomplete_ query.
    ///
    /// ## Arguments
    ///
    /// * `offset` ‧ The position, in the input term, of the last character that
    ///   the service uses to match predictions. For example, if the input is
    ///   `Google` and the offset is 3, the service will match on `Goo`. The
    ///   string determined by the offset is matched against the first word in
    ///   the input term only. For example, if the input term is `Google abc`
    ///   and the offset is 3, the service will attempt to match against
    ///   `Goo abc`. If no offset is supplied, the service will use the whole
    ///   term. The offset should generally be set to the position of the text
    ///   caret.

    pub fn with_offset(
        &'a mut self,
        offset: impl Into<u8>
    ) -> &'a mut Self {
        // Set offset in Request struct.
        self.offset = Some(offset.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
