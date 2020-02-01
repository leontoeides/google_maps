use crate::client_settings::ClientSettings;
use crate::rate_limiting::Api;
use time::Duration;

impl ClientSettings {

    /// Sets the rate limit for the specified API.
    ///
    /// ## Arguments
    ///
    /// * `api` ‧ Which Google Maps API are you setting the rate limit for? For
    /// example, Api::Directions, Api::DistanceMatrix, Api::Elevation,
    /// Api::Geocoding, Api::TimeZone, and so on. The `Api::All` rate limit is
    /// applied to all Google Maps API requests _in addition_ to the per-API
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
    /// .with_rate_limit(Api::All, 2, Duration::minutes(1))
    /// ```
    ///
    /// * Sets the rate limit for Google Maps Elevation API requests to _1
    /// requests per second_:
    /// ```rust
    /// .with_rate_limit(Api::Elevation, 1, Duration::seconds(1))
    /// ```
    ///
    /// * This method can be stacked:
    /// ```rust
    /// .with_rate_limit(Api::All, 1, Duration::seconds(1))
    /// .with_rate_limit(Api::Directions, 1, Duration::hours(1))
    /// .with_rate_limit(Api::TimeZone, 2, Duration::hours(1))
    /// ```

    pub fn with_rate_limit(&mut self, api: Api, requests: u16, per_duration: Duration) -> &mut ClientSettings {
        self.rate_limit.with_rate_limit(api, requests, per_duration);
        self
    } // fn

} // impl