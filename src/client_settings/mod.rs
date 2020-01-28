//! Contains the `ClientSettings` struct and its associated traits.
//! `ClientSettings` is used to set your API key and settings for governing
//! your requests.

use serde::{Serialize, Deserialize};

/// Contains information used for authenticating with Google and other settings.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ClientSettings {
    /// Your application's API key. This key identifies your application for
    /// purposes of quota management. Learn how to [get a
    /// key](https://developers.google.com/maps/documentation/geocoding/get-api-key).
    /// Contains the application's API key and other settings.
    pub key: String,
    /// Certain network & server-side errors may be successful if retried with
    /// the same payload. This parameter sets the maximum number of times the
    /// client should retry before giving up.
    pub maximum_retries: u8,
} // struct

impl ClientSettings {

    /// Initialize the settings needed for a Google Cloud Maps API transaction.
    pub fn new(key: &str, maximum_retries: u8) -> ClientSettings {
        ClientSettings {
            key: String::from(key),
            maximum_retries,
        } // ClientSettings
    } // fn

} // impl