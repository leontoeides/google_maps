use crate::rate_limit::api::Api;
use crate::rate_limit::api_limit::ApiLimit;
use crate::rate_limit::RateLimit;
use std::time::Duration;

impl RateLimit {

    pub fn with_rate_limit(&mut self, api: Api, request_limit: u16, per_duration: Duration) -> &mut RateLimit {
        // Select `RateLimit` field from the Api specified by the caller.
        let api_ref = match api {
            Api::All => &mut self.all,
            Api::Directions => &mut self.directions,
            Api::DistanceMatrix => &mut self.distance_matrix,
            Api::Elevation => &mut self.elevation,
            Api::Geocoding => &mut self.geocoding,
            Api::TimeZone => &mut self.time_zone,
        }; // api
        // Has the ApiLimit been set already?
        match api_ref {
            // If so, preserve the "first_request" and "total_request_count"
            // fields:
            Some(api_limit) => *api_ref = Some(ApiLimit {
                first_request: api_limit.first_request,
                total_request_count: api_limit.total_request_count,
                request_limit,
                per_duration,
            }), // ApiLimit
            // If not, initialize the structure:
            None => *api_ref = Some(ApiLimit {
                first_request: None,
                total_request_count: 0,
                request_limit,
                per_duration,
            }), // ApiLimit
        } // match
        self
    } // fn

} // impl