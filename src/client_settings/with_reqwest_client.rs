use crate::client_settings::ClientSettings;

impl ClientSettings {

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
    /// let mut google_maps_client = ClientSettings::new("YOUR_API_KEY_HERE")
    ///     .with_reqwest_client(reqwest_client)
    ///     .finalize();
    /// ```

    pub fn with_reqwest_client(
        &mut self,
        reqwest_client: reqwest::Client,
    ) -> &mut ClientSettings {
        self.reqwest_client = Some(reqwest_client);
        self
    } // fn

} // impl