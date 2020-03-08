use crate::directions::{
    error::Error,
    request::Request,
    travel_mode::TravelMode,
}; // use

impl<'a> Request<'a> {

    /// Ensures the built query is valid. This function checks the combination
    /// of parameters to ensure that they make sense together and that Google
    /// Maps Directions API will accept them - i.e. it will not allow both a
    /// arrival time and departure time in the same query. This function does
    /// not check parameter values for validity - i.e. it will not ensure
    /// Polylines or Place ID's are valid and well-formed.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn validate(&'a mut self) -> Result<&'a mut Request, Error> {

        if let Some(travel_mode) = &self.travel_mode {

            // If the travel mode has been set to TravelMode::Transit...
            if *travel_mode == TravelMode::Transit {

                // ...waypoints cannot be set:
                if let Some(waypoints) = &self.waypoints {
                    return Err(Error::EitherWaypointsOrTransitMode(waypoints.len()))
                } // if

            // If the transit mode is not set to TravelMode::Transit...
            } else {

                // ...an arrival time cannot be set:
                if let Some(arrival_time) = &self.arrival_time {
                    return Err(Error::ArrivalTimeIsForTransitOnly(
                        travel_mode.to_string(),
                        arrival_time.format("%F %r").to_string()
                    )) // Err
                } // if

                // ...a transit mode cannot be set:
                if let Some(transit_modes) = &self.transit_modes {
                    return Err(Error::TransitModeIsForTransitOnly(
                        travel_mode.to_string(),
                        String::from(transit_modes.iter().map(|mode| mode.to_string() + "|").collect::<String>().trim_end_matches('|'))
                    )) // Err
                } // if

                // ...a transit route preference cannot be set:
                if let Some(transit_route_preference) = &self.transit_route_preference {
                    return Err(Error::TransitRoutePreferenceIsForTransitOnly(
                        travel_mode.to_string(),
                        transit_route_preference.to_string(),
                    )) // Err
                } // if

            } // if

        } // if

        // If waypoints have been set...
        if let Some(waypoints) = &self.waypoints {

            // ...alternatives cannot be set to true:
            if let Some(alternatives) = &self.alternatives {
                if !alternatives {
                    return Err(Error::EitherAlternativesOrWaypoints(
                        waypoints.len(),
                    )) // Err
                } // if
            } // if

            // ...restrictions cannot be set:
            if let Some(restrictions) = &self.restrictions {
                return Err(Error::EitherRestrictionsOrWaypoints(
                    waypoints.len(),
                    String::from(restrictions.iter().map(|avoid| avoid.to_string() + "|").collect::<String>().trim_end_matches('|')),
                )) // Err
            } // if

            // ...ensure that the number of waypoints is equal to or less than 25:
            if waypoints.len() > 25 {
                return Err(Error::TooManyWaypoints(waypoints.len()))
            } // if

        } // if

        // If an arrival time has been set...
        if let Some(arrival_time) = &self.arrival_time {

            // ...a departure time cannot be set:
            if let Some(departure_time) = &self.departure_time {
                return Err(Error::EitherDepartureTimeOrArrivalTime(
                    arrival_time.format("%F %r").to_string(),
                    departure_time.to_string(),
                )) // Err
            } // if

        } // if

        // Indicate that the request passed validation.
        self.validated = true;

        // Return modified Request struct to caller.
        Ok(self)

    } // fn

} // impl