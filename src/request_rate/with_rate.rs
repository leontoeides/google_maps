use crate::request_rate::{
    api::Api,
    api_rate::ApiRate,
    current_rate::CurrentRate,
    RequestRate,
    target_rate::TargetRate,
}; // use
use std::time::Duration;

impl RequestRate {

    /// Specifies the request rate for the selected API.
    ///
    /// ## Arguments:
    ///
    /// * `api` ‧ API on which to apply the request rate limit. The argument may
    /// be `Api::All` which is evaluated _in addition_ to each of the per-API
    /// (i.e. `Api::Directions` or `Api::Elevation`) request rate limits.
    ///
    /// * `requests` ‧ The number of requests the client library is attempting
    /// to target.
    ///
    /// * `duration` ‧ The duration for the targeted request rate. For example
    /// this field may define _requests_ per _second_, per _minute_, per _day_,
    /// and so on. This can be defined using the `std::time::Duration` methods.
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
    /// API. You are billed for this SKU when your request does not use traffic
    /// information, arrival or departure times, < 10 waypoints, and no waypoint
    /// optimization, $0.005 USD per request.
    /// ```rust
    /// .with_rate(Api::Directions, (GOOGLE_CREDIT / 0.005) as u16, from_secs(SECONDS_PER_MONTH))
    /// ```
    ///
    /// * [Directions Advanced](https://developers.google.com/maps/billing/gmp-billing#directions-advanced)
    /// API. You are billed for this SKU when your request requires include
    /// traffic information, > 10 waypoints, and/or waypoint optimization, $0.01
    /// per request.
    /// ```rust
    /// .with_rate(Api::Directions, (GOOGLE_CREDIT / 0.01) as u16, from_secs(SECONDS_PER_MONTH))
    /// ```
    ///
    /// * [Distance Matrix](https://developers.google.com/maps/billing/gmp-billing#distance-matrix)
    /// API. You are billed for this SKU when your requests _does not_ require
    /// traffic information, $0.005 per _element_. **The below rate assumes an
    /// average of 10 elements per request.**
    /// ```rust
    /// .with_rate(
    ///     Api::Directions,
    ///     (GOOGLE_CREDIT / (0.005 * 10.0)) as u16,
    ///     from_secs(SECONDS_PER_MONTH)
    /// )
    /// ```
    ///
    /// * [Distance Matrix
    /// Advanced](https://developers.google.com/maps/billing/gmp-billing#distance-matrix-advanced)
    /// API. You are billed for this SKU when your requests require traffic
    /// information, $0.01 USD per _element_. **The below rate assumes an average
    /// of 10 elements per request.**
    /// ```rust
    /// .with_rate(
    ///     Api::DistanceMatrix,
    ///     (GOOGLE_CREDIT / (0.01 * 10.0)) as u16,
    ///     from_secs(SECONDS_PER_MONTH)
    /// )
    /// ```
    ///
    /// * [Elevation](https://developers.google.com/maps/billing/gmp-billing#elevation)
    /// API. $0.005 USD per request.
    /// ```rust
    /// .with_rate(Api::Elevation, (GOOGLE_CREDIT / 0.005) as u16, from_secs(SECONDS_PER_MONTH))
    /// ```
    ///
    /// * [Geocoding](https://developers.google.com/maps/billing/gmp-billing#geolocation)
    /// API. $0.005 USD per request.
    /// ```rust
    /// .with_rate(Api::Geocoding, (GOOGLE_CREDIT / 0.005) as u16, from_secs(SECONDS_PER_MONTH))
    /// ```
    ///
    /// * [Time
    /// Zone](https://developers.google.com/maps/billing/gmp-billing#time-zone)
    /// API. $0.005 USD per request.
    /// ```rust
    /// .with_rate(Api::TimeZone, (GOOGLE_CREDIT / 0.005) as u16, from_secs(SECONDS_PER_MONTH))
    /// ```

    pub fn with_rate(&mut self, api: Api, requests: u16, duration: Duration) -> &mut RequestRate {
        // Select `RequestRate` field for the API specified by the caller.
        let api_ref = match api {
            Api::All => &mut self.all,
            Api::Directions => &mut self.directions,
            Api::DistanceMatrix => &mut self.distance_matrix,
            Api::Elevation => &mut self.elevation,
            Api::Geocoding => &mut self.geocoding,
            Api::TimeZone => &mut self.time_zone,
        }; // api
        // Has the ApiRate been set already?
        match api_ref {
            // If not, initialize the structure:
            None => *api_ref = Some(ApiRate{
                target_rate: TargetRate { requests, duration, },
                current_rate: CurrentRate::default(),
            }),
            // If it has, set the new target request rate but preserve the
            // current effective request rate:
            Some(api_rate) => *api_ref = Some(ApiRate {
                // Set new target request rate:
                target_rate: TargetRate { requests, duration, },
                // Copy old actual request rate:
                current_rate: api_rate.current_rate.clone(),
            }), // ApiRate
        } // match
        self
    } // fn

} // impl