use crate::{
    client_settings::ClientSettings,
    request_rate::RequestRate,
}; // use

impl ClientSettings {

    /// Initialize the settings needed for a Google Cloud Maps API transaction.
    pub fn new(key: &str) -> ClientSettings {
        ClientSettings {
            key: String::from(key),
            max_retries: 20,
            max_backoff: 32000,
            rate_limit: RequestRate::default(),
        } // ClientSettings
    } // fn

} // impl