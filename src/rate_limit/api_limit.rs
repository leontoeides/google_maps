use std::time::{Duration, Instant};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ApiLimit {
    pub first_request: Option<Instant>,
    pub total_request_count: u64,
    pub request_limit: u16,
    pub per_duration: Duration,
} // struct