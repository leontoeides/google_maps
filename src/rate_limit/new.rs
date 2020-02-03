use crate::rate_limit::RateLimit;

impl RateLimit {

    pub fn new() -> RateLimit {
        RateLimit {
            all: None,
            directions: None,
            distance_matrix: None,
            elevation: None,
            geocoding: None,
            time_zone: None,
        } // RateLimit
    } // fn

} // impl