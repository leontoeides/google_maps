use crate::client_settings::ClientSettings;

impl ClientSettings {

    /// Sets the maximum number of retries upon a series of request failures.
    ///
    /// ## Arguments
    ///
    /// * `max_retries` ‧ The maximum number of request retries.
    ///
    /// ## Example:
    ///
    /// * Sets the maximum number of retries to 10:
    /// ```rust
    /// .with_max_retries(10)
    /// ```

    pub fn with_max_retries(&mut self, max_retries: u8) -> &mut ClientSettings {
        self.max_retries = max_retries;
        self
    } // fn

} // impl