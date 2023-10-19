use crate::{geocoding::error::Error, geocoding::forward::ForwardRequest};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

impl<'a> ForwardRequest<'a> {
    /// Builds the query string for the Google Maps Geocoding API based on the
    /// input provided by the client.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn build(&mut self) -> Result<&'a mut ForwardRequest, Error> {
        // Ensure request has been validated before building the query string:

        if !self.validated {
            return Err(Error::RequestNotValidated);
        }

        // This section builds the "required parameters" portion of the query
        // string:

        let mut query = format!("key={}", self.client.key);

        // This section builds the "optional parameters" portion of the query
        // string:

        // Address key/value pair:
        if let Some(address) = &self.address {
            query.push_str("&address=");
            query.push_str(
                &utf8_percent_encode(&String::from(address), NON_ALPHANUMERIC).to_string(),
            )
        }

        // Place Id key/value pair:
        if let Some(place_id) = &self.place_id {
            query.push_str("&place_id=");
            query.push_str(
                &utf8_percent_encode(&String::from(place_id), NON_ALPHANUMERIC).to_string(),
            )
        }

        // Bounds key/value pair:
        if let Some(bounds) = &self.bounds {
            query.push_str("&bounds=");
            query
                .push_str(&utf8_percent_encode(&String::from(bounds), NON_ALPHANUMERIC).to_string())
        }

        // Components key/value pair:
        if let Some(components) = &self.components {
            query.push_str("&components=");
            query.push_str(
                &utf8_percent_encode(
                    &components
                        .iter()
                        .map(String::from)
                        .collect::<Vec<String>>()
                        .join("|"),
                    NON_ALPHANUMERIC,
                )
                .to_string(),
            ) // push_str
        }

        // Language key/value pair:
        if let Some(language) = &self.language {
            query.push_str("&language=");
            query.push_str(&String::from(language))
        }

        // Region key/value pair:
        if let Some(region) = &self.region {
            query.push_str("&region=");
            query.push_str(&String::from(region))
        }

        // Set query string in ForwardRequest struct.
        self.query = Some(query);

        // Return modified ForwardRequest struct to caller.
        Ok(self)
    } // fn
} // impl
