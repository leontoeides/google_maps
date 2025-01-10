use crate::request_rate::api_rate::ApiRate; // use crate::request_rate

impl ApiRate {
    /// This method is not for public consumption. It is for internal use only.
    ///
    /// ## Description
    ///
    /// This method does the actual rate limiting for an API.
    /// If the current rate exceeds the targeted rate,
    /// this method will put the thread to sleep until it is ready for the next
    /// request.
    pub async fn limit(&self) {
        if let Some(wait_pool) = &self.throttle_pool { wait_pool.queue().await }
        /*
        match self.current_rate.first_request {
            // If this is the first request to the API, initialize the
            // timer.
            None => {
                // For some reason this trace! macro can cause a stack
                // overflow, so it has been commented out for now:
                /* trace!("Rate limiting is enabled for the `{}` API. First request.", api.to_string()); */
                self.current_rate.first_request = Some(Instant::now());
                self.current_rate.request_count = 1;

                //let throttle_rate = ThrottleRate::new(self.target_rate.requests as usize,  self.target_rate.duration);
                //self.throttle_pool = Some(ThrottlePool::new(throttle_rate));
            } // case

            // If this is not the first request - calculate the elapsed,
            // time & current rate, compare against the target rate, and
            // sleep if necessary:
            Some(_first_request) => {
                // Output logging information:
                // For some reason these trace! macros can cause a
                // stack overflow, so they have been commented out for
                // now:
                /* trace!(
                    "{} requests to the `{}` API this session. This API's session began {} ago.",
                    rate.current_rate.request_count,
                    api.to_string(),
                    duration_to_string(&first_request.elapsed())
                ); */
                /* trace!(
                    "Current rate: {}. Target rate: {}.",
                    rate.current_rate, rate.target_rate
                ); */

                // Calculate the current rate and target rate:
                //let target_rate = rate.target_rate.requests as f64
                //    / rate.target_rate.duration.as_secs_f64();
                //let current_rate = rate.current_rate.request_count as f64
                //    / first_request.elapsed().as_secs_f64();

                // If the current rate exceeds the targeted rate, put
                // the thread to sleep:
                /*
                let difference = current_rate - target_rate;
                if difference > 0.0 {
                    let sleep_duration = std::time::Duration::from_secs(
                        ((1.0 / target_rate) + difference).round() as u64,
                    );
                    info!(
                        "Thread is sleeping for {}.",
                        duration_to_string(&sleep_duration)
                    );
                    std::thread::sleep(sleep_duration);
                } // if

                 */
                // wait for throttle
                match &self.throttle_pool {
                    Some(wait_pool) => wait_pool.queue().await,
                    None => ()
                }

                // Increment the request counter:
                self.current_rate.request_count += 1;
            } // case
        } // match
        */
    } // fn
} // impl
