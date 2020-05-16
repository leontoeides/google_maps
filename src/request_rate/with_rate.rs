use crate::request_rate::{
    api::Api, api_rate::ApiRate, current_rate::CurrentRate,
    target_rate::TargetRate, RequestRate,
}; // use crate::request_rate
use std::time::Duration;

impl RequestRate {

    /// Specifies the request rate for the selected API. _Do not use this method
    /// to set request rate limits, use `ClientSettings.with_rate()` instead_.
    ///
    /// ## Arguments:
    ///
    /// * `api` ‧ Which Google Maps API are you setting the rate limit for? For
    /// example, `Api::Directions`, `Api::DistanceMatrix`, `Api::Elevation`,
    /// `Api::Geocoding`, `Api::TimeZone`, and so on. The `Api::All` rate limit
    /// is applied to all Google Maps API requests _in addition_ to the per-API
    /// rate limits.
    ///
    /// * `requests` ‧ The number of requests the client library is attempting
    /// to target. For example, _2 requests_ per 1 hour.
    ///
    /// * `duration` ‧ The duration for the targeted request rate. For example,
    /// 1 request _per 1 minute_. This can be defined using the
    /// `std::time::Duration` methods.
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

    pub fn with_rate(
        &mut self,
        api: &Api,
        requests: u16,
        duration: Duration
    ) -> &mut RequestRate {

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
            None => {
                *api_ref = Some(ApiRate {
                    target_rate: TargetRate { requests, duration },
                    current_rate: CurrentRate::default(),
                })
            }
            // If it has, set the new target request rate but preserve the
            // current effective request rate:
            Some(api_rate) => {
                *api_ref = Some(ApiRate {
                    // Set new target request rate:
                    target_rate: TargetRate { requests, duration },

                    // Copy old actual request rate:
                    current_rate: api_rate.current_rate.clone(),
                })
            } // ApiRate
        } // match

        self

    } // fn

} // impl