use crate::client_settings::ClientSettings;
use crate::rate_limit::RateLimit;

impl ClientSettings {

    /// Initialize the settings needed for a Google Cloud Maps API transaction.
    pub fn new(key: &str) -> ClientSettings {
        ClientSettings {
            key: String::from(key),
            max_retries: 20,
            max_backoff: 32000,
            rate_limit: RateLimit {
                all: None,
                directions: None,
                distance_matrix: None,
                elevation: None,
                geocoding: None,
                time_zone: None,
            } // RateLimit
        } // ClientSettings
    } // fn

} // impl