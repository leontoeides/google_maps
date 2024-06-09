use crate::client::GoogleMapsClient;

// =============================================================================

impl GoogleMapsClient {
    // -------------------------------------------------------------------------
    //
    /// Passes a user configured reqwest client for the Google Maps client to
    /// use. This allows the you to have more control over the how the Google
    /// Maps client connects to the Google Maps server.
    ///
    /// [Mause](https://github.com/Mause) mentioned that this feature could be
    /// useful for writing tests. Thanks for the suggestion!
    ///
    /// ## Arguments
    ///
    /// * `reqwest_client` ‧ A reqwest client built using the
    /// `reqwest::Client::builder()` function.
    ///
    /// ## Examples:
    ///
    /// ```rust
    /// let reqwest_client = reqwest::Client::builder()
    ///     .user_agent("My Cool App v1.0")
    ///     .build()?;
    ///
    /// let mut google_maps_client = GoogleMapsClient::new("YOUR_API_KEY_HERE")
    ///     .with_reqwest_client(reqwest_client)
    ///     .build();
    /// ```

    #[cfg(all(feature = "enable-reqwest", feature = "enable-reqwest-middleware"))]
    pub fn with_reqwest_client(&mut self, reqwest_client: reqwest::Client) -> &mut Self {
        self.reqwest_client = crate::reqwest_maybe_middleware::Client::Vanilla(reqwest_client);
        self
    } // fn

    #[cfg(all(feature = "enable-reqwest", not(feature = "enable-reqwest-middleware")))]
    pub fn with_reqwest_client(&mut self, reqwest_client: reqwest::Client) -> &mut Self {
        self.reqwest_client = reqwest_client;
        self
    } // fn

    #[cfg(all(feature = "enable-reqwest", feature = "enable-reqwest-middleware"))]
    pub fn with_reqwest_middleware_client(
        &mut self,
        reqwest_client: reqwest_middleware::ClientWithMiddleware
    ) -> &mut Self {
        self.reqwest_client = crate::reqwest_maybe_middleware::Client::Middleware(reqwest_client);
        self
    } // fn

    #[cfg(all(feature = "enable-reqwest", feature = "enable-reqwest-middleware"))]
    pub fn with_reqwest(
        &mut self,
        reqwest_client: crate::reqwest_maybe_middleware::Client
    ) -> &mut Self {
        self.reqwest_client = reqwest_client;
        self
    } // fn
} // impl
