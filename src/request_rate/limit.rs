use crate::request_rate::{
    api::Api,
    duration_to_string::duration_to_string,
    RequestRate,
}; // use
use log::{info, trace};
use std::time::Instant;

impl RequestRate {

    /// This method is not for public consumption. It is for internal use only.
    ///
    /// ## Description
    ///
    /// This method does the actual rate limiting. It will look up the actual
    /// effective requests/duration rate and compare it to the targeted
    /// requests/duration rate. If the current rate exceeds the targeted rate,
    /// this method will put the thread to sleep until it is ready for the next
    /// request.
    ///
    /// ## Arguments:
    ///
    /// * `api` ‧ The API for which to observe the request rate limit.

    pub fn limit(&mut self, api: Api) {

        // Select the ApiLimit requested by the caller:
        let api_ref = match api {
            Api::All => &mut self.all,
            Api::Directions => &mut self.directions,
            Api::DistanceMatrix => &mut self.distance_matrix,
            Api::Elevation => &mut self.elevation,
            Api::Geocoding => &mut self.geocoding,
            Api::TimeZone => &mut self.time_zone,
        }; // api

        match *api_ref {

            // No request rate is defined for caller's API, do nothing:
            None => (),

            // There is a request rate defined for the caller's specified API.
            // Compare the current rate to the target rate. Put the thread to
            // sleep if necessary.
            Some(ref mut rate) => {

                match rate.current_rate.first_request {

                    // If this is the first request to the API, initialize the
                    // timer.
                    None => {
                        trace!("Rate limiting is enabled for the `{}` API. First request.", String::from(&api));
                        rate.current_rate.first_request = Some(Instant::now());
                        rate.current_rate.request_count = 1;
                    }, // case

                    // If this is not the first request - calculate the elapsed,
                    // time & current rate, compare against the target rate, and
                    // sleep if necessary:
                    Some(first_request) => {

                        // Calculate the current rate and target rate:
                        let target_rate = rate.target_rate.requests as f64 / rate.target_rate.duration.as_secs_f64();
                        let current_rate = rate.current_rate.request_count as f64 / first_request.elapsed().as_secs_f64();

                        // Output logging information:
                        trace!(
                            "{} requests to the `{}` API this session. This API's session began {} ago.",
                            rate.current_rate.request_count,
                            String::from(&api),
                            duration_to_string(first_request.elapsed())
                        );
                        trace!(
                            "Current rate: {}. Target rate: {}.",
                            rate.current_rate, rate.target_rate
                        );

                        // If the current rate exceeds the targeted rate, put
                        // the thread to sleep:
                        let difference = current_rate - target_rate;
                        if difference > 0.0 {
                            let sleep_duration = std::time::Duration::from_secs(((1.0 / target_rate) + difference).round() as u64);
                            info!("Thread is sleeping for {}.", duration_to_string(sleep_duration));
                            std::thread::sleep(sleep_duration);
                        } // if

                        // Increment the request counter:
                        rate.current_rate.request_count += 1;

                    } // case

                } // match

            } // case

        } // match

    } // fn

} // impl