use crate::client_settings::ClientSettings;
use std::time::Duration;

impl ClientSettings {

    /// Sets the maximum delay between request retries upon consecutive
    /// failures.
    ///
    /// ## Arguments
    ///
    /// * `max_delay` ‧ The maximum delay between request retries.
    ///
    /// ## Description
    ///
    /// Client will continue retrying once you the `max_delay` time is reached.
    /// Retries after this point will not continue increasing backoff time. For
    /// example, if a client uses an `max_delay` time of _64_ seconds, then
    /// after reaching this value, the client can retry every _64_ seconds.
    ///
    /// How long clients should wait between retries and how many times they
    /// should retry depends on your use case and network conditions. For
    /// example, mobile clients of an application may need to retry more times
    /// and for longer intervals when compared to desktop clients of the same
    /// application.
    ///
    /// ## Example:
    ///
    /// * Sets the maximum delay between request retries to 32 seconds:
    /// ```rust
    /// .with_max_delay(Duration::from_secs(32))
    /// ```

    pub fn with_max_delay(&mut self, max_delay: Duration) -> &mut ClientSettings {
        self.max_backoff = max_delay.as_millis() as u32;
        self
    } // fn

} // impl