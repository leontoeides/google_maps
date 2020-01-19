use crate::elevation::{
    error::Error,
    request::Request,
}; // use

impl Request {

    /// Builds the query string for the Google Maps Elevation API based on the
    /// input provided by the client.
    ///
    /// # Arguments:
    ///
    /// This method accepts no arguments.

    pub fn build(&mut self) -> Result<&mut Request, Error> {

        // Ensure request has been validated before building the query string:

        if !self.validated { return Err(Error::RequestNotValidated) }

        // This section builds the "required parameters" portion of the query
        // string:

        let mut query = String::from("key=");
        query.push_str(&self.key);

        // This section builds the "positional request" portion of the query
        // string:

        // Locations key/value pair:
        if let Some(locations) = &self.locations {
            query.push_str("&locations=");
            query.push_str(&String::from(locations))
        } // if

        // This section builds the "sampled path request" portion of the query
        // string:

        if let Some(path) = &self.path {
            query.push_str("&path=");
            query.push_str(&String::from(path))
        } // if

        if let Some(samples) = &self.samples {
            query.push_str(&format!("&samples={}", samples));
        } // if

        // Set query string in Request struct.
        self.query = Some(query);

        // Return modified Request struct to caller.
        Ok(self)

    } // fn

} // impl