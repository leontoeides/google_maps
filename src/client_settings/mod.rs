//! Contains the `ClientSettings` struct and its associated traits.
//! `ClientSettings` is used to set your API key and the settings for governing
//! your requests.

mod finalize;
mod new;
mod with_max_delay;
mod with_max_retries;
mod with_rate_limit;

use crate::request_rate::RequestRate;

/// Contains information used for authenticating with Google and other settings.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ClientSettings {
    /// Your application's API key. This key identifies your application for
    /// purposes of quota management. Learn how to [get a
    /// key](https://developers.google.com/maps/documentation/geocoding/get-api-key).
    /// Contains the application's API key and other settings.
    pub key: String,
    /// Certain network & server-side errors may be successful if retried with
    /// the same payload. This parameter sets the maximum number of times the
    /// client should retry before giving up.
    pub max_retries: u8,
    /// It's okay to continue retrying once you reach the max_backoff time.
    /// Retries after this point do not need to continue increasing backoff
    /// time. For example, if a client uses an max_backoff time of 64
    /// seconds, then after reaching this value, the client can retry every 64
    /// seconds. At some point, clients should be prevented from retrying
    /// infinitely.
    ///
    /// How long clients should wait between retries and how many times they
    /// should retry depends on your use case and network conditions. For
    /// example, mobile clients of an application may need to retry more times
    /// and for longer intervals when compared to desktop clients of the same
    /// application.
    pub max_backoff: u32,
    /// Rate limits for each of the Google Cloud Maps Platform APIs.
    pub rate_limit: RequestRate,
} // struct