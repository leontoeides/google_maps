use crate::client_settings::ClientSettings;

impl ClientSettings {

    /// Completes the builder pattern into a final structure.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    #[cfg(feature = "enable-reqwest")]
    pub fn finalize(&self) -> ClientSettings {
        ClientSettings {
            key: self.key.clone(),
            rate_limit: self.rate_limit.clone(),
            reqwest_client: self.reqwest_client.clone(),
        } // ClientSettings
    } // fn

    /// Completes the builder pattern into a final structure.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    #[cfg(not(feature = "enable-reqwest"))]
    pub fn finalize(&self) -> ClientSettings {
        ClientSettings {
            key: self.key.clone(),
        } // ClientSettings
    } // fn

} // impl