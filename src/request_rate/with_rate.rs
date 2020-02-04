use crate::request_rate::api::Api;
use crate::request_rate::api_rate::ApiRate;
use crate::request_rate::current_rate::CurrentRate;
use crate::request_rate::RequestRate;
use crate::request_rate::target_rate::TargetRate;
use std::time::Duration;

impl RequestRate {

    pub fn with_rate(&mut self, api: Api, requests: u16, per_duration: Duration) -> &mut RequestRate {
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
                target_rate: TargetRate { requests, per_duration, },
                current_rate: CurrentRate::default(),
            }),
            // If it has, set the new target request rate but preserve the
            // current effective request rate:
            Some(api_rate) => *api_ref = Some(ApiRate {
                // Set new target request rate:
                target_rate: TargetRate { requests, per_duration, },
                // Copy old actual request rate:
                current_rate: api_rate.current_rate.clone(),
            }), // ApiRate
        } // match
        self
    } // fn

} // impl