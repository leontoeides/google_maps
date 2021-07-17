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
            rate_limit: self.rate_limit.clone(),
            reqwest_client: self.reqwest_client.clone(),
        } // ClientSettings
    } // fn

} // impl