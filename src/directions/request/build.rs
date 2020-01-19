use crate::directions::request::Request;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

impl Request {

    /// Builds the query string for the Google Maps Directions API based on the
    /// input provided by the client.

    pub fn build(&mut self) -> &mut Request {

        // Builds the "required parameters" portion of the query string:
        let mut query = format!(
            "origin={}&destination={}&key={}",
            String::from(&self.origin), // URL-encoding performed by From trait
            String::from(&self.destination), // URL-encoding performed by From trait
            self.key
        ); // format!

        // Builds the "optional parameters" portion of the query string:
        // Travel mode key/value pair:
        if let Some(travel_mode) = &self.travel_mode {
            query.push_str("&mode=");
            query.push_str(&String::from(travel_mode))
        } // if

        // Waypoints key/value pair:
        if let Some(waypoints) = &self.waypoints {
            query.push_str("&waypoints=");
            query.push_str(&*utf8_percent_encode(
                &String::from(waypoints.iter().map(|waypoint| String::from(waypoint) + "|").collect::<String>().trim_end_matches('|')),
                NON_ALPHANUMERIC
            ).to_string())
        } // if

        // Alternatives key/value pair:
        if let Some(alternatives) = &self.alternatives {
            query.push_str("&alternatives=");
            query.push_str(&alternatives.to_string())
        } // if

        // Avoid key/value pair:
        if let Some(restrictions) = &self.restrictions {
            query.push_str("&avoid=");
            query.push_str(&*utf8_percent_encode(
                &String::from(restrictions.iter().map(|avoid| String::from(avoid) + "|").collect::<String>().trim_end_matches('|')),
                NON_ALPHANUMERIC
            ).to_string())
        } // if

        // Language key/value pair:
        if let Some(language) = &self.language {
            query.push_str("&language=");
            query.push_str(&String::from(language))
        }

        // Unit system key/value pair:
        if let Some(unit_system) = &self.unit_system {
            query.push_str("&units=");
            query.push_str(&String::from(unit_system))
        }

        // Region key/value pair:
        if let Some(region) = &self.region {
            query.push_str("&region=");
            query.push_str(&String::from(region))
        }

        // Arrival time key/value pair:
        if let Some(arrival_time) = &self.arrival_time { query.push_str(&format!("&arrival_time={}", arrival_time.timestamp())); }

        // Departure time key/value pair:
        if let Some(departure_time) = &self.departure_time {
            query.push_str("&departure_time=");
            query.push_str(&String::from(departure_time))
        }

        // Traffic model key/value pair:
        if let Some(traffic_model) = &self.traffic_model {
            query.push_str("&traffic_model=");
            query.push_str(&String::from(traffic_model))
        }

        // Transit mode key/value pair:
        if let Some(transit_modes) = &self.transit_modes {
            query.push_str("&transit_mode=");
            query.push_str(&*utf8_percent_encode(
                &String::from(transit_modes.iter().map(|mode| String::from(mode) + "|").collect::<String>().trim_end_matches('|')),
                NON_ALPHANUMERIC
            ).to_string())
        } // if

        // Transit route preference key/value pair:
        if let Some(transit_route_preference) = &self.transit_route_preference {
            query.push_str("&transit_routing_preference=");
            query.push_str(&String::from(transit_route_preference))
        } // if

        // Commit the query string to the structure:
        self.query = Some(query);

        // Return the structure:
        self

    } // fn

} // impl