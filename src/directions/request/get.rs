use crate::directions::{
    error::Error,
    request::Request,
    response::Response,
    response::status::Status,
}; // use
use log::{info, warn};

impl Request {

    /// Performs the HTTP get request and returns the response to the caller.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn get(&mut self) -> Result<Response, Error> {

        // Build the URL stem for the HTTP get request:

        const SERVICE_URL: &str = "https://maps.googleapis.com/maps/api/directions";
        const OUTPUT_FORMAT: &str = "json"; // json or xml
        let mut uri = format!("{}/{}?", SERVICE_URL, OUTPUT_FORMAT);

        match &self.query {
            // If query string built, append it to the URL stem.
            Some(query) => uri.push_str(query.as_ref()),
            // If query string not built, return an error.
            None => return Err(Error::QueryNotBuilt),
        } // match

        info!("HTTP GET: {}", uri);

        // Initialize variables:
        let mut counter = 0;
        let mut wait_time_in_ms = 0;
        // Retries the get request until successful, an error ineligible for
        // retries is returned, or we have reached the maximum retries:
        loop {
            // Increment retry counter:
            counter += 1;
            // Query the Google Cloud Maps Platform using using an HTTP get
            // request, and return result to caller:
            let response = reqwest::blocking::get(&*uri);
            // Check response from the HTTP client:
            match response {
                Ok(response) => {
                    // HTTP client was successful getting a response from the
                    // server. Check the HTTP status code:
                    if response.status().is_success() {
                        // If the HTTP GET request was successful, deserialize
                        // the JSON response into a Rust data structure:
                        let deserialized: Response = serde_json::from_str(&response.text()?)?;
                        // If the response JSON was successfully parsed, check
                        // the Google API status before returning it to the
                        // caller:
                        if deserialized.status == Status::Ok {
                            // If Google's response was "Ok" return the
                            // struct deserialized from JSON:
                            return Ok(deserialized)
                        } else if
                            // Only Google's "Unknown Error" is eligible for
                            // retries
                            deserialized.status != Status::UnknownError ||
                            counter >= self.client_settings.maximum_retries {
                                // If there is a Google API error other than
                                // "Unknown Error," return the error and do not
                                // retry the request. Also, if we have reached
                                // the maximum retry count, do not retry
                                // further:
                                let error = Error::GoogleMapsService(deserialized.status, deserialized.error_message);
                                warn!("{}", error);
                                return Err(error)
                        } // if
                    // We got a response from the server but it was not success:
                    } else if
                        !response.status().is_server_error() || // Only HTTP "500 Server Errors", and
                        response.status() != 429 || // HTTP "429 Too Many Requests" are eligible for retries.
                        counter >= self.client_settings.maximum_retries {
                            // If the HTTP request error was not a 500 Server
                            // error or "429 Too Many Requests" error, return
                            // the error and do not retry the request. Also, if
                            // we have reached the maximum retry count, do not
                            // retry anymore:
                            warn!("HTTP client returned: `{}`.", response.status());
                            return Err(Error::HttpUnsuccessful(counter, response.status().to_string()))
                    } // if
                } // case
                Err(response) => {
                    // HTTP client did not get a response from the server:
                    warn!("HTTP client returned: `{}`", response);
                    if counter >= self.client_settings.maximum_retries {
                        // If we have reached the maximum retry count, do not
                        // retry anymore. Return the last HTTP client error:
                        return Err(Error::Reqwest(response))
                    } // if
                } // case
            }; // match

            // Truncated exponential backoff is a standard error handling
            // strategy for network applications in which a client periodically
            // retries a failed request with increasing delays between requests.
            if wait_time_in_ms < self.client_settings.maximum_backoff {
                // Wait Time = 2^N + Random
                //
                // A random number of milliseconds less than or equal to 255.
                // This helps to avoid cases where many clients get synchronized
                // by some situation and all retry at once, sending requests in
                // synchronized waves.
                wait_time_in_ms = 2_u32.pow(counter as u32) + (rand::random::<u8>() as u32);
                // Maximum backoff is typically 32 or 64 seconds. The
                // appropriate value depends on the use case.
                //
                // It's okay to continue retrying once you reach the
                // maximum_backoff time. Retries after this point do not need to
                // continue increasing backoff time. For example, if a client
                // uses an maximum_backoff time of 64 seconds, then after
                // reaching this value, the client can retry every 64 seconds.
                // At some point, clients should be prevented from retrying
                // infinitely.
                if wait_time_in_ms > self.client_settings.maximum_backoff {
                    wait_time_in_ms = self.client_settings.maximum_backoff
                } // if
            } // if

            info!("Could not successfully query the Google Maps Platform. Retry #{} of {}. Sleeping for {} milliseconds before retrying.", counter, self.client_settings.maximum_retries, wait_time_in_ms);

            std::thread::sleep(std::time::Duration::from_millis(wait_time_in_ms as u64));

        }; // loop

    } // fn

} // impl