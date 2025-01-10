//! Contains the `GoogleMapsClient` struct and its associated traits. Use its
//! implemented methods to set your _Google API key_ and other settings such as:
//! _rate limiting_, _maxium retries_, & _retry delay times_ for your requests.

// -----------------------------------------------------------------------------

mod build;
mod impls;

#[cfg(feature = "reqwest")]
mod get_request;

#[cfg(feature = "reqwest")]
mod post_request;

#[cfg(feature = "reqwest")]
mod with_rate;

#[cfg(feature = "reqwest")]
mod with_reqwest_client;

// -----------------------------------------------------------------------------

#[cfg(feature = "reqwest")]
use crate::request_rate::RequestRate;

// -----------------------------------------------------------------------------
//
/// Use the `GoogleMapsClient` struct's implemented methods to set your _Google
/// API key_ and other settings such as: _rate limiting_, _maxium retries_, &
/// _retry delay times_ for your requests.
///
/// This structure contains your API key - the preferred way for authenticating
/// with the Google Maps Platform APIs, your request rate limit settings, and
/// your automatic retry settings.
///
/// How to use this structure's methods in a builder pattern:
///
/// ```rust
/// let mut my_settings = google_maps::Client::try_new(YOUR_GOOGLE_API_KEY_HERE)
///     .with_max_delay(std::time::Duration::from_secs(32))
///     .with_max_retries(10)
///     .with_rate(&Api::All, 1, std::time::Duration::from_secs(2))
///     .build();
/// ```

#[derive(Clone, Debug)]
pub struct Client {
    /// Your application's API key. This key identifies your application for
    /// purposes of quota management. Learn how to [get a
    /// key](https://developers.google.com/maps/documentation/geocoding/get-api-key).
    pub key: String,

    /// Rate limits for each of the Google Cloud Maps Platform APIs.
    #[cfg(feature = "reqwest")]
    pub rate_limit: RequestRate,

    /// Allows you to optionally provide your own pre-configured reqwest client
    /// that will be used by the Google Maps client.
    #[cfg(all(feature = "reqwest", feature = "reqwest-middleware"))]
    pub reqwest_client: crate::reqwest_maybe_middleware::Client,

    /// Allows you to optionally provide your own pre-configured reqwest client
    /// that will be used by the Google Maps client.
    #[cfg(all(feature = "reqwest", not(feature = "reqwest-middleware")))]
    pub reqwest_client: reqwest::Client,
} // struct
