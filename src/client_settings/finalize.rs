use crate::client_settings::ClientSettings;

impl ClientSettings {

    /// Finalizes the builder pattern into a non-referential structure.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn finalize(&self) -> ClientSettings {
        ClientSettings {
            key: self.key.clone(),
            max_retries: self.max_retries,
            max_backoff: self.max_backoff,
            rate_limiting: self.rate_limiting.clone(),
        } // ClientSettings
    } // fn

} // impl