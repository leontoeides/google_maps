use crate::request_rate::{
    api::Api,
    duration_to_string::duration_to_string,
    RequestRate
};
use std::time::Instant;

// -----------------------------------------------------------------------------
//
/// Minimum wait time worth logging. Waits shorter than this are considered
/// negligible and skipped to reduce log noise.
const LOG_THRESHOLD_MS: u128 = 10;

// -----------------------------------------------------------------------------

impl RequestRate {
    /// Enforces rate limits for the specified APIs.
    ///
    /// Checks each API against its configured rate limiter and sleeps if
    /// necessary to avoid exceeding the allowed request rate. All rate
    /// limiters run concurrently, so the total wait time is determined by
    /// whichever API needs the longest delay.
    pub async fn limit_apis(&self, apis: &[Api]) {
        let limit_futures: Vec<_> = self
            .rate_map
            .iter()
            .filter(|(key, _)| apis.contains(key))
            .map(|(_, val)| val.limit())
            .collect();

        let start = Instant::now();
        futures::future::join_all(limit_futures).await;
        let duration = start.elapsed();

        if duration.as_millis() > LOG_THRESHOLD_MS {
            tracing::trace!(
                wait_duration_ms = duration.as_millis(),
                wait_duration = %duration_to_string(&duration),
                "rate limiter throttled request"
            );
        }
    }
}