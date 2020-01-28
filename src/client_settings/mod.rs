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
    /// It's okay to continue retrying once you reach the maximum_backoff time.
    /// Retries after this point do not need to continue increasing backoff
    /// time. For example, if a client uses an maximum_backoff time of 64
    /// seconds, then after reaching this value, the client can retry every 64
    /// seconds. At some point, clients should be prevented from retrying
    /// infinitely.
    ///
    /// How long clients should wait between retries and how many times they
    /// should retry depends on your use case and network conditions. For
    /// example, mobile clients of an application may need to retry more times
    /// and for longer intervals when compared to desktop clients of the same
    /// application.
    pub maximum_backoff: u32,
} // struct

impl ClientSettings {

    /// Initialize the settings needed for a Google Cloud Maps API transaction.
    pub fn new(key: &str) -> ClientSettings {
        ClientSettings {
            key: String::from(key),
            maximum_retries: 20,
            maximum_backoff: 32000,
        } // ClientSettings
    } // fn

    pub fn with_maximum_retries(&mut self, maximum_retries: u8) -> &mut ClientSettings {
        self.maximum_retries = maximum_retries;
        self
    } // fn

    pub fn with_maximum_backoff(&mut self, maximum_backoff: u32) -> &mut ClientSettings {
        self.maximum_backoff = maximum_backoff;
        self
    } // fn

} // impl