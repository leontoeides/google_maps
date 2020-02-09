use crate::{
    client_settings::ClientSettings,
    request_rate::api::Api,
}; // use
use std::time::Duration;

impl ClientSettings {

    /// Sets the rate limit for the specified API.
    ///
    /// ## Arguments
    ///
    /// * `api` ‧ Which Google Maps API are you setting the rate limit for? For
    /// example, `Api::Directions`, `Api::DistanceMatrix`, `Api::Elevation`,
    /// `Api::Geocoding`, `Api::TimeZone`, and so on. The `Api::All` rate limit
    /// is applied to all Google Maps API requests _in addition_ to the per-API
    /// rate limits.
    ///
    /// * `requests` ‧ Sets the maximum number of requests in the specified
    /// duration. For example, _2 requests_ per 1 hour.
    ///
    /// * `per_duration` ‧ Sets the duration window that the requests should be
    /// measured against. For example, 1 request _per 1 minute_.
    ///
    /// ## Examples:
    ///
    /// * Sets the rate limit for all Google Maps API requests to _2 request per
    /// minute_:
    /// ```rust
    /// .with_rate(Api::All, 2, Duration::from_secs(60)) // 1 minute
    /// ```
    ///
    /// * Sets the rate limit for Google Maps Elevation API requests to _1
    /// requests per second_:
    /// ```rust
    /// .with_rate(Api::All, 1, Duration::from_secs(1)) // 1 second
    /// ```
    ///
    /// * This method can be stacked:
    /// ```rust
    /// .with_rate(Api::All, 1, Duration::from_secs(60)) // 1 minute
    /// .with_rate(Api::Directions, 1, Duration::from_secs(3_600)) // 1 hour
    /// .with_rate(Api::TimeZone, 2, Duration::from_secs(60)) // 1 second
    /// ```

    pub fn with_rate(&mut self, api: Api, requests: u16, per_duration: Duration) -> &mut ClientSettings {
        self.rate_limit.with_rate(api, requests, per_duration);
        self
    } // fn

} // impl