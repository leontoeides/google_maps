use crate::distance_matrix::{error::Error, request::Request};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

impl<'a> Request<'a> {

    /// Builds the query string for the Google Maps Directions API based on the
    /// input provided by the client.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn build(&mut self) -> Result<&'a mut Request, Error> {

        // Ensure request has been validated before building the query string:

        if !self.validated {
            return Err(Error::RequestNotValidated);
        }

        // Builds the "required parameters" portion of the query string:

        let mut query = format!(
            "key={}&origins={}&destinations={}",
            // Key:
            self.client_settings.key,
            // Origins:
            utf8_percent_encode(
                &self.origins
                    .iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
                    .join("|"),
                NON_ALPHANUMERIC
            ),
            // Destinations:
            utf8_percent_encode(
                &self.destinations
                    .iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
                    .join("|"),
                NON_ALPHANUMERIC
            ),
        ); // format!

        // Builds the "optional parameters" portion of the query string:

        // Arrival time key/value pair:
        if let Some(arrival_time) = &self.arrival_time {
            query.push_str("&arrival_time=");
            query.push_str(&arrival_time.timestamp().to_string());
        } // if

        // Avoid key/value pair:
        if let Some(restrictions) = &self.restrictions {
            query.push_str("&avoid=");
            query.push_str(
                &*utf8_percent_encode(
                    &restrictions
                        .iter()
                        .map(String::from)
                        .collect::<Vec<String>>()
                        .join("|"),
                    NON_ALPHANUMERIC,
                ).to_string(),
            ) // push_str
        } // if

        // Departure time key/value pair:
        if let Some(departure_time) = &self.departure_time {
            query.push_str("&departure_time=");
            query.push_str(&String::from(departure_time))
        } // if

        // Language key/value pair:
        if let Some(language) = &self.language {
            query.push_str("&language=");
            query.push_str(&String::from(language))
        } // if

        // Travel mode key/value pair:
        if let Some(travel_mode) = &self.travel_mode {
            query.push_str("&mode=");
            query.push_str(&String::from(travel_mode).to_lowercase())
        } // if

        // Region key/value pair:
        if let Some(region) = &self.region {
            query.push_str("&region=");
            query.push_str(&String::from(region))
        } // if

        // Traffic model key/value pair:
        if let Some(traffic_model) = &self.traffic_model {
            query.push_str("&traffic_model=");
            query.push_str(&String::from(traffic_model))
        } // if

        // Transit mode key/value pair:
        if let Some(transit_modes) = &self.transit_modes {
            query.push_str("&transit_mode=");
            query.push_str(
                &*utf8_percent_encode(
                    &transit_modes
                        .iter()
                        .map(String::from)
                        .collect::<Vec<String>>()
                        .join("|"),
                    NON_ALPHANUMERIC,
                ).to_string(),
            ) // push_str
        } // if

        // Transit route preference key/value pair:
        if let Some(transit_route_preference) = &self.transit_route_preference {
            query.push_str("&transit_routing_preference=");
            query.push_str(&String::from(transit_route_preference))
        } // if

        // Unit system key/value pair:
        if let Some(unit_system) = &self.unit_system {
            query.push_str("&units=");
            query.push_str(&String::from(unit_system))
        } // if

        // Set query string in Request struct.
        self.query = Some(query);

        // Return modified Request struct to caller.
        Ok(self)

    } // fn

} // impl