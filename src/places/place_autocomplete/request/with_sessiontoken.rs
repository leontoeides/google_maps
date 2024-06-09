use crate::places::place_autocomplete::request::Request;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Adds the session token parameter to the Place API _Place Autocomplete_
    /// query.
    ///
    /// ## Arguments
    ///
    /// * `sessiontoken` ‧ A random string which identifies an autocomplete
    ///   [session](https://developers.google.com/maps/documentation/places/web-service/details#session_tokens)
    ///   for billing purposes.
    ///
    /// The session begins when the user starts typing a query, and concludes
    /// when they select a place and a call to Place Details is made. Each
    /// session can have multiple queries, followed by one place selection. The
    /// API key(s) used for each request within a session must belong to the
    /// same Google Cloud Console project. Once a session has concluded, the
    /// token is no longer valid; your app must generate a fresh token for each
    /// session. If the `sessiontoken` parameter is omitted, or if you reuse a
    /// session token, the session is charged as if no session token was
    /// provided (each request is billed separately).
    ///
    /// We recommend the following guidelines:
    ///
    /// * Use session tokens for all autocomplete sessions.
    ///
    /// * Generate a fresh token for each session. Using a version 4 UUID is
    ///   recommended.
    ///
    /// * Ensure that the API key(s) used for all Place Autocomplete and Place
    ///   Details requests within a session belong to the same Cloud Console
    ///   project.
    ///
    /// * Be sure to pass a unique session token for each new session. Using the
    ///   same token for more than one session will result in each request being
    ///   billed individually.

    pub fn with_sessiontoken(
        &'a mut self,
        sessiontoken: impl Into<String>
    ) -> &'a mut Self {
        // Set session token in Request struct.
        self.sessiontoken = Some(sessiontoken.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
