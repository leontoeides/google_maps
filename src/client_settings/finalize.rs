use crate::client_settings::ClientSettings;

impl ClientSettings {
    /// Completes the builder pattern into a final structure.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn finalize(&self) -> ClientSettings {
        ClientSettings {
            key: self.key.clone(),
            max_retries: self.max_retries,
            max_backoff: self.max_backoff,
            rate_limit: self.rate_limit.clone(),
        } // ClientSettings
    } // fn
} // impl
