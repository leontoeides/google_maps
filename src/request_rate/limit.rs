use crate::request_rate::{api::Api, duration_to_string::duration_to_string, RequestRate};
use futures::future;
use std::time::SystemTime;

impl RequestRate {
    /// This method is not for public consumption. It is for internal use only.
    ///
    /// ## Description
    ///
    /// This method performs rate limiting, using the throttler under `rate_map`
    /// specified by the list of apis, which was calculated using targeted
    /// requests/duration rates during initialization. If the current rate
    /// exceeds any of the targeted rate, this method will put the thread to
    /// sleep until it is ready for the next request.
    ///
    /// ## Arguments
    ///
    /// * `apis` ‧ The APIs for which to observe the request rate limit.
    pub async fn limit_apis(&self, apis: Vec<&Api>) {
        let mut limit_futures = Vec::new();
        for (key, val) in &self.rate_map {
            if apis.contains(&key) {
                limit_futures.push(val.limit());
            }
        }
        let start = SystemTime::now();
        future::join_all(limit_futures).await;
        let wait_time = SystemTime::now().duration_since(start);
        if let Ok(duration) = wait_time {
            if duration.as_millis() > 10 {
                tracing::trace!(
                    "waited for {} under rate limiter",
                    duration_to_string(&duration)
                );
            }
        } else {
            tracing::warn!("clock went backwards!");
        }
    }
} // impl
