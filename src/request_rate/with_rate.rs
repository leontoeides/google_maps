use crate::request_rate::{api::Api, api_rate::ApiRate, target_rate::TargetRate, RequestRate}; // use crate::request_rate
use std::time::Duration;
use stream_throttle::{ThrottlePool, ThrottleRate};

// =============================================================================

impl RequestRate {
    // -------------------------------------------------------------------------
    //
    /// Specifies the request rate for the selected API. _Do not use this method
    /// to set request rate limits, use `ClientSettings.with_rate()` instead_.
    ///
    /// ## Arguments
    ///
    /// * `api` ‧ Which Google Maps API are you setting the rate limit for? For
    ///   example, `Api::Directions`, `Api::DistanceMatrix`, `Api::Elevation`,
    ///   `Api::Geocoding`, `Api::TimeZone`, and so on. The `Api::All` rate limit
    ///   is applied to all Google Maps API requests _in addition_ to the per-API
    ///   rate limits.
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
    /// * Sets the rate limit for all Google Maps API requests to _2 request per
    ///   minute_:
    /// ```rust
    /// with_rate(Api::All, 2, Duration::from_secs(60)) // 1 minute
    /// ```
    ///
    /// * Sets the rate limit for Google Maps Elevation API requests to _1
    ///   requests per second_:
    /// ```rust
    /// with_rate(Api::All, 1, Duration::from_secs(1)) // 1 second
    /// ```
    ///
    /// * This method can be stacked:
    /// ```rust
    /// with_rate(Api::All, 1, Duration::from_secs(60)) // 1 minute
    /// with_rate(Api::Directions, 1, Duration::from_secs(3_600)) // 1 hour
    /// with_rate(Api::TimeZone, 2, Duration::from_secs(60)) // 1 second
    /// ```

    pub fn with_rate(&mut self, api: &Api, requests: u16, duration: Duration) -> &mut Self {
        // Select `RequestRate` field for the API specified by the caller.
        let api_ref = self.rate_map.get_mut(api);
        let throttle_pool = if requests == 0 {
            None
        } else {
            let throttle_rate = ThrottleRate::new(requests as usize, duration);
            Some(ThrottlePool::new(throttle_rate))
        };

        // Has the ApiRate been set already?
        match api_ref {
            // If not, initialize the structure:
            None => {
                self.rate_map.insert(
                    api.clone(),
                    ApiRate {
                        target_rate: TargetRate { requests, duration },
                        throttle_pool,
                    },
                );
            }
            // If it has, set the new target request rate but preserve the
            // current effective request rate:
            Some(api_rate) => {
                *api_rate = ApiRate {
                    // Set new target request rate:
                    target_rate: TargetRate { requests, duration },
                    throttle_pool,
                };
            } // ApiRate
        } // match

        self
    } // fn
} // impl
