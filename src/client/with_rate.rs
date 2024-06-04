use crate::{client::GoogleMapsClient, request_rate::api::Api};
use std::time::Duration;

// =============================================================================

impl GoogleMapsClient {
    // -------------------------------------------------------------------------
    //
    /// Sets the rate limit for the specified API.
    ///
    /// ## Arguments
    ///
    /// * `api` ‧ Which Google Maps API are you setting the rate limit for? For
    ///   example, `Api::Directions`, `Api::DistanceMatrix`, `Api::Elevation`,
    ///   `Api::Geocoding`, `Api::TimeZone`, and so on. The `Api::All` rate
    ///   limit is applied to all Google Maps API requests _in addition_ to the
    ///   per-API rate limits.
    ///
    /// * `requests` ‧ The number of requests the client library is attempting
    ///   to target. For example, _2 requests_ per 1 hour.
    ///
    /// * `duration` ‧ The duration for the targeted request rate. For example,
    ///   1 request _per 1 minute_. This can be defined using the
    ///   `std::time::Duration` methods.
    ///
    /// ## Examples:
    ///
    /// The following examples show how one might try to limit the request
    /// rate to achieve maximum throughput while minimizing charges by Google.
    ///
    /// **The following rates are subject to change by Google. Please review
    /// the current [Google Maps Platform billing
    /// rates](https://developers.google.com/maps/billing/gmp-billing).**
    ///
    /// **This Google client library's rate limiting is not persistent. If your
    /// program is often restarted, it is easily possible to exceed Google's
    /// monthly free credit. These are approximations and examples.**
    ///
    /// **To accurately minimize billing charges by Google, please use the
    /// [Google Cloud Platform Console](https://console.cloud.google.com/)
    /// _IAM & admin_ to set quotas for each API on the server's side.**
    ///
    /// You are responsible for all charges. Use with care.
    ///
    /// ```rust
    /// // Assumptions:
    /// // - $200.00 USD monthly free credit from Google. Thanks, guys!
    /// // - 2,629,746 seconds in a month.
    /// const GOOGLE_CREDIT: f64 = 200.0;
    /// const SECONDS_PER_MONTH: u64 = 2_629_746;
    /// ```
    ///
    /// * [Directions](https://developers.google.com/maps/billing/gmp-billing#directions)
    ///   API. You are billed for this SKU when your request does not use live
    ///   traffic information, arrival or departure times, < 10 waypoints, and
    ///   no waypoint optimization, $0.005 USD per request.
    /// ```rust
    /// with_rate(Api::Directions, (GOOGLE_CREDIT / 0.005) as u16, Duration::from_secs(SECONDS_PER_MONTH))
    /// ```
    ///
    /// * [Directions Advanced](https://developers.google.com/maps/billing/gmp-billing#directions-advanced)
    ///   API. You are billed for this SKU when your request requires live
    ///   traffic information, > 10 waypoints, and/or waypoint optimization, $0.01
    ///   per request.
    /// ```rust
    /// with_rate(Api::Directions, (GOOGLE_CREDIT / 0.01) as u16, Duration::from_secs(SECONDS_PER_MONTH))
    /// ```
    ///
    /// * [Distance Matrix](https://developers.google.com/maps/billing/gmp-billing#distance-matrix)
    ///   API. You are billed for this SKU when your requests _do not_ require
    ///   live traffic information, $0.005 per _element_. **The below rate
    ///   assumes an average of 10 elements per request.**
    /// ```rust
    /// with_rate(
    ///     Api::DistanceMatrix,
    ///     (GOOGLE_CREDIT / (0.005 * 10.0)) as u16,
    ///     Duration::from_secs(SECONDS_PER_MONTH)
    /// )
    /// ```
    ///
    /// * [Distance Matrix
    ///   Advanced](https://developers.google.com/maps/billing/gmp-billing#distance-matrix-advanced)
    ///   API. You are billed for this SKU when your requests require live
    ///   traffic information, $0.01 USD per _element_. **The below rate assumes
    ///   an average of 10 elements per request.**
    /// ```rust
    /// with_rate(
    ///     Api::DistanceMatrix,
    ///     (GOOGLE_CREDIT / (0.01 * 10.0)) as u16,
    ///     Duration::from_secs(SECONDS_PER_MONTH)
    /// )
    /// ```
    ///
    /// * [Elevation](https://developers.google.com/maps/billing/gmp-billing#elevation)
    ///   API. $0.005 USD per request.
    /// ```rust
    /// with_rate(Api::Elevation, (GOOGLE_CREDIT / 0.005) as u16, Duration::from_secs(SECONDS_PER_MONTH))
    /// ```
    ///
    /// * [Geocoding](https://developers.google.com/maps/billing/gmp-billing#geolocation)
    ///   API. $0.005 USD per request.
    /// ```rust
    /// with_rate(Api::Geocoding, (GOOGLE_CREDIT / 0.005) as u16, Duration::from_secs(SECONDS_PER_MONTH))
    /// ```
    ///
    /// * [Time
    ///   Zone](https://developers.google.com/maps/billing/gmp-billing#time-zone)
    ///   API. $0.005 USD per request.
    /// ```rust
    /// with_rate(Api::TimeZone, (GOOGLE_CREDIT / 0.005) as u16, Duration::from_secs(SECONDS_PER_MONTH))
    /// ```

    pub fn with_rate(&mut self, api: &Api, requests: u16, per_duration: Duration) -> &mut Self {
        self.rate_limit.with_rate(api, requests, per_duration);
        self
    } // fn
} // impl
